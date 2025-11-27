use crate::pipeline::Pipeline;
use eframe::egui::{Context, TextureHandle, TextureOptions, Ui};
use eframe::{App, Frame};
use egui_file_dialog::FileDialog;
use std::path::PathBuf;
//Mockup Scene struct, remove later
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

pub trait ViewListener {
    fn handle_event(&mut self, event: Event);
}

// to avoid having to do .as_mut().unwrap() everywhere with the listener
pub struct NullListener;
impl ViewListener for NullListener {
    fn handle_event(&mut self, _event: Event) {}
}

pub struct View {
    at_start: bool,
    listener: Box<dyn ViewListener>,
    texture: Option<TextureHandle>,
    pipeline: Pipeline,
    bottom_visible: bool,
    file_dialog_obj: FileDialog,
    file_dialog_scene: FileDialog,
    obj_path: Option<PathBuf>,
    scene_path: Option<PathBuf>,
    //remove Mockup Scene later
    scene: SceneState,
}

impl App for View {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if self.at_start {
            self.on_start(ctx, _frame);
            self.at_start = false;
        }

        let render_output_opt = self.pipeline.take_render_output();
        if let Some(output) = render_output_opt {
            self.set_image(
                ctx,
                output.width as u32,
                output.height as u32,
                output.pixels,
            )
        }

        eframe::egui::TopBottomPanel::top("Toolbar").show(ctx, |ui| {
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

        eframe::egui::SidePanel::left("SidePanel")
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
                                eframe::egui::DragValue::new(&mut self.scene.cam_x)
                                    .speed(0.1)
                                    .prefix("x: "),
                            );
                            ui.add(
                                eframe::egui::DragValue::new(&mut self.scene.cam_y)
                                    .speed(0.1)
                                    .prefix("y: "),
                            );
                            ui.add(
                                eframe::egui::DragValue::new(&mut self.scene.cam_z)
                                    .speed(0.1)
                                    .prefix("z: "),
                            );

                            ui.separator();
                            ui.add(
                                eframe::egui::Slider::new(&mut self.scene.cam_yaw, -180.0..=180.0)
                                    .text("yaw"),
                            );
                            ui.add(
                                eframe::egui::Slider::new(&mut self.scene.cam_rot, -180.0..=180.0)
                                    .text("rotation"),
                            );
                        });
                    });

                    ui.collapsing("Lights", |ui| {
                        ui.label("TODO: Light objects hinzufügen");
                    });

                    ui.collapsing("Objects", |ui| {
                        ui.collapsing("ferris", |ui| {
                            ui.add(
                                eframe::egui::DragValue::new(&mut self.scene.ferris_x)
                                    .speed(0.1)
                                    .prefix("x: "),
                            );
                            ui.add(
                                eframe::egui::DragValue::new(&mut self.scene.ferris_y)
                                    .speed(0.1)
                                    .prefix("y: "),
                            );
                            ui.add(
                                eframe::egui::DragValue::new(&mut self.scene.ferris_z)
                                    .speed(0.1)
                                    .prefix("z: "),
                            );

                            ui.separator();
                            ui.add(
                                eframe::egui::Slider::new(
                                    &mut self.scene.ferris_yaw,
                                    -180.0..=180.0,
                                )
                                .text("yaw"),
                            );
                            ui.add(
                                eframe::egui::Slider::new(
                                    &mut self.scene.ferris_rot,
                                    -180.0..=180.0,
                                )
                                .text("rotation"),
                            )
                        });
                    });
                });
                ui.separator();

                if ui.button("Render").clicked() {
                    self.listener.handle_event(Event::DoRender);
                }

                let mut fov = self.pipeline.get_fov();
                if ui
                    .add(eframe::egui::Slider::new(&mut fov, 0.1..=20.0).text("FOV"))
                    .changed()
                {
                    self.pipeline.set_fov(fov);
                    self.listener.handle_event(Event::UpdateFOV);
                }

                ui.horizontal(|ui| {
                    ui.label("Width:");
                    let mut width = self.pipeline.get_width();
                    if ui.add(eframe::egui::DragValue::new(&mut width)).changed() {
                        self.pipeline.set_width(width);
                        self.listener.handle_event(Event::UpdateResolution);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Height:");
                    let mut height = self.pipeline.get_height();
                    if ui.add(eframe::egui::DragValue::new(&mut height)).changed() {
                        self.pipeline.set_height(height);
                        self.listener.handle_event(Event::UpdateResolution);
                    }
                });
            });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            self.display_image(ui);
        });

        eframe::egui::TopBottomPanel::bottom("Log-view")
            .resizable(true)
            .min_height(10.0)
            .show_animated(ctx, self.bottom_visible, |ui| {
                ui.label("Log-view");
                let available = ui.available_rect_before_wrap();
                ui.allocate_rect(available, eframe::egui::Sense::drag());
            });
    }
}

impl View {
    pub fn new(pipeline: Pipeline) -> Self {
        View {
            listener: Box::new(NullListener),
            texture: None,
            pipeline,
            bottom_visible: true,
            at_start: true,
            file_dialog_obj: FileDialog::new()
                .add_file_filter_extensions("OBJ", vec!["obj"])
                .default_file_filter("OBJ"),
            file_dialog_scene: FileDialog::new()
                .add_file_filter_extensions("JSON", vec!["json"])
                .default_file_filter("JSON"),
            obj_path: None,
            scene_path: None,
            //remove Mockup Struct later
            scene: SceneState::default(),
        }
    }

    pub fn open(self) {
        let options = eframe::NativeOptions::default();
        let _ = eframe::run_native("RenderBaby", options, Box::new(|_cc| Ok(Box::new(self))));
    }

    fn on_start(&mut self, _ctx: &Context, _frame: &mut Frame) {
        self.listener.handle_event(Event::UpdateResolution);
        self.listener.handle_event(Event::UpdateFOV);
        self.listener.handle_event(Event::DoRender);
    }

    pub fn set_listener(&mut self, listener: Box<dyn ViewListener>) {
        self.listener = listener;
    }

    pub fn set_image(&mut self, ctx: &Context, width: u32, height: u32, image: Vec<u8>) {
        let color_image = eframe::egui::ColorImage::from_rgba_unmultiplied(
            [width as usize, height as usize],
            &image,
        );
        self.texture = Some(ctx.load_texture("image", color_image, TextureOptions::NEAREST));
    }

    fn display_image(&mut self, ui: &mut Ui) {
        if let Some(image) = &self.texture {
            let aspect = image.size_vec2().x / image.size_vec2().y;
            let size_scaled = if ui.available_size().x / ui.available_size().y > aspect {
                eframe::egui::vec2(ui.available_size().y * aspect, ui.available_size().y)
            } else {
                eframe::egui::vec2(ui.available_size().x, ui.available_size().x / aspect)
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
        self.listener.handle_event(Event::ImportObj);
    }

    pub fn set_scene_filepath(&mut self) {
        self.pipeline.submit_scene_file_path(Option::from(
            self.scene_path
                .clone()
                .expect("REASON")
                .to_string_lossy()
                .into_owned(),
        ));
        self.listener.handle_event(Event::ImportScene);
    }

    /*     fn do_render(&mut self) {
        self.listener.handle_event(Event::DoRender);
    } */
}
