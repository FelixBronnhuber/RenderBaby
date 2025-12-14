pub mod scene;
pub(crate) mod start;

pub trait Screen {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame)
    -> Option<Box<dyn Screen>>;
}
