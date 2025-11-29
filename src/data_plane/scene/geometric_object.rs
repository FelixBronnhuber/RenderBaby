use glam::Vec3;
use std::any::Any;
#[allow(dead_code)]
pub trait GeometricObject {
    fn scale(&mut self, factor: f32); // TODO: scale 3d?
    fn translate(&mut self, vec: Vec3);
    fn rotate(&mut self, vec: Vec3);
    fn as_any(&self) -> &dyn Any;
    // TODO: color?
}
#[allow(dead_code)]
pub trait FileObject: GeometricObject {
    fn get_path(&self) -> String;
    //fn set_path(&mut self, path: String);
    fn get_scale(&self) -> Vec3;
    fn get_translation(&self) -> Vec3;
    fn get_rotation(&self) -> Vec3;
    // todo color?
}
#[allow(dead_code)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
    color: [f32; 3],
    attr: ObjConf,
}
#[allow(dead_code)]
impl Sphere {
    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }
    pub fn get_color(&self) -> [f32; 3] {
        self.color
    }
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

    pub fn new(center: Vec3, radius: f32, material: Material, color: [f32; 3]) -> Self {
        let conf = ObjConf {
            name: "New Sphere".to_owned(),
            path: /*Some("todo".to_owned())*/ None,
            scale: Vec3::new(0.0, 0.0, 0.0),
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
        };
        Self {
            center,
            radius,
            material,
            color,
            attr: conf,
        }
    }
}
impl GeometricObject for Sphere {
    fn scale(&mut self, factor: f32) {
        self.radius *= factor;
        //self.radius
    }
    fn translate(&mut self, vec: Vec3) {
        self.center += vec;
        //self.center
    }
    fn rotate(&mut self, _vec: Vec3) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl FileObject for Sphere {
    fn get_path(&self) -> String {
        //self.attr.path
        todo!()
    }

    fn get_scale(&self) -> Vec3 {
        self.attr.scale
    }

    fn get_translation(&self) -> Vec3 {
        self.attr.translation
    }

    fn get_rotation(&self) -> Vec3 {
        self.attr.translation
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct TriGeometry {
    triangles: Vec<Triangle>,
    attr: ObjConf,
    file_path: String,
    name: String,
    material: Material,
    //material: Option<Material>,
}
impl GeometricObject for TriGeometry {
    fn scale(&mut self, factor: f32) {
        for tri in self.get_triangles_mut() {
            tri.scale(factor);
        }
    }

    fn translate(&mut self, vec: Vec3) {
        for tri in self.get_triangles_mut() {
            tri.translate(vec);
        }
    }

    fn rotate(&mut self, _vec: Vec3) {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl FileObject for TriGeometry {
    fn get_path(&self) -> String {
        todo!()
    }

    fn get_scale(&self) -> Vec3 {
        todo!()
    }

    fn get_translation(&self) -> Vec3 {
        todo!()
    }

    fn get_rotation(&self) -> Vec3 {
        todo!()
    }
}
#[allow(dead_code)]
impl TriGeometry {
    pub fn get_triangles_mut(&mut self) -> &mut Vec<Triangle> {
        &mut self.triangles
    }
    pub fn get_triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_path(&self) -> String {
        self.file_path.clone()
    }
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
    pub fn new(triangles: Vec<Triangle>) -> Self {
        let conf = ObjConf {
            name: "".to_owned(),
            path: Some("".to_owned()),
            scale: Vec3::new(0.0, 0.0, 0.0),
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
        };
        TriGeometry {
            triangles,
            attr: conf,
            file_path: " ".to_owned(),
            name: "unnamed".to_owned(),
            material: Material::default(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct Triangle {
    points: Vec<Vec3>, // todo: Probably introduces a typ for 3 3dPoints
    material: Option<Material>,
}
#[allow(dead_code)]
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
    pub fn add_point(&mut self, point: Vec3) {
        self.points.push(point);
    }
}
impl GeometricObject for Triangle {
    fn translate(&mut self, vec: Vec3) /* -> &Vec<Vec3>*/
    {
        for point in &mut self.points {
            *point += vec;
        }
        //self.get_points()
    }

    fn scale(&mut self, _factor: f32) {
        todo!()
    }

    fn rotate(&mut self, _vec: Vec3) {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[allow(dead_code)]
pub struct Rotation {
    //no roll?
    pitch: f32,
    yaw: f32,
}
#[allow(dead_code)]
impl Rotation {
    pub fn set(&mut self, pitch: f32, yaw: f32) {
        self.pitch = pitch;
        self.yaw = yaw;
    }
    pub fn get_rotation(&self) -> (f32, f32) {
        (self.pitch, self.yaw)
    } // maybe into (f32, f32)?
    fn get_pitch(&self) -> f32 {
        self.pitch
    }
    pub fn get_yaw(&self) -> f32 {
        self.yaw
    }
    pub fn new(pitch: f32, yaw: f32) -> Self {
        Rotation { pitch, yaw }
    }
}
#[allow(dead_code)]
pub struct LightSource {
    position: Vec3,
    luminosity: f32,
    name: String,
    color: [f32; 3],
    rotation: Vec3,
    light_type: LightType,
}
#[allow(dead_code)]
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

    pub fn get_rotation(&self) -> Vec3 {
        self.rotation
    }
    pub fn rotate(&mut self, _vec: Vec3) -> Vec3 {
        todo!()
        // rotate and return new orientation?
    }
    pub fn get_light_type(&self) -> &LightType {
        &self.light_type
    }
    pub fn get_color(&self) -> [f32; 3] {
        self.color
    }
    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    // todo LightSource should also be a FileObject!
    pub fn new(
        position: Vec3,
        luminosity: f32,
        color: [f32; 3],
        name: String,
        rotation: Vec3,
        light_type: LightType,
    ) -> Self {
        LightSource {
            position,
            luminosity,
            name,
            color,
            rotation, // some types have no ratation
            light_type,
        }
    }
}
#[derive(Debug)]
#[allow(dead_code)]
struct ObjConf {
    pub name: String,
    pub path: Option<String>,
    pub scale: Vec3,
    pub translation: Vec3,
    pub rotation: Vec3,
}
#[allow(dead_code)]
pub enum LightType {
    Ambient,
    Point,
    Directional,
}
