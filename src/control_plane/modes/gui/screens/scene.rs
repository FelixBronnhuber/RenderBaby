use eframe::emath::Align;
use egui::CollapsingHeader;
use rfd::FileDialog;
use eframe_elements::file_picker::ThreadedNativeFileDialog;
use eframe_elements::image_area::{Image, ImageArea};
use eframe_elements::message_popup::{Message, MessagePopupPipe};
use crate::control_plane::modes::gui::model::Model;
use crate::control_plane::modes::gui::screens::Screen;
use crate::control_plane::modes::gui::screens::start::StartScreen;
use crate::control_plane::modes::gui::screens::viewable::Viewable;
use crate::control_plane::modes::is_debug_mode;
use crate::data_plane::scene_proxy::proxy_light::ProxyLight;

#[allow(dead_code)]
pub struct SceneScreen {
    model: Model,
    bottom_visible: bool,
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
            file_dialog_obj: ThreadedNativeFileDialog::new(
                FileDialog::new().add_filter("OBJ", &["obj"]),
            ),
            file_dialog_export: ThreadedNativeFileDialog::new(
                FileDialog::new().add_filter("IMAGE", &["png"]),
            ),
            file_dialog_save: ThreadedNativeFileDialog::new(
                FileDialog::new().add_filter("JSON", &["json"]),
            ),
            image_area: ImageArea::new(Default::default()),
            message_popup_pipe: MessagePopupPipe::new(),
        }
    }
}

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

    fn do_render(&mut self, ctx: &egui::Context) {
        match self.model.scene.lock().unwrap().render() {
            Ok(output) => {
                self.image_area
                    .set_image(ctx, Image::new(output.width, output.height, output.pixels));
            }
            Err(e) => {
                self.message_popup_pipe.push_message(Message::from_error(e));
            }
        }
    }
}

impl Screen for SceneScreen {
    fn default_size(&self) -> egui::Vec2 {
        egui::Vec2::new(1200.0, 800.0)
    }

    fn on_start(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.do_render(ctx);
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
                ui.label("Toolbar");

                if ui.button("Toggle log-view").clicked() {
                    self.bottom_visible = !self.bottom_visible;
                }

                let scene_clone = self.model.scene.clone();
                let message_pipe_clone = self.message_popup_pipe.clone();
                if ui.button("Import Obj").clicked() {
                    self.file_dialog_obj.pick_file(move |res| {
                        if let Ok(path) = res {
                            println!("Selected file: {:?}", path.display());
                            match scene_clone.lock().unwrap().load_object_from_file(path) {
                                Ok(_) => {
                                    todo!("Update the proxy in model")
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
                    self.file_dialog_export.save_file(move |res| {
                        if let Ok(path) = res {
                            println!("Selected file: {:?}", path.display());
                            match scene_clone.lock().unwrap().export_render_img(path.clone()) {
                                Ok(_) => message_pipe_clone.push_message(Message::new(
                                    "Export successful.",
                                    format!("Saved PNG to {}", path.display()).as_str(),
                                )),
                                Err(e) => message_pipe_clone.push_message(Message::from_error(e)),
                            }
                        };
                    });
                }
                self.file_dialog_export.update_effect(ctx);

                let scene_clone = self.model.scene.clone();
                let message_pipe_clone = self.message_popup_pipe.clone();
                if ui.button("Save").clicked() {
                    self.file_dialog_save.save_file(move |res| {
                        if let Ok(path) = res {
                            println!("Selected file: {:?}", path.display());
                            match scene_clone.lock().unwrap().export_scene(path.clone()) {
                                Ok(_) => {}
                                Err(e) => message_pipe_clone.push_message(Message::from_error(e)),
                            }
                        }
                    });
                }
                self.file_dialog_save.update_effect(ctx);
            })
        });

        egui::SidePanel::left("SidePanel")
            .resizable(true)
            .min_width(220.0)
            .show(ctx, |ui| {
                if ui.button("Render").clicked() {
                    self.do_render(ctx);
                }

                ui.separator();

                self.model
                    .proxy
                    .camera
                    .ui(ui, &mut self.model.scene.lock().unwrap());

                ui.separator();

                self.model
                    .proxy
                    .misc
                    .ui(ui, &mut self.model.scene.lock().unwrap());

                ui.separator();

                {
                    let mut scene_lock = self.model.scene.lock().unwrap();
                    let real_meshes = scene_lock.get_meshes_mut();
                    ui.label("Objects");
                    let enum_meshes = self.model.proxy.objects.iter_mut();
                    if enum_meshes.len() == 0 {
                        ui.label("No objects in scene.");
                    } else {
                        for (i, proxy_mesh) in enum_meshes.enumerate() {
                            CollapsingHeader::new(format!("Object {}", i))
                                .default_open(false)
                                .show(ui, |ui| {
                                    proxy_mesh.ui(ui, &mut real_meshes[i]);
                                });

                            if ui.small_button("remove").clicked() {
                                real_meshes.remove(i);
                                self.model.proxy.objects.remove(i);
                                break;
                            }
                        }
                    }
                }

                ui.separator();

                {
                    let mut scene_lock = self.model.scene.lock().unwrap();
                    let real_lights = scene_lock.get_light_sources_mut();
                    ui.label("Lights");
                    let enum_light = self.model.proxy.lights.iter_mut();
                    if enum_light.len() == 0 {
                        ui.label("No lights in scene.");
                    } else {
                        for (i, proxy_light) in enum_light.enumerate() {
                            CollapsingHeader::new(format!("Light {}", i))
                                .default_open(false)
                                .show(ui, |ui| {
                                    proxy_light.ui(ui, &mut real_lights[i]);
                                });

                            if ui.small_button("remove").clicked() {
                                real_lights.remove(i);
                                self.model.proxy.lights.remove(i);
                                break;
                            }
                        }
                    }

                    if ui.small_button("+").clicked() {
                        let new_light = ProxyLight::default();
                        self.model
                            .scene
                            .lock()
                            .unwrap()
                            .add_lightsource(new_light.clone().into());
                        self.model.proxy.lights.push(new_light);
                    }
                }
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
