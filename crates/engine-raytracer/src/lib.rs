use anyhow::Result;
pub use engine_config::RenderConfig;
use engine_config::{RenderOutput, Renderer};
use engine_wgpu_wrapper::{GpuWrapper};

// pub struct Engine {
//     gpu_wrapper: GpuWrapper,
// }

// impl Renderer for Engine {
//     fn render(&mut self, rc: RenderConfig) -> Result<RenderOutput> {
//         self.gpu_wrapper.update(rc)?;

//         self.gpu_wrapper.update_uniforms();

//         self.gpu_wrapper.dispatch_compute()?;

//         let pixels = self.gpu_wrapper.read_pixels()?;

//         Ok(RenderOutput::new(
//             self.gpu_wrapper.get_width() as usize,
//             self.gpu_wrapper.get_height() as usize,
//             pixels,
//         ))
//     }
// }

// impl Engine {
//     pub fn new(rc: RenderConfig) -> Self {
//         let wrapper = GpuWrapper::new(rc, "engine-raytracer/src/shader.wgsl").unwrap();

//         Self {
//             gpu_wrapper: wrapper,
//         }
//     }
// }

//current test impl

pub struct Engine {
    gpu_wrapper: Option<GpuWrapper>,
}

impl Renderer for Engine {
    fn render(&mut self, rc: RenderConfig) -> Result<RenderOutput> {
        let mut wrapper = self
            .gpu_wrapper
            .take()
            .ok_or_else(|| anyhow::anyhow!("Wrapper already consumed"))?;

        wrapper.update(rc)?;
        wrapper.update_uniforms();

        let mut receiver = wrapper.render_progressive()?;

        log::info!("Starting progressive render test...");
        let mut frame_count = 0;
        while let Ok(frame) = receiver.next() {
            frame_count += 1;
            log::info!(
                "Frame {}: {}x{} ({} bytes)",
                frame_count,
                frame.width,
                frame.height,
                frame.pixels.len()
            );
        }
        log::info!(
            "Progressive render complete: {} frames received",
            frame_count
        );

        Ok(RenderOutput::new(100, 100, vec![1]))
    }
}

impl Engine {
    pub fn new(rc: RenderConfig) -> Self {
        let wrapper = GpuWrapper::new(rc, "engine-raytracer/src/shader.wgsl").unwrap();
        Self {
            gpu_wrapper: Some(wrapper),
        }
    }
}
