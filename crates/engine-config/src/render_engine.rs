use anyhow::Result;


#[derive(Debug, Clone, Copy)]
pub enum RenderEngine {
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

impl Default for RenderEngine {
    fn default() -> Self {
        RenderEngine::Raytracer
    }
}



