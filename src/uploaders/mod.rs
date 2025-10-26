/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use image::ImageFormat;

use crate::{errors::AppError, image::thumbnail::Thumbnail};

pub(crate) mod catbox;
pub(crate) mod imgur;

#[derive(Debug, PartialEq)]
pub(crate) enum UploadService {
	Imgur,
	Catbox,
}

impl UploadService {
	pub(crate) fn from_str(s: &str) -> Result<Self, &'static str> {
		match s {
			"imgur" => Ok(Self::Imgur),
			"catbox" => Ok(Self::Catbox),
			_ => Err("Unknown upload service"),
		}
	}

	pub(crate) const fn formats(&self) -> &'static [ImageFormat] {
		match self {
			UploadService::Imgur => &[ImageFormat::Jpeg, ImageFormat::Png],
			UploadService::Catbox => &[ImageFormat::Jpeg, ImageFormat::Png, ImageFormat::WebP],
		}
	}
}

pub(crate) fn upload(
	service: UploadService,
	image: &Thumbnail,
	client_id: &str,
	user_agent: &'static str,
	timeout: u8,
) -> Result<String, AppError> {
	let filename = match image.format {
		ImageFormat::Png => "thumb.png",
		ImageFormat::WebP => "thumb.webp",
		ImageFormat::Jpeg => "thumb.jpg",
		_ => "thumb.dat",
	};

	match service {
		UploadService::Imgur => {
			imgur::ImgurUploader::upload(filename, image, client_id, user_agent, timeout)
		}
		UploadService::Catbox => {
			catbox::CatboxUploader::upload(filename, image, client_id, user_agent, timeout)
		}
	}
}

pub(crate) trait UploadServiceImplementation {
	fn upload(
		filename: &'static str,
		image: &Thumbnail,
		client_id: &str,
		user_agent: &'static str,
		timeout: u8,
	) -> Result<String, AppError>;
}
