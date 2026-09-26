/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use image::{
	ExtendedColorType, ImageEncoder, ImageFormat,
	codecs::{jpeg::JpegEncoder, png::PngEncoder, webp::WebPEncoder},
};

use crate::errors::AppError;

pub(crate) struct ThumbnailOptions {
	pub size: u32,
	pub format: ImageFormat,
	pub quality: u8,
}

impl ThumbnailOptions {
	pub(crate) fn new(size: u32, format: ImageFormat) -> Self {
		Self {
			size,
			format,
			quality: 100,
		}
	}

	pub(crate) fn with_quality(mut self, quality: u8) -> Self {
		self.quality = quality;
		self
	}
}

pub(crate) struct Thumbnail {
	pub data: Vec<u8>,
	pub format: ImageFormat,
}

pub(crate) fn create_thumbnail(
	filepath: &std::path::Path,
	options: ThumbnailOptions,
) -> Result<Thumbnail, AppError> {
	let img = image::ImageReader::open(filepath)?
		.with_guessed_format()?
		.decode()?;

	// Preserves aspect ratio
	let thumbnail = img.thumbnail(options.size, options.size);

	// Encoders only accept some colour types; normalise to 8-bit RGB(A).
	let (w, h) = (thumbnail.width(), thumbnail.height());
	let (pixels, color) = if options.format != ImageFormat::Jpeg && thumbnail.color().has_alpha() {
		(thumbnail.into_rgba8().into_raw(), ExtendedColorType::Rgba8)
	} else {
		(thumbnail.into_rgb8().into_raw(), ExtendedColorType::Rgb8)
	};

	let mut data = Vec::new();
	match options.format {
		ImageFormat::Jpeg => JpegEncoder::new_with_quality(&mut data, options.quality)
			.write_image(&pixels, w, h, color),
		ImageFormat::WebP => WebPEncoder::new_lossless(&mut data).write_image(&pixels, w, h, color),
		ImageFormat::Png => PngEncoder::new(&mut data).write_image(&pixels, w, h, color),
		_ => return Err(AppError::Config("Unsupported image format for thumbnail")),
	}?;

	Ok(Thumbnail {
		data,
		format: options.format,
	})
}

#[cfg(test)]
mod tests {
	use image::{ColorType, DynamicImage, GenericImageView, Rgb, RgbImage};
	use tempfile::TempDir;

	use super::*;

	/// Every input, including awkward colour types and misnamed files, becomes
	/// a correctly sized thumbnail in every output format, keeping alpha where
	/// possible.
	#[test]
	fn converts_any_input_to_any_output() {
		let inputs = [
			("rgb8.jpg", ImageFormat::Jpeg, ColorType::Rgb8),
			("rgba8.png", ImageFormat::Png, ColorType::Rgba8),
			("rgb16.png", ImageFormat::Png, ColorType::Rgb16),
			("la8.png", ImageFormat::Png, ColorType::La8),
			("rgba8.webp", ImageFormat::WebP, ColorType::Rgba8),
			("rgb8.gif", ImageFormat::Gif, ColorType::Rgb8),
			("rgb8.bmp", ImageFormat::Bmp, ColorType::Rgb8),
			// The format comes from the content, not the extension.
			("png-named.jpg", ImageFormat::Png, ColorType::Rgba8),
		];
		let dir = TempDir::new().unwrap();

		for (name, input_format, color) in inputs {
			let path = dir.path().join(name);
			// Zero-filled: any alpha channel is fully transparent.
			DynamicImage::new(64, 32, color)
				.save_with_format(&path, input_format)
				.unwrap();

			for format in [ImageFormat::Jpeg, ImageFormat::Png, ImageFormat::WebP] {
				let case = format!("{name} -> {format:?}");
				let thumb = create_thumbnail(&path, ThumbnailOptions::new(32, format))
					.unwrap_or_else(|e| panic!("{case}: {e}"));
				assert_eq!(image::guess_format(&thumb.data).unwrap(), format, "{case}");

				let out = image::load_from_memory(&thumb.data).unwrap();
				assert_eq!(out.dimensions(), (32, 16), "{case}");
				let transparent = color.has_alpha() && format != ImageFormat::Jpeg;
				let alpha = if transparent { 0 } else { 255 };
				assert_eq!(out.to_rgba8()[(0, 0)][3], alpha, "{case}");
			}
		}
	}

	#[test]
	fn jpeg_quality_is_applied() {
		let dir = TempDir::new().unwrap();
		let path = dir.path().join("gradient.png");
		RgbImage::from_fn(64, 64, |x, y| {
			Rgb([(x * 4) as u8, (y * 4) as u8, ((x ^ y) * 4) as u8])
		})
		.save(&path)
		.unwrap();

		let jpeg_len = |quality| {
			let options = ThumbnailOptions::new(64, ImageFormat::Jpeg).with_quality(quality);
			create_thumbnail(&path, options).unwrap().data.len()
		};
		assert!(jpeg_len(10) < jpeg_len(95));
	}

	#[test]
	fn rejects_non_image_input() {
		let dir = TempDir::new().unwrap();
		let path = dir.path().join("cover.jpg");
		std::fs::write(&path, "not an image").unwrap();

		let result = create_thumbnail(&path, ThumbnailOptions::new(32, ImageFormat::Png));
		assert!(matches!(result, Err(AppError::Image(_))));
	}
}
