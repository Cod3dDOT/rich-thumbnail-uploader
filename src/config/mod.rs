/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

pub(crate) mod cli;
mod help;

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
			.opt_value_from_fn(["-d", "--dimensions"], |s| {
				s.parse()
					.ok()
					.filter(|d| (128..=512).contains(d))
					.ok_or("expected 128-512")
			})?
			.unwrap_or(256);

		let quality = pargs
			.opt_value_from_fn(["-q", "--quality"], |s| {
				s.parse()
					.ok()
					.filter(|q| (1..=100).contains(q))
					.ok_or("expected 1-100")
			})?
			.unwrap_or(80);

		let service = pargs
			.opt_value_from_fn(["-s", "--service"], UploadService::from_str)?
			.unwrap_or(UploadService::Catbox);

		let format = pargs
			.opt_value_from_fn(["-f", "--format"], |s| {
				ImageFormat::from_extension(s)
					.filter(|f| {
						matches!(f, ImageFormat::Png | ImageFormat::Jpeg | ImageFormat::WebP)
					})
					.ok_or("expected png, jpeg or webp")
			})?
			.unwrap_or(ImageFormat::Png);

		let uid = pargs.opt_value_from_str("--uid")?;

		let timeout_seconds = pargs
			.opt_value_from_fn("--timeout", |s| {
				s.parse()
					.ok()
					.filter(|&t| t > 0)
					.ok_or("expected 1-255 seconds")
			})?
			.unwrap_or(10);

		let unexpected = pargs.finish();
		if !unexpected.is_empty() {
			return Err(AppError::UnexpectedArgs(unexpected));
		}

		let client_id = match service {
			UploadService::Imgur => {
				uid.or_else(|| option_env!("IMGUR_CLIENT_ID").map(str::to_string))
			}
			UploadService::Catbox => uid,
		};

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

#[cfg(test)]
mod tests {
	use super::*;

	fn parse(args: &str) -> Result<Config, AppError> {
		Config::parse(Arguments::from_vec(
			args.split_whitespace().map(Into::into).collect(),
		))
	}

	#[test]
	fn rejects_invalid_arguments() {
		for args in [
			"-s imgurr",
			"-f gif",
			"-f bmp",
			"-d 127",
			"-d 513",
			"-q 0",
			"-q 101",
			"--timeout 0",
			"--servce catbox",
			"stray",
			"-s imgur --uid id -f webp",
		] {
			assert!(parse(args).is_err(), "`{args}` should be rejected");
		}
	}

	#[test]
	fn parses_valid_arguments() {
		let c = parse("-s imgur --uid id -f jpg -d 512 -q 90 --timeout 30").unwrap();
		assert_eq!(c.service, UploadService::Imgur);
		assert_eq!(c.client_id.as_deref(), Some("id"));
		assert_eq!(c.image_format, ImageFormat::Jpeg);
		assert_eq!(
			(c.image_dimensions, c.image_quality, c.timeout_seconds),
			(512, 90, 30)
		);
	}

	#[test]
	fn defaults_match_help() {
		let c = parse("").unwrap();
		assert_eq!(c.service, UploadService::Catbox);
		assert_eq!(c.client_id, None);
		assert_eq!(c.image_format, ImageFormat::Png);
		assert_eq!(
			(c.image_dimensions, c.image_quality, c.timeout_seconds),
			(256, 80, 10)
		);
	}
}
