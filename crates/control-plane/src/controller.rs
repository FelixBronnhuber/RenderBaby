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
                let output = self.model.generate_render_output();
                if output.validate().is_ok() {
                    self.pipeline.submit_render_output(output);
                }
            }
            Event::UpdateResolution => {
                self.model
                    .set_resolution(self.pipeline.get_width(), self.pipeline.get_height());
            }
            Event::UpdateFOV => {
                self.model.set_fov(self.pipeline.get_fov());
            }
        }
    }
}
