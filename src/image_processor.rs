/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use std::io::Cursor;

use image::ImageFormat;

use crate::errors::AppError;

pub struct ImageProcessingOptions {
    pub size: u32,
    pub format: ImageFormat,
}
pub struct ProcessedImage {
    pub data: Vec<u8>,
    pub format: ImageFormat,
}

pub fn create_thumbnail(
    filepath: &std::path::Path,
    options: &ImageProcessingOptions,
) -> Result<ProcessedImage, AppError> {
    // Decode the source image
    let img = image::open(filepath)?;

    // Create thumbnail
    let thumbnail = img.thumbnail(options.size, options.size);

    // Convert to specified format
    let mut buf = Cursor::new(Vec::new());
    thumbnail.write_to(&mut buf, options.format)?;

    Ok(ProcessedImage {
        data: buf.into_inner(),
        format: options.format,
    })
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
            format: ImageFormat::Png,
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
            format: ImageFormat::WebP,
        };

        let result = create_thumbnail(std::path::Path::new(&file_path), &options).unwrap();
        assert_eq!(result.format, ImageFormat::WebP);

        // Verify the data is actually WebP
        assert!(image::guess_format(&result.data).unwrap() == ImageFormat::WebP);

        drop(temp_dir); // Cleanup
    }

    #[test]
    fn test_create_thumbnail_invalid_file() {
        let options = ImageProcessingOptions {
            size: 100,
            format: ImageFormat::Png,
        };

        let result = create_thumbnail(std::path::Path::new("nonexistent_file.png"), &options);
        assert!(result.is_err());
    }
}
