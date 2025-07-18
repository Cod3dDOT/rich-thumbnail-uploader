/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

mod common;

use image::GenericImageView;
use image_hasher::{HashAlg, HasherConfig};
use rich_thumbnail_uploader::{
    config::SupportedImageFormat,
    errors::AppError,
    image_processor::{create_thumbnail, ImageProcessingOptions},
    uploaders::upload,
};

#[test]
fn test_full_workflow() -> Result<(), AppError> {
    let test_cases = vec![
        (["-s", "catbox", "-f", "png"], SupportedImageFormat::Png),
        (["-s", "catbox", "-f", "webp"], SupportedImageFormat::WebP),
        // TODO: add imgur tests
        // (
        //     [
        //         "-s",
        //         "imgur",
        //         "-f",
        //         "png",
        //         "--uid",
        //         option_env!("IMGUR_CLIENT_ID").unwrap(),
        //     ],
        //     SupportedImageFormat::Png,
        // ),
    ];

    for (args, expected_format) in test_cases {
        let (temp_dir, file_path) = common::create_test_image_with_format(
            400,
            300,
            expected_format.to_image_format(),
            &format!("test_image.{}", expected_format.as_str()),
        );

        let config = common::parse_args(&args)?;
        assert_eq!(config.image_format, expected_format);

        let options = ImageProcessingOptions {
            size: config.image_dimensions,
            format: config.image_format.to_image_format(),
        };

        let thumbnail = create_thumbnail(&file_path, &options)?;

        let thumbnail_image = image::load_from_memory(&thumbnail.data)?;
        let (thumb_width, thumb_height) = thumbnail_image.dimensions();

        // Aspect ratio check
        let ratio = thumb_width as f64 / thumb_height as f64;
        let expected_ratio = 4.0 / 3.0;
        assert!((ratio - expected_ratio).abs() < 0.01);

        let upload_result = upload(
            config.service,
            &thumbnail,
            config.client_id.unwrap_or_default(),
            config.user_agent.to_string(),
        )?;

        let uploaded_image = common::load_image_from_url(&upload_result)
            .map_err(|e| AppError::Upload(format!("Failed to load uploaded image: {}", e)))?;

        // Recompute and compare perceptual hashes
        let hasher = HasherConfig::new().hash_alg(HashAlg::Gradient).to_hasher();

        let uploaded_hash = hasher.hash_image(&uploaded_image);
        let thumbnail_hash = hasher.hash_image(&thumbnail_image);

        // Compare with a low Hamming distance threshold (0 = identical)
        let distance = uploaded_hash.dist(&thumbnail_hash);
        assert!(distance <= 5, "Image hash distance too high: {}", distance);

        drop(temp_dir);
    }

    Ok(())
}
