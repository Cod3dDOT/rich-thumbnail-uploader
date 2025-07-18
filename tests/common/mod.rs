/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use std::{error::Error, ffi::OsString, path::PathBuf};

use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
use rich_thumbnail_uploader::{config::Config, errors::AppError};
use tempfile::TempDir;

pub fn parse_args(args: &[&str]) -> Result<Config, AppError> {
    let mut new_args = vec![OsString::from("rich-thumbnail-uploader.exe")];
    new_args.extend(args.iter().map(|s| OsString::from(*s)));

    let pargs = pico_args::Arguments::from_vec(new_args);

    Config::parse(pargs)
}

pub fn create_test_image_with_format(
    width: u32,
    height: u32,
    format: ImageFormat,
    filename: &str,
) -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join(filename);

    let mut img = ImageBuffer::new(width, height);

    // Create a recognizable pattern for testing
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgba([
            (x % 256) as u8,
            (y % 256) as u8,
            100,
            if (x + y) % 2 == 0 { 255 } else { 128 }, // Varying transparency
        ]);
    }

    img.save_with_format(&file_path, format)
        .expect("Failed to save test image");
    assert!(file_path.exists(), "Image file was not created");

    (temp_dir, file_path)
}

pub fn create_invalid_image_file() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let file_path = temp_dir.path().join("invalid.png");

    std::fs::write(&file_path, b"This is not an image file").expect("Failed to write invalid file");
    (temp_dir, file_path)
}

pub fn load_image_from_url(url: &str) -> Result<DynamicImage, Box<dyn Error>> {
    let agent = reqwest::blocking::Client::new();
    let response = agent
        .get(url)
        .header("User-Agent", "rich-thumbnail-uploader-test")
        .send()?;
    let bytes = response.bytes()?;
    let img = image::load_from_memory(&bytes)?;
    Ok(img)
}
