/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use image::ImageFormat;

use crate::errors::AppError;
use crate::image_processor::ProcessedImage;
use crate::uploaders::UploadService;

pub struct CatboxUploader;

impl UploadService for CatboxUploader {
    fn upload(
        filename: &'static str,
        image: &ProcessedImage,
        client_id: String,
        user_agent: String,
    ) -> Result<String, AppError> {
        let file = attohttpc::MultipartFile::new("fileToUpload", &image.data)
            .with_filename(filename)
            .with_type(image.format.to_image_format().to_mime_type())
            .map_err(|e| AppError::Upload(e.to_string()))?;

        let part = attohttpc::MultipartBuilder::new()
            .with_text("reqtype", "fileupload")
            .with_text("userhash", &client_id)
            .with_file(file)
            .build()
            .map_err(|e| AppError::Upload(e.to_string()))?;

        let response = attohttpc::post("https://catbox.moe/user/api.php")
            .header("User-Agent", &user_agent)
            .body(part)
            .send()?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::Upload(format!("Catbox API error: {error_text}")));
        }

        let url = response.text()?;
        if url.is_empty() || !url.starts_with("https://") {
            return Err(AppError::Upload("Catbox returned invalid URL".to_string()));
        }

        Ok(url)
    }

    fn formats() -> Vec<ImageFormat> {
        vec![ImageFormat::Png, ImageFormat::WebP]
    }
}
