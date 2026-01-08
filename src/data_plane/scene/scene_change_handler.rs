use std::{cell::RefCell, rc::Rc};

use anyhow::Error;
use log::{info, warn};

use crate::data_plane::scene::{
    render_scene::Scene,
    scene_change::{CameraChange, LightChange, MeshChange, SceneChange, SphereChange},
};

pub(crate) struct SceneChangeHandler {}

impl SceneChangeHandler {
    pub(crate) fn handle_scene_change(
        &mut self,
        scene: &mut Scene,
        change: SceneChange,
    ) -> Result<(), Error> {
        //! Handles a Change in the scene, takes care of Logging and Errors...
        //! ## Parameter
        //! change: SceneChange to handle
        match change {
            SceneChange::CameraChange(camera_change) => {
                self.handle_camera_change(scene, camera_change)?;
                scene.update_render_config_uniform();
            }
            SceneChange::SphereChange(sphere_change) => {
                self.handle_sphere_change(scene, sphere_change)?;
                scene.update_render_config_spheres();
            }
            SceneChange::MeshChange(mesh_change) => {
                self.handle_mesh_change(scene, mesh_change)?;
                // handle_mesh_change decides if render config vertices or tris need to be updated
            }
            SceneChange::LightChange(light_change) => {
                self.handle_light_change(scene, light_change)?;
                scene.update_render_config_lights(); // maybe not needed if only rename?
            }
            SceneChange::General => scene.update_render_config(),
        }
        Ok(())
    }

    fn handle_camera_change(
        &mut self,
        scene: &mut Scene,
        camera_change: CameraChange,
    ) -> Result<(), Error> {
        match camera_change {
            CameraChange::LookAt(look_at) => {
                info!("Change in {}: Setting  camera lookAt to {}", scene, look_at);
                scene.get_camera_mut().set_look_at(look_at);
            }
            CameraChange::Position(position) => {
                info!(
                    "Change in {}: Setting camera position to {}",
                    scene, position
                );
                scene.get_camera_mut().set_position(position);
            }
            CameraChange::Up(up) => {
                info!("Change in {}: Setting camera up to {}", scene, up);
                scene.get_camera_mut().set_up(up);
            }
            CameraChange::PaneDistance(distance) => {
                //todo err if negative, check vs res?
                info!(
                    "Change in {}: Setting camera pane distance to {}",
                    scene, distance
                );
                scene.get_camera_mut().set_pane_distance(distance);
            }
            CameraChange::PaneWidth(width) => {
                info!(
                    "Change in {}: Setting camera pane width to {}",
                    scene, width
                );
                scene.get_camera_mut().set_pane_width(width);
            }
            CameraChange::Resolution(res) => {
                //todo check ratio, maybe adjust pane width
                info!(
                    "Change in {}: Setting camera resolution to {:?}",
                    scene, res
                );
                scene.get_camera_mut().set_resolution(res);
            }
            CameraChange::RaySamples(samples) => {
                //todo: 0 not allowed ...
                info!(
                    "Change in {}: Setting camera ray samples to {}",
                    scene, samples
                );
                scene.get_camera_mut().set_ray_samples(samples);
            }
        }
        if scene.get_camera_position() == scene.get_camera_look_at() {
            warn!(
                "{}: Camera position identical to camera lookAt: {}",
                scene,
                scene.get_camera_look_at()
            )
        }
        Ok(())
    }
    fn handle_light_change(
        &mut self,
        scene: &mut Scene,
        light_change: LightChange,
    ) -> Result<(), Error> {
        match light_change {
            LightChange::Type(light_type, index) => todo!(),
            LightChange::Position(position, index) => todo!(),
            LightChange::Luminosity(luminosity, index) => todo!(),
            LightChange::Color(color, index) => todo!(),
            LightChange::Direction(direction, index) => todo!(),
            LightChange::Name(name) => todo!(),
        }
        todo!()
    }

    fn handle_mesh_change(
        &mut self,
        scene: &mut Scene,
        mesh_change: MeshChange,
    ) -> Result<(), Error> {
        match mesh_change {
            MeshChange::Translate(translation, index) => todo!(),
            MeshChange::Scale(factor, index) => todo!(),
            MeshChange::Rotate(rotation, index) => todo!(),
            MeshChange::Color(color, index) => todo!(),
            MeshChange::Material(material, index) => todo!(),
            MeshChange::Name(name, index) => todo!(),
        }
    }

    fn handle_sphere_change(
        &mut self,
        scene: &mut Scene,
        sphere_change: SphereChange,
    ) -> Result<(), Error> {
        match sphere_change {
            SphereChange::Translate(translation, index) => todo!(),
            SphereChange::Scale(factor, index) => todo!(),
            SphereChange::Color(color, index) => todo!(),
            SphereChange::Material(material, index) => todo!(),
            SphereChange::Name(name, index) => todo!(),
            SphereChange::Count => {
                scene.update_render_config_spheres();
                scene.update_render_config_uniform();
                Ok(())
            }
        }
    }
}
