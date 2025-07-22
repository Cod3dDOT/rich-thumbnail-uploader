/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Upload error: {0}")]
    Upload(String),

    #[error("File not found: {0}")]
    FileNotFound(std::path::PathBuf),

    #[error("HTTP error: {0}")]
    Http(#[from] attohttpc::Error),

    #[error("Configuration error: {0}")]
    Config(String),
}
