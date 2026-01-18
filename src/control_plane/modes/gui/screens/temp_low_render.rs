use std::sync::{Arc, Mutex};
use frame_buffer::deferred_iterator::TemporaryLow;
use crate::control_plane::modes::gui::model::Model;

type CallbackCallback = dyn FnOnce(anyhow::Result<()>) + Send;

pub struct TemporaryLowRender {
    model: Arc<Model>,
    low_samples: u32,
    callback_callback: Mutex<Option<Box<CallbackCallback>>>,
}

impl TemporaryLowRender {
    pub fn new(
        model: Arc<Model>,
        callback_callback: Box<CallbackCallback>,
        low_samples: u32,
    ) -> Self {
        Self {
            model,
            low_samples,
            callback_callback: Mutex::new(Some(callback_callback)),
        }
    }
}

impl TemporaryLow for TemporaryLowRender {
    fn callback_low(&self) {
        self.model.frame_buffer.stop_current_provider();

        let original_samples = {
            let mut scene_lock = self.model.scene.lock().unwrap();
            let camera = scene_lock.get_camera_mut();
            let original_samples = camera.get_ray_samples();
            camera.set_ray_samples(self.low_samples);
            original_samples
        };

        let res = self.model.render();

        if let Ok(mut scene_lock) = self.model.scene.lock() {
            scene_lock
                .get_camera_mut()
                .set_ray_samples(original_samples);
        }

        if let Some(cb) = self.callback_callback.lock().unwrap().take() {
            cb(res);
        }
    }

    fn callback_normal(&self) {
        self.model.frame_buffer.stop_current_provider();
        let res = self.model.render();
        if let Some(cb) = self.callback_callback.lock().unwrap().take() {
            cb(res);
        }
    }

    fn is_active(&self) -> bool {
        self.model.frame_buffer.has_provider()
    }
}
