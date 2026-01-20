//! # Engine Raytracer
//!
//! `engine-raytracer` provides a placeholder for a wgpu-based ray tracing implementation.
//! This crate is not currently implemented in the production system.
//!
//! ## Status
//!
//! **Not Implemented**: This crate exists as a placeholder for future development.
//! The pathtracer (`engine-pathtracer`) is the primary rendering backend.

use anyhow::Result;
pub use engine_config::RenderConfig;
use engine_config::Renderer;
use engine_wgpu_wrapper::GpuWrapper;
use frame_buffer::frame_iterator::{Frame, FrameIterator};

/// A placeholder ray tracing renderer.
///
/// This engine is not currently implemented. Use `engine-pathtracer` instead.
#[allow(dead_code)]
pub struct Engine {
    /// GPU wrapper for future implementation.
    gpu_wrapper: GpuWrapper,
}

impl Renderer for Engine {
    /// Not implemented.
    fn render(&mut self, rc: RenderConfig) -> Result<Frame> {
        self.render(rc)
    }

    /// Not implemented.
    fn frame_iterator(&mut self, _rc: RenderConfig) -> Result<Box<dyn FrameIterator>> {
        todo!()
    }
}

impl Engine {
    /// Creates a new placeholder engine instance.
    ///
    /// # Arguments
    ///
    /// * `rc` - Initial render configuration
    pub fn new(rc: RenderConfig) -> Self {
        let wrapper = GpuWrapper::new(rc, "engine-pathtracer/src/shader.wgsl").unwrap();

        Self {
            gpu_wrapper: wrapper,
        }
    }

    /// Not implemented.
    pub fn render(&mut self, _rc: RenderConfig) -> Result<Frame> {
        todo!()
    }
}
