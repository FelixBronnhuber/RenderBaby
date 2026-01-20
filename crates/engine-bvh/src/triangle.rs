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
    pub v0: Vec3,
    pub v0_index: u32,
    pub v1: Vec3,
    pub v1_index: u32,
    pub v2: Vec3,
    pub v2_index: u32,
    pub mesh_index: u32,
    pub _pad0: u32,
    pub _pad1: u32,
    pub _pad2: u32,
}
