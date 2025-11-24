use crate::pipeline::Pipeline;
use eframe::egui::{Context, TextureHandle, TextureOptions, Ui};
use eframe::{App, Frame};
use std::sync::{Arc, Mutex};

#[derive(PartialEq)]
pub enum Event {
    DoRender,
    //SetFov(f32),
    SetPath(String),
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
    file_path: Arc<Mutex<Option<String>>>,
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
                if ui.button("Import Files").clicked() {
                    self.set_filepath(ctx);
                }
                if let Some(path) = self.file_path.lock().unwrap().take() {
                    self.listener
                        .as_mut()
                        .handle_event(Event::SetPath(path.clone()));
                }
            })
        });

        eframe::egui::SidePanel::left("SidePanel")
            .resizable(true)
            .min_width(220.0)
            .show(ctx, |ui| {
                if ui.button("Render").clicked() {
                    self.do_render();
                }

                let mut fov = self.pipeline.get_fov();
                if ui
                    .add(eframe::egui::Slider::new(&mut fov, 0.1..=20.0).text("FOV"))
                    .changed()
                {
                    self.pipeline.set_fov(fov);
                    self.listener.handle_event(Event::DoRender);
                }

                ui.horizontal(|ui| {
                    ui.label("Width:");
                    let mut width = self.pipeline.get_width();
                    if ui.add(eframe::egui::DragValue::new(&mut width)).changed() {
                        self.pipeline.set_width(width);
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Height:");
                    let mut height = self.pipeline.get_height();
                    if ui.add(eframe::egui::DragValue::new(&mut height)).changed() {
                        self.pipeline.set_height(height);
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
            file_path: Arc::new(Mutex::new(None)),
            at_start: true,
        }
    }

    pub fn open(self) {
        let options = eframe::NativeOptions::default();
        let _ = eframe::run_native("RenderBaby", options, Box::new(|_cc| Ok(Box::new(self))));
    }

    fn on_start(&mut self, _ctx: &Context, _frame: &mut Frame) {
        self.do_render();
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

    pub fn set_filepath(&mut self, ctx: &Context) {
        let selected_path = self.file_path.clone();
        let ctx = ctx.clone();

        std::thread::spawn(move || {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("OBJ / MTL", &["obj", "mtl"])
                .pick_file()
            {
                *selected_path.lock().unwrap() = Some(path.display().to_string());
                ctx.request_repaint();
            }
        });
    }

    fn do_render(&mut self) {
        self.listener.handle_event(Event::DoRender);
    }
}
