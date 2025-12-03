use crate::*;
use core::fmt;

pub struct RenderConfig {
    pub uniforms: Update<Uniforms>,
    pub spheres: Update<Vec<Sphere>>,
    pub vertices: Update<Vec<f32>>,
    pub triangles: Update<Vec<u32>>,
}

#[derive(Clone, Copy, Debug)]
pub enum Update<T> {
    Keep,
    Create(T),
    Update(T),
    Delete,
}

pub trait Validate {
    fn validate(&self) -> Result<&Self, RenderConfigBuilderError>;
}

pub trait ValidateInit {
    fn validate_init(&self) -> Result<&Self, RenderConfigBuilderError>;
}

impl RenderConfig {
    pub fn builder() -> RenderConfigBuilder {
        RenderConfigBuilder::default()
    }
}

impl ValidateInit for RenderConfig {
    fn validate_init(&self) -> Result<&Self, RenderConfigBuilderError> {
        if !matches!(self.uniforms, Update::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidUniforms);
        }
        if !matches!(self.spheres, Update::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidSpheres);
        }
        if !matches!(self.vertices, Update::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidVertices);
        }
        if !matches!(self.triangles, Update::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidTriangles);
        }
        Ok(self)
    }
}

impl Validate for RenderConfig {
    fn validate(&self) -> Result<&Self, RenderConfigBuilderError> {
        match &self.uniforms {
            Update::Update(u) => {
                if !(0.0 < u.fov && u.fov < 3.14) {
                    return Err(RenderConfigBuilderError::FOVOutOfBounds);
                }
                // Add more Uniforms validation as needed
            }
            Update::Delete => {
                // If deleting uniforms is not allowed, error
                return Err(RenderConfigBuilderError::CannotDeleteNonexistent);
            }
            Update::Keep => {}
            Update::Create(_) => todo!(),
        }

        match &self.spheres {
            Update::Update(spheres) => {
                if spheres.iter().any(|s| s.radius <= 0.0) {
                    return Err(RenderConfigBuilderError::InvalidSpheres);
                }
                // Add more Sphere validation as needed
            }
            Update::Delete => {
                // If deleting spheres is not allowed, error
                return Err(RenderConfigBuilderError::CannotDeleteNonexistent);
            }
            Update::Keep => {}
            Update::Create(_) => todo!(),
        }

        match &self.vertices {
            Update::Update(vertices) => {
                if vertices.len() % 3 != 0 {
                    return Err(RenderConfigBuilderError::InvalidVertices);
                }
            }
            Update::Delete => {
                return Err(RenderConfigBuilderError::CannotDeleteNonexistent);
            }
            Update::Keep => {}
            Update::Create(_) => todo!(),
        }

        match &self.triangles {
            Update::Update(triangles) => {
                if triangles.len() % 3 != 0 {
                    return Err(RenderConfigBuilderError::InvalidTriangles);
                }
            }
            Update::Delete => {
                return Err(RenderConfigBuilderError::CannotDeleteNonexistent);
            }
            Update::Keep => {}
            Update::Create(_) => todo!(),
        }

        Ok(self)
    }
}

#[derive(Default, Clone)]
pub struct RenderConfigBuilder {
    pub uniforms: Option<Update<Uniforms>>,
    pub spheres: Option<Update<Vec<Sphere>>>,
    pub vertices: Option<Update<Vec<f32>>>,
    pub triangles: Option<Update<Vec<u32>>>,
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
        self.uniforms = Some(Update::Update(uniforms));
        self
    }

    pub fn uniforms_no_change(mut self) -> Self {
        self.uniforms = Some(Update::Keep);
        self
    }

    pub fn uniforms_delete(mut self) -> Self {
        self.uniforms = Some(Update::Delete);
        self
    }

    pub fn spheres(mut self, spheres: Vec<Sphere>) -> Self {
        self.spheres = Some(Update::Update(spheres));
        self
    }

    pub fn spheres_no_change(mut self) -> Self {
        self.spheres = Some(Update::Keep);
        self
    }

    pub fn spheres_delete(mut self) -> Self {
        self.spheres = Some(Update::Delete);
        self
    }

    pub fn vertices(mut self, vertices: Vec<f32>) -> Self {
        self.vertices = Some(Update::Update(vertices));
        self
    }

    pub fn vertices_no_change(mut self) -> Self {
        self.vertices = Some(Update::Keep);
        self
    }

    pub fn vertices_delete(mut self) -> Self {
        self.vertices = Some(Update::Delete);
        self
    }

    pub fn triangles(mut self, triangles: Vec<u32>) -> Self {
        self.triangles = Some(Update::Update(triangles));
        self
    }

    pub fn triangles_no_change(mut self) -> Self {
        self.triangles = Some(Update::Keep);
        self
    }

    pub fn triangles_delete(mut self) -> Self {
        self.triangles = Some(Update::Delete);
        self
    }

    pub fn build(self) -> RenderConfig {
        if self.uniforms.is_none() {
            log::warn!("RenderConfigBuilder: uniforms not set, defaulting to NoChange.");
        }
        if self.spheres.is_none() {
            log::warn!("RenderConfigBuilder: spheres not set, defaulting to NoChange.");
        }
        if self.vertices.is_none() {
            log::warn!("RenderConfigBuilder: vertices not set, defaulting to NoChange.");
        }
        if self.triangles.is_none() {
            log::warn!("RenderConfigBuilder: triangles not set, defaulting to NoChange.");
        }

        RenderConfig {
            uniforms: self.uniforms.unwrap_or(Update::Keep),
            spheres: self.spheres.unwrap_or(Update::Keep),
            vertices: self.vertices.unwrap_or(Update::Keep),
            triangles: self.triangles.unwrap_or(Update::Keep),
        }
    }
}

#[derive(Debug)]
pub enum RenderConfigBuilderError {
    FOVOutOfBounds,
    InvalidUniforms,
    InvalidSpheres,
    InvalidVertices,
    InvalidTriangles,
    CannotDeleteNonexistent,
}

impl fmt::Display for RenderConfigBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderConfigBuilderError::FOVOutOfBounds => write!(f, "FOV is out of bounds"),
            RenderConfigBuilderError::InvalidUniforms => write!(f, "Invalid Uniforms"),
            RenderConfigBuilderError::InvalidSpheres => write!(f, "Invalid Spheres"),
            RenderConfigBuilderError::InvalidVertices => write!(f, "Invalid Vertices"),
            RenderConfigBuilderError::InvalidTriangles => write!(f, "Invalid Triangles"),
            RenderConfigBuilderError::CannotDeleteNonexistent => {
                write!(f, "Cannot delete none existent")
            }
        }
    }
}

impl std::error::Error for RenderConfigBuilderError {}

#[cfg(test)]
mod tests {}
