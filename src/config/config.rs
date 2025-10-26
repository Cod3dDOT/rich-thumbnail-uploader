/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use image::ImageFormat;
use pico_args::Arguments;

use crate::{errors::AppError, uploaders::UploadService};

pub(crate) const UASTRING: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub(crate) struct Config {
	pub service: UploadService,
	pub client_id: Option<String>,
	pub image_format: ImageFormat,
	pub image_dimensions: u32,
	pub image_quality: u8,
	pub timeout_seconds: u8,
	pub user_agent: &'static str,
}

impl Config {
	pub(crate) fn parse(mut pargs: Arguments) -> Result<Self, AppError> {
		let dims = pargs
			.opt_value_from_str::<[&str; 2], u32>(["-d", "--dimensions"])
			.map_err(|_| AppError::Config("Invalid dimensions"))?
			.unwrap_or(256);

		if !(128..=512).contains(&dims) {
			return Err(AppError::Config("Invalid dimensions"));
		}

		let quality = pargs
			.opt_value_from_str::<[&str; 2], u8>(["-q", "--quality"])
			.or_else(|_| pargs.opt_value_from_str::<&str, u8>("--quality"))
			.map_err(|_| AppError::Config("Invalid quality"))?
			.unwrap_or(80);

		let service = pargs
			.opt_value_from_str::<[&str; 2], String>(["-s", "--service"])
			.map_err(|_| AppError::Config("Invalid service"))?
			.as_deref()
			.and_then(|s| UploadService::from_str(s).ok())
			.unwrap_or(UploadService::Catbox);

		let format = pargs
			.opt_value_from_str::<[&str; 2], String>(["-f", "--format"])
			.or_else(|_| pargs.opt_value_from_str::<&str, String>("--format"))
			.map_err(|_| AppError::Config("Invalid format"))?
			.as_deref()
			.and_then(ImageFormat::from_extension)
			.unwrap_or(ImageFormat::Png);

		let uid = pargs
			.opt_value_from_str::<&str, String>("--uid")
			.map_err(|_| AppError::Config("Invalid uid"))?;

		let client_id = match service {
			UploadService::Imgur => {
				uid.or_else(|| option_env!("IMGUR_CLIENT_ID").map(str::to_string))
			}
			UploadService::Catbox => uid,
		};

		let timeout_seconds = pargs
			.opt_value_from_str::<&str, u8>("--timeout")
			.map_err(|_| AppError::Config("Invalid timeout"))?
			.unwrap_or(10);

		if matches!(service, UploadService::Imgur) && client_id.is_none() {
			return Err(AppError::Config("Imgur requires a client id"));
		}

		if !service.formats().contains(&format) {
			return Err(AppError::Config(
				"Format is not a valid format for this service",
			));
		}

		Ok(Self {
			service,
			client_id,
			image_format: format,
			image_dimensions: dims,
			image_quality: quality,
			timeout_seconds,
			user_agent: UASTRING,
		})
	}
}
