#[derive(Debug, Clone)]
pub struct Frame {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>, // RGBA8 data
}

impl Frame {
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
                "Frame pixel size mismatch: expected {} bytes, got {}",
                expected,
                self.pixels.len()
            );
        }
        Ok(())
    }
}

pub trait FrameIterator: Send + 'static {
    fn has_next(&self) -> bool;
    fn next(&mut self) -> anyhow::Result<Frame>;
    fn destroy(&mut self); // this deletes the iterator
}
