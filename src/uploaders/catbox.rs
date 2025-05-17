use async_trait::async_trait;
use image::ImageFormat;
use reqwest::multipart::{Form, Part};

use crate::errors::AppError;
use crate::image_processor::ProcessedImage;
use crate::uploaders::UploadService;

use super::UploadServiceIdentifier;

pub struct CatboxUploader;

#[async_trait]
impl UploadService for CatboxUploader {
    async fn upload(
        filename: String,
        image: ProcessedImage,
        client_id: String,
        user_agent: String,
    ) -> Result<String, AppError> {
        // the user agent is necessary
        let client = reqwest::Client::builder().user_agent(user_agent).build()?;

        // Create the multipart form
        let file_part = Part::stream(image.data)
            .mime_str(image.format.to_mime_type())
            .map_err(|e| AppError::Upload(e.to_string()))?
            .file_name(filename);

        let form = Form::new()
            .text("reqtype", "fileupload")
            .text("userhash", client_id)
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

    fn identifier() -> UploadServiceIdentifier {
        UploadServiceIdentifier::Catbox
    }

    fn formats() -> Vec<ImageFormat> {
        vec![ImageFormat::Png, ImageFormat::WebP]
    }
}
