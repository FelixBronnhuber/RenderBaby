use anyhow::Result;
pub use engine_config::RenderConfig;
use engine_config::{RenderOutput, Renderer};
use engine_wgpu_wrapper::{GpuWrapper};

use std::fs::File;
use std::io::BufWriter;
use frame_buffer::frame_provider::Frame;

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

        // Test progressive rendering
        let mut receiver = wrapper.render_progressive()?;

        log::info!("Starting progressive render test...");
        let mut frames = Vec::new();
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
            frames.push(frame);
        }
        log::info!(
            "Progressive render complete: {} frames received",
            frame_count
        );

        for (i, frame) in frames.iter().enumerate() {
            let path = format!("tmp/progressive_frame_{:03}.png", i);
            if let Err(e) = save_frame_as_png(&path, frame) {
                log::warn!("Failed to save frame {}: {:?}", i, e);
            } else {
                log::info!("Saved frame to {}", path);
            }
        }

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

//copied code
fn save_frame_as_png(path: &str, frame: &Frame) -> Result<()> {
    let file = File::create(path)?;
    let buf_writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(buf_writer, frame.width as u32, frame.height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(&frame.pixels)?;

    Ok(())
}
