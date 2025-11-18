// TODO: Get this from the Data-Plane!
use engine_config::*;
use engine_main::{Engine, RenderEngine};
use engine_wgpu_wrapper::RenderOutput;

// TODO: Remove this temporary Scene
const SPHERES: [Sphere; 5] = [
    Sphere {
        center: Vec3([0.0, 0.6, 1.0, 0.0]),
        radius: 0.5,
        color: Vec3::COLOR_MAGENTA,
        _pad: [0u8; 4],
    }, // Top, magenta
    Sphere {
        center: Vec3([-0.6, 0.0, 1.0, 0.0]),
        radius: 0.5,
        color: Vec3::COLOR_GREEN,
        _pad: [0u8; 4],
    }, // Left, green
    Sphere {
        center: Vec3([0.0, 0.0, 1.0, 0.0]),
        radius: 0.5,
        color: Vec3::COLOR_RED,
        _pad: [0u8; 4],
    }, // Centered, red
    Sphere {
        center: Vec3([0.6, 0.0, 1.0, 0.0]),
        radius: 0.5,
        color: Vec3::COLOR_BLUE,
        _pad: [0u8; 4],
    }, // Right, blue
    Sphere {
        center: Vec3([0.0, -0.6, 1.0, 0.0]),
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
            // .spheres(SPHERES.into())
            .spheres(vec![
                Sphere::new(
                    Vec3::new(1000000.0, 100000000.0, 10000000.0),
                    1.0,
                    Vec3::COLOR_RED,
                )
                .unwrap(),
            ])
            .camera(Camera::default())
            .verticies(vec![
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ])
            .triangles(vec![Triangle::new(0, 1, 2), Triangle::new(1, 2, 3)])
            .build()
            .unwrap();
        Self {
            engine: Engine::new(rc, RenderEngine::Raytracer),
        }
    }

    pub fn generate_render_output(&mut self, fov: f32) -> RenderOutput {
        // TODO: Get this from the Data-Plane!
        let new_camera = Camera {
            fov,
            // TODO: Camera Dimensions can also be set here
            // width: (fov as u32 * 400).clamp(128, 2046),
            // height: (fov as u32 * 400).clamp(128, 2046),
            ..Default::default()
        };
        let rc = RenderConfigBuilder::new()
            .spheres(SPHERES.into())
            .camera(new_camera)
            .verticies(vec![
                Vec3::new(0.0, 0.0, 2.0),
                Vec3::new(1.0, 0.0, 2.0),
                Vec3::new(1.0, 1.0, 2.0),
                Vec3::new(0.0, 1.0, 2.0),
            ])
            .triangles(vec![Triangle::new(0, 1, 2), Triangle::new(0, 2, 3)])
            .build()
            .unwrap();
        self.engine.render(rc).expect("Render failed")
    }
}
