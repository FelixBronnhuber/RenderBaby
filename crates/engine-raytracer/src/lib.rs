use anyhow::Result;
pub use engine_config::RenderConfig;
use engine_config::Renderer;
use engine_wgpu_wrapper::{GpuWrapper};
use frame_buffer::frame_iterator::{FrameIterator, Frame};
use log::info;

pub struct Engine {
    gpu_wrapper: GpuWrapper,
}

impl Renderer for Engine {
    fn render(&mut self, rc: RenderConfig) -> Result<Frame> {
        self.gpu_wrapper.update(rc)?;
        self.gpu_wrapper.update_uniforms();
        self.gpu_wrapper.dispatch_compute()?;
        let pixels = self.gpu_wrapper.read_pixels()?;

        Ok(Frame::new(
            self.gpu_wrapper.get_width() as usize,
            self.gpu_wrapper.get_height() as usize,
            pixels,
        ))
    }

    fn frame_iterator(&mut self, rc: RenderConfig) -> Result<Box<dyn FrameIterator>> {
        let mut gpu_wrapper = GpuWrapper::new(rc.clone(), "engine-raytracer/src/shader.wgsl")?;
        gpu_wrapper.update(rc)?;
        gpu_wrapper.update_uniforms();
        Ok(Box::new(RaytracerFrameIterator::new(gpu_wrapper)))
    }
}

impl Engine {
    pub fn new(rc: RenderConfig) -> Self {
        let wrapper = GpuWrapper::new(rc, "engine-raytracer/src/shader.wgsl").unwrap();
        Self {
            gpu_wrapper: wrapper,
        }
    }
}

pub struct RaytracerFrameIterator {
    gpu_wrapper: GpuWrapper,
    initialized: bool,
}

impl RaytracerFrameIterator {
    fn new(gpu_wrapper: GpuWrapper) -> Self {
        Self {
            gpu_wrapper,
            initialized: false,
        }
    }
}

impl FrameIterator for RaytracerFrameIterator {
    fn has_next(&self) -> bool {
        self.gpu_wrapper.prh().current_pass < self.gpu_wrapper.prh().total_passes
    }

    fn next(&mut self) -> Result<Frame> {
        if !self.has_next() {
            anyhow::bail!("No more frames available");
        }

        if !self.initialized {
            self.gpu_wrapper.queue().write_buffer(
                &self.gpu_wrapper.buffer_wrapper().accumulation,
                0,
                &vec![
                    0u8;
                    (self.gpu_wrapper.get_width() * self.gpu_wrapper.get_height() * 16) as usize
                ],
            );
            self.initialized = true;
        }

        self.gpu_wrapper.queue().write_buffer(
            &self.gpu_wrapper.buffer_wrapper().progressive_render,
            0,
            bytemuck::cast_slice(&[*self.gpu_wrapper.prh()]),
        );

        self.gpu_wrapper.dispatch_compute_progressive(
            self.gpu_wrapper.prh().current_pass,
            self.gpu_wrapper.prh().total_passes,
        )?;

        let pixels = self.gpu_wrapper.read_pixels()?;

        let frame = Frame::new(
            self.gpu_wrapper.get_width() as usize,
            self.gpu_wrapper.get_height() as usize,
            pixels,
        );

        self.gpu_wrapper.prh_mut().current_pass += 1;

        info!(
            "PASSED Sample: {}{}",
            self.gpu_wrapper.prh().current_pass,
            if self.gpu_wrapper.prh().current_pass == self.gpu_wrapper.prh().total_passes {
                ", finished rendering!"
            } else {
                ""
            }
        );

        Ok(frame)
    }

    fn destroy(&mut self) {
        info!("Cancelled Render Iterator.")
        // todo
    }
}
