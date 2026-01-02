use std::path::PathBuf;
use anyhow::Error;
use glam::Vec3;
use scene_objects::camera::Resolution;
use crate::data_plane::scene::render_scene::Scene;
use engine_config::RenderOutput;

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

    pub fn import_obj(&mut self, obj_file_path: PathBuf) -> anyhow::Result<()> {
        match self.scene.load_object_from_file(obj_file_path) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error loading OBJ file: {:?}", e);
                Err(e)
            }
        }
    }

    pub fn import_scene(&mut self, scene_file_path: PathBuf) -> anyhow::Result<&Scene> {
        match Scene::load_scene_from_file(scene_file_path) {
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
        self.scene.set_camera_pane_distance(fov);
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.scene
            .get_camera_mut()
            .set_resolution(Resolution::new(width, height));
    }

    pub fn set_camera_pos(&mut self, pos: [f32; 3]) {
        self.scene
            .get_camera_mut()
            .set_position(Vec3::from_array(pos));
    }

    pub fn set_camera_dir(&mut self, dir: [f32; 3]) {
        self.scene
            .get_camera_mut()
            .set_look_at(Vec3::from_array(dir));
    }

    pub fn set_color_hash_enabled(&mut self, enabled: bool) {
        self.scene.set_color_hash_enabled(enabled);
    }

    pub fn set_samples(&mut self, samples: u32) {
        self.scene.get_camera_mut().set_ray_samples(samples);
    }

    pub fn delete_spheres(&mut self) {
        self.scene.clear_spheres();
    }

    pub fn delete_polygons(&mut self) {
        self.scene.clear_polygons();
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

    pub fn export_image(&mut self, file_path: PathBuf) -> anyhow::Result<()> {
        match self.scene.export_render_img(file_path) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error exporting image: {:?}", e);
                Err(Error::from(e))
            }
        }
    }
}
