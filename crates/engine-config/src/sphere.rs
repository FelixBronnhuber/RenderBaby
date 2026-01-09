use core::fmt;

use bytemuck::{Pod, Zeroable};

use crate::{Material, vec3::Vec3};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

#[derive(Debug)]
pub enum SphereError {
    RadiusOutOfBounds,
}

impl fmt::Display for SphereError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SphereError::RadiusOutOfBounds => write!(f, "Sphere radius must be positive"),
        }
    }
}

impl std::error::Error for SphereError {}

impl Sphere {
    pub const DEFAULT_CENTER: Vec3 = Vec3::ZERO;
    pub const DEFAULT_RADIUS: f32 = 1.0;
    pub const DEFAULT_COLOR: Vec3 = Vec3::COLOR_RED;

    pub fn new(center: Vec3, radius: f32, material: Material) -> Result<Sphere, SphereError> {
        if radius <= 0.0 {
            return Err(SphereError::RadiusOutOfBounds);
        }
        Ok(Self {
            center,
            radius,
            material,
        })
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: Self::DEFAULT_CENTER,
            radius: Self::DEFAULT_RADIUS,
            material: Material::default(),
        }
    }
}
