use scene_objects::light_source::LightSource;
use serde::{Deserialize, Serialize};

use crate::data_plane::scene_proxy::{color::Color, position::Vec3d};
#[derive(Serialize, Deserialize)]
pub(crate) struct ProxyLight {
    pub position: Vec3d,
    pub luminosity: f32,
    pub name: String,
    pub color: Color,
    #[serde(rename = "type")]
    pub light_type: String,
    // todo: 'type' is a rust keyword! rename in serialization
    pub rotation: Vec3d,
}

impl ProxyLight {
    pub(crate) fn new_from_real_light(light: &LightSource) -> Self {
        Self {
            position: Vec3d::new_from_vec3(light.get_position()),
            luminosity: light.get_luminositoy(),
            name: light.get_name().to_string(),
            color: Color::new_from_array(light.get_color()),
            light_type: light.get_light_type().as_string(),
            rotation: Vec3d::new_from_vec3(light.get_rotation()),
        }
    }
}
