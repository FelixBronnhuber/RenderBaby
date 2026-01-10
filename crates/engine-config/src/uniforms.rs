use bytemuck::{Pod, Zeroable};
use crate::camera::Camera;
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Uniforms {
    pub width: u32,
    pub height: u32,
    pub total_samples: u32,
    pub color_hash_enabled: u32,
    pub camera: Camera,
    pub spheres_count: u32,
    pub triangles_count: u32,
    pub bvh_node_count: u32,
    pub bvh_triangle_count: u32,
    pub bvh_root: u32,
    pub ground_height: f32,
    pub ground_enabled: u32, //bool doesn't satisfy Pod
    pub _pad: u32,
    pub sky_color: [f32; 3],
    pub max_depth: u32,
}

impl Default for Uniforms {
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
            ground_height: -5.0,
            ground_enabled: 1,
            _pad: 0,
            sky_color: [0.5, 0.7, 1.0],
            max_depth: 5,
        }
    }
}

impl Uniforms {
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
        ground_enabled: u32,
        sky_color: [f32; 3],
        max_depth: u32,
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
            ground_enabled,
            sky_color,
            max_depth,
            ..Default::default()
        }
    }

    pub fn with_color_hash(mut self, enabled: bool) -> Self {
        self.color_hash_enabled = if enabled { 1 } else { 0 };
        self
    }
}
