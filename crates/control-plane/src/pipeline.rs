use engine_wgpu_wrapper::RenderOutput;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Pipeline {
    pub render_output_ppl: Arc<Mutex<Option<RenderOutput>>>,
    fov: Arc<Mutex<f32>>,
    width: Arc<Mutex<u32>>,
    height: Arc<Mutex<u32>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            render_output_ppl: Arc::new(Mutex::new(None)),
            fov: Arc::new(Mutex::new(std::f32::consts::FRAC_PI_4)),
            width: Arc::new(Mutex::new(500)),
            height: Arc::new(Mutex::new(500)),
        }
    }

    pub fn set_fov(&self, v: f32) {
        *self.fov.lock().unwrap() = v;
    }
    pub fn get_fov(&self) -> f32 {
        *self.fov.lock().unwrap()
    }
    pub fn set_width(&self, width: u32) {
        *self.width.lock().unwrap() = width;
    }
    pub fn get_width(&self) -> u32 {
        *self.width.lock().unwrap()
    }

    pub fn set_height(&self, height: u32) {
        *self.height.lock().unwrap() = height;
    }
    pub fn get_height(&self) -> u32 {
        *self.height.lock().unwrap()
    }
}
