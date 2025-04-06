use async_trait::async_trait;
use reqwest::multipart::{Form, Part};

use crate::errors::AppError;
use crate::uploaders::UploadService;

pub struct CatboxUploader;

impl CatboxUploader {
    pub fn new() -> Self {
        CatboxUploader
    }
}

#[async_trait]
impl UploadService for CatboxUploader {
    async fn upload(
        &self,
        image_data: Vec<u8>,
        _options: &super::ImageOptions,
    ) -> Result<String, AppError> {
        let client = reqwest::Client::new();

        // Create the multipart form
        let file_part = Part::bytes(image_data)
            .mime_str("image/png")
            .map_err(|e| AppError::Upload(e.to_string()))?;

        let form = Form::new()
            .text("reqtype", "fileupload")
            .part("fileToUpload", file_part);

        // Make the request to Catbox API
        let response = client
            .post("https://catbox.moe/user/api.php")
            .multipart(form)
            .send()
            .await?;

        // Check if the request was successful
        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(AppError::Upload(format!(
                "Catbox API error: {}",
                error_text
            )));
        }

        // Get the URL from response (Catbox returns just the URL as text)
        let url = response.text().await?;

        if url.is_empty() || !url.starts_with("https://") {
            return Err(AppError::Upload("Catbox returned invalid URL".to_string()));
        }

        Ok(url)
    }
}
