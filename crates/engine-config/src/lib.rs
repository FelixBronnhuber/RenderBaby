//! # Engine Config
//!
//! `engine-config` provides the core configuration types and traits for the RenderBaby rendering system.
//! It defines the scene description format, rendering parameters, and the common interface that all
//! rendering engines must implement.
//!
//! ## Features
//!
//! - **Unified Configuration**: A single [`RenderConfig`] type that encapsulates all scene data and rendering settings.
//! - **Change Tracking**: The `Change` enum allows efficient updates by tracking what has been created, updated, kept, or deleted.
//! - **Builder Pattern**: [`RenderConfigBuilder`] provides a fluent API for constructing render configurations.
//! - **Validation**: Built-in validation ensures configurations are valid before being sent to the GPU.
//! - **Renderer Trait**: The [`Renderer`] trait defines a common interface for both synchronous and progressive rendering.
//!
//! ## Core Types
//!
//! - [`Uniforms`]: Global rendering parameters (resolution, samples, camera, etc.)
//! - [`Camera`]: Camera position, orientation, and projection settings
//! - [`Sphere`]: Sphere primitive with material
//! - [`Mesh`]: Triangle mesh reference with material
//! - [`PointLight`]: Point light source
//! - [`Material`]: Surface material properties (diffuse, specular, emissive, etc.)
//! - [`Vec3`]: 3D vector for positions, directions, and colors
//! - [`TextureData`]: Texture image data
//!
//! ## Architecture
//!
//! The configuration system is designed around the `Change` enum, which tracks the lifecycle
//! of scene resources. This allows rendering engines to efficiently update only what has changed
//! between frames, avoiding unnecessary GPU buffer reallocations.
//!
//! ### Change Tracking
//!
//! Each field in [`RenderConfig`] is wrapped in a `Change` enum:
//! - `Create`: Initialize a resource for the first time
//! - `Update`: Modify an existing resource
//! - `Keep`: No changes, reuse existing GPU data
//! - `Delete`: Remove a resource
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! let config = RenderConfig::builder()
//!     .uniforms_create(Uniforms::default())
//!     .spheres_create(vec![
//!         Sphere::new(Vec3::new(0.0, 0.0, 5.0), 1.0, Material::default()).unwrap()
//!     ])
//!     .lights_create(vec![PointLight::default()])
//!     .uvs_create(vec![])
//!     .meshes_create(vec![])
//!     .textures_create(vec![])
//!     .build();
//! ```

pub mod camera;
pub mod material;
pub mod mesh;
pub mod point_lights;
pub mod render_config;
pub mod renderer;
pub mod sphere;
pub mod texture;
pub mod uniforms;
pub mod vec3;

pub use render_config::{RenderConfig, RenderConfigBuilder, RenderConfigBuilderError};
pub use sphere::{Sphere, SphereError};
pub use texture::TextureData;
pub use uniforms::Uniforms;
pub use vec3::Vec3;
pub use camera::Camera;
pub use point_lights::PointLight;
pub use renderer::Renderer;
pub use material::Material;
pub use mesh::Mesh;
