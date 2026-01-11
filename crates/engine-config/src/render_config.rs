use crate::*;
use core::fmt;
use engine_bvh::bvh::BVHNode;
use engine_bvh::triangle::GPUTriangle;

#[derive(Clone)]
pub struct RenderConfig {
    pub uniforms: Change<Uniforms>,
    pub spheres: Change<Vec<Sphere>>,
    pub uvs: Change<Vec<f32>>,
    pub meshes: Change<Vec<Mesh>>,
    pub lights: Change<Vec<PointLight>>,
    pub bvh_nodes: Change<Vec<BVHNode>>,
    pub bvh_indices: Change<Vec<u32>>,
    pub bvh_triangles: Change<Vec<GPUTriangle>>,
    pub textures: Change<Vec<TextureData>>,
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

    #[deprecated(note = "Temporary function for testing translation. Do not use.")]
    pub fn translate(&mut self, dx: f32, dy: f32, dz: f32) {
        match &mut self.spheres {
            Change::Create(spheres) | Change::Update(spheres) => {
                for sphere in spheres.iter_mut() {
                    sphere.center.0[0] += dx;
                    sphere.center.0[1] += dy;
                    sphere.center.0[2] += dz;
                }
            }
            _ => {}
        }
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
        if !matches!(self.uvs, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidUVs);
        }
        if !matches!(self.meshes, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidMeshes);
        }
        if !matches!(self.lights, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidLights);
        }
        if !matches!(self.textures, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidTextures);
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
                // TODO: Add more Uniforms validation as needed
            }
            Change::Delete => {
                return Err(RenderConfigBuilderError::CannotDeleteNonexistent);
            }
            Change::Keep => {}
        }

        match &self.spheres {
            Change::Update(spheres) | Change::Create(spheres) => {
                if spheres.iter().any(|s| s.radius <= 0.0) {
                    return Err(RenderConfigBuilderError::InvalidSpheres);
                }
                // TODO: Add more Sphere validation as needed
            }
            Change::Delete => {
                log::info!("RenderConfig: Attempting to delete spheres")
            }
            Change::Keep => {}
        }

        match &self.uvs {
            Change::Update(uvs) | Change::Create(uvs) => {
                if uvs.len() % 2 != 0 {
                    return Err(RenderConfigBuilderError::InvalidUVs);
                }
            }
            Change::Delete => {
                todo!("Implement UVs Deletion")
            }
            Change::Keep => {}
        }

        match &self.meshes {
            Change::Update(_meshes) | Change::Create(_meshes) => {
                //TODO: Mesh Validation
            }
            Change::Delete => {
                todo!("Implement meshes Deletion")
            }
            Change::Keep => {}
        }

        match &self.lights {
            Change::Update(lights) | Change::Create(lights) => {
                if lights.iter().any(|l| l.radius <= 0.0) {
                    return Err(RenderConfigBuilderError::InvalidLights);
                }
            }
            Change::Delete => {
                todo!("Implement lights Deletion")
            }
            Change::Keep => {}
        }

        match &self.textures {
            Change::Update(_) | Change::Create(_) => {
                // TODO: Texture validation
            }
            Change::Delete => {
                todo!("Implement textures Deletion")
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
    pub uvs: Option<Change<Vec<f32>>>,
    pub meshes: Option<Change<Vec<Mesh>>>,
    pub lights: Option<Change<Vec<PointLight>>>,
    pub bvh_nodes: Option<Change<Vec<BVHNode>>>,
    pub bvh_indices: Option<Change<Vec<u32>>>,
    pub bvh_triangles: Option<Change<Vec<GPUTriangle>>>,
    pub textures: Option<Change<Vec<TextureData>>>,
}

impl RenderConfigBuilder {
    pub fn new() -> Self {
        Self {
            uniforms: None,
            spheres: None,
            uvs: None,
            meshes: None,
            lights: None,
            bvh_nodes: None,
            bvh_indices: None,
            bvh_triangles: None,
            textures: None,
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

    pub fn uvs(mut self, uvs: Vec<f32>) -> Self {
        self.uvs = Some(Change::Update(uvs));
        self
    }

    pub fn uvs_create(mut self, uvs: Vec<f32>) -> Self {
        self.uvs = Some(Change::Create(uvs));
        self
    }

    pub fn uvs_no_change(mut self) -> Self {
        self.uvs = Some(Change::Keep);
        self
    }

    pub fn uvs_delete(mut self) -> Self {
        self.uvs = Some(Change::Delete);
        self
    }

    pub fn meshes(mut self, meshes: Vec<Mesh>) -> Self {
        self.meshes = Some(Change::Update(meshes));
        self
    }

    pub fn meshes_create(mut self, meshes: Vec<Mesh>) -> Self {
        self.meshes = Some(Change::Create(meshes));
        self
    }

    pub fn meshes_no_change(mut self) -> Self {
        self.meshes = Some(Change::Keep);
        self
    }

    pub fn meshes_delete(mut self) -> Self {
        self.meshes = Some(Change::Delete);
        self
    }

    pub fn lights(mut self, lights: Vec<PointLight>) -> Self {
        self.lights = Some(Change::Update(lights));
        self
    }

    pub fn lights_create(mut self, lights: Vec<PointLight>) -> Self {
        self.lights = Some(Change::Create(lights));
        self
    }

    pub fn lights_no_change(mut self) -> Self {
        self.lights = Some(Change::Keep);
        self
    }

    pub fn lights_delete(mut self) -> Self {
        self.lights = Some(Change::Delete);
        self
    }

    pub fn bvh_nodes(mut self, nodes: Vec<BVHNode>) -> Self {
        self.bvh_nodes = Some(Change::Update(nodes));
        self
    }

    pub fn bvh_nodes_create(mut self, nodes: Vec<BVHNode>) -> Self {
        self.bvh_nodes = Some(Change::Create(nodes));
        self
    }

    pub fn bvh_nodes_no_change(mut self) -> Self {
        self.bvh_nodes = Some(Change::Keep);
        self
    }

    pub fn bvh_nodes_delete(mut self) -> Self {
        self.bvh_nodes = Some(Change::Delete);
        self
    }

    pub fn bvh_indices(mut self, indices: Vec<u32>) -> Self {
        self.bvh_indices = Some(Change::Update(indices));
        self
    }

    pub fn bvh_indices_create(mut self, indices: Vec<u32>) -> Self {
        self.bvh_indices = Some(Change::Create(indices));
        self
    }

    pub fn bvh_indices_no_change(mut self) -> Self {
        self.bvh_indices = Some(Change::Keep);
        self
    }

    pub fn bvh_indices_delete(mut self) -> Self {
        self.bvh_indices = Some(Change::Delete);
        self
    }

    pub fn bvh_triangles(mut self, triangles: Vec<GPUTriangle>) -> Self {
        self.bvh_triangles = Some(Change::Update(triangles));
        self
    }

    pub fn bvh_triangles_create(mut self, triangles: Vec<GPUTriangle>) -> Self {
        self.bvh_triangles = Some(Change::Create(triangles));
        self
    }

    pub fn bvh_triangles_no_change(mut self) -> Self {
        self.bvh_triangles = Some(Change::Keep);
        self
    }

    pub fn bvh_triangles_delete(mut self) -> Self {
        self.bvh_triangles = Some(Change::Delete);
        self
    }

    pub fn textures(mut self, textures: Vec<TextureData>) -> Self {
        self.textures = Some(Change::Update(textures));
        self
    }

    pub fn textures_create(mut self, textures: Vec<TextureData>) -> Self {
        self.textures = Some(Change::Create(textures));
        self
    }

    pub fn textures_no_change(mut self) -> Self {
        self.textures = Some(Change::Keep);
        self
    }

    pub fn textures_delete(mut self) -> Self {
        self.textures = Some(Change::Delete);
        self
    }

    pub fn build(self) -> RenderConfig {
        if self.uniforms.is_none() {
            log::info!("RenderConfigBuilder: uniforms not set, defaulting to NoChange.");
        }
        if self.spheres.is_none() {
            log::info!("RenderConfigBuilder: spheres not set, defaulting to NoChange.");
        }
        if self.uvs.is_none() {
            log::info!("RenderConfigBuilder: uvs not set, defaulting to NoChange.");
        }
        if self.meshes.is_none() {
            log::info!("RenderConfigBuilder: meshes not set, defaulting to NoChange.");
        }
        if self.lights.is_none() {
            log::info!("RenderConfigBuilder: lights not set, defaulting to NoChange.");
        }
        if self.textures.is_none() {
            log::info!("RenderConfigBuilder: textures not set, defaulting to NoChange.");
        }
        if self.bvh_nodes.is_none() {
            log::info!("RenderConfigBuilder: bvh_nodes not set, defaulting to NoChange");
        }
        if self.bvh_indices.is_none() {
            log::info!("RenderConfigBuilder: bvh_indices not set, defaulting to NoChange");
        }
        if self.bvh_triangles.is_none() {
            log::info!("RenderConfigBuilder: bvh_triangles not set, defaulting to NoChange");
        }

        RenderConfig {
            uniforms: self.uniforms.unwrap_or(Change::Keep),
            spheres: self.spheres.unwrap_or(Change::Keep),
            uvs: self.uvs.unwrap_or(Change::Keep),
            meshes: self.meshes.unwrap_or(Change::Keep),
            lights: self.lights.unwrap_or(Change::Keep),
            textures: self.textures.unwrap_or(Change::Keep),
            bvh_nodes: self.bvh_nodes.unwrap_or(Change::Keep),
            bvh_indices: self.bvh_indices.unwrap_or(Change::Keep),
            bvh_triangles: self.bvh_triangles.unwrap_or(Change::Keep),
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
    InvalidUVs,
    InvalidMeshes,
    InvalidLights,
    InvalidTextures,
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
            RenderConfigBuilderError::InvalidUVs => write!(f, "Invalid UVs"),
            RenderConfigBuilderError::InvalidMeshes => write!(f, "Invalid Meshes"),
            RenderConfigBuilderError::InvalidLights => write!(f, "Invalid Lights"),
            RenderConfigBuilderError::InvalidTextures => write!(f, "Invalid Textures"),
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
