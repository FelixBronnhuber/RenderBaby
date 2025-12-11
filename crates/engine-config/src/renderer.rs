use anyhow::Result;
use crate::RenderConfig;

use crate::render_output::RenderOutput;

pub trait Renderer: Send {
    fn render(&mut self, rc: RenderConfig) -> Result<RenderOutput>;
}
