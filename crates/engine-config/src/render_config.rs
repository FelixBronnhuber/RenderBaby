use crate::*;
use core::fmt;

use anyhow::Result;

// This is a placeholder and should be granularized further into e.g:
// CameraConfiguration, Scene, ...
pub struct RenderConfig {
    pub camera: Camera,
    pub spheres: Vec<Sphere>,
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
}

impl RenderConfigBuilder {
    pub fn new() -> Self {
        Self {
            camera: None,
            spheres: None,
        }
    }

    pub fn camera(mut self, camera: Camera) -> Result<Self> {
        self.camera = Some(camera);
        Ok(self)
    }

    pub fn spheres(mut self, spheres: Vec<Sphere>) -> Result<Self> {
        self.spheres = Some(spheres);
        Ok(self)
    }

    pub fn build(self) -> Result<RenderConfig> {
        let rc = RenderConfig {
            camera: self.camera.unwrap_or_default(),
            spheres: self.spheres.unwrap_or_default(),
        };

        Ok(rc)
    }
}

#[derive(Debug)]
pub enum RenderConfigBuilderError {
    FOVOutOfBounds,
}

impl fmt::Display for RenderConfigBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderConfigBuilderError::FOVOutOfBounds => write!(f, "FOV is out of bounds"),
        }
    }
}

impl std::error::Error for RenderConfigBuilderError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_defaults() {
        let builder = RenderConfigBuilder::new();
        let config = builder.build().unwrap();
        assert_eq!(config.camera.width, Camera::DEFAULT_WIDTH);
        assert_eq!(config.camera.height, Camera::DEFAULT_HEIGHT);
        assert_eq!(config.camera.fov, Camera::DEFAULT_FOV);
        assert!(config.spheres.is_empty());
    }

    #[test]
    fn builder_sets_camera_and_spheres() {
        let camera = Camera::new(800, 600, 1.0).unwrap();
        let sphere =
            Sphere::new(Vec3::new(1.0, 2.0, 3.0), 2.0, Vec3::ONE.scale(0.5)).unwrap();
        let builder = RenderConfigBuilder::new()
            .camera(camera)
            .unwrap()
            .spheres(vec![sphere])
            .unwrap();
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
