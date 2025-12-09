use engine_config::RenderOutput;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Pipeline {
    render_output_ppl: Arc<Mutex<Option<RenderOutput>>>,
    fov: Arc<Mutex<f32>>,
    width: Arc<Mutex<u32>>,
    height: Arc<Mutex<u32>>,
    obj_file_path: Arc<Mutex<Option<String>>>,
    scene_file_path: Arc<Mutex<Option<String>>>,
    color_hash_enabled: Arc<Mutex<bool>>,
    export_file_path: Arc<Mutex<Option<String>>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            render_output_ppl: Arc::new(Mutex::new(None)),
            fov: Arc::new(Mutex::new(50.0)),
            width: Arc::new(Mutex::new(500)),
            height: Arc::new(Mutex::new(500)),
            obj_file_path: Arc::new(Mutex::new(None)),
            scene_file_path: Arc::new(Mutex::new(None)),
            color_hash_enabled: Arc::new(Mutex::new(true)),
            export_file_path: Arc::new(Mutex::new(None)),
        }
    }

    /* Distinguish between set/get and submit/take:

    set/get: shared variable
    submit/take: one-time transfer

    Always keep variables private and create methods to access them as intended.
     */

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

    pub fn submit_render_output(&self, output: RenderOutput) {
        *self.render_output_ppl.lock().unwrap() = Some(output);
    }

    pub fn take_render_output(&self) -> Option<RenderOutput> {
        self.render_output_ppl.lock().unwrap().take()
    }

    pub fn submit_obj_file_path(&self, path: Option<String>) {
        *self.obj_file_path.lock().unwrap() = path;
    }

    pub fn take_obj_file_path(&self) -> Option<String> {
        self.obj_file_path.lock().unwrap().take()
    }

    pub fn submit_scene_file_path(&self, path: Option<String>) {
        *self.scene_file_path.lock().unwrap() = path;
    }

    pub fn take_scene_file_path(&self) -> Option<String> {
        self.scene_file_path.lock().unwrap().take()
    }

    pub fn set_color_hash_enabled(&self, enabled: bool) {
        *self.color_hash_enabled.lock().unwrap() = enabled;
    }

    pub fn get_color_hash_enabled(&self) -> bool {
        *self.color_hash_enabled.lock().unwrap()
    }

    pub fn submit_export_file_path(&self, path: Option<String>) {
        *self.export_file_path.lock().unwrap() = path;
    }

    pub fn take_export_file_path(&self) -> Option<String> {
        self.export_file_path.lock().unwrap().take()
    }
}
