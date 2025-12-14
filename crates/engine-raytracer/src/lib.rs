use anyhow::Result;
pub use engine_config::RenderConfig;
use engine_config::{RenderOutput, Renderer};
use engine_wgpu_wrapper::{GpuWrapper};

use frame_buffer::frame_buffer::FrameBuffer;

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
        // Take ownership of the wrapper
        let mut wrapper = self
            .gpu_wrapper
            .take()
            .ok_or_else(|| anyhow::anyhow!("Wrapper already consumed"))?;

        // Update with new config
        wrapper.update(rc)?;
        wrapper.update_uniforms();

        let buffer = FrameBuffer::new();

        buffer.provide(wrapper.render_progressive()?);

        let mut last_frame = RenderOutput::new(100, 100, [1].to_vec());

        while buffer.is_running() {
            if let Some(Ok(recv)) = buffer.try_recv() {
                println!("Frame recieved {} x {}", recv.width, recv.height);
                last_frame = RenderOutput::new(recv.width, recv.height, recv.pixels);
                // buffer.stop_current_provider();
            }
        }

        print!("Finished");

        Ok(last_frame)
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
