use anyhow::Result;
use engine_config::RenderConfig;

pub trait Renderer {
    fn render(&mut self, rc: RenderConfig) -> Result<RenderOutput>;
}

#[derive(Debug, Clone)]
pub struct RenderOutput {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>, // RGBA8 data
}

impl RenderOutput {
    pub fn new(width: usize, height: usize, pixels: Vec<u8>) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn expected_size(&self) -> usize {
        self.width * self.height * 4
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        let expected = self.expected_size();
        if self.pixels.len() != expected {
            anyhow::bail!(
                "RenderOutput pixel size mismatch: expected {} bytes, got {}",
                expected,
                self.pixels.len()
            );
        }
        Ok(())
    }
}
