use eframe::{egui};
pub struct RaytraceApp {
    texture: Option<egui::TextureHandle>,
}

impl Default for RaytraceApp {
    fn default() -> Self {
        Self { texture: None }
    }
}

impl eframe::App for RaytraceApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Renderbaby Raytracer");
        });
    }
}

pub fn start() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "RenderBaby Raytracer",
        native_options,
        Box::new(|_cc| Ok(Box::<RaytraceApp>::default())),
    )
}
