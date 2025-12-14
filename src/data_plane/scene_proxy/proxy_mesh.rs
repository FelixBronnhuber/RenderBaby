use glam::Vec3;
use scene_objects::{geometric_object::SceneObject, tri_geometry::TriGeometry};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[allow(unused)]
pub(crate) struct ProxyMesh {
    pub name: String,
    pub path: String,
    pub scale: Vec3,
    pub rotation: Vec3,
    pub translation: Vec3,
}

impl ProxyMesh {
    pub fn new_from_real_mesh(mesh: &TriGeometry) -> Self {
        Self {
            name: mesh.get_name(),
            path: {
                match mesh.get_path() {
                    Some(pa) => pa.to_owned(),
                    None => "".to_owned(),
                }
            },
            scale: mesh.get_scale(),
            rotation: mesh.get_rotation(),
            translation: mesh.get_translation(),
        }
    }
}
