use core::fmt;
use std::f32;

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
}

#[derive(Debug)]
pub enum CameraError {
    WidthOutOfBounds,
    HeightOutOfBounds,
    FOVOutOfBounds,
}

impl fmt::Display for CameraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CameraError::WidthOutOfBounds => write!(f, "Camera width is out of bounds"),
            CameraError::HeightOutOfBounds => write!(f, "Camera height is out of bounds"),
            CameraError::FOVOutOfBounds => write!(f, "Camera FOV is out of bounds"),
        }
    }
}

impl std::error::Error for CameraError {}

impl Camera {
    pub const DEFAULT_WIDTH: u32 = 1080;
    pub const DEFAULT_HEIGHT: u32 = 720;
    pub const DEFAULT_FOV: f32 = std::f32::consts::FRAC_PI_4;

    pub const MIN_WIDTH: u32 = 1;
    pub const MAX_WIDTH: u32 = 16_384;
    pub const MIN_HEIGHT: u32 = 1;
    pub const MAX_HEIGHT: u32 = 16_384;
    pub const MIN_FOV: f32 = 0.01;
    pub const MAX_FOV: f32 = f32::consts::PI * 10.0;

    pub fn new(width: u32, height: u32, fov: f32) -> Result<Self, CameraError> {
        if width < Self::MIN_WIDTH || width > Self::MAX_WIDTH {
            return Err(CameraError::WidthOutOfBounds);
        }
        if height < Self::MIN_HEIGHT || height > Self::MAX_HEIGHT {
            return Err(CameraError::HeightOutOfBounds);
        }
        if fov < Self::MIN_FOV || fov > Self::MAX_FOV {
            return Err(CameraError::FOVOutOfBounds);
        }
        Ok(Self { width, height, fov })
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            width: Self::DEFAULT_WIDTH,
            height: Self::DEFAULT_HEIGHT,
            fov: Self::DEFAULT_FOV,
        }
    }
}
