use glam::Vec3;

use crate::geometric_object::{Camera, GeometricObject, LightSource, Rotation};

pub struct Scene {
    objects: Vec<GeometricObject>,
    camera: Camera,
    light_sources: Vec<LightSource>,
}
impl Scene {
    pub fn set_camera_position(&mut self, pos: Vec<f32>) {
        self.camera.set_position(Vec3::new(pos[0], pos[1], pos[2]));
    }
    pub fn set_camera_rotation(&mut self, pitch: f32, yaw: f32) {
        self.camera.set_rotation(pitch, yaw);
    }
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            camera: Camera::new(
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Rotation::new(0.0, 0.0),
            ),
            light_sources: Vec::new(),
        }
    }
    pub fn get_camera_rotation(&self) -> &Rotation {
        self.camera.rotation()
    }
}
/*pub fn set_camera_position(&mut self, position: Vec3){
    self.camera.set_position(position);
}
pub fn set_camera_rotation(&mut self, pitch:f32, yaw:f32){
    self.camera.set_rotation(pitch,yaw)
}
pub fn get_camera_position(&self) -> Vec3{
    self.camera.get_position()
}
pub fn get_camera_rotation(&self) -> &Rotation{
    self.camera.get_rotation()
}*/
// this will probably be part of scene graph?
