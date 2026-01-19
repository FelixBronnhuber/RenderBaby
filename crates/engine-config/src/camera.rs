//! Camera configuration and projection parameters.
//!
//! This module defines the [`Camera`] struct, which controls the viewpoint, orientation,
//! and field of view for the rendered scene.

use bytemuck::{Pod, Zeroable};

/// Camera configuration for scene rendering.
///
/// `Camera` defines the position, direction, and projection parameters used to generate
/// primary rays in the ray tracer. It uses a simple pinhole camera model with a virtual
/// image pane positioned in front of the camera.
///
/// # Projection Model
///
/// The camera works by projecting rays through a virtual rectangular pane (the "view plane")
/// positioned at `pane_distance` units in front of the camera. The pane has a width of
/// `pane_width` units, and the height is calculated based on the aspect ratio.
///
/// - **Field of View**: Controlled by the ratio of `pane_width` to `pane_distance`
/// - **Wider pane or shorter distance**: Wider field of view
/// - **Narrower pane or longer distance**: Narrower field of view (telephoto effect)
///
/// # Memory Layout
///
/// The struct uses `#[repr(C)]` and padding fields to ensure proper alignment for GPU buffers.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Camera {
    /// Distance from camera to the virtual image pane (similar to focal length).
    ///
    /// Valid range: typically 0.0-100.0
    /// - Smaller values create a wider field of view
    /// - Larger values create a narrower field of view (zoom in)
    pub pane_distance: f32,

    /// Width of the virtual image pane.
    ///
    /// Valid range: typically 0.0-100.0
    /// - Controls horizontal field of view in combination with `pane_distance`
    /// - The aspect ratio determines the height automatically
    pub pane_width: f32,

    /// Padding for GPU alignment.
    _pad0: [f32; 2],

    /// Camera position in world space (x, y, z).
    pub pos: [f32; 3],

    /// Padding for GPU alignment.
    _pad1: f32,

    /// Camera forward direction.
    ///
    /// This vector points from the camera toward the scene.
    /// It should be non-zero and is normalized in the shader.
    pub dir: [f32; 3],

    /// Padding for GPU alignment.
    _pad2: f32,
}

impl Default for Camera {
    /// Creates a default camera configuration.
    ///
    /// Default camera:
    /// - Position: (2, 2, 0)
    /// - Direction: (0, 0, 1) - looking down positive Z axis
    /// - Pane distance: 50.0
    /// - Pane width: 100.0
    fn default() -> Self {
        Self {
            pane_distance: 50.0,
            pane_width: 100.0,
            _pad0: [0.0; 2],
            pos: [2.0, 2.0, 0.0],
            _pad1: 0.0,
            dir: [0.0, 0.0, 1.0],
            _pad2: 0.0,
        }
    }
}

impl Camera {
    /// Creates a new camera with specified parameters.
    ///
    /// # Arguments
    ///
    /// * `pane_distance` - Distance to the virtual image pane (focal length)
    /// * `pane_width` - Width of the virtual image pane
    /// * `pos` - Camera position in world space [x, y, z]
    /// * `dir` - Camera forward direction [x, y, z] (should be normalized or will be normalized in shader)
    ///
    /// # Returns
    ///
    /// A new `Camera` with the specified parameters.
    pub fn new(pane_distance: f32, pane_width: f32, pos: [f32; 3], dir: [f32; 3]) -> Self {
        Self {
            pane_distance,
            pane_width,
            pos,
            dir,
            ..Default::default()
        }
    }
}
