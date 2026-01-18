use crate::effects::Effect;

/// Image struct that can be used to display an image in an [`ImageArea`].
pub struct Image {
    /// Width of the image in pixels.
    width: usize,
    /// Height of the image in pixels.
    height: usize,
    /// RGBA pixels of the image.
    pixels: Vec<u8>,
}

impl Image {
    /// Create a new [`Image`] from a width, height and RGBA pixels vector.
    pub fn new(width: usize, height: usize, pixels: Vec<u8>) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }
}

/// Area that can display an [`Image`].
pub struct ImageArea {
    /// Texture handle of the image.
    texture: Option<egui::TextureHandle>,
    /// Optional: Effect that is shown when no image is set.
    no_texture_effect: Option<Box<dyn Effect>>,
}

impl ImageArea {
    /// Reset the image to `None`.
    pub fn reset_image(&mut self) {
        self.texture = None;
    }

    /// Set the [`Image`] to display to a ctx [`egui::Context`].
    pub fn set_image(&mut self, ctx: &egui::Context, image: Image) {
        let color_image =
            egui::ColorImage::from_rgba_unmultiplied([image.width, image.height], &image.pixels);
        self.texture = Some(ctx.load_texture("image", color_image, egui::TextureOptions::NEAREST));
    }

    /// Draw the image area to the given [`egui::Ui`].
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

    /// Create a new [`ImageArea`] with the given [`no_texture_effect`].
    pub fn new(no_texture_effect: Option<Box<dyn Effect>>) -> Self {
        Self {
            texture: None,
            no_texture_effect,
        }
    }
}

impl Default for ImageArea {
    /// Create a new [`ImageArea`] with a default [`LoadingEffect`].
    fn default() -> Self {
        Self::new(Some(Box::new(crate::effects::LoadingEffect::new(
            egui::Id::new("LoadingEffect"),
            48.0,
        ))))
    }
}
