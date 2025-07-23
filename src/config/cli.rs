/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use pico_args::Arguments;
use std::path::PathBuf;
use std::process::exit;

use crate::config::help::HELP;
use crate::config::Config;
use crate::config::SupportedOutputFormat;
use crate::config::UASTRING;
use crate::errors::AppError;
use crate::uploaders::UploadServiceIdentifier;

impl Config {
    pub fn parse(mut pargs: Arguments) -> Result<Self, AppError> {
        if pargs.contains(["-v", "--version"]) {
            println!("{UASTRING}");
            exit(0);
        }

        if pargs.contains(["-h", "--help"]) {
            println!("{HELP}");
            exit(0);
        }

        let dims = pargs
            .opt_value_from_str::<[&str; 2], u32>(["-d", "--dimensions"])
            .map_err(|_| AppError::Config("Invalid dimensions".into()))?
            .unwrap_or(256);

        if !(128..=512).contains(&dims) {
            return Err(AppError::Config("Invalid dimensions".into()));
        }

        let service = pargs
            .opt_value_from_str::<[&str; 2], String>(["-s", "--service"])
            .map_err(|_| AppError::Config("Invalid service".into()))?
            .as_deref()
            .and_then(UploadServiceIdentifier::from_str)
            .unwrap_or(UploadServiceIdentifier::Catbox);

        let format = pargs
            .opt_value_from_str::<[&str; 2], String>(["-f", "--format"])
            .or_else(|_| pargs.opt_value_from_str::<&str, String>("--format"))
            .map_err(|_| AppError::Config("Invalid format".into()))?
            .as_deref()
            .and_then(SupportedOutputFormat::from_str)
            .unwrap_or(SupportedOutputFormat::Png);

        let uid = pargs
            .opt_value_from_str::<&str, String>("--uid")
            .map_err(|_| AppError::Config("Invalid uid".into()))?;

        let client_id = match service {
            UploadServiceIdentifier::Imgur => {
                uid.or_else(|| option_env!("IMGUR_CLIENT_ID").map(str::to_string))
            }
            UploadServiceIdentifier::Catbox => uid,
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
