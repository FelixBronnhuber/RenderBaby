use std::sync::{Arc, Mutex};
use engine_raytracer::RenderOutput;
use eframe::egui::{self, Context, TextureHandle};
use crossbeam::channel::Receiver;

pub enum Event {
    RenderButton,
}

pub trait ViewListener: Send + Sync {
    fn handle_event(&mut self, event: Event);
}

pub struct View {
    pub listener: Option<Arc<Mutex<dyn ViewListener>>>,
    pub texture: Option<TextureHandle>,
    pub rx: Receiver<RenderOutput>,
}

impl View {
    pub fn new(rx: Receiver<RenderOutput>) -> Self {
        Self {
            listener: None,
            texture: None,
            rx,
        }
    }

    pub fn set_listener(&mut self, listener: Arc<Mutex<dyn ViewListener>>) {
        self.listener = Some(listener);
    }

    pub fn update_image_from_output(&mut self, ctx: &Context, output: &RenderOutput) {
        let size = [output.width, output.height];
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, &output.pixels);
        self.texture = Some(ctx.load_texture("output", color_image, egui::TextureOptions::LINEAR));
    }

    fn check_channel(&mut self, ctx: &Context) {
        while let Ok(output) = self.rx.try_recv() {
            self.update_image_from_output(ctx, &output);
        }
    }
}

impl eframe::App for View {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {

        egui::SidePanel::left("SidePanel")
            .resizable(true)
            .min_width(220.0)
            .show(ctx, |ui| {
                if ui.button("Render").clicked() {
                    if let Some(listener) = &self.listener {
                        listener.lock().unwrap().handle_event(Event::RenderButton);
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.check_channel(ctx);

            if let Some(img) = &self.texture {
                ui.image((img.id(), img.size_vec2()));
            }
        });
    }
}
