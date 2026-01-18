use scene_objects::light_source::{LightSource};
use serde::{Deserialize, Serialize};

use crate::data_plane::scene_proxy::{color::Color, position::Vec3d};
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct ProxyLight {
    pub position: Vec3d,
    pub luminosity: f32,
    pub name: String,
    pub color: Color,
    #[serde(rename = "type")]
    //pub light_type: String,
    // todo: 'type' is a rust keyword! rename in serialization
    pub rotation: Vec3d,
}

impl ProxyLight {
    pub(crate) fn new_from_real_light(light: &LightSource) -> Self {
        Self {
            position: light.get_position().into(),
            luminosity: light.get_luminositoy(),
            name: light.get_name().to_string(),
            color: light.get_color().into(),
            //light_type: (*light.get_light_type()).into(),
            rotation: light.get_rotation().into(),
        }
    }
}

impl Default for ProxyLight {
    fn default() -> Self {
        Self {
            position: Vec3d {
                x: 0.0,
                y: 0.0,
                z: 5.0,
            },
            luminosity: 1.0,
            name: "Light".to_string(),
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
            //light_type: LightType::Point.into(),
            rotation: Vec3d {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
        }
    }
}

impl From<ProxyLight> for LightSource {
    fn from(value: ProxyLight) -> Self {
        Self::new(
            value.position.into(),
            value.luminosity,
            value.color.into(),
            value.name,
            value.rotation.into(),
            //value.light_type.into(),
        )
    }
}
