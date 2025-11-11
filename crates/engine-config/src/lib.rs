use bytemuck::{Pod, Zeroable};

// This is a placeholder and should be granularized further into e.g:
// CameraConfiguration, Scene, ...
pub struct RenderCommand {
    pub fov: Option<f32>,
    pub spheres: Vec<Sphere>,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,
    pub color: [f32; 3],
    _pad: [u8; 4],
}

impl Sphere {
    pub const fn new(center: [f32; 3], radius: f32, color: [f32; 3]) -> Sphere {
        Sphere {
            center,
            radius,
            color,
            _pad: [0u8; 4],
        }
    }
}
