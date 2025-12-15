use serde::{Deserialize, Serialize};

use crate::data_plane::{
    scene::render_scene::Scene,
    scene_proxy::{proxy_camera::ProxyCamera, proxy_light::ProxyLight, proxy_mesh::ProxyMesh},
};

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct ProxyScene {
    pub scene_name: String,
    pub camera: ProxyCamera,
    pub objects: Vec<ProxyMesh>,
    pub lights: Vec<ProxyLight>,
    pub background_color: [f32; 3], // todo: change to Color
    pub misc: Vec<u32>,             // temp type just for ray samples, maybe replace with map
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
            background_color: scene.get_background_color(),
            misc: vec![scene.get_camera().get_ray_samples()],
        }
    }
}
