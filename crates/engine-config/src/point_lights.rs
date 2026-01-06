use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct PointLight {
    pub position: [f32; 3],
    pub luminosity: f32,
    pub color: [f32; 3],
    pub _pad: f32,
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            position: [2.0, 4.0, 1.0],
            luminosity: 20.0,
            color: [1.0, 1.0, 1.0],
            _pad: 0.0,
        }
    }
}

impl PointLight {
    pub fn new(position: [f32; 3], luminosity: f32, color: [f32; 3]) -> Self {
        Self {
            position,
            luminosity,
            color,
            ..Default::default()
        }
    }
}
