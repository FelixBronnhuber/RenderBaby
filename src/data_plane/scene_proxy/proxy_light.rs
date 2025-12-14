use crate::data_plane::scene_proxy::{color::Color, position::Position};

pub(crate) struct ProxyLight {
    position: Position,
    luminosity: f32,
    name: String,
    color: Color,
}
