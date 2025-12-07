use crate::control_plane::gui::*;
use eframe;
use eframe::egui;
use eframe::epaint;
use egui_file_dialog;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use egui::Align;
use view_wrappers::egui_view::EframeViewWrapper;
use view_wrappers::{EventHandler, EventResult, ViewWrapper};
use eframe_elements::message_popup::*;

// E FRAME VIEW:

#[derive(PartialEq)]
pub enum Event {
    DoRender,
    ImportObj,
    ImportScene,
    UpdateResolution,
    UpdateFOV,
}

pub struct View {
    pipeline: pipeline::Pipeline,
    handler: Arc<Mutex<Box<EventHandler<Event>>>>,
    texture: Option<epaint::TextureHandle>,
    bottom_visible: bool,
    file_dialog_obj: egui_file_dialog::FileDialog,
    file_dialog_scene: egui_file_dialog::FileDialog,
    obj_path: Option<PathBuf>,
    scene_path: Option<PathBuf>,
    json_text: String,
    error_popups: MessagePopupPipe,
}

impl View {
    fn handle_event(&self, event: Event) -> EventResult {
        let mut handler_lock = self.handler.lock().unwrap();
        (handler_lock)(event)
    }

    fn handle_event_threaded<C>(&self, event: Event, callback: Option<C>)
    where
        C: FnOnce(EventResult) + Send + 'static,
    {
        let handler_clone = self.handler.clone();
        std::thread::spawn(move || {
            let result = {
                let mut handler_lock = handler_clone.lock().unwrap();
                (handler_lock)(event)
            };
            if let Some(cb) = callback {
                cb(result);
            }
        });
    }

    pub fn do_standard_render(&self) {
        let error_pipe_clone = self.error_popups.clone();
        self.handle_event_threaded(
            Event::DoRender,
            Some(move |result: EventResult| {
                if let Err(e) = result {
                    error_pipe_clone.push_message(Message::from_error(e));
                }
            }),
        );
    }

    pub fn set_image(&mut self, ctx: &egui::Context, width: u32, height: u32, image: Vec<u8>) {
        let color_image =
            egui::ColorImage::from_rgba_unmultiplied([width as usize, height as usize], &image);
        self.texture = Some(ctx.load_texture("image", color_image, egui::TextureOptions::NEAREST));
    }

    fn display_image(&self, ui: &mut egui::Ui) {
        if let Some(image) = &self.texture {
            let aspect = image.size_vec2().x / image.size_vec2().y;
            let size_scaled = if ui.available_size().x / ui.available_size().y > aspect {
                egui::vec2(ui.available_size().y * aspect, ui.available_size().y)
            } else {
                egui::vec2(ui.available_size().x, ui.available_size().x / aspect)
            };
            ui.image((image.id(), size_scaled));
        } else {
            ui.label("Render Output Area");
        }
    }

    pub fn set_obj_filepath(&self) {
        self.pipeline.submit_obj_file_path(Option::from(
            self.obj_path
                .clone()
                .expect("REASON")
                .to_string_lossy()
                .into_owned(),
        ));
    }

    pub fn set_scene_filepath(&self) {
        self.pipeline.submit_scene_file_path(Option::from(
            self.scene_path
                .clone()
                .expect("REASON")
                .to_string_lossy()
                .into_owned(),
        ));
    }
}

impl ViewWrapper<Event, pipeline::Pipeline> for View {
    fn new(pipeline: pipeline::Pipeline, handler: Box<EventHandler<Event>>) -> Self {
        Self {
            pipeline,
            handler: Arc::new(Mutex::new(handler)),
            texture: None,
            bottom_visible: true,
            file_dialog_obj: egui_file_dialog::FileDialog::new()
                .add_file_filter_extensions("OBJ", vec!["obj"])
                .default_file_filter("OBJ"),
            file_dialog_scene: egui_file_dialog::FileDialog::new()
                .add_file_filter_extensions("JSON", vec!["json"])
                .default_file_filter("JSON"),
            obj_path: None,
            scene_path: None,
            json_text: String::new(),
            error_popups: MessagePopupPipe::new(),
        }
    }

    fn open(self) {
        self.open_native("RenderBaby");
    }
}

impl eframe::App for View {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.error_popups.show_last(ctx);

        let render_output_opt = self.pipeline.take_render_output();
        if let Some(output) = render_output_opt {
            self.set_image(
                ctx,
                output.width as u32,
                output.height as u32,
                output.pixels,
            )
        }

        egui::TopBottomPanel::top("Toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Toolbar");
                if ui.button("Toggle log-view").clicked() {
                    self.bottom_visible = !self.bottom_visible;
                }
                if ui.button("Import Obj").clicked() {
                    self.file_dialog_obj.pick_file();
                }
                if let Some(path) = self.file_dialog_obj.take_picked() {
                    self.obj_path = Some(path.to_path_buf());
                    self.set_obj_filepath();
                    if let Err(e) = self.handle_event(Event::ImportObj) {
                        self.error_popups.push_message(Message::from_error(e));
                    }
                }
                self.file_dialog_obj.update(ctx);

                if ui.button("Import Scene").clicked() {
                    self.file_dialog_scene.pick_file();
                }
                if let Some(path) = self.file_dialog_scene.take_picked() {
                    self.scene_path = Some(path.to_path_buf());
                    if let Ok(text) = std::fs::read_to_string(&path) {
                        self.json_text = text;
                    }
                    self.set_scene_filepath();
                    if let Err(e) = self.handle_event(Event::ImportScene) {
                        self.error_popups.push_message(Message::from_error(e));
                    }
                }
                self.file_dialog_scene.update(ctx);
            })
        });

        egui::SidePanel::left("SidePanel")
            .resizable(true)
            .min_width(220.0)
            .show(ctx, |ui| {
                if ui.button("Render").clicked() {
                    self.do_standard_render();
                }

                let mut fov = self.pipeline.get_fov();
                // TODO: Get the fov limits from somewhere central as consts.
                if ui
                    .add(egui::Slider::new(&mut fov, 0.1..=100.0).text("FOV"))
                    .changed()
                {
                    self.pipeline.set_fov(fov);
                    self.handle_event(Event::UpdateFOV)
                        .expect("TODO: panic message");
                    self.do_standard_render();
                }

                ui.horizontal(|ui| {
                    ui.label("Width:");
                    let mut width = self.pipeline.get_width();
                    if ui
                        .add(
                            egui::DragValue::new(&mut width), //.range(ImageResolution::MIN[0]..=ImageResolution::MAX[0]),
                        )
                        .changed()
                    {
                        self.pipeline.set_width(width);
                        self.handle_event(Event::UpdateResolution)
                            .expect("TODO: panic message");
                        self.do_standard_render();
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Height:");
                    let mut height = self.pipeline.get_height();
                    if ui
                        .add(
                            egui::DragValue::new(&mut height), //.range(ImageResolution::MIN[1]..=ImageResolution::MAX[1]),
                        )
                        .changed()
                    {
                        self.pipeline.set_height(height);
                        self.handle_event(Event::UpdateResolution)
                            .expect("TODO: panic message");
                        self.do_standard_render();
                    }
                });
                ui.separator();

                ui.label("Scene JSON:");
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.json_text)
                                .frame(true)
                                .lock_focus(true),
                        );
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.display_image(ui);
        });

        if self.bottom_visible {
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
}

impl EframeViewWrapper<Event, pipeline::Pipeline> for View {
    fn on_start(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_event(Event::UpdateResolution)
            .expect("Something's wrong");
        self.handle_event(Event::UpdateFOV)
            .expect("Something's wrong");
        self.handle_event(Event::DoRender)
            .expect("Replace this later with a start screen");
    }
}
