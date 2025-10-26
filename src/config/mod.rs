/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

pub(crate) mod cli;
pub(crate) mod config;
mod help;

#[derive(Debug, PartialEq)]
pub(crate) enum SupportedOutputFormat {
	Png,
	WebP,
}

impl SupportedOutputFormat {
	pub(crate) fn to_image_format(&self) -> image::ImageFormat {
		match self {
			SupportedOutputFormat::Png => image::ImageFormat::Png,
			SupportedOutputFormat::WebP => image::ImageFormat::WebP,
		}
	}

	pub(crate) fn from_str(s: &str) -> Option<Self> {
		match s.to_lowercase().as_str() {
			"png" => Some(Self::Png),
			"webp" => Some(Self::WebP),
			_ => None,
		}
	}

	pub(crate) fn as_str(&self) -> &'static str {
		match self {
			SupportedOutputFormat::Png => "png",
			SupportedOutputFormat::WebP => "webp",
		}
	}
}
