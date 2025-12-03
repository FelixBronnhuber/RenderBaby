use glam::Vec3;
/// Camera that is used to render scenes
#[allow(dead_code)]
pub struct Camera {
    position: Vec3,
    rotation: Vec3, // fov: f32 ?
    fov: f32,
    resolution: [u32; 2],
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
        //! 'rotation': glam::Vec3 of the new position
        self.rotation = rotation
    }
    pub fn get_position(&self) -> Vec3 {
        //! ## Returns
        //! Camera position as glam::Vec3
        self.position
    }
    pub fn get_rotation(&self) -> &Vec3 {
        //! ## Returns
        //! Camera rotation as glam::Vec3
        &self.rotation
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
    pub fn get_resolution(&self) -> [u32; 2] {
        //! ## Returns
        //! Camera resolution as Array of u32
        self.resolution
    }
    pub fn set_resolution(&mut self, resolution: [u32; 2]) {
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
            resolution: [1920, 1080],
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::default(),
            rotation: Vec3::default(),
            fov: 1.0,
            resolution: [1920, 1080],
        }
    }
}
