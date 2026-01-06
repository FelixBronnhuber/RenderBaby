use log::error;
use scene_objects::camera::Resolution;
use crate::control_plane::modes::gui::*;
use crate::data_plane::scene::render_scene::Scene;
use view_wrappers::EventResult;

pub struct Controller {
    model: model::Model,
    pipeline: pipeline::Pipeline,
}

impl Controller {
    pub fn new(model: model::Model, pipeline: pipeline::Pipeline) -> Self {
        Self { model, pipeline }
    }

    fn update_pipeline(pipeline: &pipeline::Pipeline, scene: &Scene) {
        pipeline.set_fov(scene.get_camera().get_fov());
        let Resolution {
            width: res_x,
            height: res_y,
        } = scene.get_camera().get_resolution();
        pipeline.set_width(*res_x);
        pipeline.set_height(*res_y);
        pipeline.set_camera_pos(scene.get_camera().get_position().to_array());
        pipeline.set_camera_dir(scene.get_camera().get_look_at().to_array());
        pipeline.set_samples(scene.get_camera().get_ray_samples());
    }

    pub fn handle_event(&mut self, event: view::Event) -> EventResult {
        match event {
            view::Event::DoRender => {
                let output = self.model.generate_render_output();
                match output.validate() {
                    Ok(_) => {
                        self.pipeline.submit_render_output(output);
                        Ok(Box::new(()))
                    }
                    Err(e) => {
                        log::error!("Render output validation failed: {}", e);
                        Err(e)
                    }
                }
            }
            view::Event::ImportObj => {
                if let Some(path) = self.pipeline.take_obj_file_path() {
                    match self.model.import_obj(path) {
                        Ok(_) => Ok(Box::new(())),
                        Err(e) => {
                            error!("Error importing OBJ: {:?}", e);
                            Err(e)
                        }
                    }
                } else {
                    error!("ImportObj event received but no path was set");
                    Err(anyhow::anyhow!(
                        "No OBJ file path provided in ImportObj event"
                    ))
                }
            }
            view::Event::ImportScene => {
                if let Some(path) = self.pipeline.take_scene_file_path() {
                    match self.model.import_scene(path) {
                        Ok(scene) => {
                            Self::update_pipeline(&self.pipeline, scene);
                            Ok(Box::new(()))
                        }
                        Err(e) => {
                            error!("Error importing scene: {:?}", e);
                            Err(e)
                        }
                    }
                } else {
                    error!("ImportScene event received but no path was set");
                    Err(anyhow::anyhow!(
                        "No scene file path provided in ImportScene event"
                    ))
                }
            }
            view::Event::UpdateResolution => {
                self.model
                    .set_resolution(self.pipeline.get_width(), self.pipeline.get_height());
                Ok(Box::new(()))
            }
            view::Event::UpdateFOV => {
                self.model.set_fov(self.pipeline.get_fov());
                Ok(Box::new(()))
            }
            view::Event::UpdateCamera => {
                self.model.set_camera_pos(self.pipeline.get_camera_pos());
                self.model.set_camera_dir(self.pipeline.get_camera_dir());
                Ok(Box::new(()))
            }
            view::Event::UpdateColorHash => {
                self.model
                    .set_color_hash_enabled(self.pipeline.get_color_hash_enabled());
                Ok(Box::new(()))
            }
            view::Event::UpdateSamples => {
                self.model.set_samples(self.pipeline.get_samples());
                Ok(Box::new(()))
            }
            view::Event::DeleteSpheres => {
                self.model.delete_spheres();
                Ok(Box::new(()))
            }
            view::Event::DeletePolygons => {
                self.model.delete_polygons();
                Ok(Box::new(()))
            }
            view::Event::ExportImage => {
                if let Some(path) = self.pipeline.take_export_file_path() {
                    match self.model.export_image(path.clone()) {
                        Ok(_) => Ok(Box::new(())),
                        Err(_) => {
                            log::error!("Error exporting image to path: {}", path.display());
                            Err(anyhow::anyhow!(
                                "Failed to export image to {}",
                                path.display()
                            ))
                        }
                    }
                } else {
                    log::error!("ExportImage event received but no path was set");
                    Err(anyhow::anyhow!(
                        "No export file path provided in ExportImage event"
                    ))
                }
            }
        }
    }
}
