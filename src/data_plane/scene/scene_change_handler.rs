use anyhow::Error;
use log::info;

use crate::data_plane::scene::{
    render_scene::Scene,
    scene_change::{CameraChange, SceneChange},
};

pub(crate) struct SceneChangeHandler {
    pub(crate) scene: Scene,
}

impl Scene {
    /* pub (crate) fn new(scene: Scene) -> Self {
        Self { scene }
    } */

    pub(crate) fn handle_scene_change(&mut self, change: SceneChange) -> Result<(), Error> {
        //! Handles a Change in the scene, takes care of Logging and Errors...
        //! ## Parameter
        //! change: SceneChange to handle
        match change {
            SceneChange::CameraChange(camera_change) => {
                self.handle_camera_change(camera_change)?;
                self.update_render_config_uniform();
            }
            _ => todo!(),
        }
        Ok(())
    }

    fn handle_camera_change(&mut self, camera_change: CameraChange) -> Result<(), Error> {
        match camera_change {
            CameraChange::LookAt(look_at) => {
                info!("Change in {}: Setting lookAt to {}", self, look_at);
                self.get_camera_mut().set_look_at(look_at);
                Ok(())
            }
            _ => todo!(),
        }
    }
}
