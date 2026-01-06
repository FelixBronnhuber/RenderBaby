use crate::control_plane::modes::gui::*;
use eframe;
use eframe::egui;
use eframe::epaint;
use std::sync::{Arc, Mutex};
use egui::Align;
use log::info;
use rfd::FileDialog;
use view_wrappers::egui_view::EframeViewWrapper;
use view_wrappers::{EventHandler, EventResult, ViewWrapper};
use eframe_elements::{
    message_popup::{Message, MessagePopupPipe},
    file_picker::ThreadedNativeFileDialog,
};

// E FRAME VIEW:

#[derive(PartialEq)]
pub enum Event {
    DoRender,
    ImportObj,
    ImportScene,
    UpdateResolution,
    UpdateFOV,
    UpdateColorHash,
    UpdateCamera,
    UpdateSamples,
    DeleteSpheres,
    DeletePolygons,
    ExportImage,
}

pub struct View {
    pipeline: pipeline::Pipeline,
    handler: Arc<Mutex<Box<EventHandler<Event>>>>,
    texture: Option<epaint::TextureHandle>,
    bottom_visible: bool,
    file_dialog_obj: ThreadedNativeFileDialog,
    file_dialog_scene: ThreadedNativeFileDialog,
    file_dialog_export: ThreadedNativeFileDialog,
    message_popups: MessagePopupPipe,
}

impl View {
    fn handle_event(&self, event: Event) -> EventResult {
        let mut handler_lock = self.handler.lock().unwrap();
        handler_lock(event)
    }

    fn handle_event_threaded<C>(&self, event: Event, callback: Option<C>)
    where
        C: FnOnce(EventResult) + Send + 'static,
    {
        let handler_clone = self.handler.clone();
        std::thread::spawn(move || {
            let result = {
                let mut handler_lock = handler_clone.lock().unwrap();
                handler_lock(event)
            };
            if let Some(cb) = callback {
                cb(result);
            }
        });
    }

    pub fn do_standard_render(&self) {
        let error_pipe_clone = self.message_popups.clone();
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

    fn do_file_dialog<R, F, Submit>(
        &self,
        file_dialog: &ThreadedNativeFileDialog,
        file_dialog_fn: F,
        submit: Submit,
        event: Event,
    ) where
        R: Send + 'static,
        F: for<'a> FnOnce(
            &'a ThreadedNativeFileDialog,
            Box<dyn FnOnce(anyhow::Result<R>) + Send + 'static>,
        ),
        Submit: FnOnce(&pipeline::Pipeline, Option<R>) + Send + 'static,
    {
        let handle_event_clone = self.handler.clone();
        let message_popups_clone = self.message_popups.clone();
        let pipeline_clone = self.pipeline.clone();

        file_dialog_fn(
            file_dialog,
            Box::new(move |res: anyhow::Result<R>| {
                match res {
                    Ok(content) => {
                        submit(&pipeline_clone, Option::from(content));
                    }
                    Err(e) => return info!("Error: {:?}", e.to_string()),
                }
                if let Err(e) = handle_event_clone.lock().unwrap()(event) {
                    message_popups_clone.push_message(Message::from_error(e));
                }
            }),
        );
    }
}

impl ViewWrapper<Event, pipeline::Pipeline> for View {
    fn new(pipeline: pipeline::Pipeline, handler: Box<EventHandler<Event>>) -> Self {
        Self {
            pipeline,
            handler: Arc::new(Mutex::new(handler)),
            texture: None,
            bottom_visible: true,
            file_dialog_obj: ThreadedNativeFileDialog::new(
                FileDialog::new().add_filter("OBJ", &["obj"]),
            ),
            file_dialog_scene: ThreadedNativeFileDialog::new(
                FileDialog::new().add_filter("JSON", &["json"]),
            ),
            file_dialog_export: ThreadedNativeFileDialog::new(
                FileDialog::new().add_filter("IMAGE", &["png"]),
            ),
            message_popups: MessagePopupPipe::new(),
        }
    }

    fn open(self) {
        self.open_native("RenderBaby");
    }
}

impl eframe::App for View {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.message_popups.show_last(ctx);

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
                    self.do_file_dialog(
                        &self.file_dialog_obj,
                        ThreadedNativeFileDialog::pick_file,
                        pipeline::Pipeline::submit_obj_file_path,
                        Event::ImportObj,
                    );
                }
                self.file_dialog_obj.update_effect(ctx);

                if ui.button("Import Scene").clicked() {
                    self.do_file_dialog(
                        &self.file_dialog_scene,
                        ThreadedNativeFileDialog::pick_file,
                        pipeline::Pipeline::submit_scene_file_path,
                        Event::ImportScene,
                    );
                }
                self.file_dialog_scene.update_effect(ctx);

                if ui.button("Export PNG").clicked() {
                    self.do_file_dialog(
                        &self.file_dialog_export,
                        ThreadedNativeFileDialog::save_file,
                        pipeline::Pipeline::submit_export_file_path,
                        Event::ExportImage,
                    );
                }
                self.file_dialog_export.update_effect(ctx);
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
                    }
                });
                ui.separator();

                ui.label("Camera Position:");
                let mut cam_pos = self.pipeline.get_camera_pos();
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label("X:");
                    changed |= ui
                        .add(egui::DragValue::new(&mut cam_pos[0]).speed(0.1))
                        .changed();
                    ui.label("Y:");
                    changed |= ui
                        .add(egui::DragValue::new(&mut cam_pos[1]).speed(0.1))
                        .changed();
                    ui.label("Z:");
                    changed |= ui
                        .add(egui::DragValue::new(&mut cam_pos[2]).speed(0.1))
                        .changed();
                });
                if changed {
                    self.pipeline.set_camera_pos(cam_pos);
                    self.handle_event(Event::UpdateCamera)
                        .expect("Failed to handle UpdateCamera");
                }

                ui.label("Camera Direction:");
                let mut cam_dir = self.pipeline.get_camera_dir();
                changed = false;
                ui.horizontal(|ui| {
                    ui.label("X:");
                    changed |= ui
                        .add(egui::DragValue::new(&mut cam_dir[0]).speed(0.01))
                        .changed();
                    ui.label("Y:");
                    changed |= ui
                        .add(egui::DragValue::new(&mut cam_dir[1]).speed(0.01))
                        .changed();
                    ui.label("Z:");
                    changed |= ui
                        .add(egui::DragValue::new(&mut cam_dir[2]).speed(0.01))
                        .changed();
                });
                if changed {
                    self.pipeline.set_camera_dir(cam_dir);
                    self.handle_event(Event::UpdateCamera)
                        .expect("Failed to handle UpdateCamera");
                }

                ui.separator();

                let mut color_hash_enabled = self.pipeline.get_color_hash_enabled();
                if ui
                    .checkbox(&mut color_hash_enabled, "Enable Color Hash")
                    .changed()
                {
                    self.pipeline.set_color_hash_enabled(color_hash_enabled);
                    self.handle_event(Event::UpdateColorHash)
                        .expect("Failed to handle `Event::UpdateColorHash`");
                }

                ui.separator();

                let mut samples = self.pipeline.get_samples();
                if ui
                    .add(egui::Slider::new(&mut samples, 1..=2000).text("Samples"))
                    .changed()
                {
                    self.pipeline.set_samples(samples);
                    self.handle_event(Event::UpdateSamples)
                        .expect("Failed to handle `Event::UpdateSamples`");
                }

                ui.separator();

                if ui.button("Delete Spheres").clicked() {
                    self.handle_event(Event::DeleteSpheres)
                        .expect("Failed to handle DeleteSpheres");
                }

                if ui.button("Delete Polygons").clicked() {
                    self.handle_event(Event::DeletePolygons)
                        .expect("Failed to handle DeletePolygons");
                }
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
        self.handle_event(Event::UpdateCamera)
            .expect("Something's wrong");
        self.handle_event(Event::UpdateSamples)
            .expect("Something's wrong");
        self.do_standard_render();
    }
}
