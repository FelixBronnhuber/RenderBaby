use serde::{Deserialize, Serialize};

use crate::data_plane::{scene::render_scene::Scene, scene_proxy::proxy_sphere::ProxySphere};
#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Misc {
    pub ray_samples: u32,
    pub color_hash_enabled: bool,
    pub spheres: Vec<ProxySphere>,
}
impl Misc {
    pub fn new_from_scene(scene: &Scene) -> Self {
        let mut spheres = vec![];
        for sphere in scene.get_spheres() {
            spheres.push(ProxySphere::new_from_real_sphere(sphere))
        }
        Self {
            ray_samples: scene.get_camera().get_ray_samples(),
            color_hash_enabled: scene.get_color_hash_enabled(),
            spheres: spheres,
        }
    }
}
impl Default for Misc {
    fn default() -> Self {
        Self {
            ray_samples: 50,
            color_hash_enabled: false,
            spheres: vec![],
        }
    }
}
