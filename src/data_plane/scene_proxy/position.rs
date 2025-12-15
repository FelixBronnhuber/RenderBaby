use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Vec3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3d {
    pub fn new_from_vec3(vec: Vec3) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

impl PartialEq<Vec3> for Vec3d {
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl From<Vec3d> for Vec3 {
    fn from(value: Vec3d) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}
