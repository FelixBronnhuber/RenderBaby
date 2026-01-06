use std::path::PathBuf;
use scene_objects::{geometric_object::SceneObject, mesh::Mesh};
use serde::{Deserialize, Serialize};
use crate::data_plane::scene_proxy::position::Vec3d;

#[derive(Serialize, Deserialize)]
#[allow(unused)]
pub(crate) struct ProxyMesh {
    pub name: String,
    pub path: PathBuf,
    pub scale: Vec3d,
    pub rotation: Vec3d,
    pub translation: Vec3d,
}

impl ProxyMesh {
    pub fn new_from_real_mesh(mesh: &Mesh) -> Self {
        Self {
            name: mesh.get_name(),
            path: { mesh.get_path().unwrap_or_else(|| PathBuf::new()) },
            scale: mesh.get_scale().into(),
            rotation: mesh.get_rotation().into(),
            translation: mesh.get_translation().into(),
        }
    }
}

impl Default for ProxyMesh {
    fn default() -> Self {
        Self {
            name: "Mesh".to_string(),
            path: PathBuf::from("./"),
            scale: Vec3d {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            rotation: Vec3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            translation: Vec3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}
