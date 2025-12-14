use crate::control_plane::modes::gui::screens::Screen;

pub struct StartScreen {
    show_template_dialog: bool,
}

impl StartScreen {
    pub(crate) fn new() -> Self {
        Self {
            show_template_dialog: false,
        }
    }
}

impl StartScreen {
    fn template_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_template_dialog {
            return;
        }

        egui::Window::new("Choose Template")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.label("Select a template:");

                ui.add_space(10.0);

                for template in ["Basic", "Outdoor", "Studio", "Custom"] {
                    if ui.button(template).clicked() {
                        // you handle template selection
                        self.show_template_dialog = false;
                    }
                }

                ui.separator();

                if ui.button("Cancel").clicked() {
                    self.show_template_dialog = false;
                }
            });
    }
}

impl Screen for StartScreen {
    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<Box<dyn Screen>> {
        //frame.set_window_size(egui::vec2(600.0, 300.0));
        //frame.set_resizable(false);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);

                ui.heading("Start");
                ui.add_space(30.0);

                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 20.0;

                    let button_size = egui::vec2(160.0, 80.0);

                    if ui
                        .add_sized(button_size, egui::Button::new("Import Scene"))
                        .clicked()
                    {
                        // you handle logic
                    }

                    if ui
                        .add_sized(button_size, egui::Button::new("Empty Scene"))
                        .clicked()
                    {
                        // you handle logic
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

        self.template_dialog(ctx);
        None
    }
}
