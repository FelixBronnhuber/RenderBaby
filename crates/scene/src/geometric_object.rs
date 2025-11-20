use glam::Vec3;

pub enum GeometricObject {
    Triangles(Vec<Triangle>),
    Sphere(Sphere),
}
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
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
        Sphere {
            center,
            radius,
            material,
        }
    }
}
pub struct TriGeometry {
    triangles: Vec<Triangle>,
    material: Material,
}
impl TriGeometry {
    pub fn get_triangles(&self) -> &Vec<Triangle> {
        &self.triangles
    }
    pub fn get_material(&self) -> &Material {
        &self.material
    }
    pub fn new(triangles: Vec<Triangle>, material: Material) -> Self {
        TriGeometry {
            triangles,
            material,
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
    pub fn add_point(&mut self, point: Vec3) {
        self.points.push(point);
    }
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
    pub fn translate(&mut self, vec: Vec3) -> &Vec<Vec3> {
        for point in &mut self.points {
            *point += vec;
        }
        self.get_points()
    }
}
pub struct Camera {
    position: Vec3,
    rotation: Rotation,
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
    pub fn new(position: Vec3, rotation: Rotation) -> Self {
        Camera { position, rotation }
    }
}

pub(crate) struct Rotation {
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
}

impl LightSource {
    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn set_position(&mut self, position: Vec3) {
        self.position = position
    }

    fn get_luminositoy(&self) -> f32 {
        self.luminosity
    }

    fn set_luminosity(&mut self, luminosity: f32) {
        self.luminosity = luminosity
    }

    fn new(position: Vec3, luminosity: f32) -> Self {
        LightSource {
            position,
            luminosity,
        }
    }
}

// Maybe Material/Color/Texture as enum?
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
    pub rgb: u8,
}
