use crate::control_plane::modes::gui::*;
use eframe;
use eframe::egui;
use view_wrappers::egui_view::EframeViewWrapper;
use view_wrappers::ViewWrapper;
use crate::control_plane::modes::gui::screens::Screen;

pub struct View {
    current_screen: Box<dyn Screen>,
}

impl ViewWrapper for View {
    fn new() -> Self {
        Self {
            current_screen: Box::new(screens::start::StartScreen::new()),
        }
    }

    fn open(self) {
        self.open_native("RenderBaby");
    }
}

impl eframe::App for View {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(next_screen) = self.current_screen.update(ctx, _frame) {
            self.current_screen = next_screen;
        }
    }
}

impl EframeViewWrapper for View {
    fn on_start(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
