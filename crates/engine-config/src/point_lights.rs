use bytemuck::{Pod, Zeroable};
use crate::{Material, Vec3};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct PointLight {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Default for PointLight {
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
    /// Emissive-only material (used for area lights)
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
    pub fn new(position: [f32; 3], radius: f32, luminosity: f32, color: [f32; 3]) -> Self {
        Self {
            center: Vec3(position),
            radius,
            material: Material::emissive(Vec3(color), luminosity),
        }
    }
}
