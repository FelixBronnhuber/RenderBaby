use eframe::egui::{Context, TextureHandle, TextureOptions, Ui};
use eframe::{App, Frame};
use crate::pipeline::Pipeline;

#[derive(PartialEq)]
pub enum Event {
    DoRender
}

pub trait ViewListener {
    fn handle_event(&mut self, event: Event);
}

pub struct View {
    listener: Option<Box<dyn ViewListener>>,
    texture: Option<TextureHandle>,
    pipeline: Pipeline,
}

impl App for View {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        let render_output_opt = self.pipeline.render_output_ppl.lock().unwrap().take();
        if let Some(output) = render_output_opt {
            self.set_image(ctx, output.width as u32, output.height as u32, output.pixels)
        }

        eframe::egui::SidePanel::left("SidePanel")
            .resizable(true)
            .min_width(220.0)
            .show(ctx, |ui| {
                if ui.button("Render").clicked() {
                    self.listener.as_mut().unwrap().handle_event(Event::DoRender);
                }
            });

        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            self.display_image(ui);
        });
    }
}

impl View {
    pub fn new(pipeline: Pipeline) -> Self {
        View {
            listener: None,
            texture: None,
            pipeline,
        }
    }

    pub fn open(self) {
        let options = eframe::NativeOptions::default();
        let _ = eframe::run_native("RenderBaby", options, Box::new(|_cc| Ok(Box::new(self))));
    }

    pub fn set_listener(&mut self, listener: Box<dyn ViewListener>) {
        self.listener = Some(listener);
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
            ui.image((image.id(), image.size_vec2()));
        } else {
            ui.label("Render Output Area");
        }
    }
}
