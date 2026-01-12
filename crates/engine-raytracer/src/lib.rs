use anyhow::Result;
pub use engine_config::RenderConfig;
use engine_config::Renderer;
use engine_wgpu_wrapper::{GpuWrapper};
use frame_buffer::frame_iterator::{FrameIterator, Frame};
use std::time::Instant;
use chrono::Local;

use std::sync::{Arc, Mutex};

pub struct Engine {
    gpu_wrapper: Arc<Mutex<GpuWrapper>>,
}

impl Renderer for Engine {
    fn render(&mut self, rc: RenderConfig) -> Result<Frame> {
        let mut gpu_wrapper = self.gpu_wrapper.lock().unwrap();

        gpu_wrapper.update(rc)?;
        gpu_wrapper.update_uniforms();
        gpu_wrapper.dispatch_compute()?;
        let pixels = gpu_wrapper.read_pixels()?;

        Ok(Frame::new(
            gpu_wrapper.get_width() as usize,
            gpu_wrapper.get_height() as usize,
            pixels,
        ))
    }

    fn frame_iterator(&mut self, rc: RenderConfig) -> Result<Box<dyn FrameIterator>> {
        {
            let mut gpu_wrapper = self.gpu_wrapper.lock().unwrap();
            gpu_wrapper.update(rc)?;
            gpu_wrapper.update_uniforms();
            gpu_wrapper.prh_mut().current_pass = 0;
        }
        Ok(Box::new(RaytracerFrameIterator::new(Arc::clone(
            &self.gpu_wrapper,
        ))))
    }
}

impl Engine {
    pub fn new(rc: RenderConfig) -> Self {
        let wrapper = GpuWrapper::new(rc, "engine-raytracer/src/shader.wgsl").unwrap();
        Self {
            gpu_wrapper: Arc::new(Mutex::new(wrapper)),
        }
    }
}

pub struct RaytracerFrameIterator {
    gpu_wrapper: Arc<Mutex<GpuWrapper>>,
    initialized: bool,
    render_time: Option<Instant>,
}

impl RaytracerFrameIterator {
    fn new(gpu_wrapper: Arc<Mutex<GpuWrapper>>) -> Self {
        Self {
            gpu_wrapper,
            initialized: false,
            render_time: None,
        }
    }
}

impl FrameIterator for RaytracerFrameIterator {
    fn has_next(&self) -> bool {
        let wrapper = self.gpu_wrapper.lock().unwrap();
        wrapper.prh().current_pass < wrapper.prh().total_passes
    }

    fn next(&mut self) -> Result<Frame> {
        if !self.has_next() {
            // Stop render time
            if let Some(timer) = self.render_time {
                let duration = timer.elapsed();
                log::info!("Render finished in {:?}", duration);
            }
            anyhow::bail!("No more frames available");
        }

        let mut gpu_wrapper = self.gpu_wrapper.lock().unwrap();

        if !self.initialized {
            // Start render time
            self.render_time = Some(Instant::now());
            log::info!("Render started at {}", Local::now());

            gpu_wrapper.queue().write_buffer(
                &gpu_wrapper.buffer_wrapper().accumulation,
                0,
                &vec![0u8; (gpu_wrapper.get_width() * gpu_wrapper.get_height() * 16) as usize],
            );
            self.initialized = true;
        }

        gpu_wrapper.queue().write_buffer(
            &gpu_wrapper.buffer_wrapper().progressive_render,
            0,
            bytemuck::cast_slice(&[*gpu_wrapper.prh()]),
        );

        gpu_wrapper.dispatch_compute_progressive(
            gpu_wrapper.prh().current_pass,
            gpu_wrapper.prh().total_passes,
        )?;

        let pixels = gpu_wrapper.read_pixels()?;

        let frame = Frame::new(
            gpu_wrapper.get_width() as usize,
            gpu_wrapper.get_height() as usize,
            pixels,
        );

        gpu_wrapper.prh_mut().current_pass += 1;

        log::info!(
            "[ENGINE-RAYTRACER] Sample: {} / {}",
            gpu_wrapper.prh().current_pass,
            gpu_wrapper.prh().total_passes,
        );

        if gpu_wrapper.prh().current_pass == gpu_wrapper.prh().total_passes
            && let Some(timer) = self.render_time
        {
            let duration = timer.elapsed();
            log::info!("[ENGINE-RAYTRACER] Render finished in {:?}", duration);
        }
        Ok(frame)
    }

    fn destroy(&mut self) {
        log::info!("[ENGINE-RAYTRACER] Cancelled Render Iterator.")
    }
}
