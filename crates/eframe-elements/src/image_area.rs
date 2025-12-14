use crate::effects::Effect;

pub struct Image {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

pub struct ImageArea {
    texture: Option<egui::TextureHandle>,
    no_texture_effect: Option<Box<dyn Effect>>,
}

impl ImageArea {
    pub fn reset_image(&mut self) {
        self.texture = None;
    }

    pub fn set_image(&mut self, ctx: &egui::Context, image: Image) {
        let color_image =
            egui::ColorImage::from_rgba_unmultiplied([image.width, image.height], &image.pixels);
        self.texture = Some(ctx.load_texture("image", color_image, egui::TextureOptions::NEAREST));
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if let Some(image) = &self.texture {
            let aspect = image.size_vec2().x / image.size_vec2().y;
            let size_scaled = if ui.available_size().x / ui.available_size().y > aspect {
                egui::vec2(ui.available_size().y * aspect, ui.available_size().y)
            } else {
                egui::vec2(ui.available_size().x, ui.available_size().x / aspect)
            };
            ui.image((image.id(), size_scaled));
        } else if let Some(effect) = self.no_texture_effect.as_mut() {
            effect.ui(ui);
        }
    }

    pub fn new(no_texture_effect: Option<Box<dyn Effect>>) -> Self {
        Self {
            texture: None,
            no_texture_effect,
        }
    }
}

impl Default for ImageArea {
    fn default() -> Self {
        Self::new(Some(Box::new(crate::effects::LoadingEffect::new(
            egui::Id::new("LoadingEffect"),
            48.0,
        ))))
    }
}
