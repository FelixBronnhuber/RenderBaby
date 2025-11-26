use crate::*;
use core::fmt;

use anyhow::Result;

// This is a placeholder and should be granularized further into e.g:
// CameraConfiguration, Scene, ...
pub struct RenderConfig {
    pub uniforms: Uniforms,
    pub spheres: Vec<Sphere>,
    pub vertices: Vec<f32>,
    pub triangles: Vec<u32>,
}

impl RenderConfig {
    pub fn builder() -> RenderConfigBuilder {
        RenderConfigBuilder::default()
    }
}

#[derive(Default, Clone)]
pub struct RenderConfigBuilder {
    pub uniforms: Option<Uniforms>,
    pub spheres: Option<Vec<Sphere>>,
    pub vertices: Option<Vec<f32>>,
    pub triangles: Option<Vec<u32>>,
}

impl RenderConfigBuilder {
    pub fn new() -> Self {
        Self {
            uniforms: None,
            spheres: None,
            vertices: None,
            triangles: None,
        }
    }

    pub fn uniforms(mut self, uniforms: Uniforms) -> Self {
        self.uniforms = Some(uniforms);
        self
    }

    pub fn spheres(mut self, spheres: Vec<Sphere>) -> Self {
        self.spheres = Some(spheres);
        self
    }

    pub fn vertices(mut self, vertices: Vec<f32>) -> Self {
        self.vertices = Some(vertices);
        self
    }

    pub fn triangles(mut self, triangles: Vec<u32>) -> Self {
        self.triangles = Some(triangles);
        self
    }

    pub fn add_sphere(&mut self, sphere: Sphere) -> &mut Self {
        self.spheres.get_or_insert_with(Vec::new).push(sphere);
        self
    }

    pub fn add_vertex(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.vertices
            .get_or_insert_with(Vec::new)
            .extend_from_slice(&[x, y, z]);
        self
    }

    pub fn add_triangle(&mut self, v0: u32, v1: u32, v2: u32) -> &mut Self {
        self.triangles
            .get_or_insert_with(Vec::new)
            .extend_from_slice(&[v0, v1, v2]);
        self
    }

    pub fn add_quad(&mut self, v0: u32, v1: u32, v2: u32, v3: u32) -> &mut Self {
        self.add_triangle(v0, v1, v2);
        self.add_triangle(v0, v2, v3);
        self
    }

    pub fn build(self) -> Result<RenderConfig> {
        let uniforms = self.uniforms.unwrap_or_else(|| {
            log::warn!(
                "MissingUniformsWarning: No uniforms provided, initializing with default uniforms."
            );
            Uniforms::default()
        });

        let spheres = self.spheres.unwrap_or_else(|| {
            log::warn!(
                "MissingSpheresWarning: No spheres provided, initializing with empty vector."
            );
            Vec::new()
        });

        let vertices = self.vertices.unwrap_or_else(|| {
            log::warn!(
                "MissingVerticesWarning: No vertices provided, initializing with empty vector."
            );
            Vec::new()
        });

        let triangles = self.triangles.unwrap_or_else(|| {
            log::warn!(
                "MissingTrianglesWarning: No triangles provided, initializing with empty vector."
            );
            Vec::new()
        });

        let rc = RenderConfig {
            uniforms,
            spheres,
            vertices,
            triangles,
        };

        Ok(rc)
    }
}

#[derive(Debug)]
pub enum RenderConfigBuilderError {
    FOVOutOfBounds,
    MissingUniforms,
    MissingSpheres,
    MissingVertices,
    MissingTriangles,
}

impl fmt::Display for RenderConfigBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderConfigBuilderError::FOVOutOfBounds => write!(f, "FOV is out of bounds"),
            RenderConfigBuilderError::MissingUniforms => write!(f, "Uniforms are required"),
            RenderConfigBuilderError::MissingSpheres => write!(f, "Spheres are required"),
            RenderConfigBuilderError::MissingVertices => write!(f, "Vertices are required"),
            RenderConfigBuilderError::MissingTriangles => write!(f, "Triangles are required"),
        }
    }
}

impl std::error::Error for RenderConfigBuilderError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_defaults() {
        let uniforms = Uniforms::default();
        let builder = RenderConfigBuilder::new()
            .uniforms(uniforms)
            .spheres(vec![]);
        let config = builder.build().unwrap();
        assert_eq!(config.uniforms.width, Uniforms::default().width);
        assert_eq!(config.uniforms.height, Uniforms::default().height);
        assert_eq!(config.uniforms.fov, Uniforms::default().fov);
        assert!(config.spheres.is_empty());
    }

    #[test]
    fn builder_sets_uniforms_and_spheres() {
        let uniforms = Uniforms::new(800, 600, 1.0, 0, 0);
        let sphere = Sphere::new(Vec3::new(1.0, 2.0, 3.0), 2.0, Vec3::ONE.scale(0.5)).unwrap();
        let builder = RenderConfigBuilder::new()
            .uniforms(uniforms)
            .spheres(vec![sphere]);
        let config = builder.build().unwrap();
        assert_eq!(config.uniforms.width, 800);
        assert_eq!(config.uniforms.height, 600);
        assert_eq!(config.uniforms.fov, 1.0);
        assert_eq!(config.spheres.len(), 1);
        assert_eq!(config.spheres[0].radius, 2.0);
    }

    #[test]
    fn sphere_invalid_radius() {
        let result = Sphere::new(Vec3::ONE, 0.0, Vec3::COLOR_WHITE.scale(0.5));
        assert!(matches!(result, Err(SphereError::RadiusOutOfBounds)));
    }

    #[test]
    fn sphere_invalid_color() {
        let result = Sphere::new(Vec3::ZERO, 1.0, Vec3::new(-1.0, 0.0, -1.0));
        assert!(matches!(result, Err(SphereError::ColorOutOfBounds)));
    }
}
