use bytemuck::{Pod, Zeroable};
use crate::camera::Camera;
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Uniforms {
    pub width: u32,
    pub height: u32,
    _pad0: [u32; 2],
    pub camera: Camera,
    pub spheres_count: u32,
    pub triangles_count: u32,
    _pad1: [u32; 2],
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            width: 400,
            height: 300,
            _pad0: [0; 2],
            camera: Camera::default(),
            spheres_count: 0,
            triangles_count: 0,
            _pad1: [0; 2],
        }
    }
}

impl Uniforms {
    pub fn new(
        width: u32,
        height: u32,
        camera: Camera,
        spheres_count: u32,
        triangles_count: u32,
    ) -> Self {
        Self {
            width,
            height,
            camera,
            spheres_count,
            triangles_count,
            ..Default::default()
        }
    }
}
