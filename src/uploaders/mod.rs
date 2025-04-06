use crate::config::Config;
use crate::errors::AppError;
use crate::image_processor::ImageOptions;
use async_trait::async_trait;

pub mod catbox;
pub mod imgur;

#[async_trait]
pub trait UploadService {
    async fn upload(&self, image_data: Vec<u8>, options: &ImageOptions)
        -> Result<String, AppError>;
}

pub fn get_uploader(service: &str, config: &Config) -> Result<Box<dyn UploadService>, AppError> {
    match service.to_lowercase().as_str() {
        "imgur" => {
            if !config.has_imgur_config() {
                return Err(AppError::Config(
                    "Imgur client ID not configured".to_string(),
                ));
            }
            Ok(Box::new(imgur::ImgurUploader::new(&config.get_imgur_id()?)))
        }
        "catbox" => Ok(Box::new(catbox::CatboxUploader::new())),
        _ => Err(AppError::UnsupportedService(service.to_string())),
    }
}
