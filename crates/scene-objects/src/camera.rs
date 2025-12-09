use glam::Vec3;
use serde::{Deserialize, Serialize};
/// Camera that is used to render scenes
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Camera {
    position: Vec3,
    rotation: Vec3,
    fov: f32,
    resolution: Resolution,
    ray_samples: u32,
}
#[allow(dead_code)]
impl Camera {
    pub fn set_position(&mut self, position: Vec3) {
        //! sets the position of the camera
        //! ## Parameter
        //! 'position': glam::Vec3 of the new position
        self.position = position;
    }
    pub fn set_rotation(&mut self, rotation: Vec3) {
        //! sets the rotation of the camera
        //! ## Parameter
        //! 'rotation': glam::Vec3 of the new rotation
        self.rotation = rotation
    }
    pub fn get_position(&self) -> Vec3 {
        //! ## Returns
        //! Camera position as glam::Vec3
        self.position
    }
    pub fn get_rotation(&self) -> Vec3 {
        //! ## Returns
        //! Camera rotation as glam::Vec3
        self.rotation
    }
    pub fn get_fov(&self) -> f32 {
        //! ## Returns
        //! Camera field of view
        self.fov
    }
    pub fn set_fov(&mut self, fov: f32) {
        //! Sets the camera field of view. Value should be between ...
        //! ## Parameter
        //! fov: new field of view
        self.fov = fov;
    }
    pub fn get_resolution(&self) -> &Resolution {
        //! ## Returns
        //! Camera resolution as Array of u32
        &self.resolution
    }
    pub fn set_resolution(&mut self, resolution: Resolution) {
        //! Sets the camera resolution
        //! ## Parameter
        //! 'resolution': New resolution as array of u32
        self.resolution = resolution
    }
    pub fn new(position: Vec3, rotation: Vec3) -> Self {
        //! Constructor for Camera
        //! ## Parameter
        //! 'position': Position of the new Camera as glam::Vec3
        //! 'rotation': Rotation of the new Camera as glam::Vec3
        //! # Returns
        //! A new camera with the given position and rotation
        Camera {
            position,
            rotation,
            fov: 1.0,
            resolution: Resolution::default(),
            ray_samples: 20,
        }
    }
    pub fn get_ray_samples(&self) -> u32 {
        self.ray_samples
    }
    pub fn set_ray_samples(&mut self, samples: u32) {
        self.ray_samples = samples;
        // here could be a check for values [1, 100] or so
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::default(),
            rotation: Vec3::new(0.0, 0.0, 1.0),
            fov: 10.0,
            resolution: Resolution::default(),
            ray_samples: 20,
        }
    }
}

impl std::fmt::Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Camera at {}", self.get_position())
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}
// todo: later add implementation for min, max, hd,uhd, ...
impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
        }
    }
}
