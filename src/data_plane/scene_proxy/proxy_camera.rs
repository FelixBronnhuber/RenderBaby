use scene_objects::camera::{Camera};
use serde::{Deserialize, Serialize};

use crate::data_plane::scene_proxy::position::Vec3d;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[allow(unused)]
pub(crate) struct ProxyCamera {
    pub position: Vec3d,
    pub pane_distance: f32,
    pub pane_width: f32,
    pub resolution: [u32; 2],
    pub look_at: Vec3d,
    pub up: Vec3d,
}

impl ProxyCamera {
    pub(crate) fn new_from_real_camera(camera: &Camera) -> Self {
        Self {
            position: camera.get_position().into(),
            pane_distance: camera.get_pane_distance(),
            pane_width: camera.get_pane_width(),
            resolution: [
                camera.get_resolution().width,
                camera.get_resolution().height,
            ],
            look_at: camera.get_look_at().into(),
            up: camera.get_up().into(),
        }
    }
}

impl PartialEq<Camera> for ProxyCamera {
    fn eq(&self, other: &Camera) -> bool {
        self.position == other.get_position()
            && self.pane_distance == other.get_pane_distance()
            && self.pane_width == other.get_pane_width()
            && self.resolution[0] == other.get_resolution().width
            && self.resolution[1] == other.get_resolution().height
            && self.look_at == other.get_look_at()
            && self.up == other.get_up()
    }
}
