use std::sync::atomic::Ordering;
use std::time::Duration;
use egui::{Color32, RichText};
use rfd::FileDialog;
use eframe_elements::file_picker::ThreadedNativeFileDialog;
use eframe_elements::image_area::{Image, ImageArea};
use eframe_elements::message_popup::{Message, MessagePopupPipe};
use crate::control_plane::modes::gui::model::Model;
use crate::control_plane::modes::gui::screens::Screen;
use crate::control_plane::modes::gui::screens::start::StartScreen;
use crate::control_plane::modes::gui::screens::viewable::Viewable;
use crate::control_plane::modes::is_debug_mode;

static FRAME_DURATION_FPS24: Duration = Duration::from_millis(1000 / 24);

#[allow(dead_code)]
pub struct SceneScreen {
    model: Model,
    bottom_visible: bool,
    render_on_change: bool,
    file_dialog_obj: ThreadedNativeFileDialog,
    file_dialog_export: ThreadedNativeFileDialog,
    file_dialog_save: ThreadedNativeFileDialog,
    image_area: ImageArea,
    message_popup_pipe: MessagePopupPipe,
}

#[allow(dead_code)]
impl SceneScreen {
    pub fn new(model: Model) -> Self {
        Self {
            model,
            bottom_visible: false,
            render_on_change: false,
            file_dialog_obj: ThreadedNativeFileDialog::new(
                FileDialog::new().add_filter("OBJ", &["obj"]),
            ),
            file_dialog_export: ThreadedNativeFileDialog::new(
                FileDialog::new().add_filter("IMAGE", &["png"]),
            ),
            file_dialog_save: ThreadedNativeFileDialog::new(
                FileDialog::new()
                    .add_filter("RenderBaby Scene", &["rscn"])
                    .add_filter("JSON Scene", &["json"]),
            ),
            image_area: ImageArea::new(Default::default()),
            message_popup_pipe: MessagePopupPipe::new(),
        }
    }

    fn logs_ui(ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("Log-view")
            .resizable(true)
            .min_height(30.0)
            .show(ctx, |ui| {
                ui.heading("Log-view");

                let log_rect = ui.available_rect_before_wrap();
                ui.painter().rect_filled(log_rect, 0.0, Color32::BLACK);

                egui::ScrollArea::vertical()
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        let mut logs = log_buffer::get_logs();
                        let widget = egui::TextEdit::multiline(&mut logs)
                            .font(egui::TextStyle::Monospace)
                            .text_color(Color32::WHITE)
                            .frame(false)
                            .code_editor()
                            .lock_focus(true)
                            .interactive(false)
                            .desired_width(f32::INFINITY);

                        widget.show(ui);
                    });
            });
    }

    fn do_render(&self) {
        let it = self.model.render();
        match it {
            Ok(_) => {}
            Err(_) => {
                self.message_popup_pipe
                    .push_message(Message::from_error(anyhow::anyhow!(
                        "Failed to generate a frame iterator."
                    )));
            }
        };
    }
}

impl Screen for SceneScreen {
    fn default_size(&self) -> egui::Vec2 {
        egui::Vec2::new(1200.0, 800.0)
    }

    fn on_start(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.do_render();
    }

    fn update(
        &mut self,
        ctx: &egui::Context,
        _frame: &mut eframe::Frame,
    ) -> Option<Box<dyn Screen>> {
        if ctx.input(|i| i.viewport().close_requested()) && !is_debug_mode() {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            return Some(Box::new(StartScreen::new()));
        }

        self.message_popup_pipe.show_last(ctx);

        egui::TopBottomPanel::top("Toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let save_as_clicked = ui.button("Export to Scene File (.rscn)").clicked();

                let output_path = self.model.scene.lock().unwrap().get_output_path();

                // Can only be true if the button is shown and clicked.
                let mut save_clicked = false;
                if let Some(path) = output_path {
                    let previous_file_name = match &path.file_name() {
                        Some(name) => name.to_string_lossy(),
                        None => "?".into(),
                    };
                    save_clicked = ui
                        .button(format!("Quick Export ({})", previous_file_name))
                        .clicked();
                }

                let scene_clone = self.model.scene.clone();
                let message_pipe_clone = self.message_popup_pipe.clone();
                let export_misc_clone = self.model.export_misc.clone();

                if save_clicked {
                    message_pipe_clone.default_handle(
                        scene_clone
                            .lock()
                            .unwrap()
                            .save(export_misc_clone.load(Ordering::SeqCst)),
                    );
                } else if save_as_clicked || save_clicked {
                    self.file_dialog_save.save_file(move |res| {
                        if let Ok(path) = res {
                            let mut scene_lock = scene_clone.lock().unwrap();
                            message_pipe_clone
                                .default_handle(scene_lock.set_output_path(Some(path.clone())));
                            message_pipe_clone.default_handle(
                                scene_lock.save(export_misc_clone.load(Ordering::SeqCst)),
                            );
                        }
                    });
                }
                self.file_dialog_save.update_effect(ctx);

                let scene_clone = self.model.scene.clone();
                let proxy_dirty = self.model.proxy_dirty.clone();
                let message_pipe_clone = self.message_popup_pipe.clone();
                if ui.button("Import Obj").clicked() {
                    self.file_dialog_obj.pick_file(move |res| {
                        if let Ok(path) = res {
                            let res = scene_clone.lock().unwrap().load_object_from_file(path);
                            match res {
                                Ok(_) => {
                                    proxy_dirty.store(true, Ordering::SeqCst);
                                }
                                Err(e) => message_pipe_clone.push_message(Message::from_error(e)),
                            };
                        }
                    });
                }
                self.file_dialog_obj.update_effect(ctx);

                let scene_clone = self.model.scene.clone();
                let message_pipe_clone = self.message_popup_pipe.clone();
                if ui.button("Export PNG").clicked() {
                    match self.model.frame_buffer.get_last_frame() {
                        Ok(last_frame) => {
                            if let Some(last_frame) = last_frame {
                                self.model.scene.lock().unwrap().set_last_render(last_frame);
                                self.file_dialog_export.save_file(move |res| {
                                    if let Ok(path) = res {
                                        match scene_clone
                                            .lock()
                                            .unwrap()
                                            .export_render_img(path.clone())
                                        {
                                            Ok(_) => message_pipe_clone.push_message(Message::new(
                                                "Export successful.",
                                                format!("Saved PNG to {}", path.display()).as_str(),
                                            )),
                                            Err(e) => message_pipe_clone
                                                .push_message(Message::from_error(e)),
                                        }
                                    };
                                });
                            } else {
                                message_pipe_clone.push_message(Message::new(
                                    "No rendered image to export.",
                                    "Please render the scene before exporting an image.",
                                ));
                            }
                        }
                        Err(e) => {
                            self.message_popup_pipe.push_message(Message::from_error(e));
                        }
                    }
                }
                self.file_dialog_export.update_effect(ctx);

                if ui.button("Toggle log-view").clicked() {
                    self.bottom_visible = !self.bottom_visible;
                }
            })
        });

        egui::SidePanel::left("SidePanel")
            .resizable(true)
            .min_width(220.0)
            .show(ctx, |ui| {
                self.model.consume_proxy_dirty_and_reload();

                let mut export_misc_loaded = self.model.export_misc.load(Ordering::SeqCst);

                if !export_misc_loaded {
                    ui.label(
                        RichText::new("⚠ Currently not exporting misc objects.")
                            .color(Color32::ORANGE)
                            .strong(),
                    );
                    ui.label(
                        RichText::new(
                            "Enable to also export: Spheres, Ray Samples and Color Hash to rscn.",
                        )
                        .small(),
                    );
                }

                if ui
                    .checkbox(&mut export_misc_loaded, "Export Additional Data")
                    .clicked()
                {
                    self.model
                        .export_misc
                        .store(export_misc_loaded, Ordering::SeqCst);
                }

                ui.separator();

                if self.model.frame_buffer.has_provider() {
                    if ui.button("Cancel Render").clicked() {
                        self.model.frame_buffer.stop_current_provider();
                    }
                } else if ui.button("Start Render").clicked() {
                    self.do_render();
                }

                ui.separator();

                let mut proxy_tmp = std::mem::take(&mut self.model.proxy);

                ui.label("Camera");
                if proxy_tmp.camera.ui_with_settings(
                    ui,
                    &mut self.model.scene.clone(),
                    &mut self.render_on_change,
                ) && self.render_on_change
                {
                    self.do_render();
                }

                ui.separator();

                if proxy_tmp.misc.ui(ui, &mut self.model.scene.clone()) && self.render_on_change {
                    self.do_render();
                }

                ui.separator();

                ui.label("Objects");
                if proxy_tmp.objects.ui(ui, &mut self.model.scene.clone()) && self.render_on_change
                {
                    self.do_render();
                }

                ui.separator();

                ui.label("Lights");
                if proxy_tmp.lights.ui(ui, &mut self.model.scene.clone()) && self.render_on_change {
                    self.do_render();
                }

                self.model.proxy = proxy_tmp;
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.image_area.ui(ui);
        });

        if self.bottom_visible {
            SceneScreen::logs_ui(ctx);
        }

        if let Some(output) = self.model.frame_buffer.try_recv() {
            match output {
                Ok(output) => {
                    self.image_area
                        .set_image(ctx, Image::new(output.width, output.height, output.pixels));
                }
                Err(e) => {
                    self.message_popup_pipe.push_message(Message::from_error(e));
                }
            }
        }

        // hopefully temporary fix to keep painting while render is going even if the user is doing nothing.
        if self.model.frame_buffer.has_provider() {
            ctx.request_repaint_after(FRAME_DURATION_FPS24);
        }

        None
    }
}
