/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use std::ffi::OsString;
use std::path::PathBuf;

use image::{GenericImageView, ImageBuffer, Rgb};
use rich_thumbnail_uploader::config::Config;
use rich_thumbnail_uploader::errors::AppError;
use rich_thumbnail_uploader::image_processor::{create_thumbnail, ImageProcessingOptions};
use tempfile::TempDir;

fn parse_args(args: &[&str]) -> Result<Config, AppError> {
    let mut new_args = vec![OsString::from("rich-thumbnail-uploader.exe")];
    new_args.extend(args.iter().map(|s| OsString::from(*s)));

    let pargs = pico_args::Arguments::from_vec(new_args);

    Config::parse(pargs)
}

#[test]
fn test_end_to_end_image_processing() -> Result<(), Box<dyn std::error::Error>> {
    let (temp_dir, file_path) = create_test_image(200, 200);

    let args = ["-d", "200", "-f", "png", "-s", "catbox"];
    let config = parse_args(&args)?;

    let options = ImageProcessingOptions {
        size: config.image_dimensions,
        format: config.image_format.to_image_format(),
    };

    let result = create_thumbnail(&file_path, &options)?;
    let processed_img = image::load_from_memory(&result.data)?;

    assert_eq!(processed_img.dimensions(), (200, 200));
    assert_eq!(result.format, image::ImageFormat::Png);

    drop(temp_dir);

    Ok(())
}

#[test]
fn test_config_validation() {
    // Valid config for catbox
    let args = ["-s", "catbox", "-f", "png"];
    assert!(parse_args(&args).is_ok());

    // Invalid format for imgur
    let args = ["-s", "imgur", "-f", "webp", "--uid", "test_id"];
    assert!(parse_args(&args).is_err());
}

fn create_test_image(width: u32, height: u32) -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join("test_image.png");

    let mut img = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([x as u8, y as u8, 100]);
    }

    img.save(&file_path).expect("Failed to save test image");
    assert!(file_path.exists(), "Image file was not created");

    (temp_dir, file_path)
}
