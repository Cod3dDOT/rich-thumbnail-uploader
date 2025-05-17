use clap::Parser;
use image::{GenericImageView, ImageBuffer, Rgb};
use rich_thumbnail_uploader::{
    cli::Cli,
    config::Config,
    image_processor::{create_thumbnail, ImageProcessingOptions},
};
use std::path::PathBuf;
use tempfile::TempDir;

fn create_test_image(width: u32, height: u32) -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_image.png");

    let mut img = ImageBuffer::new(width, height);

    // Fill with a simple pattern
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([x as u8, y as u8, 100]);
    }

    img.save(&file_path).unwrap();

    (temp_dir, file_path)
}

#[test]
fn test_end_to_end_image_processing() {
    let (temp_dir, file_path) = create_test_image(400, 400);

    // Create CLI args
    let args = vec![
        "rich-thumbnail-uploader",
        "--dims",
        "200",
        "--format",
        "png",
        "--service",
        "catbox", // Using catbox as it doesn't require authentication
    ];

    let cli = Cli::parse_from(args);

    // Initialize config
    let config = Config::new(&cli).unwrap();

    // Process image
    let options = ImageProcessingOptions {
        size: config.image_dimensions.0,
        format: config.image_format.to_image_format(),
    };

    let result = create_thumbnail(file_path.to_str().unwrap(), &options).unwrap();

    // Verify the result
    let processed_img = image::load_from_memory(&result.data).unwrap();
    assert_eq!(processed_img.dimensions(), (200, 200));
    assert_eq!(result.format, image::ImageFormat::Png);

    drop(temp_dir); // Cleanup
}

#[test]
fn test_config_validation() {
    // Test valid config
    let args = vec![
        "rich-thumbnail-uploader",
        "--service",
        "catbox",
        "--format",
        "png",
    ];
    let cli = Cli::parse_from(args);
    let config = Config::new(&cli);
    assert!(config.is_ok());

    // Test invalid format for service
    let args = vec![
        "rich-thumbnail-uploader",
        "--service",
        "imgur",
        "--format",
        "webp", // Imgur doesn't support WebP
        "--uid",
        "test_id",
    ];
    let cli = Cli::parse_from(args);
    let config = Config::new(&cli);
    assert!(config.is_err());
}
