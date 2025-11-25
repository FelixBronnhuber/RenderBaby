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

    pub fn import_obj(&mut self, obj_file_path: &str) {
        println!("Received path (obj): {}", obj_file_path);
        let mut scene = scene::scene::Scene::new();
        let _ = scene.load_object_from_file(obj_file_path.to_string());
        scene.proto_init();
    }

    pub fn import_scene(&mut self, scene_file_path: &str) {
        println!("Received path (scene): {}", scene_file_path);
        let _ = scene::scene::Scene::new().load_scene_from_file(scene_file_path.to_string());
    }
}
