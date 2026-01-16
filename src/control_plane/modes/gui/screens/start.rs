use eframe::emath::Vec2;
use log::{info, warn};
use rfd::FileDialog;
use eframe_elements::effects::{Effect, FillEffect};
use eframe_elements::message_popup::{Message, MessagePopupPipe};
use crate::control_plane::modes::gui::model::Model;
use crate::control_plane::modes::gui::screens::scene::SceneScreen;
use crate::control_plane::modes::gui::screens::Screen;
use crate::included_files::AutoPath;

pub struct StartScreen {
    show_template_dialog: bool,
    fill_effect: FillEffect,
    templates: Vec<AutoPath<'static>>,
    message_popup_pipe: MessagePopupPipe,
    file_dialog_scene: FileDialog,
}

impl StartScreen {
    pub(crate) fn new() -> Self {
        let templates = match AutoPath::try_from("$INCLUDED/templates/scene") {
            Ok(dir) => dir.all_from_extensions(&["json"]),
            Err(err) => {
                warn!("Failed to load templates: {}", err);
                Vec::new()
            }
        };

        Self {
            show_template_dialog: false,
            fill_effect: FillEffect::new(
                egui::Id::new("template_dialog_overlay_effect"),
                egui::Color32::from_rgba_unmultiplied(0, 0, 0, 60),
                false,
                None,
            ),
            templates,
            message_popup_pipe: MessagePopupPipe::new(),
            file_dialog_scene: FileDialog::new().add_filter("Scene", &["rscn", "json"]),
        }
    }
}

impl StartScreen {
    fn template_dialog(&mut self, ctx: &egui::Context) -> Option<Box<dyn Screen>> {
        if !self.show_template_dialog {
            return None;
        }

        self.fill_effect.update(ctx);

        let mut next_screen: Option<Box<dyn Screen>> = None;

        let max_height = ctx.available_rect().height() * 0.8;

        egui::Window::new("Choose Template")
            .collapsible(false)
            .resizable(false)
            .default_width(230.0)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(max_height - 85.0)
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        for template in self.templates.iter() {
                            let path = template.path();
                            let name = path.file_name().unwrap().to_str().unwrap();
                            if ui
                                .add_sized([ui.available_width(), 30.0], egui::Button::new(name))
                                .clicked()
                            {
                                self.show_template_dialog = false;
                                info!("Selected template: {}", name);
                                match Model::new_from_path(template.clone()) {
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
                    });

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
                            let path = AutoPath::try_from(path);

                            match path {
                                Ok(path) => match Model::new_from_path(path) {
                                    Ok(model) => {
                                        next_screen = Some(Box::new(SceneScreen::new(model)));
                                    }
                                    Err(err) => {
                                        self.message_popup_pipe
                                            .push_message(Message::from_error(err));
                                    }
                                },
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
                        next_screen = Some(Box::new(SceneScreen::new(Model::new_empty())));
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
