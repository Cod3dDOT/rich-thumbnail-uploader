/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use std::time::{SystemTime, UNIX_EPOCH};

use crate::{errors::AppError, image_processor::ProcessedImage};
use clap::ValueEnum;
use image::ImageFormat;

pub mod catbox;
pub mod imgur;

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum UploadServiceIdentifier {
    Imgur,
    Catbox,
}

impl UploadServiceIdentifier {
    pub fn as_str(&self) -> &'static str {
        match self {
            UploadServiceIdentifier::Imgur => "imgur",
            UploadServiceIdentifier::Catbox => "catbox",
        }
    }

    pub fn formats(&self) -> Vec<ImageFormat> {
        match self {
            UploadServiceIdentifier::Imgur => imgur::ImgurUploader::formats(),
            UploadServiceIdentifier::Catbox => catbox::CatboxUploader::formats(),
        }
    }
}

pub fn upload(
    service: UploadServiceIdentifier,
    image: &ProcessedImage,
    client_id: String,
    user_agent: String,
) -> Result<String, AppError> {
    // random filename
    let filename = format!(
        "{}.{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros(),
        image.format.extensions_str().first().unwrap()
    );
    match service {
        UploadServiceIdentifier::Imgur => {
            imgur::ImgurUploader::upload(filename, image, client_id, user_agent)
        }
        UploadServiceIdentifier::Catbox => {
            catbox::CatboxUploader::upload(filename, image, client_id, user_agent)
        }
    }
}

pub trait UploadService {
    fn upload(
        filename: String,
        image: &ProcessedImage,
        client_id: String,
        user_agent: String,
    ) -> Result<String, AppError>;

    fn formats() -> Vec<ImageFormat>;
}
