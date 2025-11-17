use engine_raytracer::{RenderCommand, RenderOutput};
use engine_wgpu_wrapper::{EngineType, SPHERES, WgpuWrapper};

pub struct Model {
    renderer: WgpuWrapper,
}

impl Model {
    pub fn new() -> Self {
        Self {
            renderer: WgpuWrapper::new(
                EngineType::Raytracer,
                500,
                500,
                std::f32::consts::FRAC_PI_4,
            )
            .expect("Renderer initialization failed"),
        }
    }

    pub fn generate_render_output(&mut self, fov: f32) -> RenderOutput {
        self.renderer
            .render(RenderCommand {
                fov: Some(fov),
                spheres: SPHERES.into(),
            })
            .expect("Render failed")
    }
}
