//! Point light sources for scene illumination.
//!
//! This module defines the [`PointLight`] struct, representing spherical area lights
//! that emit light in all directions.

use bytemuck::{Pod, Zeroable};
use crate::{Material, Vec3};

/// A spherical area light source.
///
/// `PointLight` represents a spherical emissive object that illuminates the scene.
/// Unlike traditional point lights, these are actual geometric spheres with radius,
/// allowing for soft shadows and area lighting effects.
///
/// # Implementation Note
///
/// Despite the name "PointLight", these are actually spherical area lights. The name
/// is retained for compability reasons.
///
/// # Memory Layout
///
/// The struct uses `#[repr(C)]` to ensure consistent memory layout for GPU buffers.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct PointLight {
    /// Center position of the light in world space.
    pub center: Vec3,
    /// Radius of the light sphere.
    /// Larger radius creates softer shadows.
    pub radius: f32,
    /// Material properties (primarily emissive component for light emission).
    pub material: Material,
}

impl Default for PointLight {
    /// Creates a default point light.
    ///
    /// Default configuration:
    /// - Position: (2, 2, 1)
    /// - Radius: 0.5
    /// - Color: white
    /// - Luminosity: 150.0
    fn default() -> Self {
        let color = Vec3::new(1.0, 1.0, 1.0);
        let luminosity = 150.0;
        Self {
            center: Vec3::new(2.0, 2.0, 1.0),
            radius: 0.5,
            material: Material::emissive(color, luminosity),
        }
    }
}

impl Material {
    /// Creates an emissive-only material for area lights.
    ///
    /// This is a convenience method for creating light-emitting materials.
    /// All other material properties are set to zero except for the emissive component.
    ///
    /// # Arguments
    ///
    /// * `color` - The base color of the emitted light
    /// * `luminosity` - The brightness multiplier (higher = brighter)
    ///
    /// # Returns
    ///
    /// A material that emits light with color * luminosity.
    pub fn emissive(color: Vec3, luminosity: f32) -> Self {
        Self {
            ambient: [0.0, 0.0, 0.0],
            _pad0: 0.0,
            diffuse: Vec3::ZERO,
            specular: [0.0, 0.0, 0.0],
            _pad1: 0.0,
            shininess: 0.0,
            emissive: [
                color.x() * luminosity,
                color.y() * luminosity,
                color.z() * luminosity,
            ],
            ior: 1.0,
            opacity: 1.0,
            illum: 0,
            texture_index: -1,
            _pad2: 0,
        }
    }
}

impl PointLight {
    /// Creates a new point light with specified parameters.
    ///
    /// # Arguments
    ///
    /// * `position` - World space position [x, y, z]
    /// * `radius` - Radius of the light sphere (affects shadow softness)
    /// * `luminosity` - Brightness multiplier (higher = brighter)
    /// * `color` - RGB color of the emitted light [r, g, b] (each 0.0-1.0)
    ///
    /// # Returns
    ///
    /// A new `PointLight` configured with the specified parameters.
    pub fn new(position: [f32; 3], radius: f32, luminosity: f32, color: [f32; 3]) -> Self {
        Self {
            center: Vec3(position),
            radius,
            material: Material::emissive(Vec3(color), luminosity),
        }
    }
}
