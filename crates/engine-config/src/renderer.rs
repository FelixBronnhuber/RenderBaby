//! Renderer trait definitions for the RenderBaby system.
//!
//! This module defines the core [`Renderer`] trait that all rendering engines must implement.
//! It provides both synchronous (blocking) and progressive (iterative) rendering modes.

use anyhow::Result;
use crate::RenderConfig;

use frame_buffer::frame_iterator::{Frame, FrameIterator};

/// Core rendering interface for all RenderBaby engines.
///
/// The `Renderer` trait defines the contract that all rendering backends must fulfill.
/// It supports two rendering modes:
///
/// - **Synchronous Rendering**: [`render`](Renderer::render) blocks until the entire image is complete.
/// - **Progressive Rendering**: [`frame_iterator`](Renderer::frame_iterator) returns an iterator
///   that yields partial results, allowing for interactive preview and incremental refinement.
///
/// # Thread Safety
///
/// Implementors must be `Send` to support multi-threaded rendering workflows.
///
/// # Usage Example
///
/// ```rust,ignore
/// use engine_config::{Renderer, RenderConfig};
///
/// fn render_scene(renderer: &mut impl Renderer, config: RenderConfig) -> anyhow::Result<()> {
///     // Option 1: Synchronous rendering
///     let final_frame = renderer.render(config.clone())?;
///     
///     // Option 2: Progressive rendering
///     let mut iter = renderer.frame_iterator(config)?;
///     while iter.has_next() {
///         let partial_frame = iter.next()?;
///         // Display or process partial result
///     }
///     
///     Ok(())
/// }
/// ```
pub trait Renderer: Send {
    /// Renders a scene synchronously and returns the final frame.
    ///
    /// This method blocks until the entire rendering process is complete. It computes
    /// all samples and returns the fully converged result.
    ///
    /// # Arguments
    ///
    /// * `rc` - The render configuration containing scene description and settings
    ///
    /// # Returns
    ///
    /// * `Result<Frame>` - The rendered frame with pixel data, or an error if rendering fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - GPU resources fail to initialize
    /// - The render configuration is invalid
    /// - GPU operations fail during rendering
    fn render(&mut self, rc: RenderConfig) -> Result<Frame>;

    /// Creates a frame iterator for progressive rendering.
    ///
    /// This method initializes the rendering session and returns an iterator that yields
    /// frames incrementally. Each call to `next()` on the iterator computes additional
    /// samples and returns the current accumulated result.
    ///
    /// This is particularly useful for:
    /// - Interactive preview while rendering
    /// - Showing progress in real-time
    /// - Allowing early termination if the result is satisfactory
    ///
    /// # Arguments
    ///
    /// * `rc` - The render configuration containing scene description and settings
    ///
    /// # Returns
    ///
    /// * `Result<Box<dyn FrameIterator>>` - A boxed iterator yielding frames progressively,
    ///   or an error if initialization fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - GPU resources fail to initialize
    /// - The render configuration is invalid
    fn frame_iterator(&mut self, rc: RenderConfig) -> Result<Box<dyn FrameIterator>>;
}

/// Alternative renderer trait for iterative rendering workflows.
///
/// `RendererIterable` provides a similar interface to [`Renderer`] but with method names
/// that may be more intuitive in certain contexts. This trait is currently experimental
/// and may be unified with [`Renderer`] in the future.
///
/// # Note
///
/// This trait is functionally equivalent to [`Renderer`] and is primarily provided for
/// API compatibility and naming preferences.
pub trait RendererIterable {
    /// Renders a scene synchronously and returns the final frame.
    ///
    /// See [`Renderer::render`] for details.
    fn render(&mut self, rc: RenderConfig) -> Result<Frame>;

    /// Creates and returns a frame iterator for progressive rendering.
    ///
    /// See [`Renderer::frame_iterator`] for details.
    fn get_frame_iterator(&mut self, rc: RenderConfig) -> Result<Box<dyn FrameIterator>>;
}
