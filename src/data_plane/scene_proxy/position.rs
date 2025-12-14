use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Position {
    pub fn new_from_vec3(vec: Vec3) -> Self {
        Self {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}
