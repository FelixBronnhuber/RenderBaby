use anyhow::{Error, anyhow};
use log::{info, warn};
use scene_objects::geometric_object::GeometricObject;

use crate::data_plane::scene::{
    render_scene::Scene,
    scene_change::{CameraChange, LightChange, MeshChange, SceneChange, SphereChange},
};

impl Scene {
    pub(crate) fn handle_scene_change(&mut self, change: SceneChange) -> Result<(), Error> {
        //! Handles a Change in the scene, takes care of Logging and Errors...
        //! ## Parameter
        //! change: SceneChange to handle
        // some update_render_confix_x might be not needed, p.e. if its a name change
        match change {
            SceneChange::CameraChange(camera_change) => {
                self.handle_camera_change(camera_change)?;
                self.update_render_config_uniform();
            }
            SceneChange::SphereChange(sphere_change) => {
                self.handle_sphere_change(sphere_change)?;
                self.update_render_config_spheres();
            }
            SceneChange::MeshChange(mesh_change) => {
                self.handle_mesh_change(mesh_change)?;
                // handle_mesh_change decides if render config vertices or tris need to be updated
            }
            SceneChange::LightChange(light_change) => {
                self.handle_light_change(light_change)?;
                self.update_render_config_lights(); // maybe not needed if only rename?
            }
            SceneChange::_General => todo!(),
        }
        Ok(())
    }

    fn handle_camera_change(&mut self, camera_change: CameraChange) -> Result<(), Error> {
        match camera_change {
            CameraChange::LookAt(look_at) => {
                info!("Change in {}: Setting  camera lookAt to {}", self, look_at);
                self.get_camera_mut().set_look_at(look_at);
            }
            CameraChange::Position(position) => {
                info!(
                    "Change in {}: Setting camera position to {}",
                    self, position
                );
                self.get_camera_mut().set_position(position);
            }
            CameraChange::Up(up) => {
                info!("Change in {}: Setting camera up to {}", self, up);
                self.get_camera_mut().set_up(up);
            }
            CameraChange::PaneDistance(distance) => {
                //todo err if negative, check vs res?
                info!(
                    "Change in {}: Setting camera pane distance to {}",
                    self, distance
                );
                self.get_camera_mut().set_pane_distance(distance);
            }
            CameraChange::PaneWidth(width) => {
                info!("Change in {}: Setting camera pane width to {}", self, width);
                self.get_camera_mut().set_pane_width(width);
            }
            CameraChange::Resolution(res) => {
                //todo check ratio, maybe adjust pane width
                info!("Change in {}: Setting camera resolution to {:?}", self, res);
                self.get_camera_mut().set_resolution(res);
            }
            CameraChange::RaySamples(samples) => {
                //todo: 0 not allowed ...
                info!(
                    "Change in {}: Setting camera ray samples to {}",
                    self, samples
                );
                self.get_camera_mut().set_ray_samples(samples);
            }
        }
        if self.get_camera_position() == self.get_camera_look_at() {
            warn!(
                "{}: Camera position identical to camera lookAt: {}",
                self,
                self.get_camera_look_at()
            )
        }
        Ok(())
    }
    fn handle_light_change(&mut self, light_change: LightChange) -> Result<(), Error> {
        match light_change {
            LightChange::Type(light_type, index) => {
                info!(
                    "Change in {}: Setting ligt {} type to {:?}",
                    self, index, light_type
                );
                match self.get_light_sources_mut().get_mut(index) {
                    Some(light) => {
                        light.set_light_type(light_type);
                        Ok(())
                    }
                    None => Err(anyhow!("Index out of bounds")),
                }
            }
            LightChange::Position(position, index) => {
                info!(
                    "Change in {}: Setting ligt {} positon to {:?}",
                    self, index, position
                );
                match self.get_light_sources_mut().get_mut(index) {
                    Some(light) => {
                        light.set_position(position);
                        Ok(())
                    }
                    None => Err(anyhow!("Index out of bounds")),
                }
            }
            LightChange::Luminosity(luminosity, index) => {
                info!(
                    "Change in {}: Setting ligt {} luminosity to {:?}",
                    self, index, luminosity
                );
                match self.get_light_sources_mut().get_mut(index) {
                    Some(light) => {
                        light.set_luminosity(luminosity);
                        Ok(())
                    }
                    None => Err(anyhow!("Index out of bounds")),
                }
            }
            LightChange::Color(color, index) => {
                info!(
                    "Change in {}: Setting ligt {} color to {:?}",
                    self, index, color
                );
                match self.get_light_sources_mut().get_mut(index) {
                    Some(light) => {
                        light.set_color(color);
                        Ok(())
                    }
                    None => Err(anyhow!("Index out of bounds")),
                }
            }
            LightChange::_Direction(_direction, _index) => {
                todo!("Directional lights not supported")
            }
            LightChange::_Name(name, index) => {
                info!(
                    "Change in {}: Setting ligt {} name to {:?}",
                    self, index, name
                );
                match self.get_light_sources_mut().get_mut(index) {
                    Some(light) => {
                        light.set_name(name);
                        Ok(())
                    }
                    None => Err(anyhow!("Index out of bounds")),
                }
            }
        }
    }

    fn handle_mesh_change(&mut self, mesh_change: MeshChange) -> Result<(), Error> {
        match mesh_change {
            MeshChange::Translate(translation, index) => {
                info!(
                    "Change in {}: Translating mesh {} by {:?}",
                    self, index, translation
                );
                match self.get_meshes_mut().get_mut(index) {
                    Some(mesh) => {
                        mesh.translate(translation);
                        self.update_render_config_vertices();
                        Ok(())
                    }
                    None => Err(anyhow!("Index out of bounds")),
                }
            }
            MeshChange::Scale(factor, index) => {
                info!("Change in {}: Scaling mesh {} by {:?}", self, index, factor);
                match self.get_meshes_mut().get_mut(index) {
                    Some(mesh) => {
                        mesh.scale(factor);
                        self.update_render_config_vertices();
                        Ok(())
                    }
                    None => Err(anyhow!("Index out of bounds")),
                }
            }
            MeshChange::Rotate(rotation, index) => {
                info!(
                    "Change in {}: Rotating mesh {} by {:?}",
                    self, index, rotation
                );
                match self.get_meshes_mut().get_mut(index) {
                    Some(mesh) => {
                        mesh.rotate(rotation);
                        self.update_render_config_vertices();
                        Ok(())
                    }
                    None => Err(anyhow!("Index out of bounds")),
                }
            }
            MeshChange::_Material(_material, _index) => {
                todo!("Material change not supported");
            }
            MeshChange::_Name(name, index) => {
                info!(
                    "Change in {}: Setting mesh {} name to {:?}",
                    self, index, name
                );
                match self.get_meshes_mut().get_mut(index) {
                    Some(mesh) => {
                        mesh.set_name(name);
                        Ok(())
                    }
                    None => Err(anyhow!("Index out of bounds")),
                }
            }
        }
    }

    fn handle_sphere_change(&mut self, sphere_change: SphereChange) -> Result<(), Error> {
        match sphere_change {
            SphereChange::Translate(translation, index) => {
                info!(
                    "Change in {}: Translating sphere {} by {:?}",
                    self, index, translation
                );
                match self.get_spheres_mut().get_mut(index) {
                    Some(sphere) => {
                        sphere.translate(translation);
                        Ok(())
                    }
                    None => Err(anyhow!("Index out of bounds")),
                }
            }
            SphereChange::Radius(factor, index) => {
                info!(
                    "Change in {}: Scaling sphere {} by {:?}",
                    self, index, factor
                );
                match self.get_spheres_mut().get_mut(index) {
                    Some(sphere) => {
                        sphere.scale(factor);
                        Ok(())
                    }
                    None => Err(anyhow!("Index out of bounds")),
                }
            }
            SphereChange::Color(color, index) => {
                info!(
                    "Change in {}: Setting sphere {} color to {:?}",
                    self, index, color
                );
                match self.get_spheres_mut().get_mut(index) {
                    Some(sphere) => {
                        sphere.set_color(color);
                        Ok(())
                    }
                    None => Err(anyhow!("Index out of bounds")),
                }
            }
            SphereChange::Material(material, index) => {
                info!(
                    "Change in {}: Setting sphere {} material to {:?}",
                    self, index, material
                );
                match self.get_spheres_mut().get_mut(index) {
                    Some(sphere) => {
                        sphere.set_material(material);
                        Ok(())
                    }
                    None => Err(anyhow!("Index out of bounds")),
                }
            }
            SphereChange::_Name(name, index) => {
                info!(
                    "Change in {}: Setting sphere {} name to {:?}",
                    self, index, name
                );
                match self.get_spheres_mut().get_mut(index) {
                    Some(sphere) => {
                        sphere.set_name(name);
                        Ok(())
                    }
                    None => Err(anyhow!("Index out of bounds")),
                }
            }
            SphereChange::_Count => {
                self.update_render_config_uniform();
                // probably all that is need right now
                Ok(())
            }
        }
    }
}
