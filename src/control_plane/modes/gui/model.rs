use std::path::PathBuf;
use include_dir::File;
use crate::data_plane::scene::render_scene::Scene;
use crate::data_plane::scene_proxy::proxy_scene::ProxyScene;

#[allow(dead_code)]
pub struct Model {
    pub scene: Scene,
    pub proxy: ProxyScene,
}

#[allow(dead_code)]
impl Model {
    pub fn new_from_path(path: PathBuf) -> anyhow::Result<Self> {
        match Scene::load_scene_from_file(path) {
            Ok(scene) => Ok(Self::new(scene)),
            Err(e) => Err(e),
        }
    }

    pub fn new_from_template(_file: &'static File<'static>) -> anyhow::Result<Self> {
        todo!()
    }

    pub fn new_empty() -> Self {
        Self::new(Scene::new())
    }

    pub fn new(scene: Scene) -> Self {
        let proxy = scene.get_proxy_scene();
        Self { scene, proxy }
    }

    pub fn set_output_path(_path: PathBuf) -> anyhow::Result<()> {
        // ask scene to change the output path. This would require the destination not to already exist
        todo!()
    }

    pub fn save() -> anyhow::Result<()> {
        // throws an error if an output path isn't set
        todo!()
    }

    pub fn render(&self) -> anyhow::Result<()> {
        todo!()
    }

    pub fn frame_buffer_ref(&self) -> &String {
        todo!()
    }
}
