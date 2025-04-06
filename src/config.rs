use crate::errors::AppError;

pub struct Config {
    pub imgur_client_id: Option<&'static str>,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        // Get credentials from environment variables at compile time
        let imgur_client_id = option_env!("IMGUR_CLIENT_ID");

        Ok(Config { imgur_client_id })
    }

    pub fn has_imgur_config(&self) -> bool {
        self.imgur_client_id.is_some()
    }

    pub fn get_imgur_id(&self) -> Result<&'static str, AppError> {
        match self.imgur_client_id {
            Some(id) => Ok(id),
            None => Err(AppError::Config(
                "Imgur client ID not configured at compile time".to_string(),
            )),
        }
    }
}
