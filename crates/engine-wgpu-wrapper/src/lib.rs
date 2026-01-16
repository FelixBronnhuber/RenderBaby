//! # Engine WGPU Wrapper
//!
//! This crate provides a shared `wgpu` infrastructure for the rendering engines (`engine-raytracer` and `engine-pathtracer`).
//! It encapsulates the boilerplate code required to set up a `wgpu` compute pipeline, manage buffers, handles binding groups
//! and perform progressive rendering.
//!
//! ## Key Components
//!
//! - [`GpuWrapper`]: The main entry point, managing the device, queue, pipeline, and resources.
//! - [`GpuBuffers`]: Manages all GPU buffers (uniforms, geometry, textures, accumulation, etc.).
//! - [`BindGroup`] & [`BindGroupLayout`]: Defines and creates the bind groups used by the compute shaders.
//! - [`ComputePipeline`]: Handles the creation of the wgpu compute pipeline and shader module loading.
//! - [`GpuDevice`]: Provides a singleton-like access to the `wgpu::Device` and `wgpu::Queue`.
//!
//! ## Usage
//!
//! Engines typically instantiate `GpuWrapper` with a `RenderConfig` and the path to their specific shader file.
//!
//! ```rust,ignore
//! use engine_wgpu_wrapper::GpuWrapper;
//! use engine_config::RenderConfig;
//!
//! let rc = RenderConfig::default();
//! let mut wrapper = GpuWrapper::new(rc, "path/to/shader.wgsl").unwrap();
//!
//! // Update resources
//! wrapper.update(new_config).unwrap();
//! wrapper.update_uniforms();
//!
//! // Execute compute pass
//! wrapper.dispatch_compute().unwrap();
//!
//! // Read result
//! let pixels = wrapper.read_pixels().unwrap();
//! ```

mod bind_group;
mod buffers;
pub mod gpu_device;
mod gpu_wrapper;
mod pipeline;

pub use bind_group::*;
pub use buffers::*;
pub use gpu_device::*;
pub use gpu_wrapper::*;
pub use pipeline::*;

pub use anyhow::Result;
pub use engine_config::RenderConfig;
