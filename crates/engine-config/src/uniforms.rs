use bytemuck::{Pod, Zeroable};
use std::f32;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Uniforms {
    pub width: u32,
    pub height: u32,
    pub pane_distance: f32,
    pub pane_width: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub x_dir: f32,
    pub y_dir: f32,
    pub z_dir: f32,
    pub spheres_count: u32,
    pub triangles_count: u32,
    pub _pad: [u32; 3],
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            width: 400,
            height: 300,
            pane_distance: 50.00,
            pane_width: 100.00,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            x_dir: 0.0,
            y_dir: 0.0,
            z_dir: 1.0,
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
        pane_distance: f32,
        pane_width: f32,
        spheres_count: u32,
        triangles_count: u32,
    ) -> Self {
        Self {
            width,
            height,
            pane_distance,
            pane_width,
            spheres_count,
            triangles_count,
            ..Default::default()
        }
    }
}
