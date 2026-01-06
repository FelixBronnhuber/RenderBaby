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
            path: {
                match mesh.get_path() {
                    Some(pa) => pa,
                    None => PathBuf::new(),
                }
            },
            scale: mesh.get_scale().into(),
            rotation: mesh.get_rotation().into(),
            translation: mesh.get_translation().into(),
        }
    }
}
