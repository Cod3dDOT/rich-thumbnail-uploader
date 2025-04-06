use crate::errors::AppError;
use image::{self, ImageFormat};
use std::io::Cursor;

pub struct ImageOptions {
    pub size: u32,
    pub format: ImageFormat,
}

pub fn create_thumbnail(filepath: &str, options: &ImageOptions) -> Result<Vec<u8>, AppError> {
    // Open the image
    let img = image::open(filepath)?;

    // Create thumbnail
    let thumbnail = img.thumbnail(options.size, options.size);

    let mut buf = Vec::new();
    thumbnail.write_to(&mut Cursor::new(&mut buf), options.format)?;

    Ok(buf)
}
