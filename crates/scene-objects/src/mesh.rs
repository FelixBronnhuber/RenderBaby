use crate::geometric_object::{GeometricObject, SceneObject};

/// This is where the mesh as an alternative to trigeometry will be implemented
#[allow(dead_code)]
#[derive(Debug)]
pub struct Mesh {
    vertices: Vec<f32>,
    tris: Vec<u32>,
}

#[allow(dead_code)]
impl Mesh {
    fn new(vertices: Vec<f32>, tris: Vec<u32>) -> Self {
        //! Constructor for new Mesh
        Self { vertices, tris }
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
    }

    fn rotate(&mut self, vec: glam::Vec3) {
        todo!()
    }
}

impl SceneObject for Mesh {
    fn get_path(&self) -> String {
        todo!()
    }

    fn get_scale(&self) -> glam::Vec3 {
        todo!()
    }

    fn get_translation(&self) -> glam::Vec3 {
        todo!()
    }

    fn get_rotation(&self) -> glam::Vec3 {
        todo!()
    }
}
