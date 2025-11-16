use anyhow::Result;
pub use engine_config::RenderConfig;
use engine_core::{RenderOutput, Renderer};
use engine_wgpu_wrapper::GpuWrapper;

pub struct Engine {
    gpu_wrapper: GpuWrapper,
}

impl Renderer for Engine {
    fn render(&mut self, rc: RenderConfig) -> Result<RenderOutput> {
        self.render(rc)
    }
}

impl Engine {
    pub fn new(rc: RenderConfig) -> Self {
        let wrapper = GpuWrapper::new(rc).unwrap();

        Self {
            gpu_wrapper: wrapper,
        }
    }

    pub fn render(&mut self, rc: RenderConfig) -> Result<RenderOutput> {
        self.gpu_wrapper.update(rc);

        self.gpu_wrapper.update_uniforms();

        self.gpu_wrapper.dispatch_compute()?;

        let pixels = self.gpu_wrapper.read_pixels()?;

        Ok(RenderOutput::new(
            self.gpu_wrapper.get_width() as usize,
            self.gpu_wrapper.get_height() as usize,
            pixels,
        ))
    }
}
