use crate::model::*;
use crate::pipeline::Pipeline;
use crate::view::*;

pub struct Controller {
    model: Model,
    pipeline: Pipeline,
}

impl Controller {
    pub fn new(pipeline: Pipeline, model: Model) -> Self {
        Self { model, pipeline }
    }
}

impl ViewListener for Controller {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::DoRender => {
                let output = self.model.generate_render_output(
                    self.pipeline.get_fov(),
                    self.pipeline.get_width(),
                    self.pipeline.get_height(),
                );
                if output.validate().is_ok() {
                    self.pipeline.submit_render_output(output);
                }
            }

            /*             Event::SetFov(fov) => {
                self.pipeline.set_fov(fov);
            } */
            Event::ImportObj => {
                self.model
                    .import_obj(&self.pipeline.take_obj_file_path().unwrap_or("".into()));
            }
            Event::ImportScene => {
                self.model
                    .import_scene(&self.pipeline.take_scene_file_path().unwrap_or("".into()));
            }
        }
    }
}
