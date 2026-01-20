use std::path::PathBuf;
use image::{ImageBuffer, Rgba};
use frame_buffer::frame_iterator::Frame;

pub fn export_img_png(path: PathBuf, frame: Frame) -> image::ImageResult<()> {
    let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
        frame.width as u32,
        frame.height as u32,
        frame.pixels.clone(),
    )
    .ok_or_else(|| {
        image::ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::DimensionMismatch,
        ))
    })?;

    img.save(path)
}
