use anyhow::{Error, Ok, Result};
use log::info;
use crate::data_plane::{scene::render_scene::Scene, scene_proxy::proxy_scene::ProxyScene};
pub(super) struct SceneComposit {
    real_scene: Scene,
    pub(super) proxy_scene: ProxyScene,
}
#[allow(unused)]
impl SceneComposit {
    pub(crate) fn new() -> Self {
        SceneComposit {
            real_scene: Scene::default(),
            proxy_scene: ProxyScene::new_from_real_scene(&Scene::default()),
        }
    }
    pub(crate) fn update_proxy(&mut self) -> Result<(), Error> {
        self.proxy_scene = ProxyScene::new_from_real_scene(&self.real_scene);
        Ok(())
    }
    pub(crate) fn update_real(&mut self) -> Result<(), Error> {
        self.update_real_name()?;
        self.update_real_objects()?;
        self.update_real_camera()?;
        self.update_real_lights()?;
        self.update_real_background_color()?;
        self.update_real_misc()?;
        Ok(())
    }

    fn update_real_name(&mut self) -> Result<(), Error> {
        if &self.proxy_scene.scene_name != self.real_scene.get_name() {
            info!(
                "{}:Changing name to  {}",
                self.real_scene, self.proxy_scene.scene_name
            );
            self.real_scene
                .set_name(self.proxy_scene.scene_name.clone());
        }
        Ok(())
    }

    fn update_real_background_color(&mut self) -> Result<(), Error> {
        if self.proxy_scene.background_color != self.real_scene.get_background_color() {
            info!(
                "{}: Changing Background color to {:?}",
                self.real_scene, self.proxy_scene.background_color
            );
            self.real_scene
                .set_background_color(self.proxy_scene.background_color);
        }
        Ok(())
    }
    fn update_real_objects(&mut self) -> Result<(), Error> {
        let mut real_objects = self.real_scene.get_tri_geometries();
        let proxy_objects = &self.proxy_scene.objects;
        for i in 0..real_objects.len() {}
        todo!()
    }
    fn update_real_camera(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn update_real_lights(&mut self) -> Result<(), Error> {
        todo!()
    }
    fn update_real_misc(&mut self) -> Result<(), Error> {
        todo!()
    }

    pub fn load_obj(&mut self, path: String) -> Result<(), Error> {
        self.real_scene.load_object_from_file(path.into())?;
        self.update_proxy()?;
        Ok(())
    }
}
