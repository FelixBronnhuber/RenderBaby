use log::info;
use crate::data_plane::scene::render_scene::Scene;
use engine_wgpu_wrapper::RenderOutput;
use crate::data_plane::scene_io::scene_parser::SceneParseError;

pub struct Model {
    scene: Scene,
}

impl Model {
    pub fn new() -> Self {
        let mut scene = Scene::new();
        scene.proto_init();
        Self { scene }
    }

    pub fn import_obj(&mut self, obj_file_path: &str) {
        info!("Received path (obj): {}", obj_file_path);

        let _ = self.scene.load_object_from_file(obj_file_path.to_string());
    }

    pub fn import_scene(&mut self, scene_file_path: &str) -> Result<(), SceneParseError> {
        info!("Received path (scene): {}", scene_file_path);
        let scene_res = Scene::load_scene_from_file(scene_file_path.to_string());
        match scene_res {
            Err(e) => {
                eprintln!("Error loading scene: {:?}", e);
                Err(e)
            }
            Ok(s) => {
                self.scene = s;
                Ok(())
            }
        }
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
