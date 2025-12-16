use scene_objects::camera::{Camera, Resolution};
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
impl From<ProxyCamera> for Camera {
    fn from(value: ProxyCamera) -> Self {
        let resolution = value.resolution;
        Self {
            position: value.position.into(),
            pane_distance: value.pane_distance,
            pane_width: value.pane_width,
            pane_height: value.pane_width * resolution[0] as f32 / resolution[0] as f32,
            look_at: value.look_at.into(),
            up: value.up.into(),
            resolution: Resolution {
                width: resolution[0],
                height: resolution[1],
            },
            ray_samples: 100,
        }
    }
}
