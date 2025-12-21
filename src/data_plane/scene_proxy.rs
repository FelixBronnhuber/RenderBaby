use anyhow::Error;

use crate::data_plane::{scene::render_scene::Scene, scene_proxy::proxy_scene::ProxyScene};

pub mod color;
pub mod misc;
pub mod position;
pub mod proxy_camera;
pub mod proxy_light;
pub mod proxy_mesh;
pub mod proxy_scene;
pub mod proxy_sphere;

#[allow(unused)]
impl Scene {
    pub fn get_proxy_scene(&self) -> ProxyScene {
        ProxyScene::new_from_real_scene(self)
    }
    pub fn serialized(&self) -> Result<String, Error> {
        match serde_json::to_string(&self.get_proxy_scene()) {
            Ok(s) => Ok(s),
            Err(err) => Err(Error::msg(format!("Failed to serialize {self}: {err}"))),
        }
    }
}
