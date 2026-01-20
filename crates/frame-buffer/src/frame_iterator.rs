/// A Frame is a single image rendered by the render engine.
#[derive(Debug, Clone)]
pub struct Frame {
    /// Width of the image in pixels.
    pub width: usize,
    /// Height of the image in pixels.
    pub height: usize,
    /// Pixels of the image as RGBA8 data.
    pub pixels: Vec<u8>, // RGBA8 data
}

impl Frame {
    /// Creates a new [`Frame`].
    pub fn new(width: usize, height: usize, pixels: Vec<u8>) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }

    /// Returns the size of the image in bytes.
    pub fn expected_size(&self) -> usize {
        self.width * self.height * 4
    }

    /// Validates that the pixel data matches the expected size.
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

/// Trait for objects that can provide a sequence of [`Frame`]s.
pub trait FrameIterator: Send + 'static {
    /// Returns `true` if there are more frames to be rendered.
    fn has_next(&self) -> bool;

    /// Returns the next [`Frame`] in the sequence.
    fn next(&mut self) -> anyhow::Result<Frame>;

    /// Destroys/deletes the iterator.
    fn destroy(&mut self); // this deletes the iterator
}
