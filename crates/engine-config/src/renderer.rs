use anyhow::Result;
use crate::RenderConfig;

use crate::render_output::RenderOutput;

pub trait Renderer {
    fn render(&mut self, rc: RenderConfig) -> Result<RenderOutput>;
}
