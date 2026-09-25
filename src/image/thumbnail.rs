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
	use image::{GenericImageView, ImageBuffer, ImageFormat, Rgb};
	use tempfile::TempDir;

	use super::*;

	fn create_test_image(width: u32, height: u32, format: ImageFormat) -> (TempDir, String) {
		let temp_dir = TempDir::new().unwrap();
		let filename = match format {
			ImageFormat::Jpeg => "test_image.jpeg",
			ImageFormat::Png => "test_image.png",
			ImageFormat::WebP => "test_image.webp",
			_ => "test_image.dat",
		};
		let file_path = temp_dir.path().join(filename);

		let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);

		for pixel in img.pixels_mut() {
			let r = (pixel.0[0] + 1) % 255;
			let g = (pixel.0[1] + 2) % 255;
			let b = (pixel.0[2] + 3) % 255;
			*pixel = Rgb([r, g, b]);
		}

		img.save(&file_path).unwrap();
		(temp_dir, file_path.to_string_lossy().to_string())
	}

	#[test]
	fn test_create_thumbnail_resizes_correctly() {
		let (temp_dir, file_path) = create_test_image(200, 200, ImageFormat::Png);

		let options = ThumbnailOptions::new(100, ImageFormat::Png);

		let result = create_thumbnail(std::path::Path::new(&file_path), options).unwrap();

		// Load the resulting image to verify dimensions
		let img = image::load_from_memory(&result.data).unwrap();
		assert_eq!(img.dimensions(), (100, 100));

		drop(temp_dir); // Cleanup
	}

	#[test]
	fn test_create_thumbnail_converts_format() {
		let (temp_dir, file_path) = create_test_image(200, 200, ImageFormat::Png);

		let options = ThumbnailOptions::new(100, ImageFormat::WebP);

		let result = create_thumbnail(std::path::Path::new(&file_path), options).unwrap();
		assert_eq!(result.format, ImageFormat::WebP);

		// Verify the data is actually WebP
		assert!(image::guess_format(&result.data).unwrap() == ImageFormat::WebP);

		drop(temp_dir); // Cleanup
	}

	/// Cover art is often RGBA or 16-bit; every output format must still encode
	/// it.
	#[test]
	fn test_create_thumbnail_encodes_any_source_color_type() {
		let sources = [
			(
				"rgba8.png",
				image::DynamicImage::ImageRgba8(image::RgbaImage::new(64, 64)),
			),
			(
				"rgb16.png",
				image::DynamicImage::ImageRgb16(ImageBuffer::new(64, 64)),
			),
			(
				"la8.png",
				image::DynamicImage::ImageLumaA8(ImageBuffer::new(64, 64)),
			),
		];
		let temp_dir = TempDir::new().unwrap();

		for (name, source) in sources {
			let path = temp_dir.path().join(name);
			source.save(&path).unwrap();

			for format in [ImageFormat::Jpeg, ImageFormat::Png, ImageFormat::WebP] {
				let result = create_thumbnail(&path, ThumbnailOptions::new(32, format))
					.unwrap_or_else(|e| panic!("{name} -> {format:?}: {e}"));
				assert_eq!(image::guess_format(&result.data).unwrap(), format, "{name}");
				assert_eq!(
					image::load_from_memory(&result.data).unwrap().dimensions(),
					(32, 32),
					"{name} -> {format:?}"
				);
			}
		}
	}

	#[test]
	fn test_create_thumbnail_invalid_file() {
		let options = ThumbnailOptions::new(100, ImageFormat::Png);

		let result = create_thumbnail(std::path::Path::new("nonexistent_file.png"), options);
		assert!(result.is_err());
	}
}
