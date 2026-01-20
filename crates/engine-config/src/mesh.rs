// ============================================================================
// mesh.rs
// ============================================================================

//! Triangle mesh primitives.
//!
//! This module defines the [`Mesh`] struct, which represents a collection of triangles
//! sharing a common material.

use bytemuck::{Pod, Zeroable};
use crate::Material;

/// A triangle mesh with shared material properties.
///
/// `Mesh` represents a contiguous range of triangles in the scene's triangle buffer.
/// All triangles in a mesh share the same material properties.
///
/// # Memory Layout
///
/// The struct uses `#[repr(C)]` and padding to ensure proper GPU alignment.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Mesh {
    /// Index of the first triangle in the global triangle array.
    pub triangle_index_start: u32,
    /// Number of triangles in this mesh.
    pub triangle_count: u32,
    /// Padding for GPU alignment.
    _pad: [u32; 2],
    /// Material properties applied to all triangles in this mesh.
    pub material: Material,
}

impl Default for Mesh {
    /// Creates a default mesh.
    ///
    /// Default values:
    /// - Triangle index start: 0
    /// - Triangle count: 1
    /// - Material: default material
    fn default() -> Self {
        Self {
            triangle_index_start: 0,
            triangle_count: 1,
            _pad: [0; 2],
            material: Material::default(),
        }
    }
}

impl Mesh {
    /// Creates a new mesh with specified parameters.
    ///
    /// # Arguments
    ///
    /// * `triangle_index_start` - Index of the first triangle in the global triangle buffer
    /// * `triangle_count` - Number of consecutive triangles belonging to this mesh
    /// * `material` - Material properties for all triangles in the mesh
    ///
    /// # Returns
    ///
    /// A new `Mesh` with the specified configuration.
    pub fn new(triangle_index_start: u32, triangle_count: u32, material: Material) -> Self {
        Self {
            triangle_index_start,
            triangle_count,
            material,
            ..Default::default()
        }
    }
}
