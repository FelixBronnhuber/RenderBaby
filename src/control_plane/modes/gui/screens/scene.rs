use eframe::emath::Align;
use rfd::FileDialog;
use eframe_elements::file_picker::ThreadedNativeFileDialog;
use eframe_elements::image_area::ImageArea;
use eframe_elements::message_popup::MessagePopupPipe;
use crate::control_plane::modes::gui::model::Model;
use crate::control_plane::modes::gui::screens::Screen;
use crate::control_plane::modes::gui::screens::start::StartScreen;

#[allow(dead_code)]
pub struct SceneScreen {
    model: Model,
    bottom_visible: bool,
    file_dialog_obj: ThreadedNativeFileDialog,
    file_dialog_export: ThreadedNativeFileDialog,
    image_area: ImageArea,
    message_popup_pipe: MessagePopupPipe,
}

#[allow(dead_code)]
impl SceneScreen {
    pub fn new(model: Model) -> Self {
        Self {
            model,
            bottom_visible: false,
            file_dialog_obj: ThreadedNativeFileDialog::new(
                FileDialog::new().add_filter("OBJ", &["obj"]),
            ),
            file_dialog_export: ThreadedNativeFileDialog::new(
                FileDialog::new().add_filter("IMAGE", &["png"]),
            ),
            image_area: ImageArea::new(Default::default()),
            message_popup_pipe: MessagePopupPipe::new(),
        }
    }
}

#[allow(dead_code)]
impl SceneScreen {
    fn logs_ui(ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("Log-view")
            .resizable(true)
            .min_height(30.0)
            .show(ctx, |ui| {
                ui.heading("Log-view");
                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        let logs = log_buffer::get_logs();
                        ui.add_sized(
                            ui.available_size(),
                            egui::Label::new(logs).halign(Align::LEFT).selectable(false),
                        );
                    });
            });
    }
}

impl Screen for SceneScreen {
    fn default_size(&self) -> egui::Vec2 {
        egui::Vec2::new(1200.0, 800.0)
    }

    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<Box<dyn Screen>> {
        if ctx.input(|i| i.viewport().close_requested()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            return Some(Box::new(StartScreen::new()));
        }

        // TODO: set image_area image

        self.message_popup_pipe.show_last(ctx);

        egui::TopBottomPanel::top("Toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Toolbar");
                if ui.button("Toggle log-view").clicked() {
                    self.bottom_visible = !self.bottom_visible;
                }
                if ui.button("Import Obj").clicked() {
                    self.file_dialog_obj.pick_file(|res| {
                        if let Ok(path) = res {
                            println!("Selected file: {:?}", path);
                        }
                    });
                }
                self.file_dialog_obj.update_effect(ctx);

                if ui.button("Export PNG").clicked() {
                    self.file_dialog_export.pick_file(|res| {
                        if let Ok(path) = res {
                            println!("Selected file: {:?}", path);
                        }
                    });
                }
                self.file_dialog_export.update_effect(ctx);
            })
        });

        egui::SidePanel::left("SidePanel")
            .resizable(true)
            .min_width(220.0)
            .show(ctx, |ui| {
                if ui.button("Render").clicked() {
                    // TODO: probably call render function in view model
                }

                // fov.ui(ui)

                // resolution.ui(ui)

                ui.separator();

                // camera.ui(ui)

                ui.separator();

                // color_hash.ui(ui)

                ui.separator();

                // samples.ui(ui)
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.image_area.ui(ui);
        });

        if self.bottom_visible {
            SceneScreen::logs_ui(ctx);
        }

        None
    }
}
