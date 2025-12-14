use scene_objects::{geometric_object::SceneObject, tri_geometry::TriGeometry};
use serde::{Deserialize, Serialize};
use crate::data_plane::scene_proxy::position::Position;

#[derive(Serialize, Deserialize)]
#[allow(unused)]
pub(crate) struct ProxyMesh {
    pub name: String,
    pub path: String,
    pub scale: Position,
    pub rotation: Position,
    pub translation: Position,
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
            scale: Position::new_from_vec3(mesh.get_scale()),
            rotation: Position::new_from_vec3(mesh.get_rotation()),
            translation: Position::new_from_vec3(mesh.get_translation()),
        }
    }
}
