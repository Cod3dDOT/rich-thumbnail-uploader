/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use crate::{
	errors::AppError, image::thumbnail::Thumbnail, uploaders::UploadServiceImplementation,
};

pub(crate) struct ImgurUploader;

impl UploadServiceImplementation for ImgurUploader {
	fn upload(
		filename: &'static str,
		image: &Thumbnail,
		client_id: &str,
		user_agent: &'static str,
		timeout: u8,
	) -> Result<String, AppError> {
		let file = attohttpc::MultipartFile::new("image", &image.data)
			.with_filename(filename)
			.with_type(image.format.to_mime_type())
			.map_err(|e| AppError::Upload(e.to_string()))?;

		let part = attohttpc::MultipartBuilder::new()
			.with_file(file)
			.build()
			.map_err(|e| AppError::Upload(e.to_string()))?;

		let response = attohttpc::post("https://api.imgur.com/3/image")
			.connect_timeout(std::time::Duration::from_secs(timeout.into()))
			.timeout(std::time::Duration::from_secs(timeout.into()))
			.header("Authorization", format!("Client-ID {client_id}"))
			.header("User-Agent", user_agent)
			.body(part)
			.send()?;

		if !response.status().is_success() {
			let err = response
				.text()
				.unwrap_or_else(|_| "Unknown error".to_string());
			return Err(AppError::Upload(format!("Imgur API error: {err}")));
		}

		let body = response.text()?;
		if !body.contains(r#""success":true"#) {
			return Err(AppError::Upload(format!(
				"Imgur reported upload failure: {body}"
			)));
		}

		parse_link(&body)
			.ok_or_else(|| AppError::Upload(format!("Failed to parse Imgur response: {body}")))
	}
}

/// Extracts `data.link`. Imgur escapes `/` as `\/`; links contain no other
/// escapes.
fn parse_link(body: &str) -> Option<String> {
	let (_, rest) = body.split_once(r#""link":""#)?;
	let (link, _) = rest.split_once('"')?;
	Some(link.replace(r"\/", "/"))
}

#[cfg(test)]
mod tests {
	use super::parse_link;

	#[test]
	fn parse_link_unescapes_slashes() {
		let body = r#"{"data":{"id":"a1B2c3","title":null,"link":"https:\/\/i.imgur.com\/a1B2c3.png","tags":[]},"success":true,"status":200}"#;
		assert_eq!(
			parse_link(body).as_deref(),
			Some("https://i.imgur.com/a1B2c3.png")
		);
	}

	#[test]
	fn parse_link_missing_is_none() {
		assert_eq!(
			parse_link(r#"{"data":{"error":"x"},"success":false}"#),
			None
		);
	}
}
