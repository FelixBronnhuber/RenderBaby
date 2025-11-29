use crate::control_plane::gui::*;

pub struct Controller {
    model: model::Model,
    pipeline: pipeline::Pipeline,
}

impl Controller {
    pub fn new(model: model::Model, pipeline: pipeline::Pipeline) -> Self {
        Self { model, pipeline }
    }

    pub fn handle_event(&mut self, event: view::Event) {
        match event {
            view::Event::DoRender => {
                let output = self.model.generate_render_output();
                if output.validate().is_ok() {
                    self.pipeline.submit_render_output(output);
                }
            }
            view::Event::ImportObj => {
                self.model
                    .import_obj(&self.pipeline.take_obj_file_path().unwrap_or("".into()));
                self.handle_event(view::Event::DoRender);
            }
            view::Event::ImportScene => {
                self.model
                    .import_scene(&self.pipeline.take_scene_file_path().unwrap_or("".into()));
                self.handle_event(view::Event::DoRender);
                // todo: also set all sliders, update tree ...
            }
            view::Event::UpdateResolution => {
                self.model
                    .set_resolution(self.pipeline.get_width(), self.pipeline.get_height());
            }
            view::Event::UpdateFOV => {
                self.model.set_fov(self.pipeline.get_fov());
            }
        }
    }
}
