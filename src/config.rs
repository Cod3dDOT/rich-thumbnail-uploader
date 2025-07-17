/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use image::ImageFormat;
use pico_args::Arguments;
use std::path::PathBuf;

use crate::errors::AppError;
use crate::uploaders::UploadServiceIdentifier;

static UASTRING: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedImageFormat {
    Png,
    Webp,
}

impl SupportedImageFormat {
    pub fn to_image_format(self) -> ImageFormat {
        match self {
            SupportedImageFormat::Png => ImageFormat::Png,
            SupportedImageFormat::Webp => ImageFormat::WebP,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "png" => Some(Self::Png),
            "webp" => Some(Self::Webp),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SupportedImageFormat::Png => "png",
            SupportedImageFormat::Webp => "webp",
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub service: UploadServiceIdentifier,
    pub client_id: Option<String>,
    pub image_format: SupportedImageFormat,
    pub image_dimensions: u32,
    pub user_agent: &'static str,
}

impl Config {
    pub fn parse(mut pargs: Arguments) -> Result<Self, AppError> {
        let dims = pargs
            .opt_value_from_str::<&str, u32>("--dims")
            .map_err(|_| AppError::Config("Invalid dimensions".into()))?
            .unwrap_or(256);

        let service = pargs
            .opt_value_from_str::<&str, String>("--service")
            .map_err(|_| AppError::Config("Invalid service".into()))?
            .as_deref()
            .and_then(UploadServiceIdentifier::from_str)
            .ok_or(AppError::Config("Invalid service".into()))?;

        let format = pargs
            .opt_value_from_str::<&str, String>("--format")
            .map_err(|_| AppError::Config("Invalid format".into()))?
            .as_deref()
            .and_then(SupportedImageFormat::from_str)
            .unwrap_or(SupportedImageFormat::Png);

        let uid = pargs
            .opt_value_from_str::<&str, String>("--uid")
            .map_err(|_| AppError::Config("Invalid uid".into()))?;

        // Validate
        let client_id = match service {
            UploadServiceIdentifier::Imgur => uid
                .clone()
                .or(option_env!("IMGUR_CLIENT_ID").map(str::to_string)),
            UploadServiceIdentifier::Catbox => uid.clone(),
        };

        if matches!(service, UploadServiceIdentifier::Imgur) && client_id.is_none() {
            return Err(AppError::Config("Imgur requires a client id".into()));
        }

        if !service.formats().contains(&format.to_image_format()) {
            return Err(AppError::Config(format!(
                "{} is not a valid format for {}",
                format.as_str(),
                service.as_str()
            )));
        }

        Ok(Self {
            service,
            client_id,
            image_format: format,
            image_dimensions: dims,
            user_agent: UASTRING,
        })
    }

    pub fn from_env() -> Result<Self, AppError> {
        Config::parse(Arguments::from_env())
    }

    pub fn read_input_path(&self) -> Result<PathBuf, AppError> {
        use std::io::BufRead;
        let stdin = std::io::stdin();
        let mut lines = stdin.lock().lines();
        let filepath = lines
            .next()
            .transpose()?
            .ok_or_else(|| AppError::Config("Expected input file path".into()))?;

        Ok(PathBuf::from(filepath.trim()))
    }
}
