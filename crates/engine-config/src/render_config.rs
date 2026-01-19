//! Scene configuration and change tracking system.
//!
//! This module provides the core [`RenderConfig`] type and supporting infrastructure for
//! describing scenes and tracking changes efficiently. It uses a change-tracking pattern
//! to allow rendering engines to update only modified resources.

use crate::*;
use core::fmt;
use engine_bvh::bvh::BVHNode;
use engine_bvh::triangle::GPUTriangle;

/// Complete scene configuration for rendering.
///
/// `RenderConfig` encapsulates all data needed to render a scene, including geometry,
/// materials, lights, and global settings. Each field is wrapped in a [`Change`] enum
/// to track whether it should be created, updated, kept unchanged, or deleted.
///
/// # Change Tracking
///
/// The change-tracking system allows efficient GPU resource management:
/// - On first render: All fields should be `Change::Create`
/// - On subsequent renders: Use `Update`, `Keep`, or `Delete` as appropriate
/// - GPU buffers are only reallocated when necessary
///
/// # Builder Pattern
///
/// Use [`RenderConfig::builder()`] to construct configurations fluently:
///
/// ```rust,ignore
/// let config = RenderConfig::builder()
///     .uniforms_create(Uniforms::default())
///     .spheres_create(vec![sphere1, sphere2])
///     .lights_create(vec![light])
///     // ... other fields
///     .build();
/// ```
#[derive(Clone)]
pub struct RenderConfig {
    /// Global rendering parameters (resolution, camera, samples, etc.).
    pub uniforms: Change<Uniforms>,
    /// Sphere primitives in the scene.
    pub spheres: Change<Vec<Sphere>>,
    /// UV coordinates for texture mapping (flattened array).
    pub uvs: Change<Vec<f32>>,
    /// Triangle meshes in the scene.
    pub meshes: Change<Vec<Mesh>>,
    /// Point/area light sources.
    pub lights: Change<Vec<PointLight>>,
    /// BVH nodes for triangle acceleration structure.
    pub bvh_nodes: Change<Vec<BVHNode>>,
    /// BVH triangle indices for indirect indexing.
    pub bvh_indices: Change<Vec<u32>>,
    /// Triangles in BVH-compatible format.
    pub bvh_triangles: Change<Vec<GPUTriangle>>,
    /// Texture image data for material mapping.
    pub textures: Change<Vec<TextureData>>,
}

/// Change tracking enum for resource lifecycle management.
///
/// `Change<T>` wraps each field in `RenderConfig` to indicate what action should be
/// taken with that resource. This allows rendering engines to minimize GPU buffer
/// reallocations and data transfers.
///
/// # Variants
///
/// - `Keep`: Resource hasn't changed, reuse existing GPU data
/// - `Create`: Initialize resource for the first time (first render only)
/// - `Update`: Resource has changed, update GPU data
/// - `Delete`: Remove resource and free GPU memory
///
/// # Usage Rules
///
/// **First render** (via [`validate_init`](RenderConfig::validate_init)):
/// - All required fields must be `Create`
/// - `Keep`, `Update`, `Delete` are not allowed
///
/// **Subsequent renders** (via [`validate`](Validate::validate)):
/// - Use `Update` to change data
/// - Use `Keep` to preserve existing data
/// - Use `Delete` to remove data (where supported)
/// - `Create` is not allowed after initialization
///
/// # Examples
///
/// ```rust,ignore
/// // First render - create everything
/// let initial = RenderConfig::builder()
///     .uniforms_create(uniforms)
///     .spheres_create(spheres)
///     .build();
///
/// // Update camera only, keep spheres
/// let updated = RenderConfig::builder()
///     .uniforms(updated_uniforms)  // Update
///     .spheres_no_change()          // Keep
///     .build();
/// ```
#[derive(Clone, Copy, Debug)]
pub enum Change<T> {
    /// Keep existing resource unchanged.
    Keep,
    /// Create resource for the first time (first render only).
    Create(T),
    /// Update existing resource with new data.
    Update(T),
    /// Delete resource and free GPU memory.
    Delete,
}

/// Trait for validating render configurations.
///
/// Ensures that a [`RenderConfig`] is valid for rendering by checking constraints
/// on camera parameters, geometry, and other settings.
pub trait Validate {
    /// Validates the configuration for subsequent renders.
    ///
    /// Checks that:
    /// - Camera parameters are within valid ranges
    /// - Geometry data is valid (positive radii, valid UVs, etc.)
    /// - No invalid transitions occur (e.g., deleting non-existent resources)
    ///
    /// # Returns
    ///
    /// * `Ok(&Self)` - Configuration is valid
    /// * `Err(RenderConfigBuilderError)` - Configuration has errors
    fn validate(&self) -> Result<&Self, RenderConfigBuilderError>;
}

/// Trait for validating initial render configurations.
///
/// Similar to [`Validate`] but specifically for the first render, where all
/// required fields must be `Change::Create`.
pub trait ValidateInit {
    /// Validates the configuration for the first render.
    ///
    /// Ensures all required fields are present as `Change::Create`.
    ///
    /// # Returns
    ///
    /// * `Ok(&Self)` - Configuration is valid for initialization
    /// * `Err(RenderConfigBuilderError)` - Configuration is missing required fields
    fn validate_init(&self) -> Result<&Self, RenderConfigBuilderError>;
}

impl RenderConfig {
    /// Creates a new [`RenderConfigBuilder`] for fluent configuration.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let config = RenderConfig::builder()
    ///     .uniforms_create(Uniforms::default())
    ///     .spheres_create(vec![])
    ///     .lights_create(vec![])
    ///     .build();
    /// ```
    pub fn builder() -> RenderConfigBuilder {
        RenderConfigBuilder::default()
    }
}

impl ValidateInit for RenderConfig {
    fn validate_init(&self) -> Result<&Self, RenderConfigBuilderError> {
        if !matches!(self.uniforms, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidUniforms);
        }
        if !matches!(self.spheres, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidSpheres);
        }
        if !matches!(self.uvs, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidUVs);
        }
        if !matches!(self.meshes, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidMeshes);
        }
        if !matches!(self.lights, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidLights);
        }
        if !matches!(self.textures, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidTextures);
        }
        Ok(self)
    }
}

impl Validate for RenderConfig {
    fn validate(&self) -> Result<&Self, RenderConfigBuilderError> {
        match &self.uniforms {
            Change::Update(u) | Change::Create(u) => {
                // Validate camera projection parameters
                if !(0.0..=100.0).contains(&u.camera.pane_distance) {
                    return Err(RenderConfigBuilderError::PaneDistanceOutOfBounds);
                }
                if !(0.0..=1000.0).contains(&u.camera.pane_width) {
                    return Err(RenderConfigBuilderError::PaneWidthOutOfBounds);
                }
                if is_zero(&u.camera.dir) {
                    return Err(RenderConfigBuilderError::InvalidCameraDirection);
                }
                // TODO: Add more Uniforms validation as needed
            }
            Change::Delete => {
                return Err(RenderConfigBuilderError::CannotDeleteNonexistent);
            }
            Change::Keep => {}
        }

        match &self.spheres {
            Change::Update(spheres) | Change::Create(spheres) => {
                if spheres.iter().any(|s| s.radius <= 0.0) {
                    return Err(RenderConfigBuilderError::InvalidSpheres);
                }
                // TODO: Add more Sphere validation as needed
            }
            Change::Delete => {
                log::info!("RenderConfig: Attempting to delete spheres")
            }
            Change::Keep => {}
        }

        match &self.uvs {
            Change::Update(uvs) | Change::Create(uvs) => {
                if uvs.len() % 2 != 0 {
                    return Err(RenderConfigBuilderError::InvalidUVs);
                }
            }
            Change::Delete => {
                todo!("Implement UVs Deletion")
            }
            Change::Keep => {}
        }

        match &self.meshes {
            Change::Update(_meshes) | Change::Create(_meshes) => {
                // TODO: Mesh validation
            }
            Change::Delete => {
                todo!("Implement meshes Deletion")
            }
            Change::Keep => {}
        }

        match &self.lights {
            Change::Update(lights) | Change::Create(lights) => {
                if lights.iter().any(|l| l.radius <= 0.0) {
                    return Err(RenderConfigBuilderError::InvalidLights);
                }
            }
            Change::Delete => {
                todo!("Implement lights Deletion")
            }
            Change::Keep => {}
        }

        match &self.textures {
            Change::Update(_) | Change::Create(_) => {
                // TODO: Texture validation
            }
            Change::Delete => {
                todo!("Implement textures Deletion")
            }
            Change::Keep => {}
        }

        Ok(self)
    }
}

/// Builder for constructing [`RenderConfig`] instances.
///
/// `RenderConfigBuilder` provides a fluent API for building render configurations
/// with compile-time guidance on available options.
///
/// # Usage Pattern
///
/// 1. Create builder: `RenderConfig::builder()`
/// 2. Set fields using typed methods
/// 3. Build: `.build()`
///
/// # Method Variants
///
/// Each field has four method variants:
/// - `field_create(value)` - Creates the field (`Change::Create`)
/// - `field(value)` - Updates the field (`Change::Update`)
/// - `field_no_change()` - Keeps existing (`Change::Keep`)
/// - `field_delete()` - Deletes the field (`Change::Delete`)
///
/// # Examples
///
/// ```rust,ignore
/// // First render - create all fields
/// let initial = RenderConfig::builder()
///     .uniforms_create(Uniforms::default())
///     .spheres_create(vec![sphere])
///     .uvs_create(vec![])
///     .meshes_create(vec![])
///     .lights_create(vec![light])
///     .textures_create(vec![])
///     .build();
///
/// // Update camera, keep geometry
/// let updated = RenderConfig::builder()
///     .uniforms(new_uniforms)
///     .spheres_no_change()
///     .uvs_no_change()
///     .meshes_no_change()
///     .lights_no_change()
///     .textures_no_change()
///     .build();
/// ```
#[derive(Default, Clone)]
pub struct RenderConfigBuilder {
    pub uniforms: Option<Change<Uniforms>>,
    pub spheres: Option<Change<Vec<Sphere>>>,
    pub uvs: Option<Change<Vec<f32>>>,
    pub meshes: Option<Change<Vec<Mesh>>>,
    pub lights: Option<Change<Vec<PointLight>>>,
    pub bvh_nodes: Option<Change<Vec<BVHNode>>>,
    pub bvh_indices: Option<Change<Vec<u32>>>,
    pub bvh_triangles: Option<Change<Vec<GPUTriangle>>>,
    pub textures: Option<Change<Vec<TextureData>>>,
}

impl RenderConfigBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self {
            uniforms: None,
            spheres: None,
            uvs: None,
            meshes: None,
            lights: None,
            bvh_nodes: None,
            bvh_indices: None,
            bvh_triangles: None,
            textures: None,
        }
    }

    /// Updates uniforms (`Change::Update`).
    pub fn uniforms(mut self, uniforms: Uniforms) -> Self {
        self.uniforms = Some(Change::Update(uniforms));
        self
    }

    /// Creates uniforms (`Change::Create`) - use for first render.
    pub fn uniforms_create(mut self, uniforms: Uniforms) -> Self {
        self.uniforms = Some(Change::Create(uniforms));
        self
    }

    /// Keeps uniforms unchanged (`Change::Keep`).
    pub fn uniforms_no_change(mut self) -> Self {
        self.uniforms = Some(Change::Keep);
        self
    }

    /// Deletes uniforms (`Change::Delete`).
    pub fn uniforms_delete(mut self) -> Self {
        self.uniforms = Some(Change::Delete);
        self
    }

    /// Updates spheres (`Change::Update`).
    pub fn spheres(mut self, spheres: Vec<Sphere>) -> Self {
        self.spheres = Some(Change::Update(spheres));
        self
    }

    /// Creates spheres (`Change::Create`) - use for first render.
    pub fn spheres_create(mut self, spheres: Vec<Sphere>) -> Self {
        self.spheres = Some(Change::Create(spheres));
        self
    }

    /// Keeps spheres unchanged (`Change::Keep`).
    pub fn spheres_no_change(mut self) -> Self {
        self.spheres = Some(Change::Keep);
        self
    }

    /// Deletes spheres (`Change::Delete`).
    pub fn spheres_delete(mut self) -> Self {
        self.spheres = Some(Change::Delete);
        self
    }

    /// Updates UV coordinates (`Change::Update`).
    pub fn uvs(mut self, uvs: Vec<f32>) -> Self {
        self.uvs = Some(Change::Update(uvs));
        self
    }

    /// Creates UV coordinates (`Change::Create`) - use for first render.
    pub fn uvs_create(mut self, uvs: Vec<f32>) -> Self {
        self.uvs = Some(Change::Create(uvs));
        self
    }

    /// Keeps UV coordinates unchanged (`Change::Keep`).
    pub fn uvs_no_change(mut self) -> Self {
        self.uvs = Some(Change::Keep);
        self
    }

    /// Deletes UV coordinates (`Change::Delete`).
    pub fn uvs_delete(mut self) -> Self {
        self.uvs = Some(Change::Delete);
        self
    }

    /// Updates meshes (`Change::Update`).
    pub fn meshes(mut self, meshes: Vec<Mesh>) -> Self {
        self.meshes = Some(Change::Update(meshes));
        self
    }

    /// Creates meshes (`Change::Create`) - use for first render.
    pub fn meshes_create(mut self, meshes: Vec<Mesh>) -> Self {
        self.meshes = Some(Change::Create(meshes));
        self
    }

    /// Keeps meshes unchanged (`Change::Keep`).
    pub fn meshes_no_change(mut self) -> Self {
        self.meshes = Some(Change::Keep);
        self
    }

    /// Deletes meshes (`Change::Delete`).
    pub fn meshes_delete(mut self) -> Self {
        self.meshes = Some(Change::Delete);
        self
    }

    /// Updates lights (`Change::Update`).
    pub fn lights(mut self, lights: Vec<PointLight>) -> Self {
        self.lights = Some(Change::Update(lights));
        self
    }

    /// Creates lights (`Change::Create`) - use for first render.
    pub fn lights_create(mut self, lights: Vec<PointLight>) -> Self {
        self.lights = Some(Change::Create(lights));
        self
    }

    /// Keeps lights unchanged (`Change::Keep`).
    pub fn lights_no_change(mut self) -> Self {
        self.lights = Some(Change::Keep);
        self
    }

    /// Deletes lights (`Change::Delete`).
    pub fn lights_delete(mut self) -> Self {
        self.lights = Some(Change::Delete);
        self
    }

    /// Updates BVH nodes (`Change::Update`).
    pub fn bvh_nodes(mut self, nodes: Vec<BVHNode>) -> Self {
        self.bvh_nodes = Some(Change::Update(nodes));
        self
    }

    /// Creates BVH nodes (`Change::Create`).
    pub fn bvh_nodes_create(mut self, nodes: Vec<BVHNode>) -> Self {
        self.bvh_nodes = Some(Change::Create(nodes));
        self
    }

    /// Keeps BVH nodes unchanged (`Change::Keep`).
    pub fn bvh_nodes_no_change(mut self) -> Self {
        self.bvh_nodes = Some(Change::Keep);
        self
    }

    /// Deletes BVH nodes (`Change::Delete`).
    pub fn bvh_nodes_delete(mut self) -> Self {
        self.bvh_nodes = Some(Change::Delete);
        self
    }

    /// Updates BVH indices (`Change::Update`).
    pub fn bvh_indices(mut self, indices: Vec<u32>) -> Self {
        self.bvh_indices = Some(Change::Update(indices));
        self
    }

    /// Creates BVH indices (`Change::Create`).
    pub fn bvh_indices_create(mut self, indices: Vec<u32>) -> Self {
        self.bvh_indices = Some(Change::Create(indices));
        self
    }

    /// Keeps BVH indices unchanged (`Change::Keep`).
    pub fn bvh_indices_no_change(mut self) -> Self {
        self.bvh_indices = Some(Change::Keep);
        self
    }

    /// Deletes BVH indices (`Change::Delete`).
    pub fn bvh_indices_delete(mut self) -> Self {
        self.bvh_indices = Some(Change::Delete);
        self
    }

    /// Updates BVH triangles (`Change::Update`).
    pub fn bvh_triangles(mut self, triangles: Vec<GPUTriangle>) -> Self {
        self.bvh_triangles = Some(Change::Update(triangles));
        self
    }

    /// Creates BVH triangles (`Change::Create`).
    pub fn bvh_triangles_create(mut self, triangles: Vec<GPUTriangle>) -> Self {
        self.bvh_triangles = Some(Change::Create(triangles));
        self
    }

    /// Keeps BVH triangles unchanged (`Change::Keep`).
    pub fn bvh_triangles_no_change(mut self) -> Self {
        self.bvh_triangles = Some(Change::Keep);
        self
    }

    /// Deletes BVH triangles (`Change::Delete`).
    pub fn bvh_triangles_delete(mut self) -> Self {
        self.bvh_triangles = Some(Change::Delete);
        self
    }

    /// Updates textures (`Change::Update`).
    pub fn textures(mut self, textures: Vec<TextureData>) -> Self {
        self.textures = Some(Change::Update(textures));
        self
    }

    /// Creates textures (`Change::Create`) - use for first render.
    pub fn textures_create(mut self, textures: Vec<TextureData>) -> Self {
        self.textures = Some(Change::Create(textures));
        self
    }

    /// Keeps textures unchanged (`Change::Keep`).
    pub fn textures_no_change(mut self) -> Self {
        self.textures = Some(Change::Keep);
        self
    }

    /// Deletes textures (`Change::Delete`).
    pub fn textures_delete(mut self) -> Self {
        self.textures = Some(Change::Delete);
        self
    }

    /// Builds the [`RenderConfig`] from the builder.
    ///
    /// Fields not explicitly set will default to `Change::Keep`.
    /// Logs warnings for any fields that weren't set.
    ///
    /// # Returns
    ///
    /// A complete `RenderConfig` ready for validation and rendering.
    pub fn build(self) -> RenderConfig {
        if self.uniforms.is_none() {
            log::info!("RenderConfigBuilder: uniforms not set, defaulting to NoChange.");
        }
        if self.spheres.is_none() {
            log::info!("RenderConfigBuilder: spheres not set, defaulting to NoChange.");
        }
        if self.uvs.is_none() {
            log::info!("RenderConfigBuilder: uvs not set, defaulting to NoChange.");
        }
        if self.meshes.is_none() {
            log::info!("RenderConfigBuilder: meshes not set, defaulting to NoChange.");
        }
        if self.lights.is_none() {
            log::info!("RenderConfigBuilder: lights not set, defaulting to NoChange.");
        }
        if self.textures.is_none() {
            log::info!("RenderConfigBuilder: textures not set, defaulting to NoChange.");
        }
        if self.bvh_nodes.is_none() {
            log::info!("RenderConfigBuilder: bvh_nodes not set, defaulting to NoChange");
        }
        if self.bvh_indices.is_none() {
            log::info!("RenderConfigBuilder: bvh_indices not set, defaulting to NoChange");
        }
        if self.bvh_triangles.is_none() {
            log::info!("RenderConfigBuilder: bvh_triangles not set, defaulting to NoChange");
        }

        RenderConfig {
            uniforms: self.uniforms.unwrap_or(Change::Keep),
            spheres: self.spheres.unwrap_or(Change::Keep),
            uvs: self.uvs.unwrap_or(Change::Keep),
            meshes: self.meshes.unwrap_or(Change::Keep),
            lights: self.lights.unwrap_or(Change::Keep),
            textures: self.textures.unwrap_or(Change::Keep),
            bvh_nodes: self.bvh_nodes.unwrap_or(Change::Keep),
            bvh_indices: self.bvh_indices.unwrap_or(Change::Keep),
            bvh_triangles: self.bvh_triangles.unwrap_or(Change::Keep),
        }
    }
}

/// Errors that can occur when building or validating render configurations.
#[derive(Debug)]
pub enum RenderConfigBuilderError {
    /// Camera pane distance is outside valid range (0.0-100.0).
    PaneDistanceOutOfBounds,
    /// Camera pane width is outside valid range (0.0-100.0).
    PaneWidthOutOfBounds,
    /// Camera direction is zero vector (invalid).
    InvalidCameraDirection,
    /// Uniforms are invalid or missing.
    InvalidUniforms,
    /// Spheres contain invalid data (e.g., non-positive radius).
    InvalidSpheres,
    /// UV coordinates are invalid (e.g., odd length).
    InvalidUVs,
    /// Meshes contain invalid data.
    InvalidMeshes,
    /// Lights contain invalid data (e.g., non-positive radius).
    InvalidLights,
    /// Textures contain invalid data.
    InvalidTextures,
    /// Attempted to delete a non-existent resource.
    CannotDeleteNonexistent,
}

impl fmt::Display for RenderConfigBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderConfigBuilderError::PaneDistanceOutOfBounds => {
                write!(f, "Pane-Distance is out of bounds")
            }
            RenderConfigBuilderError::PaneWidthOutOfBounds => {
                write!(f, "Pane-Distance is out of bounds")
            }
            RenderConfigBuilderError::InvalidCameraDirection => {
                write!(f, "Invalid camera direction")
            }
            RenderConfigBuilderError::InvalidUniforms => write!(f, "Invalid Uniforms"),
            RenderConfigBuilderError::InvalidSpheres => write!(f, "Invalid Spheres"),
            RenderConfigBuilderError::InvalidUVs => write!(f, "Invalid UVs"),
            RenderConfigBuilderError::InvalidMeshes => write!(f, "Invalid Meshes"),
            RenderConfigBuilderError::InvalidLights => write!(f, "Invalid Lights"),
            RenderConfigBuilderError::InvalidTextures => write!(f, "Invalid Textures"),
            RenderConfigBuilderError::CannotDeleteNonexistent => {
                write!(f, "Cannot delete none existent")
            }
        }
    }
}

impl std::error::Error for RenderConfigBuilderError {}

/// Checks if a 3D vector is approximately zero.
///
/// Used for validating camera direction vectors.
///
/// # Arguments
///
/// * `v` - The vector to check
///
/// # Returns
///
/// `true` if the vector's squared length is less than `f32::EPSILON`.
fn is_zero(v: &[f32; 3]) -> bool {
    let len_sq = v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
    len_sq < f32::EPSILON
}

#[cfg(test)]
mod tests {
    // TODO: Write tests for new CRUD style RenderConfigBuilder
}
