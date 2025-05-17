use crate::errors::AppError;
use image::{self, ImageFormat};
use std::io::Cursor;

pub struct ImageProcessingOptions {
    pub size: u32,
    pub format: ImageFormat,
}

pub struct ProcessedImage {
    pub data: Vec<u8>,
    pub format: ImageFormat,
}

pub fn create_thumbnail(
    filepath: &str,
    options: &ImageProcessingOptions,
) -> Result<ProcessedImage, AppError> {
    // Open the image
    let img = image::open(filepath)?;

    // Create thumbnail
    let thumbnail = img.thumbnail(options.size, options.size);

    // Convert to specified format
    let mut buf = Cursor::new(Vec::new());
    thumbnail
        .write_to(&mut buf, options.format)
        .map_err(|e| AppError::Image(e))?;

    Ok(ProcessedImage {
        data: buf.into_inner(),
        format: options.format,
    })
}
