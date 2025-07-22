/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use std::io::Cursor;

use crate::{config::SupportedOutputFormat, errors::AppError};

pub struct ImageProcessingOptions {
    pub size: u32,
    pub format: SupportedOutputFormat,
}
pub struct ProcessedImage {
    pub data: Vec<u8>,
    pub format: SupportedOutputFormat,
}

pub fn create_thumbnail(
    filepath: &std::path::Path,
    options: &ImageProcessingOptions,
) -> Result<ProcessedImage, AppError> {
    // Decode the source image
    let reader = image::ImageReader::open(filepath)?.with_guessed_format()?;
    let format = reader.format();
    let img = reader.decode()?;

    // Create thumbnail
    let thumbnail = img.thumbnail(options.size, options.size);

    if format == Some(options.format.to_image_format()) {
        return Ok(ProcessedImage {
            data: thumbnail.into_bytes(),
            format: options.format,
        });
    }

    let estimated_size = (thumbnail.width() * thumbnail.height()) as usize;
    let capacity = estimated_size.next_power_of_two();
    let mut buffer = Vec::with_capacity(capacity);
    thumbnail.write_to(
        &mut Cursor::new(&mut buffer),
        options.format.to_image_format(),
    )?;

    return Ok(ProcessedImage {
        data: buffer,
        format: options.format,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GenericImageView, ImageBuffer, ImageFormat, Rgb};
    use tempfile::TempDir;

    fn create_test_image(width: u32, height: u32) -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test_image.png");

        let mut img = ImageBuffer::new(width, height);

        // Fill with a simple pattern
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            *pixel = Rgb([x as u8, y as u8, 100]);
        }

        img.save(&file_path).unwrap();

        (temp_dir, file_path.to_string_lossy().to_string())
    }

    #[test]
    fn test_create_thumbnail_resizes_correctly() {
        let (temp_dir, file_path) = create_test_image(200, 200);

        let options = ImageProcessingOptions {
            size: 100,
            format: SupportedOutputFormat::Png,
        };

        let result = create_thumbnail(std::path::Path::new(&file_path), &options).unwrap();

        // Load the resulting image to verify dimensions
        let img = image::load_from_memory(&result.data).unwrap();
        assert_eq!(img.dimensions(), (100, 100));

        drop(temp_dir); // Cleanup
    }

    #[test]
    fn test_create_thumbnail_converts_format() {
        let (temp_dir, file_path) = create_test_image(200, 200);

        let options = ImageProcessingOptions {
            size: 100,
            format: SupportedOutputFormat::WebP,
        };

        let result = create_thumbnail(std::path::Path::new(&file_path), &options).unwrap();
        assert_eq!(result.format, SupportedOutputFormat::WebP);

        // Verify the data is actually WebP
        assert!(image::guess_format(&result.data).unwrap() == ImageFormat::WebP);

        drop(temp_dir); // Cleanup
    }

    #[test]
    fn test_create_thumbnail_invalid_file() {
        let options = ImageProcessingOptions {
            size: 100,
            format: SupportedOutputFormat::Png,
        };

        let result = create_thumbnail(std::path::Path::new("nonexistent_file.png"), &options);
        assert!(result.is_err());
    }
}
