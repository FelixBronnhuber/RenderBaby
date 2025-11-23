use crate::*;
use core::fmt;

use anyhow::Result;

// This is a placeholder and should be granularized further into e.g:
// CameraConfiguration, Scene, ...
pub struct RenderConfig {
    pub camera: Camera,
    pub spheres: Vec<Sphere>,
    pub verticies: Vec<f32>,
    pub triangles: Vec<u32>,
}

impl RenderConfig {
    pub fn builder() -> RenderConfigBuilder {
        RenderConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct RenderConfigBuilder {
    camera: Option<Camera>,
    spheres: Option<Vec<Sphere>>,
    verticies: Option<Vec<f32>>,
    triangles: Option<Vec<u32>>,
}

impl RenderConfigBuilder {
    pub fn new() -> Self {
        Self {
            camera: None,
            spheres: None,
            verticies: None,
            triangles: None,
        }
    }

    pub fn camera(mut self, camera: Camera) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn spheres(mut self, spheres: Vec<Sphere>) -> Self {
        self.spheres = Some(spheres);
        self
    }

    pub fn verticies(mut self, verticies: Vec<f32>) -> Self {
        self.verticies = Some(verticies);
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
        self.verticies
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

    pub fn build(self) -> Result<RenderConfig> {
        let camera = match self.camera {
            Some(camera) => camera,
            None => {
                log::warn!(
                    "MissingCameraWarning: No camera provided, initializing with default camera."
                );
                Camera::default() // Replace with your actual default if needed
            }
        };

        let spheres = match self.spheres {
            Some(spheres) => spheres,
            None => {
                log::warn!(
                    "MissingSpheresWarning: No spheres provided, initializing with empty vector."
                );
                Vec::new()
            }
        };

        let verticies = match self.verticies {
            Some(verticies) => verticies,
            None => {
                log::warn!(
                    "MissingVerticiesWarning: No verticies provided, initializing with empty vector."
                );
                Vec::new()
            }
        };

        let triangles = match self.triangles {
            Some(triangles) => triangles,
            None => {
                log::warn!(
                    "MissingTrianglesWarning: No triangles provided, initializing with empty vector."
                );
                Vec::new()
            }
        };

        let rc = RenderConfig {
            camera,
            spheres,
            verticies,
            triangles,
        };

        Ok(rc)
    }
}

#[derive(Debug)]
pub enum RenderConfigBuilderError {
    FOVOutOfBounds,
    MissingCamera,
    MissingSpheres,
    MisingVerticies,
    MisingTriangles,
}

impl fmt::Display for RenderConfigBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderConfigBuilderError::FOVOutOfBounds => write!(f, "FOV is out of bounds"),
            RenderConfigBuilderError::MissingCamera => write!(f, "Camera is required"),
            RenderConfigBuilderError::MissingSpheres => write!(f, "Spheres are required"),
            RenderConfigBuilderError::MisingVerticies => write!(f, "Verticies are required"),
            RenderConfigBuilderError::MisingTriangles => write!(f, "Triangles are required"),
        }
    }
}

impl std::error::Error for RenderConfigBuilderError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_defaults() {
        let camera = Camera::default();
        let builder = RenderConfigBuilder::new().camera(camera).spheres(vec![]);
        let config = builder.build().unwrap();
        assert_eq!(config.camera.width, Camera::DEFAULT_WIDTH);
        assert_eq!(config.camera.height, Camera::DEFAULT_HEIGHT);
        assert_eq!(config.camera.fov, Camera::DEFAULT_FOV);
        assert!(config.spheres.is_empty());
    }

    #[test]
    fn builder_sets_camera_and_spheres() {
        let camera = Camera::new(800, 600, 1.0).unwrap();
        let sphere = Sphere::new(Vec3::new(1.0, 2.0, 3.0), 2.0, Vec3::ONE.scale(0.5)).unwrap();
        let builder = RenderConfigBuilder::new()
            .camera(camera)
            .spheres(vec![sphere]);
        let config = builder.build().unwrap();
        assert_eq!(config.camera.width, 800);
        assert_eq!(config.camera.height, 600);
        assert_eq!(config.camera.fov, 1.0);
        assert_eq!(config.spheres.len(), 1);
        assert_eq!(config.spheres[0].radius, 2.0);
    }

    #[test]
    fn camera_invalid_width() {
        let result = Camera::new(0, 600, 1.0);
        assert!(matches!(result, Err(CameraError::WidthOutOfBounds)));
    }

    #[test]
    fn camera_invalid_fov() {
        let result = Camera::new(800, 600, 100.0);
        assert!(matches!(result, Err(CameraError::FOVOutOfBounds)));
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
