use bytemuck::{Pod, Zeroable};
use std::f32;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Uniforms {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub spheres_count: u32,
    pub triangles_count: u32,
    pub _pad: [u32; 3],
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            width: 400,
            height: 300,
            fov: f32::consts::FRAC_PI_4,
            spheres_count: 0,
            triangles_count: 0,
            _pad: [0; 3],
        }
    }
}

impl Uniforms {
    pub fn new(
        width: u32,
        height: u32,
        fov: f32,
        spheres_count: u32,
        triangles_count: u32,
    ) -> Self {
        Self {
            width,
            height,
            fov,
            spheres_count,
            triangles_count,
            ..Default::default()
        }
    }
}
