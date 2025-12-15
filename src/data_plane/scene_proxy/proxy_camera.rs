use scene_objects::camera::Camera;
use serde::{Deserialize, Serialize};

use crate::data_plane::scene_proxy::position::Vec3d;

#[derive(Serialize, Deserialize)]
#[allow(unused)]
pub(crate) struct ProxyCamera {
    position: Vec3d,
    pane_distance: f32,
    pane_width: f32,
    resolution: [u32; 2],
    look_at: Vec3d,
    up: Vec3d,
}

impl ProxyCamera {
    pub(crate) fn new_from_real_camera(camera: &Camera) -> Self {
        Self {
            position: Vec3d::new_from_vec3(camera.get_position()),
            pane_distance: camera.pane_distance,
            pane_width: camera.pane_width,
            resolution: [
                camera.get_resolution().width,
                camera.get_resolution().height,
            ],
            look_at: Vec3d::new_from_vec3(camera.get_look_at()),
            up: Vec3d::new_from_vec3(camera.up),
        }
    }
}

impl PartialEq<Camera> for ProxyCamera {
    fn eq(&self, other: &Camera) -> bool {
        self.position == other.get_position()
            && self.pane_distance == other.pane_distance
            && self.pane_width == other.pane_width
            && self.resolution[0] == other.get_resolution().width
            && self.resolution[1] == other.get_resolution().height
            && self.look_at == other.look_at
            && self.up == other.up
    }
}
