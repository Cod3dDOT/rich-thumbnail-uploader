/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use image::ImageFormat;
use reqwest::blocking::{
    multipart::{Form, Part},
    Client,
};
use std::io::Cursor;

use crate::errors::AppError;
use crate::image_processor::ProcessedImage;
use crate::uploaders::UploadService;

pub struct CatboxUploader;

impl UploadService for CatboxUploader {
    fn upload(
        filename: String,
        image: &ProcessedImage,
        client_id: String,
        user_agent: String,
    ) -> Result<String, AppError> {
        let client = Client::builder().user_agent(user_agent).build()?;

        let part = Part::reader(Cursor::new(image.data.clone()))
            .file_name(filename)
            .mime_str(image.format.to_mime_type())
            .map_err(|e| AppError::Upload(e.to_string()))?;

        let form = Form::new()
            .text("reqtype", "fileupload")
            .text("userhash", client_id)
            .part("fileToUpload", part);

        let response = client
            .post("https://catbox.moe/user/api.php")
            .multipart(form)
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
