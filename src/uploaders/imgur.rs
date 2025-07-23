/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use image::ImageFormat;
use miniserde::json;

use crate::errors::AppError;
use crate::image_processor::ProcessedImage;
use crate::models::imgur::ImgurResponse;
use crate::uploaders::UploadService;

pub struct ImgurUploader;

impl UploadService for ImgurUploader {
    fn upload(
        filename: &'static str,
        image: &ProcessedImage,
        client_id: String,
        user_agent: String,
    ) -> Result<String, AppError> {
        let file = attohttpc::MultipartFile::new("image", &image.data)
            .with_filename(filename)
            .with_type(image.format.to_image_format().to_mime_type())
            .map_err(|e| AppError::Upload(e.to_string()))?;

        let part = attohttpc::MultipartBuilder::new()
            .with_file(file)
            .build()
            .map_err(|e| AppError::Upload(e.to_string()))?;

        let response = attohttpc::post("https://api.imgur.com/3/image")
            .header("Authorization", format!("Client-ID {client_id}"))
            .header("User-Agent", &user_agent)
            .body(part)
            .send()?;

        if !response.status().is_success() {
            let err = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::Upload(format!("Imgur API error: {err}")));
        }

        let imgur_response: ImgurResponse = json::from_str(&response.text()?)
            .map_err(|e| AppError::Upload(format!("Failed to parse Imgur response: {e}")))?;

        if !imgur_response.success {
            return Err(AppError::Upload(
                "Imgur reported upload failure".to_string(),
            ));
        }

        Ok(imgur_response.data.link)
    }

    fn formats() -> Vec<ImageFormat> {
        vec![ImageFormat::Png]
    }
}
