use engine_raytracer::{RenderCommand, RenderOutput};
use engine_wgpu_wrapper::{EngineType, SPHERES, WgpuWrapper};

pub struct Model {
    renderer: WgpuWrapper,
    width: usize,
    height: usize,
}

impl Model {
    pub fn new() -> Self {
        let width = 500;
        let height = 500;

        Self {
            renderer: WgpuWrapper::new(EngineType::Raytracer, width, height, std::f32::consts::FRAC_PI_4)
                .expect("Renderer initialization failed"),
            width,
            height,
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
