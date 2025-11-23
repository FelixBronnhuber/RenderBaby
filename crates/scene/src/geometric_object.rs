use glam::Vec3;
use std::any::Any;

/*pub enum GeometricObject{
    Triangles(Vec<Triangle>),
    Sphere(Sphere)
}*/

//marker for now
pub trait GeometricObject {
    fn scale(&mut self, factor: f32); // todo: scale 3d?
    fn translate(&mut self, vec: Vec3);
    fn rotate(&mut self, vec: Vec3);
    fn as_any(&self) -> &dyn Any;
    // todo color?
}

pub trait FileObject: GeometricObject {
    fn get_path(&self) -> String;
    //fn set_path(&mut self, path: String);
    fn get_scale(&self) -> Vec3;
    fn get_translation(&self) -> Vec3;
    fn get_rotation(&self) -> Vec3;
}
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
    color: [f32; 3],
    attr: ObjConf,
}
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
    fn rotate(&mut self, vec: Vec3) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl FileObject for Sphere {
    fn get_path(&self) -> String {
        //self.attr.path
        //todo!()
        "path".to_string()
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

pub struct TriGeometry {
    triangles: Vec<Triangle>,
    attr: ObjConf,
    file_path: String,
    name: String,
    material: Material,
}
impl GeometricObject for TriGeometry {
    fn scale(&mut self, factor: f32) {
        for tri in self.get_triangles() {
            tri.scale(factor);
        }
    }

    fn translate(&mut self, vec: Vec3) {
        for tri in self.get_triangles() {
            tri.translate(vec);
        }
    }

    fn rotate(&mut self, vec: Vec3) {
        //todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl FileObject for TriGeometry {
    fn get_path(&self) -> String {
        self.file_path.clone()
    }

    fn get_scale(&self) -> Vec3 {
        //todo!()
        Vec3::new(0.0,0.0,0.0)
    }

    fn get_translation(&self) -> Vec3 {
        //todo!()
        Vec3::new(0.0,0.0,0.0)
    }

    fn get_rotation(&self) -> Vec3 {
        //todo!()
        Vec3::new(0.0,0.0,0.0)
    }
}
impl TriGeometry {
    pub fn get_triangles(&mut self) -> &mut Vec<Triangle> {
        &mut self.triangles
        // maybe one fn for mut, one for immut?
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
pub struct Triangle {
    points: Vec<Vec3>, // todo: Probably introduces a typ for 3 3dPoints
    material: Option<Material>,
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

    fn scale(&mut self, factor: f32) {
        //todo!()
    }

    fn rotate(&mut self, vec: Vec3) {
        //todo!()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
pub struct Camera {
    position: Vec3,
    rotation: Rotation, // fov: f32 ?
    fov: f32,
    resolution: [usize; 2],
}
impl Camera {
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
    pub fn set_rotation(&mut self, pitch: f32, yaw: f32) {
        self.rotation = Rotation { pitch, yaw }
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }
    pub fn rotation(&self) -> &Rotation {
        &self.rotation
    }
    pub fn get_fov(&self) -> f32 {
        self.fov
    }
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }
    pub fn get_resolution(&self) -> [usize; 2] {
        self.resolution
    }
    pub fn set_resolution(&mut self, resolution: [usize; 2]) {
        self.resolution = resolution
    }
    pub fn new(position: Vec3, rotation: Rotation) -> Self {
        Camera {
            position,
            rotation,
            fov: 1.0,
            resolution: [1920, 1080],
        }
    }
}

pub struct Rotation {
    //no roll?
    pitch: f32,
    yaw: f32,
}
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
pub struct LightSource {
    position: Vec3,
    luminosity: f32,
    name: String,
    color: [f32; 3],
    rotation: Vec3,
    light_type: LightType,
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

    pub fn get_rotation(&self) -> Vec3 {
        self.rotation
    }
    pub fn rotate(&mut self, vec: Vec3) -> Vec3 {
        //todo!()
        // rotate and return new orientation?
        Vec3::new(0.0,0.0,0.0)
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_color(&self) -> [f32; 3] {
        self.color
    }
    pub fn get_light_type(&self) -> &LightType {
        &self.light_type
    }

    pub fn new(
        position: Vec3,
        luminosity: f32,
        color: [f32; 3],
        name: String,
        rotation: Rotation,
        light_type: LightType,
    ) -> Self {
        LightSource {
            position,
            luminosity,
            name,
            color,
            rotation: Vec3::new(0.0, 0.0, 0.0), // some types have no ratation
            light_type: LightType::Ambient,
        }
    }
}

#[derive(Debug)]
pub struct Material {
    ambient_reflectivity: Vec<f64>,  //Ka
    diffuse_reflectivity: Vec<f64>,  //Kd
    specular_reflectivity: Vec<f64>, //Ks
    shininess: f64,                  //Ns
    transparency: f64,               //d
}
impl Material {
    pub fn new(
        ambient_reflectivity: Vec<f64>,
        diffuse_reflectivity: Vec<f64>,
        specular_reflectivity: Vec<f64>,
        shininess: f64,
        transparency: f64,
    ) -> Self {
        Material {
            ambient_reflectivity,
            diffuse_reflectivity,
            specular_reflectivity,
            shininess,
            transparency,
        }
    }
    pub fn default() -> Self {
        Material {
            ambient_reflectivity: vec![0.0, 0.0, 0.0],
            diffuse_reflectivity: vec![0.0, 0.0, 0.0],
            specular_reflectivity: vec![0.0, 0.0, 0.0],
            shininess: 0.0,
            transparency: 0.0,
        }
    }
}
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

struct ObjConf {
    pub name: String,
    pub path: Option<String>,
    pub scale: Vec3,
    pub translation: Vec3,
    pub rotation: Vec3,
}

pub enum LightType {
    Ambient,
    Point,
    Directional,
}
