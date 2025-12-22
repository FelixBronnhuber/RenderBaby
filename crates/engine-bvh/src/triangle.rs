use glam::Vec3;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable, Default)]
pub struct GPUTriangle {
    pub v0: Vec3,
    pub _pad0: u32,
    pub v1: Vec3,
    pub _pad1: u32,
    pub v2: Vec3,
    pub _pad2: u32,
    pub material_id: u32,
}
