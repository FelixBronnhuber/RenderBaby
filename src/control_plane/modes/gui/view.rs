use crate::control_plane::modes::gui::*;
use eframe;
use eframe::egui;
use view_wrappers::egui_view::EframeViewWrapper;
use view_wrappers::ViewWrapper;
use crate::control_plane::modes::gui::screens::Screen;

pub struct View {
    current_screen: Box<dyn Screen>,
    first_frame: bool,
}

impl ViewWrapper for View {
    fn new() -> Self {
        let current_screen: Box<dyn Screen> = if is_debug_mode() {
            // TODO: best to replace this with a proto_scene.json from templates
            // so that model::Model::new_from_template is used
            let mut model = model::Model::new_with_capsule();
            model.reload_proxy();
            Box::new(screens::scene::SceneScreen::new(model))
        } else {
            Box::new(screens::start::StartScreen::new())
        };
        Self {
            current_screen,
            first_frame: true,
        }
    }

    fn open(self) {
        self.open_native("RenderBaby");
    }
}

impl eframe::App for View {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.first_frame {
            self.first_frame = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::Vec2::new(
                self.current_screen.default_size().x,
                self.current_screen.default_size().y,
            )));
            ctx.send_viewport_cmd(egui::ViewportCommand::Resizable(
                self.current_screen.resizable(),
            ));
            self.current_screen.on_start(ctx, frame);
        }

        if let Some(next_screen) = self.current_screen.update(ctx, frame) {
            self.current_screen = next_screen;
            self.first_frame = true;
            ctx.request_repaint();
        }
    }
}

impl EframeViewWrapper for View {
    fn on_start(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
}
