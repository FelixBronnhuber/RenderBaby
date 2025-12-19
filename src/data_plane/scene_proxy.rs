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
#[cfg(test)]
mod tests {
    /*use glam::Vec3;

    use crate::data_plane::{scene::render_scene::Scene, scene_proxy::position::Vec3d};*/
    #[test]
    fn basic_proxy_test() {
        /* let mut scene = Scene::new();
        let camera_pos = scene.get_camera().get_position();
        let diff = Vec3::new(1.0, 1.0, 1.0);
        scene.proxy_scene.camera.position = Vec3d {
            x: scene.proxy_scene.camera.position.x + diff.x,
            y: scene.proxy_scene.camera.position.y + diff.y,
            z: scene.proxy_scene.camera.position.z + diff.z,
        };
        if scene.update_from_proxy().is_ok() {
            assert_eq!(scene.get_camera().get_position() - camera_pos, diff);
        } else {
            panic!("Scene: Update from proxy failed")
        } */
    }

    #[test]
    fn object_test() {}
}
