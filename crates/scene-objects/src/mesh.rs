use std::fmt::Display;
use std::path::PathBuf;
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
    materials: Option<Vec<Material>>,
    material_index: Option<Vec<usize>>,
    path: Option<PathBuf>,
    name: String,
    scale: Vec3,
    translation: Vec3,
    rotation: Vec3,
    centroid: Vec3,
}

#[allow(unused)]
impl Mesh {
    pub fn new(
        vertices: Vec<f32>,
        tris: Vec<u32>,
        materials: Option<Vec<Material>>,
        material_index: Option<Vec<usize>>,
        name: Option<String>,
        _path: Option<PathBuf>,
    ) -> Result<Self, Error> {
        //! Constructor for new Mesh
        match Self::calculate_centroid(&vertices) {
            Ok(c) => Ok(Self {
                vertices,
                tris,
                materials,
                material_index,
                path: _path,
                name: name.unwrap_or("unnamed mesh".to_owned()),
                scale: Vec3::new(1.0, 1.0, 1.0),
                rotation: Vec3::default(),
                translation: Vec3::default(),
                centroid: c,
            }),
            Err(error) => Err(Error::msg(format!("Failed to create new mesh: {error}"))),
        }
    }
    fn update_centroid(&mut self) -> Result<(), Error> {
        //! Calculates the centroid based on self.vertices
        //! Should be called after all changes to self.vertices
        //! ## Returns
        //! Result<(), Error>

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
        //! Calculates the centroid for a sclice of vertices, where 3 entries in the sclice are x, y, z of one point
        //! ## Parameter
        //! 'vertices': slice of f32 representing the vertices
        //! ## Returns
        //! Centroid of the given vertices as Result<Vec3, Error>
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

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_vertices(&self) -> &Vec<f32> {
        //! ## Returns
        //! Reference to Vec<f32>, where three entries define one point in 3d space
        &self.vertices
    }
    pub fn get_tri_indices(&self) -> &Vec<u32> {
        //! Returns
        //! Reference to Vec<u32>, where three entries define the indices of the vertices that make up one triangle#
        &self.tris
    }
    pub fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }
}

#[allow(unused)]
impl GeometricObject for Mesh {
    fn scale(&mut self, factor: f32) {
        //! scales the geometry by the given factor
        //! ## Parameter
        //! 'factor': factor for scale
        for i in 0..self.vertices.len() / 3 {
            self.vertices[i * 3] =
                self.centroid.x + (self.vertices[i * 3] - self.centroid.x) * factor;
            self.vertices[i * 3 + 1] =
                self.centroid.y + (self.vertices[i * 3 + 1] - self.centroid.y) * factor;
            self.vertices[i * 3 + 2] =
                self.centroid.z + (self.vertices[i * 3 + 2] - self.centroid.z) * factor;
        }
        // if switch to 3d transformation: use factor.x, factor.y, factor.z
        self.scale *= factor
    }

    fn translate(&mut self, vec: glam::Vec3) {
        //! translates the points by the direction given
        //! ## Parameter
        //! 'vec': Vector by which the geometries are translated
        for i in 0..self.vertices.len() / 3 {
            self.vertices[i * 3] += vec.x;
            self.vertices[i * 3 + 1] += vec.y;
            self.vertices[i * 3 + 2] += vec.z;
        }
        self.translation += vec;
    }

    fn rotate(&mut self, vec: glam::Vec3) {
        //! Rotates the points around the centroid
        //! ## Parameter
        //! 'vec': Rotation: Euler angles in degree (Z, Y, X) = yaw, pitch, roll
        // https://en.wikipedia.org/wiki/Euler_angles
        // https://en.wikipedia.org/wiki/Rotation_matrix

        let conv_factor = std::f32::consts::PI / 180.0;
        let yaw = vec.z * conv_factor;
        let pitch = vec.y * conv_factor;
        let roll = vec.x * conv_factor;

        let r = [
            [yaw.cos(), yaw.sin()],
            [pitch.cos(), pitch.sin()],
            [roll.cos(), roll.sin()],
        ];
        let rotate_x = [
            [1.0, 0.0, 0.0],
            [0.0, r[2][0], -r[2][1]],
            [0.0, r[2][1], r[2][0]],
        ];
        let rotate_y = [
            [r[1][0], 0.0, r[1][1]],
            [0.0, 1.0, 0.0],
            [-r[1][1], 0.0, r[1][0]],
        ];
        let rotate_z = [
            [r[0][0], -r[0][1], 0.0],
            [r[0][1], r[0][0], 0.0],
            [0.0, 0.0, 1.0],
        ];
        let multiplied = matrix_mult_helper(matrix_mult_helper(rotate_z, rotate_y), rotate_x);
        for i in 0..self.vertices.len() / 3 {
            let x_translated = self.vertices[i * 3] - self.centroid.x;
            let y_translated = self.vertices[i * 3 + 1] - self.centroid.y;
            let z_translated = self.vertices[i * 3 + 2] - self.centroid.z;
            self.vertices[i * 3] = multiplied[0][0] * x_translated
                + r[0][1] * y_translated
                + multiplied[0][2] * z_translated
                + self.centroid.x;
            self.vertices[i * 3 + 1] = multiplied[1][0] * x_translated
                + r[1][1] * y_translated
                + multiplied[1][2] * z_translated
                + self.centroid.x;
            self.vertices[i * 3 + 2] = multiplied[2][0] * x_translated
                + r[2][1] * y_translated
                + multiplied[2][2] * z_translated
                + self.centroid.x;
        }
        self.rotation += vec;
    }
}

impl SceneObject for Mesh {
    fn get_path(&self) -> Option<PathBuf> {
        self.path.clone()
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

fn matrix_mult_helper(a: [[f32; 3]; 3], b: [[f32; 3]; 3]) -> [[f32; 3]; 3] {
    //! Helper fn for matrix multiplication
    //! Parameter
    //! 'a', 'b': Arrays representing 3x3 matrices
    //! ## Returns
    //! a*b as [f32; 3]
    let mut res = [[0.0f32; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            res[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
        }
    }
    res
}

impl Display for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Mesh {}: {} vertices, {} triangles, center: {:?}",
            self.name,
            self.vertices.len() / 3,
            self.tris.len() / 3,
            self.centroid
        )
    }
}
