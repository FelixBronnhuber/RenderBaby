use bytemuck::{Pod, Zeroable};
use crate::Material;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Mesh {
    pub triangle_index_start: u32,
    pub triangle_count: u32,
    _pad: [u32; 2],
    pub material: Material,
}

impl Default for Mesh {
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
    pub fn new(triangle_index_start: u32, triangle_count: u32, material: Material) -> Self {
        Self {
            triangle_index_start,
            triangle_count,
            material,
            ..Default::default()
        }
    }
}
