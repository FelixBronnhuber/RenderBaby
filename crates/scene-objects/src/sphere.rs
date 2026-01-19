use glam::Vec3;
use crate::{
    geometric_object::{GeometricObject, SceneObject},
    material::Material,
};
use std::path::PathBuf;

#[allow(dead_code)]
#[derive(Debug)]
/// Simple Sphere defined by a 3d center and a radius
pub struct Sphere {
    center: Vec3,
    radius: f32, // maybe use radius only internally?
    material: Material,
    color: [f32; 3],
    name: String,
    path: Option<PathBuf>,
    scale: Vec3,
    translation: Vec3,
    rotation: Vec3,
}
#[allow(dead_code)]
impl Sphere {
    /// scales the sphere so that the given scale is the new scale
    /// ## Parameter
    /// 'scale': new absolute scale
    pub fn scale_to(&mut self, scale: f32) {
        let factor = self.scale.x * scale;
        if factor != 0.0 {
            self.scale(factor);
        }
    }
    /// translates the sphere so that the given translation is the new absolute translation
    /// ## Parameter
    /// 'translation': new absolute translation as glam::Vec3
    pub fn translate_to(&mut self, translation: Vec3) {
        self.translate(self.translation - translation);
    }
    /// Sets the LightSource color
    /// ## Parameter
    /// 'color': New LightSource color as array of f32, values in \[0, 1]
    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }
    /// ## Returns
    /// LightSource color as rgb array of f32, values in \[0, 1]
    pub fn get_color(&self) -> [f32; 3] {
        self.color
    }
    /// Sets the radius
    /// ## Parameter
    /// 'radius': new radius
    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }
    /// ## Returns
    /// Sphere radius
    pub fn get_radius(&self) -> f32 {
        self.radius
    }
    /// ## Returns
    /// Sphere center as glam::Vec3
    pub fn get_center(&self) -> Vec3 {
        self.center
    }
    /// sets the Sphere center
    /// ## Parameter
    /// 'center'
    pub fn set_center(&mut self, center: Vec3) {
        self.center = center;
    }
    /// ## Returns
    /// Reference to Sphere material
    pub fn get_material(&self) -> &Material {
        &self.material
    }
    /// Sets the Sphere Material
    /// ## Parameter
    /// 'material': New material
    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
    /// Constructor for a new sphere
    /// ## Params
    /// 'center': glam::Vec3 for the center <br>
    /// 'radius': radius around the center for the new Sphere <br>
    /// 'material': Material for the new Sphere <br>
    /// 'color': Array of f32 for the color of the new Sphere, values in \[0, 1]
    pub fn new(center: Vec3, radius: f32, material: Material, color: [f32; 3]) -> Self {
        Self {
            center,
            radius,
            material,
            color,
            name: "New Sphere".to_owned(),
            path: /*Some("todo".to_owned())*/ None,
            scale:  Vec3::new(1.0, 1.0, 1.0),
            translation: Vec3::default(),
            rotation: Vec3::default(),
        }
    }
}

impl SceneObject for Sphere {
    /// ## Returns
    /// Path of the reference file. Does a sphere need one?
    fn get_path(&self) -> Option<PathBuf> {
        self.path.clone()
    }
    /// ## Returns
    /// Scale in relation to the reference
    fn get_scale(&self) -> Vec3 {
        self.scale
    }
    /// ## Returns
    /// Translation in relation to the reference as glam::Vec3
    fn get_translation(&self) -> Vec3 {
        self.translation
    }
    /// ## Returns
    /// Rotation in relation
    fn get_rotation(&self) -> Vec3 {
        self.rotation
    }
}
impl GeometricObject for Sphere {
    /// scales the radius of the sphere
    /// ## Parameter
    /// 'factor': scale factor
    fn scale(&mut self, factor: f32) {
        self.radius *= factor;
        self.scale *= factor;
    }
    /// Moves the center of the sphere
    /// ## Parameter
    /// 'vec': Translation vector as glam::Vec3
    fn translate(&mut self, vec: Vec3) {
        self.center += vec;
        self.translation += vec;
    }
    /// Rotates the sphere? Rly? But it could be usefull if sphere is extended to have a texture
    fn rotate(&mut self, vec: Vec3) {
        self.rotation += vec;
    }
}
