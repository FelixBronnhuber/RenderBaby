use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextureData {
    pub width: u32,
    pub height: u32,
    pub rgba_data: Vec<u32>, // Packed RGBA
}

impl TextureData {
    pub fn new(width: u32, height: u32, rgba_data: Vec<u32>) -> Self {
        Self {
            width,
            height,
            rgba_data,
        }
    }
}
