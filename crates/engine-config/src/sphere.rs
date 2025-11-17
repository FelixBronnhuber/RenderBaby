use core::fmt;

use bytemuck::{Pod, Zeroable};

use crate::vec3::Vec3;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Vec3,
    pub _pad: [u8; 4],
}

#[derive(Debug)]
pub enum SphereError {
    RadiusOutOfBounds,
    ColorOutOfBounds,
}

impl fmt::Display for SphereError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SphereError::RadiusOutOfBounds => write!(f, "Sphere radius must be positive"),
            SphereError::ColorOutOfBounds => {
                write!(f, "Sphere color values must be in [0.0, 1.0]")
            }
        }
    }
}

impl std::error::Error for SphereError {}

impl Sphere {
    pub const DEFAULT_CENTER: Vec3 = Vec3::ZERO;
    pub const DEFAULT_RADIUS: f32 = 1.0;
    pub const DEFAULT_COLOR: Vec3 = Vec3::COLOR_RED;

    pub fn new(center: Vec3, radius: f32, color: Vec3) -> Result<Sphere, SphereError> {
        if radius <= 0.0 {
            return Err(SphereError::RadiusOutOfBounds);
        }
        match color.is_valid_color() {
            false => return Err(SphereError::ColorOutOfBounds),
            true => Ok(Sphere {
                center,
                radius,
                color,
                _pad: [0u8; 4],
            }),
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: Self::DEFAULT_CENTER,
            radius: Self::DEFAULT_RADIUS,
            color: Self::DEFAULT_COLOR,
            _pad: [0u8; 4],
        }
    }
}
