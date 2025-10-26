/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use pico_args::Arguments;

use crate::config::SupportedOutputFormat;
use crate::errors::AppError;
use crate::uploaders::UploadServiceIdentifier;

pub(crate) const UASTRING: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub(crate) struct Config {
	pub service: UploadServiceIdentifier,
	pub client_id: Option<String>,
	pub image_format: SupportedOutputFormat,
	pub image_dimensions: u32,
	pub timeout_seconds: u8,
	pub user_agent: &'static str,
}

impl Config {
	pub(crate) fn parse(mut pargs: Arguments) -> Result<Self, AppError> {
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

		let timeout_seconds = pargs
			.opt_value_from_str::<&str, u8>("--timeout")
			.map_err(|_| AppError::Config("Invalid timeout".into()))?
			.unwrap_or(10);

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
			timeout_seconds,
			user_agent: UASTRING,
		})
	}
}
