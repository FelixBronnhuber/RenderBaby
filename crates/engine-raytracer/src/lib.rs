use std::sync::{Arc, Mutex};

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
    gpu_wrapper: Option<Arc<Mutex<GpuWrapper>>>,
}

impl Engine {
    pub fn new(rc: RenderConfig) -> Self {
        let wrapper = GpuWrapper::new(rc, "engine-raytracer/src/shader.wgsl").unwrap();
        Self {
            gpu_wrapper: Some(Arc::new(Mutex::new(wrapper))),
        }
    }
}

impl Renderer for Engine {
    fn render(&mut self, rc: RenderConfig) -> Result<RenderOutput> {
        let wrapper = self
            .gpu_wrapper
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Wrapper not available"))?
            .clone(); // Clone the Arc (cheap)

        // Update config
        {
            let mut w = wrapper.lock().unwrap();
            w.update(rc)?;
            w.update_uniforms();
        }

        let buffer = FrameBuffer::new();
        buffer.provide(GpuWrapper::render_progressive(wrapper.clone())?);

        let mut last_frame = RenderOutput::new(100, 100, vec![1]);

        while buffer.is_running() {
            if let Some(recv) = buffer.try_recv() {
                if let Ok(frame) = recv {
                    println!("Frame received {} x {}", frame.width, frame.height);
                    last_frame = RenderOutput::new(frame.width, frame.height, frame.pixels);
                } else {
                    println!("out");
                    break;
                }
            }
        }

        println!("Finished");
        Ok(last_frame)
    }
}
