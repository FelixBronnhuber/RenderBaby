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
    pub ground_enabled: u32,       //bool doesn't satisfy Pod
    pub checkerboard_enabled: u32, //bool doesn't satisfy Pod
    pub sky_color: [f32; 3],
    pub max_depth: u32,
    pub checkerboard_color_1: [f32; 3],
    pub _pad1: u32,
    pub checkerboard_color_2: [f32; 3],
    pub _pad2: u32,
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
            checkerboard_enabled: 1,
            sky_color: [0.5, 0.7, 1.0],
            max_depth: 5,
            checkerboard_color_1: [0.0; 3], //Black
            _pad1: 0,
            checkerboard_color_2: [1.0, 0.5, 1.0], //Magenta
            _pad2: 0,
        }
    }
}

impl Uniforms {
    pub const CHECKERBOARD_ENABLED: bool = true;
    pub const GROUND_ENABLED: bool = true;

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
        ground_enabled: bool,
        checkerboard_enabled: bool,
        sky_color: [f32; 3],
        max_depth: u32,
        checkerboard_color_1: [f32; 3],
        checkerboard_color_2: [f32; 3],
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
            ground_enabled: if ground_enabled { 1 } else { 0 },
            checkerboard_enabled: if checkerboard_enabled { 1 } else { 0 },
            sky_color,
            max_depth,
            checkerboard_color_1,
            checkerboard_color_2,
            ..Default::default()
        }
    }

    pub fn with_color_hash(mut self, enabled: bool) -> Self {
        self.color_hash_enabled = if enabled { 1 } else { 0 };
        self
    }
}
