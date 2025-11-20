use anyhow::Result;
pub use engine_config::RenderConfig;
use engine_wgpu_wrapper::{GpuWrapper, RenderOutput, Renderer};

#[allow(dead_code)]
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
        let wrapper = GpuWrapper::new(rc, "engine-pathtracer/shader.wgsl").unwrap();

        Self {
            gpu_wrapper: wrapper,
        }
    }

    pub fn render(&mut self, _rc: RenderConfig) -> Result<RenderOutput> {
        todo!()
    }
}
