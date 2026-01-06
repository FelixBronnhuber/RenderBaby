use scene_objects::sphere::Sphere;
use serde::{Deserialize, Serialize};

use crate::data_plane::scene_proxy::{color::Color, position::Vec3d};
#[derive(Serialize, Deserialize, Debug)]
pub struct ProxySphere {
    pub radius: f32,
    pub center: Vec3d,
    pub color: Color,
}
impl ProxySphere {
    pub fn new_from_real_sphere(sphere: &Sphere) -> Self {
        Self {
            radius: sphere.get_radius(),
            center: sphere.get_center().into(),
            color: sphere.get_color().into(),
        }
    }
}

impl Default for ProxySphere {
    fn default() -> Self {
        Self {
            radius: 1.0,
            center: Vec3d {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
        }
    }
}
