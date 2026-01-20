//! GPU-friendly triangle representation.
use glam::Vec3;
use bytemuck::{Pod, Zeroable};

/// A triangle structure optimized for GPU usage.
///
/// The layout is `#[repr(C)]` and padded to ensure compatibility
/// with GPU buffers and shader interfaces.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, Default)]
pub struct GPUTriangle {
    /// First vertex position.
    pub v0: Vec3,
    /// Index of the first vertex.
    pub v0_index: u32,
    /// Second vertex position.
    pub v1: Vec3,
    /// Index of the second vertex.
    pub v1_index: u32,
    /// Third vertex position.
    pub v2: Vec3,
    /// Index of the third vertex.
    pub v2_index: u32,
    /// Index of the mesh this triangle belongs to.
    pub mesh_index: u32,
    pub _pad0: u32,
    pub _pad1: u32,
    pub _pad2: u32,
}
