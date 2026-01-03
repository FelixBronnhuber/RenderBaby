use bytemuck::{Pod, Zeroable};
use core::fmt;

use crate::Vec3;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Material {
    // Ambient color (Ka)
    pub ambient: [f32; 3],
    pub _pad0: f32,
    // Diffuse color (Kd) - main surface color
    pub diffuse: Vec3,
    pub _pad1: f32,
    // Specular color (Ks)
    pub specular: [f32; 3],
    // Specular exponent (Ns) - shininess (0-1000)
    pub shininess: f32,
    // Emissive color (Ke) - light emission
    pub emissive: [f32; 3],
    // Optical density / Index of refraction (Ni) - typically 1.0-2.5
    pub ior: f32,
    // Dissolve / Transparency (d or Tr) - 0.0 = transparent, 1.0 = opaque
    pub opacity: f32,
    // Illumination model (illum) - 0-10
    // 0: color, 1: diffuse, 2: specular, 3: reflection, 4: refraction + reflection, 5: Fresnel reflection + ray-traced reflection, 6: Transparency + Fresnel + ray-traced refraction
    pub illum: u32,
    pub texture_index: i32,
    _pad2: u32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: [0.0, 0.0, 0.0], //has to be fixed
            _pad0: 0.0,
            diffuse: Vec3::new(0.8, 0.8, 0.8),
            _pad1: 0.0,
            specular: [1.0, 0.5, 0.3],
            shininess: 1000.0,
            emissive: [150.0, 150.0, 150.0],
            ior: 1.0,
            opacity: 1.0,
            illum: 1,
            texture_index: -1,
            _pad2: 0,
        }
    }
}

impl Material {
    pub const DEFAULT_CENTER: Vec3 = Vec3::ZERO;
    pub const DEFAULT_RADIUS: f32 = 1.0;
    pub const DEFAULT_COLOR: Vec3 = Vec3::COLOR_RED;

    pub fn new_temp(diffuse: Vec3) -> Result<Material, MaterialError> {
        match diffuse.is_valid_color() {
            false => Err(MaterialError::ColorOutOfBounds),
            true => Ok(Self {
                diffuse,
                ..Default::default()
            }),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ambient: [f32; 3],
        diffuse: Vec3,
        specular: [f32; 3],
        shininess: f32,
        emissive: [f32; 3],
        ior: f32,
        opacity: f32,
        illum: u32,
        texture_index: i32,
    ) -> Result<Material, MaterialError> {
        match diffuse.is_valid_color() {
            false => Err(MaterialError::ColorOutOfBounds),
            true => Ok(Self {
                ambient,
                diffuse,
                specular,
                shininess,
                emissive,
                ior,
                opacity,
                illum,
                texture_index,
                ..Default::default()
            }),
        }
    }
}

#[derive(Debug)]
pub enum MaterialError {
    ColorOutOfBounds,
}

impl fmt::Display for MaterialError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &MaterialError::ColorOutOfBounds => {
                write!(f, "Sphere color values must be in [0.0, 1.0]")
            }
        }
    }
}
