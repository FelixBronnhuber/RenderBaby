//! Sphere primitive for ray tracing.
//!
//! This module defines the [`Sphere`] struct, representing a sphere primitive
//! with position, radius, and material properties.

use core::fmt;
use bytemuck::{Pod, Zeroable};
use crate::{Material, vec3::Vec3};

/// A sphere primitive for ray tracing.
///
/// `Sphere` represents a sphere in 3D space, defined by a center point and radius.
/// Each sphere has an associated material that determines its visual appearance.
///
/// # Memory Layout
///
/// The struct uses `#[repr(C)]` to ensure consistent memory layout for GPU transmission.
///
/// # Examples
///
/// ```rust,ignore
/// use engine_config::{Sphere, Vec3, Material};
///
/// // Create a red sphere at the origin
/// let sphere = Sphere::new(
///     Vec3::ZERO,
///     1.0,
///     Material::new_temp(Vec3::COLOR_RED).unwrap()
/// ).unwrap();
///
/// // Create a sphere with default settings
/// let default_sphere = Sphere::default();
/// ```
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Sphere {
    /// Center position of the sphere in world space.
    pub center: Vec3,
    /// Radius of the sphere. Must be positive.
    pub radius: f32,
    /// Material properties determining visual appearance.
    pub material: Material,
}

/// Errors that can occur when creating a sphere.
#[derive(Debug)]
pub enum SphereError {
    /// The sphere radius is zero or negative.
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

    /// Creates a new sphere with validation.
    ///
    /// # Arguments
    ///
    /// * `center` - The center position in world space
    /// * `radius` - The sphere radius (must be positive)
    /// * `material` - The material properties
    ///
    /// # Returns
    ///
    /// * `Ok(Sphere)` - A valid sphere
    /// * `Err(SphereError::RadiusOutOfBounds)` - If radius is zero or negative
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
    /// Creates a default sphere.
    ///
    /// Default values:
    /// - Center: origin (0, 0, 0)
    /// - Radius: 1.0
    /// - Material: default material
    fn default() -> Self {
        Sphere {
            center: Self::DEFAULT_CENTER,
            radius: Self::DEFAULT_RADIUS,
            material: Material::default(),
        }
    }
}
