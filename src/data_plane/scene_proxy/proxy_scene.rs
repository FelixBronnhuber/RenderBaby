use scene_objects::{camera::Camera, light_source::LightSource};
use serde::{Deserialize, Serialize};

use crate::data_plane::{
    scene::render_scene::Scene,
    scene_proxy::{proxy_mesh::ProxyMesh},
};

#[allow(unused)]
#[derive(Serialize, Deserialize)]
pub struct ProxyScene {
    pub scene_name: String,
    pub camera: Camera,
    pub objects: Vec<ProxyMesh>,
    pub lights: Vec<LightSource>,
    pub background_color: [f32; 3],
    pub misc: Vec<u32>, // temp type just for ray samples
}
#[allow(unused)]
impl ProxyScene {
    /*     pub fn new() -> Self {
        ProxyScene {
            objects: todo!(),
            scene_name: todo!(),
            camera: todo!(),
            lights: todo!(),
            background_color: todo!(),
            misc: todo!(),
        }
    } */
    pub fn new_from_real_scene(scene: &Scene) -> Self {
        let mut objects = vec![];
        for obj in scene.get_tri_geometries() {
            objects.push(ProxyMesh::new_from_real_mesh(obj));
        }
        Self {
            scene_name: scene.get_name().to_string(),
            camera: *scene.get_camera(),
            objects,
            lights: scene.get_light_sources().to_vec(),
            background_color: scene.get_background_color(),
            misc: vec![scene.get_camera().get_ray_samples()],
        }
    }
}
