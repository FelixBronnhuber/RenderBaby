use std::path::PathBuf;
use image::{ImageBuffer, Rgba};
use engine_config::RenderOutput;

pub fn export_img_png(path: PathBuf, render: RenderOutput) -> image::ImageResult<()> {
    let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(
        render.width as u32,
        render.height as u32,
        render.pixels.clone(),
    )
    .ok_or_else(|| {
        image::ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::DimensionMismatch,
        ))
    })?;

    img.save(path)
}
