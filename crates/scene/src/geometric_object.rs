use glam::{Vec3};

pub enum GeometricObject{
    Triangles(Vec<Triangle>),
    Sphere(Sphere)
}
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material
}
impl Sphere {
    pub fn scale(&mut self, factor: f32) -> f32 {
        self.radius *= factor;
        self.radius
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
    
    pub fn center(&self) -> Vec3 {
        self.center
    }

    pub fn set_senter(&mut self, center: Vec3) {
        self.center = center;
    }

    pub fn translate(&mut self, vec: Vec3) -> Vec3 {
        self.center += vec;
        self.center
    }

    pub fn get_material(&self) -> &Material {
        &self.material
    }
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
    pub fn rotate(&mut self) {}

    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Sphere { center, radius, material }
    }

} 
pub struct TriGeometry {
    triangles: Vec<Triangle>
}  
impl TriGeometry{
    pub fn get_triangles(&self)-> &Vec<Triangle> {
        &self.triangles
    }

    pub fn new(triangles: Vec<Triangle>) -> Self {
        TriGeometry { triangles }
    }
}
pub struct Triangle{
    points: Vec<Vec3>, // todo: Probably introduces a typ for 3 3dPoints
    material: Material
}

impl Triangle {
    pub fn get_material(&self) -> &Material {
        &self.material
    }
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
    pub fn get_points(&self) -> &Vec<Vec3> {
        &self.points
    }
    pub fn set_points(&mut self, points: Vec<Vec3>) {
        self.points = points;
        // todo: check for points length, otherwise error
    }

    pub fn translate(&mut self, vec: Vec3) -> &Vec<Vec3> {
        for point in &mut self.points {
            *point += vec;
        }
        self.get_points()
    }

    pub fn new(points: Vec<Vec3>, material: Material) -> Self {
        // todo: Check length of points, throw error...
        Triangle { points, material }
    }
}
pub struct Camera{
    position: Vec3,
    rotation: Rotation
}
impl Camera{
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
    pub fn set_rotation(&mut self, pitch: f32, yaw: f32){
        self.rotation = Rotation { pitch, yaw }
    }
    pub fn get_position(&self) -> Vec3 {
        self.position
    }
    pub fn get_rotation(&self) -> &Rotation {
        &self.rotation
    }

    pub fn new(position: Vec3, rotation: Rotation) -> Self {
        Camera { position, rotation }
    }
}

pub(crate) struct Rotation{
    pitch: f32,
    yaw: f32,
}
impl Rotation{
    fn set(&mut self,pitch:f32,yaw:f32){
        self.pitch = pitch;
        self.yaw = yaw;
    }
    fn get_rotation(&self) -> (f32, f32) {(self.pitch,self.yaw)} // maybe into (f32, f32)?
    fn get_pitch(&self) -> f32 {
        self.pitch
    }
    fn get_yaw(&self) -> f32 {
        self.yaw
    }
}
pub struct LightSource{
    position: Vec3
}

pub struct Material{

}
pub struct Color{
    pub rgb: u8

}