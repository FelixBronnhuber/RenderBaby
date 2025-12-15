pub mod scene;
pub(crate) mod start;

pub trait Screen {
    fn default_size(&self) -> egui::Vec2;
    fn resizable(&self) -> bool {
        true
    }
    fn on_start(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {}
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame)
    -> Option<Box<dyn Screen>>;
}
