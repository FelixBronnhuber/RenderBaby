use std::path::PathBuf;
use eframe::emath::Vec2;
use log::info;
use rfd::FileDialog;
use eframe_elements::message_popup::{Message, MessagePopupPipe};
use crate::control_plane::modes::gui::model::Model;
use crate::control_plane::modes::gui::screens::scene::SceneScreen;
use crate::control_plane::modes::gui::screens::Screen;

pub struct StartScreen {
    show_template_dialog: bool,
    templates: Vec<PathBuf>,
    message_popup_pipe: MessagePopupPipe,
    file_dialog_scene: FileDialog,
}

impl StartScreen {
    pub(crate) fn new() -> Self {
        let templates = vec![PathBuf::from("ferris"), PathBuf::from("cube")];

        Self {
            show_template_dialog: false,
            templates,
            message_popup_pipe: MessagePopupPipe::new(),
            file_dialog_scene: FileDialog::new().add_filter("JSON", &["json"]),
        }
    }
}

impl StartScreen {
    fn template_dialog(&mut self, ctx: &egui::Context) -> Option<Box<dyn Screen>> {
        if !self.show_template_dialog {
            return None;
        }

        let mut next_screen: Option<Box<dyn Screen>> = None;

        egui::Window::new("Choose Template")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.label("Select a template:");

                ui.add_space(10.0);

                for template in self.templates.iter() {
                    let name = template.file_name().unwrap().to_str().unwrap();
                    if ui.button(name).clicked() {
                        self.show_template_dialog = false;
                        info!("Selected template: {}", name);
                        match Model::new_from_template(PathBuf::from(template)) {
                            Ok(model) => {
                                next_screen = Some(Box::new(SceneScreen::new(model)));
                            }
                            Err(err) => {
                                self.message_popup_pipe
                                    .push_message(Message::from_error(err));
                            }
                        }
                    }
                }

                ui.separator();

                if ui.button("Cancel").clicked() {
                    self.show_template_dialog = false;
                }
            });
        next_screen
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
        self.message_popup_pipe.show_last(ctx);

        let mut next_screen: Option<Box<dyn Screen>> = None;

        let file_dialog = self.file_dialog_scene.clone();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.spacing_mut().item_spacing.y = 5.0;

                    let button_size = egui::vec2(400.0, 80.0);

                    if ui
                        .add_sized(button_size, egui::Button::new("Import Scene"))
                        .clicked()
                    {
                        if let Some(path) = file_dialog.pick_file() {
                            info!("Importing scene from {:?}", path);
                            match Model::new_from_path(path) {
                                Ok(model) => {
                                    next_screen = Some(Box::new(SceneScreen::new(model)));
                                }
                                Err(err) => {
                                    self.message_popup_pipe
                                        .push_message(Message::from_error(err));
                                }
                            }
                        } else {
                            info!("No file selected or aborted file dialog.");
                        }
                    }

                    if ui
                        .add_sized(button_size, egui::Button::new("Empty Scene"))
                        .clicked()
                    {
                        next_screen = Some(Box::new(SceneScreen::new(Model::new())));
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

        if let Some(template_screen) = self.template_dialog(ctx) {
            return Option::from(template_screen);
        }

        next_screen
    }
}
