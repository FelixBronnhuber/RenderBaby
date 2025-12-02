#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default)]
pub enum RenderEngine {
    #[default]
    Raytracer,
    Pathtracer,
}
