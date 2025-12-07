use log::info;
use crate::data_plane::scene::render_scene::Scene;
use engine_config::RenderOutput;
use crate::data_plane::scene_io::scene_parser::SceneParseError;

pub struct Model {
    scene: Scene,
    currently_rendering: bool, // should later be replaced with some mutex guard
}

impl Model {
    pub fn new() -> Self {
        let mut scene = Scene::new();
        scene.proto_init();
        Self {
            scene,
            currently_rendering: false,
        }
    }

    pub fn import_obj(&mut self, obj_file_path: &str) -> anyhow::Result<()> {
        info!("Received path (obj): {}", obj_file_path);
        match self.scene.load_object_from_file(obj_file_path.to_string()) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error loading OBJ file: {:?}", e);
                Err(e)
            }
        }
    }

    pub fn import_scene(&mut self, scene_file_path: &str) -> Result<&Scene, SceneParseError> {
        info!("Received path (scene): {}", scene_file_path);
        let scene_res = Scene::load_scene_from_file(scene_file_path.to_string());
        match scene_res {
            Err(e) => {
                eprintln!("Error loading scene: {:?}", e);
                Err(e)
            }
            Ok(s) => {
                self.scene = s;
                Ok(&self.scene)
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
        if self.currently_rendering {
            log::warn!("Render already in progress, skipping new render request.");
            return RenderOutput::new(1, 1, vec![0, 0, 0, 255]);
        }
        self.currently_rendering = true;
        let output = self.scene.render();
        self.currently_rendering = false;
        output.unwrap_or_else(|e| {
            log::error!("Render failed: {}", e);
            // Return a dummy output (magenta image) to avoid panic for now...
            RenderOutput::new(1, 1, vec![255, 0, 255, 255])
        })
    }

    pub fn export_image(&mut self, file_path: &str) {
        match self.scene.export_render_img(file_path) {
            Ok(_) => log::info!("Image exported to {}", file_path),
            Err(e) => log::error!("Failed to export image: {}", e),
        };
    }
}
