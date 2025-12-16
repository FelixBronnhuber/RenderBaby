pub mod color;
pub mod position;
pub mod proxy_camera;
pub mod proxy_light;
pub mod proxy_mesh;
pub mod proxy_scene;
pub mod scene_composit;
pub mod util;

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::data_plane::{scene::render_scene::Scene, scene_proxy::position::Vec3d};
    #[test]
    fn basic_test() {
        let mut scene = Scene::new();
        let camera_pos = scene.get_camera().get_position();
        scene.proxy_scene.camera.position = Vec3d {
            x: scene.proxy_scene.camera.position.x + 1.0,
            y: scene.proxy_scene.camera.position.y + 1.0,
            z: scene.proxy_scene.camera.position.z + 1.0,
        };
        let _ = scene.update_from_proxy();
        let diff = scene.get_camera().get_position() - camera_pos;
        assert_eq!(diff, Vec3::new(1.0, 1.0, 1.0));
    }
}
