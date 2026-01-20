//! Unified rendering engine wrapper.
//!
//! This module provides the [`Engine`] struct, which serves as a unified interface
//! for all rendering backends in the RenderBaby system. It wraps individual engine
//! implementations (raytracer, pathtracer) and provides a consistent API with
//! additional features like automatic timing, logging, and runtime engine switching.

use anyhow::Result;
use engine_config::{Renderer, RenderConfig};
use engine_config::renderer::RendererIterable;
use frame_buffer::frame_iterator::{Frame, FrameIterator};
use std::time::Instant;
use chrono::Local;
use crate::compute_plane::render_engine::RenderEngine;

/// Unified rendering engine that wraps backend-specific implementations.
///
/// `Engine` provides a high-level interface for rendering operations by wrapping
/// any implementation of the [`Renderer`] trait. It adds functionality such as:
/// - Automatic timing and performance logging
/// - Runtime switching between rendering backends
/// - Unified error handling and logging
/// - Consistent API regardless of underlying engine
///
/// # Architecture
///
/// The `Engine` uses dynamic dispatch via `Box<dyn Renderer + Sync>` to support
/// multiple rendering backends. The current backend is selected at construction time
/// via the [`RenderEngine`] enum and can be changed at runtime using `switch_engine`.
///
/// # Thread Safety
///
/// The wrapped renderer must implement both `Renderer` and `Sync`, allowing the
/// engine to be used in multi-threaded contexts.
///
/// # Example
///
/// ```rust,ignore
/// let mut engine = Engine::new(renderconfig, RenderEngine::Raytracer);
///
/// // Render synchronously with automatic timing
/// let frame = engine.render(config)?;
/// ```
pub struct Engine {
    /// The wrapped renderer implementation (raytracer or pathtracer).
    renderer: Box<dyn Renderer + Sync>,
    /// The type of the current rendering engine.
    engine_type: RenderEngine,
}

impl RendererIterable for Engine {
    /// Renders a scene synchronously using the frame iterator with automatic timing.
    ///
    /// This implementation of [`RendererIterable::render`] provides automatic timing
    /// and logging by internally using the frame iterator. It:
    /// 1. Logs the render start time
    /// 2. Creates a frame iterator
    /// 3. Consumes all frames until completion
    /// 4. Logs the total duration
    /// 5. Returns the final frame
    ///
    /// # Arguments
    ///
    /// * `rc` - The render configuration containing scene description and settings
    ///
    /// # Returns
    ///
    /// * `Ok(Frame)` - The final rendered frame with all samples computed
    /// * `Err(_)` - An error if rendering fails at any stage
    ///
    /// # Logging
    ///
    /// Produces INFO-level logs:
    /// - "Render started at [timestamp]"
    /// - "Render finished in [duration]"
    fn render(&mut self, rc: RenderConfig) -> Result<Frame> {
        log::info!("Render started at {}", Local::now());
        let start = Instant::now();

        let mut frame_iterator = self.get_frame_iterator(rc)?;
        let mut last_frame: Frame = Frame::new(0, 0, vec![]);
        loop {
            if frame_iterator.has_next() {
                last_frame = frame_iterator.next()?;
            } else {
                break;
            }
        }

        let duration = start.elapsed();
        log::info!("Render finished in {:?}", duration);
        Ok(last_frame)
    }

    /// Creates a frame iterator for progressive rendering.
    ///
    /// Delegates to the underlying renderer's [`Renderer::frame_iterator`] method.
    ///
    /// # Arguments
    ///
    /// * `rc` - The render configuration containing scene description and settings
    ///
    /// # Returns
    ///
    /// * `Ok(Box<dyn FrameIterator>)` - An iterator yielding progressive frames
    /// * `Err(_)` - An error if iterator creation fails
    fn get_frame_iterator(&mut self, rc: RenderConfig) -> Result<Box<dyn FrameIterator>> {
        self.renderer.frame_iterator(rc)
    }
}

impl Engine {
    /// Creates a new rendering engine with the specified backend.
    ///
    /// Initializes the appropriate rendering backend (raytracer or pathtracer) based
    /// on the `engine_type` parameter. The backend is initialized with the provided
    /// configuration.
    ///
    /// # Arguments
    ///
    /// * `rc` - Initial render configuration for engine initialization
    /// * `engine_type` - The type of rendering engine to create
    ///
    /// # Returns
    ///
    /// A new `Engine` instance wrapping the selected renderer.
    pub fn new(rc: RenderConfig, engine_type: RenderEngine) -> Self {
        let renderer: Box<dyn Renderer + Sync> = match engine_type {
            RenderEngine::Raytracer => Box::new(engine_raytracer::Engine::new(rc)),
            RenderEngine::Pathtracer => Box::new(engine_pathtracer::Engine::new(rc)),
        };

        Self {
            renderer,
            engine_type,
        }
    }

    /// Renders a scene synchronously by delegating directly to the underlying renderer.
    ///
    /// This method provides an alternative to the [`RendererIterable::render`] implementation.
    /// It directly calls the underlying renderer's [`Renderer::render`] method with
    /// automatic timing and logging.
    ///
    /// # Arguments
    ///
    /// * `rc` - The render configuration containing scene description and settings
    ///
    /// # Returns
    ///
    /// * `Ok(Frame)` - The final rendered frame
    /// * `Err(_)` - An error if rendering fails
    ///
    /// # Logging
    ///
    /// Produces INFO-level logs:
    /// - "Render started at [timestamp]"
    /// - "Render finished in [duration]"
    pub fn render(&mut self, rc: RenderConfig) -> Result<Frame> {
        log::info!("Render started at {}", Local::now());
        let start = Instant::now();

        let result = self.renderer.render(rc);

        let duration = start.elapsed();
        log::info!("Render finished in {:?}", duration);

        result
    }

    /// Switches the rendering backend at runtime.
    ///
    /// Destroys the current renderer and creates a new one of the specified type.
    /// The new renderer is initialized with the provided configuration.
    ///
    /// # Arguments
    ///
    /// * `rc` - Configuration to initialize the new renderer
    /// * `engine_type` - The type of rendering engine to switch to
    ///
    /// # Performance Note
    ///
    /// This operation destroys the current renderer and all its GPU resources,
    /// then allocates new ones. It is not intended for frequent switching.
    #[allow(dead_code)]
    pub fn switch_engine(&mut self, rc: RenderConfig, engine_type: RenderEngine) {
        self.renderer = match engine_type {
            RenderEngine::Raytracer => Box::new(engine_raytracer::Engine::new(rc)),
            RenderEngine::Pathtracer => Box::new(engine_pathtracer::Engine::new(rc)),
        };
        self.engine_type = engine_type;
    }

    /// Returns the type of the currently active rendering engine.
    ///
    /// # Returns
    ///
    /// The [`RenderEngine`] variant representing the current backend.
    #[allow(dead_code)]
    pub fn current_engine(&self) -> RenderEngine {
        self.engine_type
    }
}
