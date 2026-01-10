use anyhow::Result;
use crate::RenderConfig;

use crate::render_output::RenderOutput;
use frame_buffer::frame_iterator::{Frame, FrameIterator};

pub trait Renderer: Send {
    fn render(&mut self, rc: RenderConfig) -> Result<RenderOutput>;
    fn frame_iterator(&mut self, rc: RenderConfig) -> Result<Box<dyn FrameIterator>>;
}

pub trait RendererIterable {
    fn render(&mut self, rc: RenderConfig) -> Result<Frame>;
    fn get_frame_iterator(&mut self, rc: RenderConfig) -> Result<Box<dyn FrameIterator>>;
}
