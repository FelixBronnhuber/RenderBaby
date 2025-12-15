use std::path::PathBuf;
use crate::data_plane::scene::render_scene::Scene;

#[allow(dead_code)]
pub struct Model {
    pub scene: Scene,
}

#[allow(dead_code)]
impl Model {
    pub fn new_from_path(path: PathBuf) -> anyhow::Result<Self> {
        match Scene::load_scene_from_file(path) {
            Ok(scene) => Ok(Self { scene }),
            Err(e) => Err(e),
        }
    }

    pub fn new_from_template(_path: PathBuf) -> anyhow::Result<Self> {
        todo!()
    }

    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
        }
    }

    pub fn set_path(_path: PathBuf) -> anyhow::Result<()> {
        // ask scene to change the output path. This would require the destination not to already exist
        Ok(())
    }

    pub fn save() -> anyhow::Result<()> {
        // throws an error if an output path isn't set
        Ok(())
    }

    pub fn render(&self) {}
}
