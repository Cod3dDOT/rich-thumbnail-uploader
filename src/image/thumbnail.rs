/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use image::{ImageEncoder, ImageFormat};

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
	// Decode the source image
	let img = image::ImageReader::open(filepath)
		.map_err(AppError::from)?
		.with_guessed_format()
		.map_err(AppError::from)?
		.decode()
		.map_err(AppError::from)?;

	// Create thumbnail (preserves aspect ratio)
	let thumbnail = img.thumbnail(options.size, options.size);

	let bytes_per_pixel = match options.format {
		ImageFormat::Jpeg => 3,
		ImageFormat::Png => 4,
		ImageFormat::WebP => 4,
		_ => 4,
	};
	let estimated_capacity = (thumbnail.width() * thumbnail.height() * bytes_per_pixel) as usize;
	let mut buffer = Vec::with_capacity(estimated_capacity);

	match options.format {
		ImageFormat::Jpeg => {
			let mut encoder =
				image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, options.quality);

			encoder
				.encode(
					thumbnail.as_bytes(),
					thumbnail.width(),
					thumbnail.height(),
					thumbnail.color().into(),
				)
				.map_err(AppError::from)?;
		}
		ImageFormat::WebP => {
			let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut buffer);
			encoder
				.encode(
					thumbnail.as_bytes(),
					thumbnail.width(),
					thumbnail.height(),
					thumbnail.color().into(),
				)
				.map_err(AppError::from)?;
		}
		ImageFormat::Png => {
			let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
			encoder
				.write_image(
					thumbnail.as_bytes(),
					thumbnail.width(),
					thumbnail.height(),
					thumbnail.color().into(),
				)
				.map_err(AppError::from)?;
		}
		_ => return Err(AppError::Config("Unsupported image format for thumbnail")),
	}

	buffer.shrink_to_fit();

	Ok(Thumbnail {
		data: buffer,
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

	#[test]
	fn test_create_thumbnail_invalid_file() {
		let options = ThumbnailOptions::new(100, ImageFormat::Png);

		let result = create_thumbnail(std::path::Path::new("nonexistent_file.png"), options);
		assert!(result.is_err());
	}
}
