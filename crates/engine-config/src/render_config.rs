use crate::*;
use core::fmt;

pub struct RenderConfig {
    pub uniforms: Change<Uniforms>,
    pub spheres: Change<Vec<Sphere>>,
    pub vertices: Change<Vec<f32>>,
    pub triangles: Change<Vec<u32>>,
}

#[derive(Clone, Copy, Debug)]
pub enum Change<T> {
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
        if !matches!(self.uniforms, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidUniforms);
        }
        if !matches!(self.spheres, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidSpheres);
        }
        if !matches!(self.vertices, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidVertices);
        }
        if !matches!(self.triangles, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidTriangles);
        }
        Ok(self)
    }
}

impl Validate for RenderConfig {
    fn validate(&self) -> Result<&Self, RenderConfigBuilderError> {
        match &self.uniforms {
            Change::Update(u) | Change::Create(u) => {
                // TODO: Get fov limits from constants somewhere central / bound to the struct?
                if !(0.0..=100.0).contains(&u.camera.pane_distance) {
                    return Err(RenderConfigBuilderError::PaneDistanceOutOfBounds);
                }
                if !(0.0..=1000.0).contains(&u.camera.pane_width) {
                    return Err(RenderConfigBuilderError::PaneWidthOutOfBounds);
                }
                if is_zero(&u.camera.dir) {
                    return Err(RenderConfigBuilderError::InvalidCameraDirection);
                }
                // Add more Uniforms validation as needed
            }
            Change::Delete => {
                // If deleting uniforms is not allowed, error
                return Err(RenderConfigBuilderError::CannotDeleteNonexistent);
            }
            Change::Keep => {}
        }

        match &self.spheres {
            Change::Update(spheres) | Change::Create(spheres) => {
                if spheres.iter().any(|s| s.radius <= 0.0) {
                    return Err(RenderConfigBuilderError::InvalidSpheres);
                }
                // Add more Sphere validation as needed
            }
            Change::Delete => {
                // If deleting spheres is not allowed, error
                return Err(RenderConfigBuilderError::CannotDeleteNonexistent);
            }
            Change::Keep => {}
        }

        match &self.vertices {
            Change::Update(vertices) | Change::Create(vertices) => {
                if vertices.len() % 3 != 0 {
                    return Err(RenderConfigBuilderError::InvalidVertices);
                }
            }
            Change::Delete => {
                return Err(RenderConfigBuilderError::CannotDeleteNonexistent);
            }
            Change::Keep => {}
        }

        match &self.triangles {
            Change::Update(triangles) | Change::Create(triangles) => {
                if triangles.len() % 3 != 0 {
                    return Err(RenderConfigBuilderError::InvalidTriangles);
                }
            }
            Change::Delete => {
                return Err(RenderConfigBuilderError::CannotDeleteNonexistent);
            }
            Change::Keep => {}
        }

        Ok(self)
    }
}

#[derive(Default, Clone)]
pub struct RenderConfigBuilder {
    pub uniforms: Option<Change<Uniforms>>,
    pub spheres: Option<Change<Vec<Sphere>>>,
    pub vertices: Option<Change<Vec<f32>>>,
    pub triangles: Option<Change<Vec<u32>>>,
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
        self.uniforms = Some(Change::Update(uniforms));
        self
    }

    pub fn uniforms_create(mut self, uniforms: Uniforms) -> Self {
        self.uniforms = Some(Change::Create(uniforms));
        self
    }

    pub fn uniforms_no_change(mut self) -> Self {
        self.uniforms = Some(Change::Keep);
        self
    }

    pub fn uniforms_delete(mut self) -> Self {
        self.uniforms = Some(Change::Delete);
        self
    }

    pub fn spheres(mut self, spheres: Vec<Sphere>) -> Self {
        self.spheres = Some(Change::Update(spheres));
        self
    }

    pub fn spheres_create(mut self, spheres: Vec<Sphere>) -> Self {
        self.spheres = Some(Change::Create(spheres));
        self
    }

    pub fn spheres_no_change(mut self) -> Self {
        self.spheres = Some(Change::Keep);
        self
    }

    pub fn spheres_delete(mut self) -> Self {
        self.spheres = Some(Change::Delete);
        self
    }

    pub fn vertices(mut self, vertices: Vec<f32>) -> Self {
        self.vertices = Some(Change::Update(vertices));
        self
    }

    pub fn vertices_create(mut self, vertices: Vec<f32>) -> Self {
        self.vertices = Some(Change::Create(vertices));
        self
    }

    pub fn vertices_no_change(mut self) -> Self {
        self.vertices = Some(Change::Keep);
        self
    }

    pub fn vertices_delete(mut self) -> Self {
        self.vertices = Some(Change::Delete);
        self
    }

    pub fn triangles(mut self, triangles: Vec<u32>) -> Self {
        self.triangles = Some(Change::Update(triangles));
        self
    }

    pub fn triangles_create(mut self, triangles: Vec<u32>) -> Self {
        self.triangles = Some(Change::Create(triangles));
        self
    }

    pub fn triangles_no_change(mut self) -> Self {
        self.triangles = Some(Change::Keep);
        self
    }

    pub fn triangles_delete(mut self) -> Self {
        self.triangles = Some(Change::Delete);
        self
    }

    pub fn build(self) -> RenderConfig {
        if self.uniforms.is_none() {
            log::info!("RenderConfigBuilder: uniforms not set, defaulting to NoChange.");
        }
        if self.spheres.is_none() {
            log::info!("RenderConfigBuilder: spheres not set, defaulting to NoChange.");
        }
        if self.vertices.is_none() {
            log::info!("RenderConfigBuilder: vertices not set, defaulting to NoChange.");
        }
        if self.triangles.is_none() {
            log::info!("RenderConfigBuilder: triangles not set, defaulting to NoChange.");
        }

        RenderConfig {
            uniforms: self.uniforms.unwrap_or(Change::Keep),
            spheres: self.spheres.unwrap_or(Change::Keep),
            vertices: self.vertices.unwrap_or(Change::Keep),
            triangles: self.triangles.unwrap_or(Change::Keep),
        }
    }
}

#[derive(Debug)]
pub enum RenderConfigBuilderError {
    PaneDistanceOutOfBounds,
    PaneWidthOutOfBounds,
    InvalidCameraDirection,
    InvalidUniforms,
    InvalidSpheres,
    InvalidVertices,
    InvalidTriangles,
    CannotDeleteNonexistent,
}

impl fmt::Display for RenderConfigBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderConfigBuilderError::PaneDistanceOutOfBounds => {
                write!(f, "Pane-Distance is out of bounds")
            }
            RenderConfigBuilderError::PaneWidthOutOfBounds => {
                write!(f, "Pane-Distance is out of bounds")
            }
            RenderConfigBuilderError::InvalidCameraDirection => {
                write!(f, "Invalid camera direction")
            }
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

fn is_zero(v: &[f32; 3]) -> bool {
    let len_sq = v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
    len_sq < f32::EPSILON
}

#[cfg(test)]
mod tests {
    // TODO: Write tests for new CRUD style RenderConfigBuilder
}
