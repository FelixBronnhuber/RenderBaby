use log::error;
use crate::control_plane::gui::*;
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
        let [res_x, res_y] = scene.get_camera().get_resolution();
        pipeline.set_width(res_x);
        pipeline.set_height(res_y);
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
                let res = self
                    .model
                    .import_obj(&self.pipeline.take_obj_file_path().unwrap_or("".into()));
                match res {
                    Ok(_) => Ok(Box::new(())),
                    Err(e) => {
                        error!("Error importing OBJ: {:?}", e);
                        Err(e)
                    }
                }
            }
            view::Event::ImportScene => {
                let scene_path = self.pipeline.take_scene_file_path().unwrap_or("".into());
                let import_res = self.model.import_scene(&scene_path);

                match import_res {
                    Ok(scene) => {
                        Self::update_pipeline(&self.pipeline, scene);
                        Ok(Box::new(()))
                    }
                    Err(e) => {
                        error!("Error importing scene: {:?}", e);
                        Err(e.into())
                    }
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
            view::Event::ExportImage => {
                if let Some(path) = self.pipeline.take_export_file_path() {
                    match self.model.export_image(&path) {
                        Ok(_) => Ok(Box::new(())),
                        Err(_) => {
                            log::error!("Error exporting image to path: {}", path);
                            Err(anyhow::anyhow!("Failed to export image to {}", path))
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
