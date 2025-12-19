use scene_objects::camera::Camera;
use serde::{Deserialize, Serialize};

use crate::data_plane::{
    scene::render_scene::Scene,
    scene_proxy::{
        color::Color, misc::Misc, proxy_camera::ProxyCamera, proxy_light::ProxyLight,
        proxy_mesh::ProxyMesh,
    },
};

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct ProxyScene {
    pub scene_name: String,
    pub camera: ProxyCamera,
    pub objects: Vec<ProxyMesh>,
    pub lights: Vec<ProxyLight>,
    pub background_color: Color, // todo: change to Color
    pub misc: Misc,              // temp type just for ray samples, maybe replace with map
}
// todo: ray samples are missing: have to be moved from camera to scene!
#[allow(unused)]
impl ProxyScene {
    pub fn new_from_real_scene(scene: &Scene) -> Self {
        let mut objects = vec![];
        for obj in scene.get_tri_geometries() {
            objects.push(ProxyMesh::new_from_real_mesh(obj));
        }
        let mut lights = vec![];
        for light in scene.get_light_sources() {
            lights.push(ProxyLight::new_from_real_light(light))
        }
        Self {
            scene_name: scene.get_name().to_string(),
            camera: ProxyCamera::new_from_real_camera(scene.get_camera()),
            objects,
            lights,
            background_color: scene.get_background_color().into(),
            misc: Misc::new_from_scene(scene),
        }
    }
}

impl Default for ProxyScene {
    fn default() -> Self {
        Self {
            scene_name: "scene".to_owned(),
            camera: ProxyCamera::new_from_real_camera(&Camera::default()),
            objects: vec![],
            lights: vec![],
            background_color: Color {
                r: 0.0,
                g: 0.0,
                b: 1.0,
            },
            misc: Misc::default(),
        }
    }
}
