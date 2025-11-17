use engine_raytracer::RenderOutput;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Pipeline {
    pub render_output_ppl: Arc<Mutex<Option<RenderOutput>>>,
    fov: Arc<Mutex<f32>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            render_output_ppl: Arc::new(Mutex::new(None)),
            fov: Arc::new(Mutex::new(std::f32::consts::FRAC_PI_4)),
        }
    }

    pub fn set_fov(&self, v: f32) {
        *self.fov.lock().unwrap() = v;
    }
    pub fn get_fov(&self) -> f32 {
        *self.fov.lock().unwrap()
    }
}
