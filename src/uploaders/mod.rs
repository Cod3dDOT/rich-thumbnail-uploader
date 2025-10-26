/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use crate::{config::SupportedOutputFormat, errors::AppError, image_processor::ProcessedImage};
use image::ImageFormat;

pub(crate) mod catbox;
pub(crate) mod imgur;

#[derive(Debug, PartialEq)]
pub(crate) enum UploadServiceIdentifier {
	Imgur,
	Catbox,
}

impl UploadServiceIdentifier {
	pub(crate) fn as_str(&self) -> &'static str {
		match self {
			UploadServiceIdentifier::Imgur => "imgur",
			UploadServiceIdentifier::Catbox => "catbox",
		}
	}

	pub(crate) fn formats(&self) -> Vec<ImageFormat> {
		match self {
			UploadServiceIdentifier::Imgur => imgur::ImgurUploader::formats(),
			UploadServiceIdentifier::Catbox => catbox::CatboxUploader::formats(),
		}
	}

	pub(crate) fn from_str(s: &str) -> Option<Self> {
		match s.to_lowercase().as_str() {
			"imgur" => Some(Self::Imgur),
			"catbox" => Some(Self::Catbox),
			_ => None,
		}
	}
}

pub(crate) fn upload(
	service: UploadServiceIdentifier,
	image: &ProcessedImage,
	client_id: &str,
	user_agent: &'static str,
	timeout: u8,
) -> Result<String, AppError> {
	let filename = match image.format {
		SupportedOutputFormat::Png => "thumb.png",
		SupportedOutputFormat::WebP => "thumb.webp",
	};
	match service {
		UploadServiceIdentifier::Imgur => {
			imgur::ImgurUploader::upload(filename, image, client_id, user_agent, timeout)
		}
		UploadServiceIdentifier::Catbox => {
			catbox::CatboxUploader::upload(filename, image, client_id, user_agent, timeout)
		}
	}
}

pub(crate) trait UploadService {
	fn upload(
		filename: &'static str,
		image: &ProcessedImage,
		client_id: &str,
		user_agent: &'static str,
		timeout: u8,
	) -> Result<String, AppError>;

	fn formats() -> Vec<ImageFormat>;
}
