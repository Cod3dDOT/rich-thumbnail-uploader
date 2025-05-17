use std::time::{SystemTime, UNIX_EPOCH};

use crate::{errors::AppError, image_processor::ProcessedImage};
use async_trait::async_trait;
use clap::ValueEnum;
use image::ImageFormat;

pub mod catbox;
pub mod imgur;

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum UploadServiceIdentifier {
    Imgur,
    Catbox,
}

impl UploadServiceIdentifier {
    pub fn to_string(&self) -> String {
        match self {
            UploadServiceIdentifier::Imgur => "imgur".to_string(),
            UploadServiceIdentifier::Catbox => "catbox".to_string(),
        }
    }

    pub fn formats(&self) -> Vec<ImageFormat> {
        match self {
            UploadServiceIdentifier::Imgur => imgur::ImgurUploader::formats(),
            UploadServiceIdentifier::Catbox => catbox::CatboxUploader::formats(),
        }
    }
}

pub async fn upload(
    service: UploadServiceIdentifier,
    image: ProcessedImage,
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
            imgur::ImgurUploader::upload(filename, image, client_id, user_agent).await
        }
        UploadServiceIdentifier::Catbox => {
            catbox::CatboxUploader::upload(filename, image, client_id, user_agent).await
        }
    }
}

#[async_trait]
pub trait UploadService {
    async fn upload(
        filename: String,
        image: ProcessedImage,
        client_id: String,
        user_agent: String,
    ) -> Result<String, AppError>;

    fn identifier() -> UploadServiceIdentifier;

    fn formats() -> Vec<ImageFormat>;
}
