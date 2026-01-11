use anyhow::Result;
pub use engine_config::RenderConfig;
use engine_config::Renderer;
use engine_wgpu_wrapper::GpuWrapper;
use frame_buffer::frame_iterator::{Frame, FrameIterator};

#[allow(dead_code)]
pub struct Engine {
    gpu_wrapper: GpuWrapper,
}

impl Renderer for Engine {
    fn render(&mut self, rc: RenderConfig) -> Result<Frame> {
        self.render(rc)
    }

    fn frame_iterator(&mut self, _rc: RenderConfig) -> Result<Box<dyn FrameIterator>> {
        todo!()
    }
}

impl Engine {
    pub fn new(rc: RenderConfig) -> Self {
        let wrapper = GpuWrapper::new(rc, "engine-pathtracer/src/shader.wgsl").unwrap();

        Self {
            gpu_wrapper: wrapper,
        }
    }

    pub fn render(&mut self, _rc: RenderConfig) -> Result<Frame> {
        todo!()
    }
}
