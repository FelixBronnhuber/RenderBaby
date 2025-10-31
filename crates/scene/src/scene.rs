pub struct Scene{
    object: GeometricObject,
    camera: Camera,
    light_source: LightSource
}
impl Scene{
    pub fn set_camera_position(&mut self, x:f32, y:f32, z:f32){
        self.camera.position.set(x, y, z);
    }
    pub fn set_camera_rotation(&mut self, pitch:f32, yaw:f32){
        self.camera.rotation.set(pitch,yaw)
    }
    pub fn get_camera_position(&self) -> (f32, f32, f32){
        self.camera.position.get_position()
    }
    pub fn get_camera_rotation(&self) -> (f32, f32){
        self.camera.rotation.get_rotation()
    }
}