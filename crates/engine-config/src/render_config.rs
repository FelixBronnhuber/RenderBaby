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

    pub fn camera(mut self, camera: Camera) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn spheres(mut self, spheres: Vec<Sphere>) -> Self {
        self.spheres = Some(spheres);
        self
    }

    pub fn build(self) -> Result<RenderConfig> {
        let camera = self.camera.ok_or(RenderConfigBuilderError::MissingCamera)?;
        let spheres = self
            .spheres
            .ok_or(RenderConfigBuilderError::MissingSpheres)?;

        let rc = RenderConfig { camera, spheres };

        Ok(rc)
    }
}

#[derive(Debug)]
pub enum RenderConfigBuilderError {
    FOVOutOfBounds,
    MissingCamera,
    MissingSpheres,
}

impl fmt::Display for RenderConfigBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderConfigBuilderError::FOVOutOfBounds => write!(f, "FOV is out of bounds"),
            RenderConfigBuilderError::MissingCamera => write!(f, "Camera is required"),
            RenderConfigBuilderError::MissingSpheres => write!(f, "Spheres are required"),
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
        let sphere =
            Sphere::new(Vec3::new(1.0, 2.0, 3.0), 2.0, Vec3::ONE.scale(0.5)).unwrap();
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
    fn builder_missing_camera() {
        let builder = RenderConfigBuilder::new().spheres(vec![]);
        let result = builder.build();
        assert!(
            matches!(result, Err(e) if e.downcast_ref::<RenderConfigBuilderError>()
            .map(|e| matches!(e, RenderConfigBuilderError::MissingCamera))
            .unwrap_or(false))
        );
    }

    #[test]
    fn builder_missing_spheres() {
        let camera = Camera::default();
        let builder = RenderConfigBuilder::new().camera(camera);
        let result = builder.build();
        assert!(
            matches!(result, Err(e) if e.downcast_ref::<RenderConfigBuilderError>()
            .map(|e| matches!(e, RenderConfigBuilderError::MissingSpheres))
            .unwrap_or(false))
        );
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
