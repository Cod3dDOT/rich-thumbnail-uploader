/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use std::io::Cursor;

use image::ImageFormat;
use miniserde::json;
use reqwest::blocking::{
    multipart::{Form, Part},
    Client,
};

use crate::errors::AppError;
use crate::image_processor::ProcessedImage;
use crate::models::imgur::ImgurResponse;
use crate::uploaders::UploadService;

pub struct ImgurUploader;

impl UploadService for ImgurUploader {
    fn upload(
        filename: String,
        image: &ProcessedImage,
        client_id: String,
        user_agent: String,
    ) -> Result<String, AppError> {
        let client = Client::builder().user_agent(user_agent).build()?;

        let part = Part::reader(Cursor::new(image.data.clone()))
            .mime_str(image.format.to_mime_type())
            .map_err(|e| AppError::Upload(e.to_string()))?
            .file_name(filename);

        let form = Form::new().part("image", part);

        let response = client
            .post("https://api.imgur.com/3/image")
            .header("Authorization", &format!("Client-ID {client_id}"))
            .multipart(form)
            .send()?;

        if !response.status().is_success() {
            let err = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AppError::Upload(format!("Imgur API error: {err}")));
        }

        let imgur_response: ImgurResponse = json::from_str(&response.text()?).unwrap();

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
