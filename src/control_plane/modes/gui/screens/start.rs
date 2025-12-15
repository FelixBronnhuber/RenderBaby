use std::path::PathBuf;
use eframe::emath::Vec2;
use crate::control_plane::modes::gui::screens::scene::SceneScreen;
use crate::control_plane::modes::gui::screens::Screen;

pub struct StartScreen {
    show_template_dialog: bool,
    templates: Vec<PathBuf>,
}

impl StartScreen {
    pub(crate) fn new() -> Self {
        let templates = vec![];

        Self {
            show_template_dialog: false,
            templates,
        }
    }
}

impl StartScreen {
    fn template_dialog(&mut self, ctx: &egui::Context) -> Option<Box<dyn Screen>> {
        if !self.show_template_dialog {
            return None;
        }

        let mut res: Option<Box<dyn Screen>> = None;

        egui::Window::new("Choose Template")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.label("Select a template:");

                ui.add_space(10.0);

                for template in ["ferris", "cube"] {
                    if ui.button(template).clicked() {
                        self.show_template_dialog = false;
                        println!("Selected template: {}", template);
                        res = Some(Box::new(SceneScreen::new()));
                    }
                }

                ui.separator();

                if ui.button("Cancel").clicked() {
                    self.show_template_dialog = false;
                }
            });
        res
    }
}

impl Screen for StartScreen {
    fn default_size(&self) -> Vec2 {
        Vec2::new(440.0, 265.0)
    }

    fn resizable(&self) -> bool {
        false
    }

    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<Box<dyn Screen>> {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.spacing_mut().item_spacing.y = 5.0;

                    let button_size = egui::vec2(400.0, 80.0);

                    if ui
                        .add_sized(button_size, egui::Button::new("Import Scene"))
                        .clicked()
                    {
                        //
                    }

                    if ui
                        .add_sized(button_size, egui::Button::new("Empty Scene"))
                        .clicked()
                    {
                        //
                    }

                    if ui
                        .add_sized(button_size, egui::Button::new("From Template"))
                        .clicked()
                    {
                        self.show_template_dialog = true;
                    }
                });
            });
        });

        self.template_dialog(ctx)
    }
}
