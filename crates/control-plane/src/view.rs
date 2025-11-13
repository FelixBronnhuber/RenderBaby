use eframe::egui::{Context, TextureHandle, TextureOptions, Ui};
use eframe::{App, Frame};

#[allow(dead_code)]
pub enum Event {}

pub trait ViewListener {
    #[allow(dead_code)]
    fn handle_event(&mut self, event: Event);
}

pub struct View {
    listener: Option<Box<dyn ViewListener>>,
    texture: Option<TextureHandle>,
}

impl App for View {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            self.display_image(ui);
        });
    }
}

impl View {
    pub fn new() -> Self {
        View {
            listener: None,
            texture: None,
        }
    }

    pub fn open(self) {
        let options = eframe::NativeOptions::default();
        let _ = eframe::run_native("RenderBaby", options, Box::new(|_cc| Ok(Box::new(self))));
    }

    pub fn set_listener(&mut self, listener: Box<dyn ViewListener>) {
        self.listener = Some(listener);
    }

    #[allow(dead_code)]
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