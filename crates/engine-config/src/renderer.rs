//! Renderer trait definitions for the RenderBaby system.
//!
//! This module defines the core rendering interfaces that all rendering engines must implement.
//! It provides both synchronous (blocking) and progressive (iterative) rendering modes through
//! two complementary traits: [`Renderer`] and [`RendererIterable`].
//!
//! ### Synchronous Rendering
//!
//! The [`render`](Renderer::render) method blocks until the entire image is complete,
//! executing all passes sequentially and returning the final result.
//!
//! ### Progressive Rendering
//!
//! The [`frame_iterator`](Renderer::frame_iterator) method returns an iterator that yields
//! partial results incrementally. This enables:
//! - Interactive preview while rendering
//! - Real-time progress monitoring
//! - Early termination if results are satisfactory
//! - Responsive UI during long renders

use anyhow::Result;
use crate::RenderConfig;

use frame_buffer::frame_iterator::{Frame, FrameIterator};

/// Core rendering interface for all RenderBaby rendering backends.
///
/// The `Renderer` trait defines the low-level contract that all rendering engines
/// must fulfill. It provides both synchronous and progressive
/// rendering capabilities.
///
/// # Thread Safety
///
/// Implementors must be `Send` to support multi-threaded rendering workflows.
pub trait Renderer: Send {
    /// Renders a scene synchronously and returns the final frame.
    ///
    /// This method blocks until the entire rendering process is complete. It computes
    /// all samples across all passes and returns the fully converged result.
    ///
    /// # Arguments
    ///
    /// * `rc` - The render configuration containing scene description and rendering settings
    ///
    /// # Returns
    ///
    /// * `Ok(Frame)` - The rendered frame with complete pixel data
    /// * `Err(_)` - An error if rendering fails
    fn render(&mut self, rc: RenderConfig) -> Result<Frame>;

    /// Creates a frame iterator for progressive rendering.
    ///
    /// This method initializes a rendering session and returns an iterator that yields
    /// frames incrementally. Each call to `next()` on the iterator computes additional
    /// samples and returns the current accumulated result.
    ///
    /// # Arguments
    ///
    /// * `rc` - The render configuration containing scene description and settings
    ///
    /// # Returns
    ///
    /// * `Ok(Box<dyn FrameIterator>)` - An iterator yielding frames progressively
    /// * `Err(_)` - An error if initialization fails
    fn frame_iterator(&mut self, rc: RenderConfig) -> Result<Box<dyn FrameIterator>>;
}

/// High-level renderer interface with additional convenience methods.
///
/// `RendererIterable` extends the basic [`Renderer`] trait with higher-level rendering
/// methods that provide automatic timing, logging, and iterator management. This trait
/// is primarily implemented by the unified `Engine` wrapper.
///
/// # Relationship to Renderer
///
/// While [`Renderer`] is the low-level interface implemented by rendering backends,
/// `RendererIterable` is the high-level interface used by applications. The unified
/// `Engine` implements `RendererIterable` by wrapping a `Box<dyn Renderer>`.
pub trait RendererIterable {
    /// Renders a scene synchronously.
    ///
    /// This method internally creates a frame iterator, consumes all frames,
    /// and returns the final result.
    ///
    /// # Implementation Note
    ///
    /// # Arguments
    ///
    /// * `rc` - The render configuration containing scene description and settings
    fn render(&mut self, rc: RenderConfig) -> Result<Frame>;

    /// Creates a frame iterator for progressive rendering.
    ///
    /// Returns an iterator that yields frames incrementally, allowing for
    /// interactive preview and progress monitoring.
    ///
    /// # Arguments
    ///
    /// * `rc` - The render configuration containing scene description and settings
    ///
    /// # Returns
    ///
    /// * `Ok(Box<dyn FrameIterator>)` - An iterator yielding progressive frames
    /// * `Err(_)` - An error if initialization fails
    fn get_frame_iterator(&mut self, rc: RenderConfig) -> Result<Box<dyn FrameIterator>>;
}
