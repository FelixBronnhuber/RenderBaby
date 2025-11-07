use glam::{Vec3};

/*pub enum GeometricObject{
    Triangles(Vec<Triangle>),
    Sphere(Sphere)
}*/

//marker for now
pub trait GeometricObject {
    //todo!();
    fn scale(&mut self, factor: f32);
    fn translate(&mut self, vec: Vec3);
    fn rotate(&mut self, vec: Vec3);

}
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material
}
impl Sphere {
    

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }
    
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
    
    pub fn get_center(&self) -> Vec3 {
        self.center
    }

    pub fn set_senter(&mut self, center: Vec3) {
        self.center = center;
    }

    
    pub fn get_material(&self) -> &Material {
        &self.material
    }
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self { center, radius, material }
    }
}
impl GeometricObject for Sphere{
    fn scale(&mut self, factor: f32){
        self.radius *= factor;
        //self.radius
    }
    fn translate(&mut self, vec: Vec3){
        self.center += vec;
        //self.center
    }
    fn rotate(&mut self, vec: Vec3) {}
}

pub struct TriGeometry {
    triangles: Vec<Triangle>
}  
impl GeometricObject for TriGeometry {
    fn scale(&mut self, factor: f32) {
        for tri in self.get_triangles(){
            tri.scale(factor);
        }
    }

    fn translate(&mut self, vec: Vec3) {
        for tri in self.get_triangles() {
            tri.translate(vec);
        }
    }

    fn rotate(&mut self, vec: Vec3) {
        todo!()
    }
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
    material: Option<Material>
}
impl Triangle {
    pub fn new(points: Vec<Vec3>, material: Option<Material>) -> Self {
        Triangle { points, material }
    }
    pub fn get_points(&self) -> &Vec<Vec3> {
        &self.points
    }
    /*pub fn add_point(&mut self, point: Vec3) {
        self.points.push(point);
    }*/
    pub fn get_material(&self) -> &Option<Material> {
        &self.material
    }
    pub fn set_material(&mut self, material: Option<Material>) {
        self.material = material;
    }
    pub fn set_points(&mut self, points: Vec<Vec3>) {
        self.points = points;
        // todo: check for points length, otherwise error
    }
    
}
impl GeometricObject for Triangle {
    fn translate(&mut self, vec: Vec3)/* -> &Vec<Vec3>*/  {
        for point in &mut self.points {
            *point += vec;
        }
        //self.get_points()
    }
    
    fn scale(&mut self, factor: f32) {
        todo!()
    }
    
    fn rotate(&mut self, vec: Vec3) {
        todo!()
    }

}

pub struct Camera{
    position: Vec3,
    rotation: Rotation
    // fov: f32 ?
}
impl Camera{
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
    pub fn set_rotation(&mut self, pitch: f32, yaw: f32){
        self.rotation = Rotation { pitch, yaw }
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }
    pub fn rotation(&self) -> &Rotation {
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
    pub fn set(&mut self,pitch:f32,yaw:f32){
        self.pitch = pitch;
        self.yaw = yaw;
    }
    pub fn get_rotation(&self) -> (f32, f32) {(self.pitch,self.yaw)} // maybe into (f32, f32)?
    fn get_pitch(&self) -> f32 {
        self.pitch
    }
    pub fn get_yaw(&self) -> f32 {
        self.yaw
    }
    pub fn new(pitch:f32, yaw:f32) -> Self {
        Rotation { pitch, yaw }
    }
}
pub struct LightSource{
    position: Vec3,
    luminosity: f32
}

impl LightSource {
    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position
    }

    pub fn get_luminositoy(&self) -> f32 {
        self.luminosity
    }

    pub fn set_luminosity(&mut self, luminosity: f32) {
        self.luminosity = luminosity
    }

    pub fn new(position: Vec3, luminosity: f32) -> Self {
        LightSource { position, luminosity }
    }
}

// Maybe Material/Color/Texture as enum?
pub struct Material{

}
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8

}