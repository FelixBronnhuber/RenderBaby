use scene_objects::camera::Camera;
use serde::{Deserialize, Serialize};

use crate::data_plane::scene_proxy::position::Position;

#[derive(Serialize, Deserialize)]
#[allow(unused)]
pub(crate) struct ProxyCamera {
    position: Position,
    rotation: Position,
    fov: f32,
    resolution: [f32; 2],
}

impl ProxyCamera {
    pub(crate) fn new_from_real_camera(camera: &Camera) -> Self {
        todo!("camera needs scene standard first");
        /* Self {
            position: (),
            rotation: (),
            fov: (),
            resolution: (),
        } */
    }
}
