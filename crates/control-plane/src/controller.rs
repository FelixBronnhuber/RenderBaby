use crate::model::*;
use crate::pipeline::Pipeline;
use crate::view::*;

pub struct Controller {
    model: Model,
    pipeline: Pipeline
}

impl Controller {
    pub fn new(pipeline: Pipeline, model: Model) -> Self {
        Self {
            model,
            pipeline
        }
    }
}

impl ViewListener for Controller {
    fn handle_event(&mut self, _event: Event) {
        if (_event == Event::DoRender) {
            let output = self.model.generate_render_output();
            if (output.validate().is_ok()) {
                *self.pipeline.render_output_ppl.lock().unwrap() = Some(output);
            }
        }
    }
}
