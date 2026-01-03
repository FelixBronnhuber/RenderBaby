use anyhow::Error;
use log::{info, warn};

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
                self.scene.update_render_config_spheres();
                todo!()
            }
            SceneChange::TriangleChange => {
                self.scene.update_render_config_triangles();
                todo!()
            }
            SceneChange::VerticesChange => {
                self.scene.update_render_config_vertices();
                todo!()
            }
            SceneChange::LightChange => {
                self.scene.update_render_config_vertices();
                todo!()
            }
            //_ => todo!("Unknown change type"),
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
                info!(
                    "Change in {}: Setting camera position to {}",
                    self.scene, position
                );
                self.scene.get_camera_mut().set_position(position);
            }
            CameraChange::Up(up) => {
                info!("Change in {}: Setting camera up to {}", self.scene, up);
                self.scene.get_camera_mut().set_up(up);
            }
            CameraChange::PaneDistance(distance) => {
                //todo err if negative, check vs res?
                info!(
                    "Change in {}: Setting camera pane distance to {}",
                    self.scene, distance
                );
                self.scene.get_camera_mut().set_pane_distance(distance);
            }
            CameraChange::PaneWidth(width) => {
                info!(
                    "Change in {}: Setting camera pane width to {}",
                    self.scene, width
                );
                self.scene.get_camera_mut().set_pane_width(width);
            }
            CameraChange::Resolution(res) => {
                //todo check ratio, maybe adjust pane width
                info!(
                    "Change in {}: Setting camera resolution to {:?}",
                    self.scene, res
                );
                self.scene.get_camera_mut().set_resolution(res);
            }
            CameraChange::RaySamples(samples) => {
                //todo: 0 not allowed ...
                info!(
                    "Change in {}: Setting camera ray samples to {}",
                    self.scene, samples
                );
                self.scene.get_camera_mut().set_ray_samples(samples);
            }
        }
        if self.scene.get_camera_position() == self.scene.get_camera_look_at() {
            warn!(
                "{}: Camera position identical to camera lookAt: {}",
                self.scene,
                self.scene.get_camera_look_at()
            )
        }
        Ok(())
    }
}
