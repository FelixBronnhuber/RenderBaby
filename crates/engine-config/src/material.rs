//! Material properties for surfaces and lights.
//!
//! This module defines the [`Material`] struct, which describes how surfaces interact
//! with light using the Phong reflection model and extensions for physically-based rendering.

use bytemuck::{Pod, Zeroable};
use core::fmt;
use crate::Vec3;

/// Material properties for surfaces and lights.
///
/// `Material` encapsulates all surface properties needed for rendering, including:
/// - Phong reflection model components (ambient, diffuse, specular)
/// - Emissive lighting for area lights
/// - Physical properties (index of refraction, opacity)
/// - Texture mapping
///
/// The material model is based on the Wavefront OBJ/MTL format with extensions.
///
/// # Illumination Models
///
/// The `illum` field specifies the illumination model:
/// - `0`: Color only (no lighting)
/// - `1`: Diffuse lighting
/// - `2`: Diffuse + specular (Phong)
/// - `3`: Reflection
/// - `4`: Refraction + reflection (glass)
/// - `5`: Fresnel reflection + ray-traced reflection
/// - `6`: Transparency + Fresnel + ray-traced refraction
///
/// # Memory Layout
///
/// The struct uses padding (`_pad0`, `_pad1`, `_pad2`) to satisfy GPU alignment requirements.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Material {
    /// Ambient color (Ka) - color in shadow.
    pub ambient: [f32; 3],
    /// Padding for GPU alignment.
    pub _pad0: f32,
    /// Diffuse color (Kd) - main surface color under diffuse lighting.
    pub diffuse: Vec3,
    /// Padding for GPU alignment.
    pub _pad1: f32,
    /// Specular color (Ks) - color of specular highlights.
    pub specular: [f32; 3],
    /// Specular exponent (Ns) - shininess factor (0-1000).
    /// Higher values create tighter, sharper highlights.
    pub shininess: f32,
    /// Emissive color (Ke) - light emitted by the surface.
    /// Used for area lights and glowing materials.
    pub emissive: [f32; 3],
    /// Optical density / Index of refraction (Ni).
    /// Typical values: 1.0 (air), 1.33 (water), 1.5 (glass), 2.4 (diamond).
    pub ior: f32,
    /// Dissolve / Transparency (d or Tr).
    /// 0.0 = fully transparent, 1.0 = fully opaque.
    pub opacity: f32,
    /// Illumination model (0-10). See struct documentation for details.
    pub illum: u32,
    /// Index into the texture array (-1 = no texture).
    pub texture_index: i32,
    /// Padding for GPU alignment.
    pub _pad2: u32,
}

impl Default for Material {
    /// Creates a default diffuse material.
    ///
    /// Default values:
    /// - Diffuse: light gray (0.8, 0.8, 0.8)
    /// - Specular: bronze (1.0, 0.5, 0.3)
    /// - Shininess: 1000.0 (very shiny)
    /// - No emission
    /// - IOR: 1.0 (air)
    /// - Fully opaque
    /// - Illumination model 1 (diffuse only)
    fn default() -> Self {
        Self {
            ambient: [0.0, 0.0, 0.0],
            _pad0: 0.0,
            diffuse: Vec3::new(0.8, 0.8, 0.8),
            _pad1: 0.0,
            specular: [1.0, 0.5, 0.3],
            shininess: 1000.0,
            emissive: [0.0, 0.0, 0.0],
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

    /// Creates a complete material with all properties.
    ///
    /// # Arguments
    ///
    /// * `ambient` - Ambient color [r, g, b]
    /// * `diffuse` - Diffuse color (must be valid)
    /// * `specular` - Specular highlight color [r, g, b]
    /// * `shininess` - Specular exponent (0-1000, higher = shinier)
    /// * `emissive` - Emissive color [r, g, b]
    /// * `ior` - Index of refraction (typically 1.0-2.5)
    /// * `opacity` - Opacity (0.0-1.0)
    /// * `illum` - Illumination model (0-10)
    /// * `texture_index` - Index into texture array, or -1 for no texture
    ///
    /// # Returns
    ///
    /// * `Ok(Material)` - A fully configured material
    /// * `Err(MaterialError::ColorOutOfBounds)` - If diffuse color is invalid
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

/// Errors that can occur when creating materials.
#[derive(Debug)]
pub enum MaterialError {
    /// Color values are outside the valid range [0.0, 1.0].
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
