// TODO: Get this from the Data-Plane!
use engine_config::*;
use engine_main::{Engine, RenderEngine};
use engine_wgpu_wrapper::RenderOutput;

// TODO: Remove this temporary Scene
const SPHERES: [Sphere; 5] = [
    Sphere {
        center: Vec3([0.0, 0.6, 1.0]),
        radius: 0.5,
        color: Vec3::COLOR_MAGENTA,
        _pad: [0u8; 4],
    }, // Top, magenta
    Sphere {
        center: Vec3([-0.6, 0.0, 1.0]),
        radius: 0.5,
        color: Vec3::COLOR_GREEN,
        _pad: [0u8; 4],
    }, // Left, green
    Sphere {
        center: Vec3([0.0, 0.0, 1.0]),
        radius: 0.5,
        color: Vec3::COLOR_RED,
        _pad: [0u8; 4],
    }, // Centered, red
    Sphere {
        center: Vec3([0.6, 0.0, 1.0]),
        radius: 0.5,
        color: Vec3::COLOR_BLUE,
        _pad: [0u8; 4],
    }, // Right, blue
    Sphere {
        center: Vec3([0.0, -0.6, 1.0]),
        radius: 0.5,
        color: Vec3::COLOR_CYAN,
        _pad: [0u8; 4],
    }, // Bottom, cyan
];

pub struct Model {
    engine: Engine,
}

impl Model {
    pub fn new() -> Self {
        // TODO: Get this from the Data-Plane!
        let rc = RenderConfigBuilder::new()
            .spheres(SPHERES.into())
            .camera(Camera::default())
            .build()
            .unwrap();
        Self {
            engine: Engine::new(rc, RenderEngine::Raytracer),
        }
    }

    pub fn generate_render_output(&mut self, fov: f32, width: u32, height: u32) -> RenderOutput {
        // TODO: Get this from the Data-Plane!
        let new_camera = Camera {
            fov,
            width,
            height,
            // width: (fov as u32 * 400).clamp(128, 2046),
            // height: (fov as u32 * 400).clamp(128, 2046),
        };
        let rc = RenderConfigBuilder::new()
            .spheres(SPHERES.into())
            .camera(new_camera)
            .build()
            .unwrap();
        self.engine.render(rc).expect("Render failed")
    }
}
