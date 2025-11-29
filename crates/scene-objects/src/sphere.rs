use glam::Vec3;

use crate::{
    geometric_object::{GeometricObject, SceneObject, SceneObjectAttributes},
    material::Material,
};

#[allow(dead_code)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
    color: [f32; 3],
    attr: SceneObjectAttributes,
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
        let conf = SceneObjectAttributes {
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
}

impl SceneObject for Sphere {
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
