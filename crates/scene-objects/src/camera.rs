use glam::Vec3;

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
        self.position = position;
    }
    pub fn set_rotation(&mut self, rotation: Vec3) {
        self.rotation = rotation
    }
    pub fn get_position(&self) -> Vec3 {
        self.position
    }
    pub fn get_getrotation(&self) -> &Vec3 {
        &self.rotation
    }
    pub fn get_fov(&self) -> f32 {
        self.fov
    }
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }
    pub fn get_resolution(&self) -> [u32; 2] {
        self.resolution
    }
    pub fn set_resolution(&mut self, resolution: [u32; 2]) {
        self.resolution = resolution
    }
    pub fn new(position: Vec3, rotation: Vec3) -> Self {
        Camera {
            position,
            rotation,
            fov: 1.0,
            resolution: [1920, 1080],
        }
    }
}
