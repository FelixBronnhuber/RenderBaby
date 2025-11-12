use std::sync::{Arc, Mutex};
use engine_raytracer::RenderOutput;

#[derive(Clone)]
pub struct Pipeline {
    pub render_output_ppl: Arc<Mutex<Option<RenderOutput>>>
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            render_output_ppl: Arc::new(Mutex::new(None))
        }
    }
}
