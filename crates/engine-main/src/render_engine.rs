#[derive(Debug, Clone, Copy, Default)]
pub enum RenderEngine {
    #[default]
    Raytracer,
    Pathtracer,
}
