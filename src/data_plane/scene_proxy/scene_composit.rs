use anyhow::{Error, Ok, Result};
use crate::data_plane::{scene::render_scene::RealScene, scene_proxy::proxy_scene::ProxyScene};
pub(super) struct SceneComposit {
    real_scene: RealScene,
    pub(super) proxy_scene: ProxyScene,
}
#[allow(unused)]
impl SceneComposit {
    pub(crate) fn new() -> Self {
        SceneComposit {
            real_scene: RealScene::default(),
            proxy_scene: ProxyScene::new_from_real_scene(&RealScene::default()),
        }
    }
    pub(crate) fn update_proxy(&mut self) -> Result<(), Error> {
        todo!()
    }
    pub(crate) fn update_real(&mut self) -> Result<(), Error> {
        self.update_real_name()?;
        self.update_real_path()?;
        self.update_real_scales()?;
        self.update_real_rotations()?;
        self.update_real_translations()?;
        self.update_real_background_color()?;
        Ok(())
    }

    fn update_real_name(&mut self) -> Result<(), Error> {
        if &self.proxy_scene.scene_name != self.real_scene.get_name() {
            self.real_scene
                .set_name(self.proxy_scene.scene_name.clone());
        }
        Ok(())
    }
    fn update_real_path(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn update_real_scales(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn update_real_rotations(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn update_real_translations(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn update_real_background_color(&mut self) -> Result<(), Error> {
        todo!()
    }
}
