use crate::{
    cli::{Cli, SupportedImageFormat},
    errors::AppError,
    uploaders::UploadServiceIdentifier,
};

static UASTRING: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

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
        let imgur_client_id = option_env!("IMGUR_CLIENT_ID");

        // use provided id if set, else use environment variable
        let client_id: Option<String> = options.uid.clone().or(imgur_client_id.map(str::to_string));

        let config = Config {
            service: options.service,
            client_id,
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
