use crate::control_plane::gui::*;
use eframe;
use eframe::egui;
use eframe::epaint;
use egui_file_dialog;
use std::path::PathBuf;
use egui::Align;
use view_wrappers::egui_view::EframeViewWrapper;
use view_wrappers::ViewWrapper;
use crate::data_plane::scene::geometric_object::ImageResolution;

// E FRAME VIEW:

#[derive(Default)]
pub struct SceneState {
    pub cam_x: f32,
    pub cam_y: f32,
    pub cam_z: f32,
    pub cam_yaw: f32,
    pub cam_rot: f32,

    pub ferris_x: f32,
    pub ferris_y: f32,
    pub ferris_z: f32,
    pub ferris_yaw: f32,
    pub ferris_rot: f32,
}

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
    handler: Box<dyn FnMut(Event)>,
    texture: Option<epaint::TextureHandle>,
    bottom_visible: bool,
    file_dialog_obj: egui_file_dialog::FileDialog,
    file_dialog_scene: egui_file_dialog::FileDialog,
    obj_path: Option<PathBuf>,
    scene_path: Option<PathBuf>,
    //remove Mockup Scene later
    scene: SceneState,
}

impl View {
    pub fn set_image(&mut self, ctx: &egui::Context, width: u32, height: u32, image: Vec<u8>) {
        let color_image =
            egui::ColorImage::from_rgba_unmultiplied([width as usize, height as usize], &image);
        self.texture = Some(ctx.load_texture("image", color_image, egui::TextureOptions::NEAREST));
    }

    fn display_image(&mut self, ui: &mut egui::Ui) {
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

    pub fn set_obj_filepath(&mut self) {
        self.pipeline.submit_obj_file_path(Option::from(
            self.obj_path
                .clone()
                .expect("REASON")
                .to_string_lossy()
                .into_owned(),
        ));
        (self.handler)(Event::ImportObj);
    }

    pub fn set_scene_filepath(&mut self) {
        self.pipeline.submit_scene_file_path(Option::from(
            self.scene_path
                .clone()
                .expect("REASON")
                .to_string_lossy()
                .into_owned(),
        ));
        (self.handler)(Event::ImportScene);
    }
}

impl ViewWrapper<Event, pipeline::Pipeline> for View {
    fn new(pipeline: pipeline::Pipeline, handler: Box<dyn FnMut(Event)>) -> Self {
        Self {
            pipeline,
            handler,
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
            //remove Mockup Struct later
            scene: SceneState::default(),
        }
    }

    fn open(self) {
        self.open_native("RenderBaby");
    }
}

impl eframe::App for View {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                }
                self.file_dialog_obj.update(ctx);
                if ui.button("Import Scene").clicked() {
                    self.file_dialog_scene.pick_file();
                }
                if let Some(path) = self.file_dialog_scene.take_picked() {
                    self.scene_path = Some(path.to_path_buf());
                    self.set_scene_filepath();
                }
                self.file_dialog_scene.update(ctx);
            })
        });

        egui::SidePanel::left("SidePanel")
            .resizable(true)
            .min_width(220.0)
            .show(ctx, |ui| {
                //temporary mockup code for demo presentation
                ui.separator();
                ui.heading("Scene Explorer");
                ui.collapsing("Scene", |ui| {
                    ui.collapsing("Uniforms", |ui| {
                        ui.collapsing("Camera", |ui| {
                            ui.add(
                                egui::DragValue::new(&mut self.scene.cam_x)
                                    .speed(0.1)
                                    .prefix("x: "),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.scene.cam_y)
                                    .speed(0.1)
                                    .prefix("y: "),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.scene.cam_z)
                                    .speed(0.1)
                                    .prefix("z: "),
                            );

                            ui.separator();
                            ui.add(
                                egui::Slider::new(&mut self.scene.cam_yaw, -180.0..=180.0)
                                    .text("yaw"),
                            );
                            ui.add(
                                egui::Slider::new(&mut self.scene.cam_rot, -180.0..=180.0)
                                    .text("rotation"),
                            );
                        });
                    });

                    ui.collapsing("Lights", |ui| {
                        ui.label("TODO: Add light objects");
                    });

                    ui.collapsing("Objects", |ui| {
                        ui.collapsing("ferris", |ui| {
                            ui.add(
                                egui::DragValue::new(&mut self.scene.ferris_x)
                                    .speed(0.1)
                                    .prefix("x: "),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.scene.ferris_y)
                                    .speed(0.1)
                                    .prefix("y: "),
                            );
                            ui.add(
                                egui::DragValue::new(&mut self.scene.ferris_z)
                                    .speed(0.1)
                                    .prefix("z: "),
                            );

                            ui.separator();
                            ui.add(
                                egui::Slider::new(&mut self.scene.ferris_yaw, -180.0..=180.0)
                                    .text("yaw"),
                            );
                            ui.add(
                                egui::Slider::new(&mut self.scene.ferris_rot, -180.0..=180.0)
                                    .text("rotation"),
                            )
                        });
                    });
                });
                ui.separator();

                if ui.button("Render").clicked() {
                    (self.handler)(Event::DoRender);
                }

                let mut fov = self.pipeline.get_fov();
                if ui
                    .add(egui::Slider::new(&mut fov, 0.1..=20.0).text("FOV"))
                    .changed()
                {
                    self.pipeline.set_fov(fov);
                    (self.handler)(Event::UpdateFOV);
                }

                ui.horizontal(|ui| {
                    ui.label("Width:");
                    let mut width = self.pipeline.get_width();
                    if ui
                        .add(
                            egui::DragValue::new(&mut width)
                                .range(ImageResolution::MIN[0]..=ImageResolution::MAX[0]),
                        )
                        .changed()
                    {
                        self.pipeline.set_width(width);
                        (self.handler)(Event::UpdateResolution);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Height:");
                    let mut height = self.pipeline.get_height();
                    if ui
                        .add(
                            egui::DragValue::new(&mut height)
                                .range(ImageResolution::MIN[1]..=ImageResolution::MAX[1]),
                        )
                        .changed()
                    {
                        self.pipeline.set_height(height);
                        (self.handler)(Event::UpdateResolution);
                    }
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
                                egui::Label::new(logs).halign(Align::LEFT),
                            );
                        });
                });
        }
    }
}

impl EframeViewWrapper<Event, pipeline::Pipeline> for View {
    fn on_start(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        (self.handler)(Event::UpdateResolution);
        (self.handler)(Event::UpdateFOV);
        (self.handler)(Event::DoRender);
    }
}
