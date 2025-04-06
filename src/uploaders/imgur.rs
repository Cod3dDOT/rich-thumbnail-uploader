use async_trait::async_trait;
use reqwest::multipart::{Form, Part};

use crate::errors::AppError;
use crate::models::imgur::ImgurResponse;
use crate::uploaders::UploadService;

pub struct ImgurUploader {
    client_id: String,
}

impl ImgurUploader {
    pub fn new(client_id: &str) -> Self {
        ImgurUploader {
            client_id: client_id.to_string(),
        }
    }
}

#[async_trait]
impl UploadService for ImgurUploader {
    async fn upload(
        &self,
        image_data: Vec<u8>,
        _options: &super::ImageOptions,
    ) -> Result<String, AppError> {
        let client = reqwest::Client::new();

        // Create the multipart form
        let part = Part::bytes(image_data)
            .mime_str("image/png")
            .map_err(|e| AppError::Upload(e.to_string()))?;

        let form = Form::new().part("image", part);

        // Make the request to Imgur API
        let response = client
            .post("https://api.imgur.com/3/image")
            .header("Authorization", format!("Client-ID {}", self.client_id))
            .multipart(form)
            .send()
            .await?;

        // Check if the request was successful
        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(AppError::Upload(format!("Imgur API error: {}", error_text)));
        }

        // Parse the JSON response
        let imgur_response = response.json::<ImgurResponse>().await?;

        // Check if the upload was successful
        if !imgur_response.success {
            return Err(AppError::Upload(
                "Imgur reported upload failure".to_string(),
            ));
        }

        Ok(imgur_response.data.link)
    }
}
