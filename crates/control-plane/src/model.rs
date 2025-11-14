use engine_raytracer::{RenderCommand, RenderOutput};
use engine_wgpu_wrapper::{EngineType, SPHERES, WgpuWrapper};

static FOV: f32 = std::f32::consts::FRAC_PI_4; // temporary

pub struct Model {
    renderer: WgpuWrapper,
}

impl Model {
    pub fn new() -> Self {
        Self {
            renderer: WgpuWrapper::new(EngineType::Raytracer, 500, 500, FOV)
                .expect("Renderer initialization failed"),
        }
    }

    pub fn generate_render_output(&mut self) -> RenderOutput {
        self.renderer
            .render(RenderCommand {
                fov: Some(FOV),
                spheres: SPHERES.into(),
            })
            .expect("Render failed")
    }
}
