// TODO: Get this from the Data-Plane!
use engine_config::*;
use engine_wgpu_wrapper::RenderOutput;

type RBScene = scene::scene::Scene; // RenderBabyScene, es gibt auch eine egui scene

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
    scene: RBScene,
}

impl Model {
    pub fn new() -> Self {
        let mut scene = RBScene::new();
        scene.proto_init();
        Self { scene }
    }

    pub fn generate_render_output(&mut self, fov: f32) -> RenderOutput {
        // TODO: Get this from the Data-Plane!
        self.scene.render().unwrap()
    }
}
