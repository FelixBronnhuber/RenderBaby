use anyhow::Error;
use log::info;

use crate::data_plane::scene::{
    render_scene::Scene,
    scene_change::{CameraChange, SceneChange},
};

pub(crate) struct SceneChangeHandler {
    pub(crate) scene: Scene,
}

impl SceneChangeHandler {
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
                self.scene.update_render_config_uniform();
            }
            SceneChange::SphereChange => {
                todo!()
            }
            SceneChange::TriangleChange => {
                todo!()
            }
            SceneChange::VerticesChange => {
                todo!()
            }
            SceneChange::LightChange => {
                todo!()
            }
            _ => todo!("Unknown change type"),
        }
        Ok(())
    }

    fn handle_camera_change(&mut self, camera_change: CameraChange) -> Result<(), Error> {
        match camera_change {
            CameraChange::LookAt(look_at) => {
                info!(
                    "Change in {}: Setting  camera lookAt to {}",
                    self.scene, look_at
                );
                self.scene.get_camera_mut().set_look_at(look_at);
            }
            CameraChange::Position(position) => {
                todo!()
            }
            CameraChange::Up(up) => {
                todo!()
            }
            CameraChange::PaneDistance(distance) => {
                todo!()
            }
            CameraChange::PaneWidth(width) => {
                todo!()
            }
            CameraChange::Resolution(res) => {
                todo!()
            }
            CameraChange::RaySamples(samples) => {
                todo!()
            }

            _ => todo!(),
        }
        Ok(())
    }
}
