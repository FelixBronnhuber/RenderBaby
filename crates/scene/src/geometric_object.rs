use glam

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
    points: Vec<vec3>,
    material: Material
}
struct Camera{
    position: Position,
    rotation: Rotation
}

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
    position: Position
}