mod obj_parser;
use glam::Vec3;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Scene{
    object: GeometricObject,
    camera: Camera,
    light_source: LightSource
}
impl Scene{
    pub fn set_camera_position(&mut self, x:f32, y:f32, z:f32) -> (f32,f32,f32){
        (self.camera.position.x, self.camera.position.y, self.camera.position.z)
    }
    pub fn set_camera_rotation(&mut self, pitch:f32, yaw:f32){
        self.camera.rotation.set(pitch,yaw)
    }
    pub fn get_camera_position(&self) -> (f32, f32, f32){
        (self.camera.position.x, self.camera.position.y, self.camera.position.z)
    }
    pub fn get_camera_rotation(&self) -> (f32, f32){
        self.camera.rotation.get_rotation()
    }
}
struct GeometricObject{
    //one or more triangles
    triangles: Option<Vec<Triangle>>,
    circle: Option<Circle>
}
struct Circle {
    radius: f32,
    material: Material
}
struct Triangle{
    points: Vec3,
    material: Material
}
struct Camera{
    position: Vec3,
    rotation: Rotation
}
/*
pub struct Position{
    x: f32,
    y: f32,
    z: f32
}*//*
impl Position{
    fn set(&mut self,x:f32,y:f32,z:f32){
        self.x = x;
        self.y = y;
        self.z = z;
    }
    fn get_x(&self) -> f32{self.x}
    fn get_y(&self) -> f32{self.y}
    fn get_z(&self) -> f32{self.z}
    pub fn get_position(&self) -> (f32, f32, f32) {(self.x,self.y,self.z)}
}*/
struct Rotation{
    pitch: f32,
    yaw: f32,
}
impl Rotation{
    fn set(&mut self,pitch:f32,yaw:f32){
        self.pitch = pitch;
        self.yaw = yaw;
    }
    fn get_rotation(&self) -> (f32, f32) {(self.pitch,self.yaw)}
}
struct LightSource{
    position: Vec3
}
struct Material{

}
struct Color{

}

#[cfg(test)]
mod tests {
    use crate::obj_parser::parseobj;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

    }
    #[test]
    fn parse(){
        parseobj();
    }
}
