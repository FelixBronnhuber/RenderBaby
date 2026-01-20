//! Texture data for material mapping.
//!
//! This module defines the [`TextureData`] struct, which stores RGBA texture images
//! used for material texture mapping.

use serde::{Deserialize, Serialize};

/// RGBA texture image data.
///
/// `TextureData` represents a 2D texture image with packed RGBA pixel data.
/// Textures can be applied to materials via the `texture_index` field in `Material`.
///
/// # Data Format
///
/// The `rgba_data` field stores pixels as packed `u32` values in RGBA format:
/// - Bits 0-7: Red channel
/// - Bits 8-15: Green channel
/// - Bits 16-23: Blue channel
/// - Bits 24-31: Alpha channel
///
/// # Serialization
///
/// This struct derives `Serialize` and `Deserialize` to support saving and loading
/// scenes with embedded textures.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextureData {
    /// Width of the texture in pixels.
    pub width: u32,
    /// Height of the texture in pixels.
    pub height: u32,
    /// Packed RGBA pixel data.
    ///
    /// Length should be `width * height`.
    /// Each `u32` represents one pixel in RGBA8888 format.
    pub rgba_data: Vec<u32>,
}

impl TextureData {
    /// Creates a new texture with specified dimensions and pixel data.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the texture in pixels
    /// * `height` - Height of the texture in pixels
    /// * `rgba_data` - Packed RGBA pixel data (length should be width * height)
    ///
    /// # Returns
    ///
    /// A new `TextureData` with the specified configuration.
    ///
    /// # Panics
    ///
    /// Does not validate that `rgba_data.len() == width * height`. Callers should
    /// ensure the data length matches the dimensions.
    pub fn new(width: u32, height: u32, rgba_data: Vec<u32>) -> Self {
        Self {
            width,
            height,
            rgba_data,
        }
    }
}
