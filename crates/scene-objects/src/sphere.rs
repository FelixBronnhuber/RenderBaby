use glam::Vec3;
use crate::{
    geometric_object::{GeometricObject, SceneObject},
    material::Material,
};

#[allow(dead_code)]
#[derive(Debug)]
/// Simple Sphere defined by a 3d center and a radius
pub struct Sphere {
    center: Vec3,
    radius: f32, // maybe use radius only internally?
    material: Material,
    color: [f32; 3],
    name: String,
    path: Option<String>,
    scale: Vec3,
    translation: Vec3,
    rotation: Vec3,
}
#[allow(dead_code)]
impl Sphere {
    pub fn set_color(&mut self, color: [f32; 3]) {
        //! Sets the Sphere color
        //! ## Parameter
        //! 'color': New LightSource color as array of f32, values in \[0, 1]
        self.color = color;
    }
    pub fn get_color(&self) -> [f32; 3] {
        //! ## Returns
        //! Sphere color as rgb array of f32, values in \[0, 1]
        self.color
    }
    pub fn set_radius(&mut self, radius: f32) {
        //! Sets the radius
        //! ## Parameter
        //! 'radius': new radius
        self.radius = radius;
    }

    pub fn get_radius(&self) -> f32 {
        //! ## Returns
        //! Sphere radius
        self.radius
    }

    pub fn get_center(&self) -> Vec3 {
        //! ## Returns
        //! Sphere center as glam::Vec3
        self.center
    }

    pub fn set_senter(&mut self, center: Vec3) {
        //! sets the Sphere center
        //! ## Parameter
        //! 'center'
        self.center = center;
    }

    pub fn get_material(&self) -> &Material {
        //! ## Returns
        //! Reference to Sphere material
        &self.material
    }
    pub fn set_material(&mut self, material: Material) {
        //! Sets the Sphere Material
        //! ## Parameter
        //! 'material': New material
        self.material = material;
    }

    pub fn new(center: Vec3, radius: f32, material: Material, color: [f32; 3]) -> Self {
        //! Constructor for a new sphere
        //! ## Params
        //! 'center': glam::Vec3 for the center <br>
        //! 'radius': radius around the center for the new Sphere <br>
        //! 'material': Material for the new Sphere <br>
        //! 'color': Array of f32 for the color of the new Sphere, values in \[0, 1]
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
    fn get_path(&self) -> Option<&str> {
        //! ## Returns
        //! Path of the reference file. Does a sphere need one?
        self.path.as_deref()
    }

    fn get_scale(&self) -> Vec3 {
        //! ## Returns
        //! Scale in relation to the reference
        self.scale
    }

    fn get_translation(&self) -> Vec3 {
        //! ## Returns
        //! Translation in relation to the reference as glam::Vec3
        self.translation
    }

    fn get_rotation(&self) -> Vec3 {
        //! ## Returns
        //! Rotation in relation
        self.rotation
    }
}
impl GeometricObject for Sphere {
    fn scale(&mut self, factor: f32) {
        //! scales the radius of the sphere
        //! ## Parameter
        //! 'factor': scale factor
        self.radius *= factor;
        self.scale *= factor;
    }
    fn translate(&mut self, vec: Vec3) {
        //! Moves the center of the sphere
        //! ## Parameter
        //! 'vec': Translation vector as glam::Vec3
        self.center += vec;
        self.translation += vec;
    }
    fn rotate(&mut self, vec: Vec3) {
        //! Rotates the sphere? Rly?
        self.rotation += vec;
    }
}
