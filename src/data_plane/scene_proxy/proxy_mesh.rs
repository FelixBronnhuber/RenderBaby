use glam::Vec3;
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
