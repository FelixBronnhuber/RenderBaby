//! Global rendering parameters and settings.
//!
//! This module defines the [`Uniforms`] struct, which contains all global rendering parameters
//! that are passed to the GPU shaders. This includes resolution, camera settings, sample counts,
//! scene object counts, and visual settings like ground plane and sky color.

use bytemuck::{Pod, Zeroable};
use crate::camera::Camera;

/// Global rendering parameters passed to GPU shaders.
///
/// `Uniforms` is a GPU-compatible struct (via `Pod` and `Zeroable`) that contains all
/// global rendering settings. It must maintain a specific memory layout to match the
/// shader's uniform buffer layout.
///
/// # Memory Layout
///
/// The struct uses `#[repr(C)]` to ensure consistent memory layout across platforms.
/// Padding fields (`_pad1`, `_pad2`) are used to satisfy GPU alignment requirements.
///
/// # Boolean Fields
///
/// Note that boolean flags (`ground_enabled`, `checkerboard_enabled`, `color_hash_enabled`)
/// are stored as `u32` instead of `bool` because `bool` doesn't satisfy the `Pod` trait
/// requirements for GPU data.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Uniforms {
    /// Output image width in pixels.
    pub width: u32,
    /// Output image height in pixels.
    pub height: u32,
    /// Total number of samples per pixel for progressive rendering.
    pub total_samples: u32,
    /// Enable color hashing for debugging (0 = disabled, 1 = enabled).
    pub color_hash_enabled: u32,
    /// Camera configuration (position, direction, projection).
    pub camera: Camera,
    /// Number of spheres in the scene.
    pub spheres_count: u32,
    /// Number of triangles in the scene (deprecated, use `bvh_triangle_count`).
    pub triangles_count: u32,
    /// Number of BVH nodes for triangle acceleration.
    pub bvh_node_count: u32,
    /// Number of triangles in the BVH structure.
    pub bvh_triangle_count: u32,
    /// Index of the root BVH node.
    pub bvh_root: u32,
    /// Y-coordinate of the ground plane.
    pub ground_height: f32,
    /// Enable ground plane rendering (0 = disabled, 1 = enabled).
    pub ground_enabled: u32,
    /// Enable checkerboard pattern on ground (0 = disabled, 1 = enabled).
    pub checkerboard_enabled: u32,
    /// Sky color in RGB (each component 0.0-1.0).
    pub sky_color: [f32; 3],
    /// Maximum ray bounce depth for path tracing.
    pub max_depth: u32,
    /// First checkerboard color in RGB.
    pub checkerboard_color_1: [f32; 3],
    /// Padding for GPU alignment.
    pub _pad1: u32,
    /// Second checkerboard color in RGB.
    pub checkerboard_color_2: [f32; 3],
    /// Padding for GPU alignment.
    pub _pad2: u32,
}

impl Default for Uniforms {
    /// Creates default rendering parameters.
    ///
    /// Default settings:
    /// - Resolution: 400x300
    /// - Samples: 500
    /// - Ground plane enabled at y = -1.0
    /// - Checkerboard pattern enabled (black and magenta)
    /// - Sky color: light blue (0.5, 0.7, 1.0)
    /// - Max ray depth: 5 bounces
    fn default() -> Self {
        Self {
            width: 400,
            height: 300,
            total_samples: 500,
            color_hash_enabled: 1,
            camera: Camera::default(),
            spheres_count: 0,
            triangles_count: 0,
            bvh_node_count: 0,
            bvh_triangle_count: 0,
            bvh_root: 0,
            ground_height: -1.0,
            ground_enabled: 1,
            checkerboard_enabled: 1,
            sky_color: [0.5, 0.7, 1.0],
            max_depth: 5,
            checkerboard_color_1: [0.0; 3], // Black
            _pad1: 0,
            checkerboard_color_2: [1.0, 0.0, 1.0], // Magenta
            _pad2: 0,
        }
    }
}

impl Uniforms {
    /// Default checkerboard pattern enabled state.
    pub const CHECKERBOARD_ENABLED: bool = true;
    /// Default ground plane enabled state.
    pub const GROUND_ENABLED: bool = true;

    /// Creates a new `Uniforms` instance with custom parameters.
    ///
    /// # Arguments
    ///
    /// * `width` - Output image width in pixels
    /// * `height` - Output image height in pixels
    /// * `camera` - Camera configuration
    /// * `total_samples` - Total samples per pixel
    /// * `spheres_count` - Number of spheres in the scene
    /// * `bvh_node_count` - Number of BVH nodes
    /// * `bvh_triangle_count` - Number of triangles in BVH
    /// * `ground_height` - Y-coordinate of ground plane
    /// * `ground_enabled` - Whether to render the ground plane
    /// * `checkerboard_enabled` - Whether to render checkerboard pattern
    /// * `sky_color` - Sky color in RGB (each component 0.0-1.0)
    /// * `max_depth` - Maximum ray bounce depth
    /// * `checkerboard_color_1` - First checkerboard color in RGB
    /// * `checkerboard_color_2` - Second checkerboard color in RGB
    ///
    /// # Returns
    ///
    /// A new `Uniforms` instance with the specified parameters and defaults for unspecified fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        width: u32,
        height: u32,
        camera: Camera,
        total_samples: u32,
        spheres_count: u32,
        bvh_node_count: u32,
        bvh_triangle_count: u32,
        ground_height: f32,
        ground_enabled: bool,
        checkerboard_enabled: bool,
        sky_color: [f32; 3],
        max_depth: u32,
        checkerboard_color_1: [f32; 3],
        checkerboard_color_2: [f32; 3],
    ) -> Self {
        Self {
            width,
            height,
            camera,
            total_samples,
            spheres_count,
            bvh_node_count,
            bvh_triangle_count,
            ground_height,
            ground_enabled: if ground_enabled { 1 } else { 0 },
            checkerboard_enabled: if checkerboard_enabled { 1 } else { 0 },
            sky_color,
            max_depth,
            checkerboard_color_1,
            checkerboard_color_2,
            ..Default::default()
        }
    }

    /// Enables or disables color hash mode for debugging.
    ///
    /// Color hashing assigns colors based on object IDs, which is useful for
    /// debugging BVH traversal, material assignment, and other rendering issues.
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable color hash mode
    ///
    /// # Returns
    ///
    /// Self with the color hash setting updated, for method chaining.
    pub fn with_color_hash(mut self, enabled: bool) -> Self {
        self.color_hash_enabled = if enabled { 1 } else { 0 };
        self
    }
}
