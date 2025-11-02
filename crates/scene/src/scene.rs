use glam::Vec3;
use crate::geometric_object::GeometricObject;
use crate::geometric_object::Camera;
use crate::geometric_object::LightSource;
pub struct Scene{
    object: GeometricObject,
    camera: Camera,
    light_source: LightSource,
}
impl Scene{


    pub fn set_camera_position(&mut self, pos:Vec<f32>){
        self.camera.set_position(Vec3::new(pos[0],pos[1],pos[2]));
    }
    pub fn set_camera_rotation(&mut self, pitch:f32, yaw:f32){
        self.camera.set_rotation(pitch,yaw);
    }
    pub fn get_camera_position(&self) -> (Vec<f32>){
        vec![self.camera.position().x, self.camera.position().y, self.camera.position().z]
    }
    pub fn get_camera_rotation(&self) -> (Vec<f32>){
        vec![self.camera.rotation().get_rotation().0,self.camera.rotation().get_rotation().1]
    }
}