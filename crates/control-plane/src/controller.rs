use crate::model::*;
use crate::pipeline::Pipeline;
use crate::view::*;

pub struct Controller {
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    fn handle_event(&mut self, _event: Event) {
        todo!()
    }
}
