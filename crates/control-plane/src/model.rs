// TODO: Get this from the Data-Plane!
use engine_wgpu_wrapper::RenderOutput;

type RBScene = scene::scene::Scene; // RenderBabyScene, es gibt auch eine egui scene

pub struct Model {
    scene: RBScene,
}

impl Model {
    pub fn new() -> Self {
        let mut scene = RBScene::new();
        scene.proto_init();
        Self { scene }
    }

    pub fn generate_render_output(&mut self, _fov: f32) -> RenderOutput {
        // TODO: Get this from the Data-Plane!
        self.scene.render().unwrap()
    }
}
