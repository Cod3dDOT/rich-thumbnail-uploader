/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */
#![windows_subsystem = "console"]

// Blow up if we try to compile without msvc, x64 arch, or windows.
// The code should work, but is untested on all other platforms.
#[cfg(not(all(target_env = "msvc", target_arch = "x86_64", target_os = "windows")))]
compile_error!("Platform not supported!");

mod config;
mod errors;
mod image_processor;
mod models;
mod uploaders;

use crate::config::Config;
use crate::errors::AppError;
use crate::image_processor::{ImageProcessingOptions, create_thumbnail};
use crate::uploaders::upload;

fn main() -> Result<(), AppError> {
    let config = Config::from_env()?;
    let input_file = config.read_input_path()?;

    // Validate file exists
    if !input_file.exists() {
        return Err(AppError::FileNotFound(input_file));
    }

    let thumbnail = create_thumbnail(
        &input_file,
        ImageProcessingOptions {
            size: config.image_dimensions,
            format: config.image_format,
        },
    )?;

    let upload_result = upload(
        config.service,
        &thumbnail,
        config.client_id.unwrap_or_default(),
        config.user_agent.to_string(),
    )?;

    println!("{upload_result}");
    Ok(())
}
