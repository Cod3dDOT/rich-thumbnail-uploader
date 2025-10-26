/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum AppError {
	#[error("IO error: {0}")]
	IO(#[from] std::io::Error),

	#[error("Image processing error: {0}")]
	Image(#[from] image::ImageError),

	#[error("Upload error: {0}")]
	Upload(String),

	#[error("HTTP error: {0}")]
	Http(#[from] attohttpc::Error),

	#[error("Configuration error: {0}")]
	Config(&'static str),
}
