use crate::{
    cli::{Cli, SupportedImageFormat},
    errors::AppError,
    uploaders::UploadServiceIdentifier,
};

static UASTRING: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug)]
pub struct Config {
    pub service: UploadServiceIdentifier,
    pub image_format: SupportedImageFormat,
    pub image_dimensions: (u32, u32),
    pub client_id: Option<String>,
    pub user_agent: &'static str,
}

impl Config {
    pub fn new(options: &Cli) -> Result<Self, AppError> {
        // Get credentials from environment variables at compile time
        let imgur_client_id_env = option_env!("IMGUR_CLIENT_ID");

        // use provided id if set, else use environment variable
        let user_id: Option<String> = options.uid.clone();

        let client_id = match options.service {
            UploadServiceIdentifier::Imgur => {
                user_id.clone().or(imgur_client_id_env.map(str::to_string))
            }
            UploadServiceIdentifier::Catbox => user_id,
        };

        let config = Config {
            service: options.service,
            client_id: client_id,
            user_agent: UASTRING,
            image_format: options.format,
            image_dimensions: (options.dims, options.dims),
        };

        Config::valid(config)
    }

    fn valid(config: Config) -> Result<Self, AppError> {
        let client_id_check = match config.service {
            UploadServiceIdentifier::Imgur => config.client_id.is_some(),
            UploadServiceIdentifier::Catbox => true,
        };

        if !client_id_check {
            return Err(AppError::Config(format!(
                "{} requires a client id",
                config.service
            )));
        }

        let valid_format = config
            .service
            .formats()
            .contains(&config.image_format.to_image_format());

        if !valid_format {
            return Err(AppError::Config(format!(
                "{} is not a valid format for {}",
                config.image_format.to_string(),
                config.service
            )));
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::SupportedImageFormat;

    #[test]
    fn test_config_new_with_imgur_and_client_id() {
        let cli = Cli {
            service: UploadServiceIdentifier::Imgur,
            uid: Some("test_client_id".to_string()),
            format: SupportedImageFormat::Png,
            dims: 128,
            output: crate::cli::OutputFormat::Url,
        };

        let config = Config::new(&cli).unwrap();
        assert_eq!(config.service, UploadServiceIdentifier::Imgur);
        assert_eq!(config.client_id, Some("test_client_id".to_string()));
        assert_eq!(config.image_format, SupportedImageFormat::Png);
        assert_eq!(config.image_dimensions, (128, 128));
    }

    #[test]
    fn test_config_new_with_imgur_no_client_id() {
        let cli = Cli {
            service: UploadServiceIdentifier::Imgur,
            uid: None,
            format: SupportedImageFormat::Png,
            dims: 128,
            output: crate::cli::OutputFormat::Url,
        };

        let result = Config::new(&cli);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Config(_)));
    }

    #[test]
    fn test_config_new_with_catbox() {
        let cli = Cli {
            service: UploadServiceIdentifier::Catbox,
            uid: None,
            format: SupportedImageFormat::Png,
            dims: 128,
            output: crate::cli::OutputFormat::Url,
        };

        let config = Config::new(&cli).unwrap();
        assert_eq!(config.service, UploadServiceIdentifier::Catbox);
        assert_eq!(config.client_id, None);
    }

    #[test]
    fn test_config_with_invalid_format() {
        let cli = Cli {
            service: UploadServiceIdentifier::Imgur,
            uid: Some("test_client_id".to_string()),
            format: SupportedImageFormat::Webp, // Assuming Webp is not supported by Imgur
            dims: 128,
            output: crate::cli::OutputFormat::Url,
        };

        let result = Config::new(&cli);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Config(_)));
    }
}
