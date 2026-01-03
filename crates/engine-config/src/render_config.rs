use crate::*;
use core::fmt;
use engine_bvh::bvh::BVHNode;
use engine_bvh::triangle::GPUTriangle;

pub struct RenderConfig {
    pub uniforms: Change<Uniforms>,
    pub spheres: Change<Vec<Sphere>>,
    pub vertices: Change<Vec<f32>>,
    pub uvs: Change<Vec<f32>>,
    pub triangles: Change<Vec<u32>>,
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

        match &mut self.vertices {
            Change::Create(vertices) | Change::Update(vertices) => {
                for i in (0..vertices.len()).step_by(3) {
                    vertices[i] += dx;
                    vertices[i + 1] += dy;
                    vertices[i + 2] += dz;
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
        if !matches!(self.vertices, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidVertices);
        }
        if !matches!(self.uvs, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidUVs);
        }
        if !matches!(self.triangles, Change::Create(_)) {
            return Err(RenderConfigBuilderError::InvalidTriangles);
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

        match &self.vertices {
            Change::Update(vertices) | Change::Create(vertices) => {
                if vertices.len() % 3 != 0 {
                    return Err(RenderConfigBuilderError::InvalidVertices);
                }
            }
            Change::Delete => {
                todo!("Implement Vertices Deletion")
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

        match &self.triangles {
            Change::Update(triangles) | Change::Create(triangles) => {
                if triangles.len() % 3 != 0 {
                    return Err(RenderConfigBuilderError::InvalidTriangles);
                }
            }
            Change::Delete => {
                todo!("Implement triangles Deletion")
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
                if lights.iter().any(|l| l.luminosity <= 0.0) {
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
    pub vertices: Option<Change<Vec<f32>>>,
    pub uvs: Option<Change<Vec<f32>>>,
    pub triangles: Option<Change<Vec<u32>>>,
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
            vertices: None,
            uvs: None,
            triangles: None,
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
        if self.vertices.is_none() {
            log::info!("RenderConfigBuilder: vertices not set, defaulting to NoChange.");
        }
        if self.uvs.is_none() {
            log::info!("RenderConfigBuilder: uvs not set, defaulting to NoChange.");
        }
        if self.triangles.is_none() {
            log::info!("RenderConfigBuilder: triangles not set, defaulting to NoChange.");
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
            log::info!("bvh_nodes not set, defaulting to Keep");
        }
        if self.bvh_indices.is_none() {
            log::info!("bvh_indices not set, defaulting to Keep");
        }
        if self.bvh_triangles.is_none() {
            log::info!("bvh_triangles not set, defaulting to Keep");
        }

        RenderConfig {
            uniforms: self.uniforms.unwrap_or(Change::Keep),
            spheres: self.spheres.unwrap_or(Change::Keep),
            vertices: self.vertices.unwrap_or(Change::Keep),
            uvs: self.uvs.unwrap_or(Change::Keep),
            triangles: self.triangles.unwrap_or(Change::Keep),
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
    InvalidVertices,
    InvalidUVs,
    InvalidTriangles,
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
            RenderConfigBuilderError::InvalidVertices => write!(f, "Invalid Vertices"),
            RenderConfigBuilderError::InvalidUVs => write!(f, "Invalid UVs"),
            RenderConfigBuilderError::InvalidTriangles => write!(f, "Invalid Triangles"),
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
