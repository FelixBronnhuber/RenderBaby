use serde::{Deserialize, Serialize};

use crate::data_plane::{
    scene::{render_parameter::RenderParameter, render_scene::Scene},
    scene_proxy::proxy_sphere::ProxySphere,
};
#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Misc {
    pub(crate) ray_samples: u32,
    pub(crate) spheres: Vec<ProxySphere>,
    pub(crate) render_param: RenderParameter,
}
impl Misc {
    pub fn new_from_scene(scene: &Scene) -> Self {
        let mut spheres = vec![];
        for sphere in scene.get_spheres() {
            spheres.push(ProxySphere::new_from_real_sphere(sphere))
        }
        Self {
            ray_samples: scene.get_camera().get_ray_samples(),
            spheres,
            render_param: scene.get_render_parameter(),
        }
    }
}
impl Default for Misc {
    fn default() -> Self {
        Self {
            ray_samples: 50,
            spheres: vec![],
            render_param: RenderParameter::default(),
        }
    }
}
