use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::thread;
use egui::Color32;
use rfd::FileDialog;
use crate::effects::{Effect, FillEffect};

/// Wrapper around [`rfd::FileDialog`] that allows for threaded file picking.
pub struct ThreadedNativeFileDialog {
    /// The underlying [`FileDialog`].
    file_dialog: FileDialog,
    /// Whether the file picker is currently running.
    running: Arc<AtomicBool>,
    /// Effect that is shown when the file picker is running.
    running_effect: FillEffect,
    //last_directory: Arc<Mutex<Option<PathBuf>>, TODO: implement this
}

impl ThreadedNativeFileDialog {
    /// Create a new [`ThreadedNativeFileDialog`].
    ///
    /// Receives the [`FileDialog`] to use it for file picking.
    pub fn new(file_dialog: FileDialog) -> Self {
        Self {
            file_dialog,
            running: Arc::new(AtomicBool::new(false)),
            running_effect: FillEffect::new(
                egui::Id::new("color_overlay_effect"),
                Color32::from_rgba_unmultiplied(0, 0, 0, 60),
                false,
                None,
            ),
        }
    }

    /// Return whether the file picker is currently running.
    pub fn is_running(&self) -> bool {
        self.running.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Helper function that spawns a new thread and runs the given dialog function.
    ///
    /// A dialog function is called with the underlying [`FileDialog`] and should return the result of the file picking or `None` if the dialog was canceled.
    ///
    /// Once the dialog completes, the `after` function is called with the result of the file picking.
    ///
    /// # Type Parameters
    ///
    /// - `R`: Result type produced by the dialog function. This is typically either a single [`PathBuf`] or a [`Vec<PathBuf>`].
    /// - `A`: Type of the `after` callback function that is called with the result `R` wrapped in an [`anyhow::Result`].
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
        self.running
            .store(true, std::sync::atomic::Ordering::Relaxed);
        let dialog_clone = self.file_dialog.clone();
        let running_mutex = self.running.clone();

        thread::spawn(move || {
            let res = dialog_fn(dialog_clone);
            running_mutex.store(false, std::sync::atomic::Ordering::Relaxed);
            if let Some(res) = res {
                after(Ok(res));
            } else {
                after(Err(anyhow::anyhow!("No file(s)/path(s) selected")));
            }
        });
    }

    /// Start the file picker dialog.
    ///
    /// Passes [`anyhow::Result<PathBuf>`] to the `after` function.
    /// The `pick_file` dialog returns a path to an existing file.
    pub fn pick_file<A>(&self, after: A)
    where
        A: FnOnce(anyhow::Result<PathBuf>) + Send + 'static,
    {
        self.do_threaded(after, |dialog| dialog.pick_file())
    }

    /// Start the file picker dialog for multiple files.
    ///
    /// Passes [`anyhow::Result<Vec<PathBuf>>`] to the `after` function.
    /// The `pick_files` dialog returns a vector of paths to existing files.
    pub fn pick_files<A>(&self, after: A)
    where
        A: FnOnce(anyhow::Result<Vec<PathBuf>>) + Send + 'static,
    {
        self.do_threaded(after, |dialog| dialog.pick_files())
    }

    /// Start the file save dialog.
    ///
    /// Passes [`anyhow::Result<PathBuf>`] to the `after` function.
    /// The `save_file` dialog returns a path to save a file to, meaning a new directory can be chosen.
    pub fn save_file<A>(&self, after: A)
    where
        A: FnOnce(anyhow::Result<PathBuf>) + Send + 'static,
    {
        self.do_threaded(after, |dialog| dialog.save_file())
    }

    /// Update the fill effect shown when the file picker is running.
    pub fn update_effect(&mut self, ctx: &egui::Context) {
        if self.is_running() {
            self.running_effect.update(ctx);
        }
    }
}
