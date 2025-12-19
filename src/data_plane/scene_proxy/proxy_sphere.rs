use scene_objects::sphere::Sphere;
use serde::{Deserialize, Serialize};

use crate::data_plane::scene_proxy::{color::Color, position::Vec3d};
#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ProxySphere {
    pub radius: f32,
    pub center: Vec3d,
    pub color: Color,
}
#[allow(unused)]
impl ProxySphere {
    pub fn new_from_real_sphere(sphere: &Sphere) -> Self {
        Self {
            radius: sphere.get_radius(),
            center: sphere.get_center().into(),
            color: sphere.get_color().into(),
        }
    }
}
