use engine_wgpu_wrapper::RenderOutput;
use scene::scene::Scene;

pub struct Model {
    scene: Scene,
}

impl Model {
    pub fn new() -> Self {
        let mut scene = Scene::new();
        scene.proto_init(); // remove this later when we have proper fixtures

        Self { scene }
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.scene.get_camera_mut().set_fov(fov);
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.scene.get_camera_mut().set_resolution([width, height]);
    }

    pub fn generate_render_output(&mut self) -> RenderOutput {
        self.scene.render().unwrap()
    }
}
