use async_trait::async_trait;
use image::ImageFormat;
use reqwest::multipart::{Form, Part};

use crate::errors::AppError;
use crate::image_processor::ProcessedImage;
use crate::models::imgur::ImgurResponse;
use crate::uploaders::UploadService;

use super::UploadServiceIdentifier;

pub struct ImgurUploader;

#[async_trait]
impl UploadService for ImgurUploader {
    async fn upload(
        filename: String,
        image: ProcessedImage,
        client_id: String,
        user_agent: String,
    ) -> Result<String, AppError> {
        let client = reqwest::Client::builder().user_agent(user_agent).build()?;

        // Create the multipart form
        let part = Part::stream(image.data)
            .mime_str(image.format.to_mime_type())
            .map_err(|e| AppError::Upload(e.to_string()))?
            .file_name(filename);

        let form = Form::new().part("image", part);

        // Make the request to Imgur API
        let response = client
            .post("https://api.imgur.com/3/image")
            .header("Authorization", format!("Client-ID {}", client_id))
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

    fn identifier() -> UploadServiceIdentifier {
        UploadServiceIdentifier::Imgur
    }

    fn formats() -> Vec<ImageFormat> {
        vec![ImageFormat::Png]
    }
}
