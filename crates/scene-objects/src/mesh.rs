use glam::Vec3;
use anyhow::Error;

use crate::{
    geometric_object::{GeometricObject, SceneObject},
    material::Material,
};

/// This is where the mesh as an alternative to trigeometry will be implemented
#[allow(dead_code)]
#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<f32>,
    tris: Vec<u32>,
    materials: Vec<Material>,
    path: Option<String>,
    name: String,
    scale: Vec3,
    translation: Vec3,
    rotation: Vec3,
    centroid: Vec3,
}

#[allow(dead_code)]
impl Mesh {
    pub fn new(
        vertices: Vec<f32>,
        tris: Vec<u32>,
        materials: Vec<Material>,
    ) -> Result<Self, Error> {
        //! Constructor for new Mesh
        match Self::calculate_centroid(&vertices) {
            Ok(c) => Ok(Self {
                vertices,
                tris,
                materials,
                path: None,
                name: "new mash".to_owned(),
                scale: Vec3::new(1.0, 1.0, 1.0),
                rotation: Vec3::default(),
                translation: Vec3::default(),
                centroid: c,
            }),
            Err(error) => Err(Error::msg(format!("Failed to create new mesh: {error}"))),
        }
    }
    pub fn update_centroid(&mut self) -> Result<(), Error> {
        match Self::calculate_centroid(&self.vertices) {
            Ok(c) => {
                self.centroid = c;
                Ok(())
            }
            Err(error) => Err(Error::msg(format!(
                "Error: Failed to update centroid: {error}"
            ))),
        }
    }
    pub(crate) fn calculate_centroid(vertices: &[f32]) -> Result<Vec3, Error> {
        if vertices.is_empty() {
            return Err(Error::msg("Cannot calculate centroid of 0 points"));
        }
        if !vertices.len().is_multiple_of(3) {
            return Err(Error::msg(
                "Cannot calculate centroid: input length invalid",
            ));
        }
        let mut x_sum = 0.0;
        let mut y_sum = 0.0;
        let mut z_sum = 0.0;
        let len = vertices.len() / 3;
        for i in 0..len {
            x_sum += vertices[i * 3];
            y_sum += vertices[i * 3 + 1];
            z_sum += vertices[i * 3 + 2];
        }
        let len = len as f32;
        Ok(Vec3::new(x_sum / len, y_sum / len, z_sum / len))
    }
}

#[allow(unused)]
impl GeometricObject for Mesh {
    fn scale(&mut self, factor: f32) {
        todo!()
    }

    fn translate(&mut self, vec: glam::Vec3) {
        for i in 0..self.vertices.len() / 3 {
            self.vertices[i * 3] += vec.x;
            self.vertices[i * 3 + 1] += vec.y;
            self.vertices[i * 3 + 2] += vec.z;
        }
        self.translation += vec;
    }

    fn rotate(&mut self, vec: glam::Vec3) {
        todo!();
        self.rotation += vec;
    }
}

impl SceneObject for Mesh {
    fn get_path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    fn get_scale(&self) -> glam::Vec3 {
        self.scale
    }

    fn get_translation(&self) -> glam::Vec3 {
        self.translation
    }

    fn get_rotation(&self) -> glam::Vec3 {
        self.rotation
    }
}
