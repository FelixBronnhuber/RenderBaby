#[derive(Debug, Clone, Copy, Default)]
pub enum RenderEngine {
    #[default]
    Raytracer,
    Pathtracer,
}

impl RenderEngine {
    pub fn default_shader(&self) -> &'static str {
        match self {
            RenderEngine::Raytracer => "engine-shader/shaders/raytrace-shader.wgsl",
            RenderEngine::Pathtracer => "engine-shader/shaders/pathtracer-shader.wgsl",
        }
    }
}
