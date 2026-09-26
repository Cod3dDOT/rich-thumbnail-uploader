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
			_ => Err("expected imgur or catbox"),
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

	let start = std::time::Instant::now();
	let result = match service {
		UploadService::Imgur => {
			imgur::ImgurUploader::upload(filename, image, client_id, user_agent, timeout)
		}
		UploadService::Catbox => {
			catbox::CatboxUploader::upload(filename, image, client_id, user_agent, timeout)
		}
	};

	// On deadline attohttpc closes the socket, surfacing as an arbitrary socket
	// error
	match result {
		Err(AppError::Http(_)) if start.elapsed().as_secs() >= timeout.into() => {
			Err(AppError::Timeout(timeout))
		}
		result => check_link(result?),
	}
}

/// The link is passed to foo_discord_rich as-is: allow one https URL only.
fn check_link(link: String) -> Result<String, AppError> {
	let link = link.trim();
	let valid = link.len() > "https://".len()
		&& link.starts_with("https://")
		&& !link.chars().any(|c| c.is_whitespace() || c.is_control());
	if !valid {
		return Err(AppError::Upload(format!(
			"Service returned an invalid link: {link:?}"
		)));
	}
	Ok(link.to_owned())
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

#[cfg(test)]
mod tests {
	use super::check_link;

	#[test]
	fn check_link_accepts_only_one_https_url() {
		for (body, expected) in [
			(
				"https://files.catbox.moe/a1b2c3.png",
				Some("https://files.catbox.moe/a1b2c3.png"),
			),
			(
				"https://files.catbox.moe/a1b2c3.png\n",
				Some("https://files.catbox.moe/a1b2c3.png"),
			),
			("http://files.catbox.moe/a1b2c3.png", None),
			("https://", None),
			("https://a.png\nhttps://b.png", None),
			("https://a.png \x1b[31m", None),
			("", None),
		] {
			assert_eq!(
				check_link(body.to_owned()).ok().as_deref(),
				expected,
				"{body:?}"
			);
		}
	}
}
