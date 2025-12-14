use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use egui::Color32;
use rfd::FileDialog;
use crate::effects::{Effect, FillEffect};

pub struct ThreadedNativeFileDialog {
    file_dialog: FileDialog,
    running: Arc<Mutex<bool>>,
    running_effect: FillEffect,
    // last_directory: Option<PathBuf>, TODO: implement this feature.
}

impl ThreadedNativeFileDialog {
    pub fn new(file_dialog: FileDialog) -> Self {
        Self {
            file_dialog,
            running: Arc::new(Mutex::new(false)),
            running_effect: FillEffect::new(
                egui::Id::new("color_overlay_effect"),
                Color32::from_rgba_unmultiplied(0, 0, 0, 60),
                false,
                None,
            ),
        }
    }

    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }

    fn do_threaded<R, A>(&self, after: A, dialog_fn: fn(FileDialog) -> Option<R>)
    where
        R: Send + 'static,
        A: FnOnce(anyhow::Result<R>) + Send + 'static,
    {
        if self.is_running() {
            thread::spawn(move || {
                after(Err(anyhow::anyhow!("File picker already running")));
            });
            return;
        }
        let dialog_clone = self.file_dialog.clone();
        let running_mutex = self.running.clone();
        *running_mutex.lock().unwrap() = true;
        thread::spawn(move || {
            let res = dialog_fn(dialog_clone);
            *running_mutex.lock().unwrap() = false;
            if let Some(res) = res {
                after(Ok(res));
            } else {
                after(Err(anyhow::anyhow!("No file(s)/path(s) selected")));
            }
        });
    }

    pub fn pick_file<A>(&self, after: A)
    where
        A: FnOnce(anyhow::Result<PathBuf>) + Send + 'static,
    {
        self.do_threaded(after, |dialog| dialog.pick_file())
    }

    pub fn pick_files<A>(&self, after: A)
    where
        A: FnOnce(anyhow::Result<Vec<PathBuf>>) + Send + 'static,
    {
        self.do_threaded(after, |dialog| dialog.pick_files())
    }

    pub fn save_file<A>(&self, after: A)
    where
        A: FnOnce(anyhow::Result<PathBuf>) + Send + 'static,
    {
        self.do_threaded(after, |dialog| dialog.save_file())
    }

    pub fn update_effect(&mut self, ctx: &egui::Context) {
        if self.is_running() {
            self.running_effect.update(ctx);
        }
    }
}
