//! # Engine Raytracer
//!
//! `engine-raytracer` provides a wgpu-based ray tracing implementation for the RenderBaby project.
//! It extends the capabilities of `engine-wgpu-wrapper` to perform compute-shader-based ray tracing.
//!
//! ## Features
//!
//! - **GPU Acceleration**: Utilizes `wgpu` for hardware-accelerated ray tracing via compute shaders.
//! - **Progressive Rendering**: Supports progressive rendering through [`RaytracerFrameIterator`], allowing for interactive updates and improved image quality over time.
//! - **Scene Support**: Handles complex scenes with:
//!   - Spheres and Triangle Meshes (accelerated via BVH).
//!   - Point Lights and Global Illumination.
//!   - Materials with texture mapping.
//!
//! ## Architecture
//!
//! This crate implements the [`Renderer`] trait from `engine_config`. It encapsulates a
//! [`GpuWrapper`] which manages the underlying GPU resources (buffers, bind groups, pipelines).
//! The core ray tracing logic resides in the associated WGSL shader (`shader.wgsl`).
//!
//! Path Tracer Module
#![doc = include_str!("shader_docs.md")]

use anyhow::Result;
pub use engine_config::RenderConfig;
use engine_config::Renderer;
use engine_wgpu_wrapper::{GpuWrapper};
use frame_buffer::frame_iterator::{FrameIterator, Frame};
use std::time::Instant;
use chrono::Local;

use std::sync::{Arc, Mutex};

/// A wgpu-based ray tracing renderer.
///
/// The `Engine` struct serves as the main entry point for the ray tracing backend.
/// It holds a thread-safe reference to the [`GpuWrapper`] and implements the [`Renderer`] trait
/// to integrate with the rest of the application.
pub struct Engine {
    /// Shared access to the GPU wrapper, managing device, queue, and resources.
    gpu_wrapper: Arc<Mutex<GpuWrapper>>,
}

impl Renderer for Engine {
    /// Renders a scene synchronously.
    ///
    /// This method blocks until the entire rendering process (all passes) is complete.
    /// It updates the GPU resources with the provided configuration, dispatches the compute shader,
    /// and returns the final frame.
    ///
    /// # Arguments
    ///
    /// * `rc` - The render configuration containing the scene description and settings.
    ///
    /// # Returns
    ///
    /// * `Result<Frame>` - The rendered frame containing pixel data, or an error if rendering fails.
    fn render(&mut self, rc: RenderConfig) -> Result<Frame> {
        let mut gpu_wrapper = self.gpu_wrapper.lock().unwrap();

        gpu_wrapper.update(rc)?;
        gpu_wrapper.update_uniforms();
        gpu_wrapper.dispatch_compute()?;
        let pixels = gpu_wrapper.read_pixels()?;

        Ok(Frame::new(
            gpu_wrapper.get_width() as usize,
            gpu_wrapper.get_height() as usize,
            pixels,
        ))
    }

    /// Creates a frame iterator for progressive rendering.
    ///
    /// This method prepares the GPU for a new rendering session and returns an iterator
    /// that yields frames progressively. This is useful for interactive viewing where
    /// intermediate results are displayed while the image converges.
    ///
    /// # Arguments
    ///
    /// * `rc` - The render configuration containing the scene description and settings.
    ///
    /// # Returns
    ///
    /// * `Result<Box<dyn FrameIterator>>` - A boxed iterator yielding frames, or an error if initialization fails.
    fn frame_iterator(&mut self, rc: RenderConfig) -> Result<Box<dyn FrameIterator>> {
        {
            let mut gpu_wrapper = self.gpu_wrapper.lock().unwrap();
            gpu_wrapper.update(rc)?;
            gpu_wrapper.update_uniforms();
            gpu_wrapper.prh_mut().current_pass = 0;
        }
        Ok(Box::new(RaytracerFrameIterator::new(Arc::clone(
            &self.gpu_wrapper,
        ))))
    }
}

impl Engine {
    /// Creates a new `Engine` instance.
    ///
    /// Initializes the underlying `GpuWrapper` with the ray tracing compute shader.
    ///
    /// # Arguments
    ///
    /// * `rc` - The initial render configuration.
    ///
    /// # Returns
    ///
    /// * `Self` - A new instance of the ray tracing engine.
    pub fn new(rc: RenderConfig) -> Self {
        // Embed the shader source code into the binary at compile time.
        // This ensures the shader is available regardless of the execution environment.
        let shader_source = include_str!("shader.wgsl");
        let wrapper = GpuWrapper::new(rc, shader_source).unwrap();
        Self {
            gpu_wrapper: Arc::new(Mutex::new(wrapper)),
        }
    }
}

/// A frame iterator for progressive ray tracing.
///
/// This struct manages the state of a progressive rendering session. It handles
/// multi-pass rendering where each `next()` call computes a portion of the total samples,
/// accumulating results to reduce noise over time.
pub struct RaytracerFrameIterator {
    /// Shared access to the GPU wrapper.
    gpu_wrapper: Arc<Mutex<GpuWrapper>>,
    /// Flag indicating if the rendering session has been initialized (e.g., accumulation buffer cleared).
    initialized: bool,
    /// Timer to track the duration of the rendering process.
    render_time: Option<Instant>,
}

impl RaytracerFrameIterator {
    /// Creates a new `RaytracerFrameIterator`.
    ///
    /// # Arguments
    ///
    /// * `gpu_wrapper` - Shared access to the GPU wrapper.
    fn new(gpu_wrapper: Arc<Mutex<GpuWrapper>>) -> Self {
        Self {
            gpu_wrapper,
            initialized: false,
            render_time: None,
        }
    }
}

impl FrameIterator for RaytracerFrameIterator {
    /// Checks if there are more passes to render.
    fn has_next(&self) -> bool {
        let wrapper = self.gpu_wrapper.lock().unwrap();
        wrapper.prh().current_pass < wrapper.prh().total_passes
    }

    /// Computes the next frame in the progressive sequence.
    ///
    /// This method:
    /// 1. Initializes the accumulation buffer on the first call.
    /// 2. Updates the progressive render helper uniforms.
    /// 3. Dispatches the compute shader for the current pass.
    /// 4. Reads back the result and increments the pass counter.
    ///
    /// # Returns
    ///
    /// * `Result<Frame>` - The current accumulated frame.
    fn next(&mut self) -> Result<Frame> {
        if !self.has_next() {
            // Stop render time
            if let Some(timer) = self.render_time {
                let duration = timer.elapsed();
                log::info!("Render finished in {:?}", duration);
            }
            anyhow::bail!("No more frames available");
        }

        let mut gpu_wrapper = self.gpu_wrapper.lock().unwrap();

        if !self.initialized {
            // Start render time
            self.render_time = Some(Instant::now());
            log::info!("Render started at {}", Local::now());

            gpu_wrapper.queue().write_buffer(
                &gpu_wrapper.buffer_wrapper().accumulation,
                0,
                &vec![0u8; (gpu_wrapper.get_width() * gpu_wrapper.get_height() * 16) as usize],
            );
            self.initialized = true;
        }

        gpu_wrapper.queue().write_buffer(
            &gpu_wrapper.buffer_wrapper().progressive_render,
            0,
            bytemuck::cast_slice(&[*gpu_wrapper.prh()]),
        );

        gpu_wrapper.dispatch_compute_progressive(
            gpu_wrapper.prh().current_pass,
            gpu_wrapper.prh().total_passes,
        )?;

        let pixels = gpu_wrapper.read_pixels()?;

        let frame = Frame::new(
            gpu_wrapper.get_width() as usize,
            gpu_wrapper.get_height() as usize,
            pixels,
        );

        gpu_wrapper.prh_mut().current_pass += 1;

        log::info!(
            "[ENGINE-RAYTRACER] Sample: {} / {}",
            gpu_wrapper.prh().current_pass,
            gpu_wrapper.prh().total_passes,
        );

        if gpu_wrapper.prh().current_pass == gpu_wrapper.prh().total_passes
            && let Some(timer) = self.render_time
        {
            let duration = timer.elapsed();
            log::info!("[ENGINE-RAYTRACER] Render finished in {:?}", duration);
        }
        Ok(frame)
    }

    /// Cancels the current rendering iterator.
    fn destroy(&mut self) {
        log::info!("[ENGINE-RAYTRACER] Cancelled Render Iterator.")
    }
}
