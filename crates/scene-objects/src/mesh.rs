use std::fmt::Display;
use std::path::PathBuf;
use glam::{Vec3, Mat3, EulerRot};
use anyhow::Error;
use log::debug;

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
    uvs: Option<Vec<f32>>,
    materials: Option<Vec<Material>>,
    material_index: Option<Vec<usize>>,
    path: Option<PathBuf>,
    name: String,
    scale: Vec3,
    translation: Vec3,
    rotation: Vec3,
    accumulated_rotation: Mat3,
    centroid: Vec3,
}

#[allow(unused)]
impl Mesh {
    pub fn new(
        vertices: Vec<f32>,
        tris: Vec<u32>,
        uvs: Option<Vec<f32>>,
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
                uvs,
                materials,
                material_index,
                path: _path,
                name: name.unwrap_or("unnamed mesh".to_owned()),
                scale: Vec3::new(1.0, 1.0, 1.0),
                rotation: Vec3::default(),
                translation: Vec3::default(),
                accumulated_rotation: Mat3::IDENTITY,
                centroid: c,
            }),
            Err(error) => Err(Error::msg(format!("Failed to create new mesh: {error}"))),
        }
    }

    pub fn get_materials(&self) -> Option<&Vec<Material>> {
        self.materials.as_ref()
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

    pub fn rotate_to(&mut self, rotation: Vec3) {
        //! rotates the mesh so that the given vector is the new rotation
        //! ## Parameter
        //! 'rotation': new absolute rotation as glam::Vec3 (Euler angles in degrees)
        debug!("Mesh {}: rotate_to absolute {:?}", self.name, rotation);

        // 1. Calculate target matrix
        let yaw = rotation.z.to_radians();
        let pitch = rotation.y.to_radians();
        let roll = rotation.x.to_radians();
        let target_matrix = Mat3::from_euler(EulerRot::ZYX, yaw, pitch, roll);

        // 2. Calculate delta matrix: Target = Delta * Current => Delta = Target * Current^-1
        // Since rotation matrices are orthogonal, Inverse = Transpose.
        let delta_rot = target_matrix * self.accumulated_rotation.transpose();

        // 3. Apply delta
        self.apply_rotation_matrix(delta_rot);

        // 4. Update state directly
        self.accumulated_rotation = target_matrix;
        self.rotation = rotation;
    }

    fn apply_rotation_matrix(&mut self, matrix: Mat3) {
        self.update_centroid();
        debug!(
            "Mesh {}: applying rotation matrix around centroid {:?}",
            self.name, self.centroid
        );
        for i in 0..self.vertices.len() / 3 {
            let p = Vec3::new(
                self.vertices[i * 3],
                self.vertices[i * 3 + 1],
                self.vertices[i * 3 + 2],
            );
            let p_centered = p - self.centroid;
            let p_rotated = matrix * p_centered;
            let p_final = p_rotated + self.centroid;

            self.vertices[i * 3] = p_final.x;
            self.vertices[i * 3 + 1] = p_final.y;
            self.vertices[i * 3 + 2] = p_final.z;
        }
        self.update_centroid();
    }

    pub fn scale_to(&mut self, scale: f32) {
        //! scales the mesh so that the given scale is the new scale
        //! ## Parameter
        //! 'scale': new absolute scale
        let factor = scale.abs() / self.scale.x;
        if factor != 0.0 {
            self.scale(factor);
        }
    }

    pub fn translate_to(&mut self, translation: Vec3) {
        //! translates the mesh so that the given translation is the new absolute translation
        //! ## Parameter
        //! 'translation': new absolute translation as glam::Vec3
        self.translate(translation - self.translation);
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
    pub fn get_uvs(&self) -> Option<&Vec<f32>> {
        self.uvs.as_ref()
    }
    pub fn get_tri_indices(&self) -> &Vec<u32> {
        //! Returns
        //! Reference to Vec<u32>, where three entries define the indices of the vertices that make up one triangle#
        &self.tris
    }
    pub fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    pub fn get_material_indices(&self) -> Option<&Vec<usize>> {
        self.material_index.as_ref()
    }
}

#[allow(unused)]
impl GeometricObject for Mesh {
    fn scale(&mut self, factor: f32) {
        //! scales the geometry by the given factor
        //! ## Parameter
        //! 'factor': factor for scale
        self.update_centroid();
        for i in 0..self.vertices.len() / 3 {
            self.vertices[i * 3] =
                self.centroid.x + (self.vertices[i * 3] - self.centroid.x) * factor;
            self.vertices[i * 3 + 1] =
                self.centroid.y + (self.vertices[i * 3 + 1] - self.centroid.y) * factor;
            self.vertices[i * 3 + 2] =
                self.centroid.z + (self.vertices[i * 3 + 2] - self.centroid.z) * factor;
        }
        // if switch to 3d transformation: use factor.x, factor.y, factor.z
        self.scale *= factor;
        self.update_centroid();
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
        self.update_centroid();
    }

    fn rotate(&mut self, vec: glam::Vec3) {
        //! Rotates the points around the centroid
        //! ## Parameter
        //! 'vec': Rotation: Euler angles in degree (Z, Y, X) = yaw, pitch, roll
        debug!("Mesh {}: rotate relative {:?}", self.name, vec);

        let yaw = vec.z.to_radians();
        let pitch = vec.y.to_radians();
        let roll = vec.x.to_radians();

        // Construct the rotation matrix for this operation (Rz * Ry * Rx)
        let delta_rot = Mat3::from_euler(EulerRot::ZYX, yaw, pitch, roll);

        self.apply_rotation_matrix(delta_rot);

        // Update accumulated rotation
        // The new accumulated rotation M' is delta_rot * M
        self.accumulated_rotation = delta_rot * self.accumulated_rotation;

        // Update self.rotation from the accumulated matrix
        let (z, y, x) = self.accumulated_rotation.to_euler(EulerRot::ZYX);
        self.rotation = Vec3::new(x.to_degrees(), y.to_degrees(), z.to_degrees());
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
