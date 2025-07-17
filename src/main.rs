/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

mod cli;
mod config;
mod errors;
mod image_processor;
mod models;
mod uploaders;

use clap::Parser;

use crate::cli::read_filepath;
use crate::cli::{Cli, OutputFormat};
use crate::config::Config;
use crate::errors::AppError;
use crate::image_processor::{create_thumbnail, ImageProcessingOptions};
use crate::uploaders::upload;

fn main() -> Result<(), AppError> {
    // Parse command line arguments
    let args = Cli::parse();

    // Initialize configuration
    let config = Config::new(&args)?;

    let input_file = read_filepath()?;

    // Validate file exists
    if !input_file.exists() {
        return Err(AppError::FileNotFound(input_file));
    }

    // Create image processing options
    let options = ImageProcessingOptions {
        size: config.image_dimensions.0,
        format: config.image_format.to_image_format(),
    };

    // Process the image
    let thumbnail = create_thumbnail(&input_file, &options)?;

    // Upload the image
    let upload_result = upload(
        config.service,
        &thumbnail,
        config.client_id.unwrap_or_default(),
        config.user_agent.to_string(),
    )?;

    // Output the result
    match args.output {
        OutputFormat::Url => println!("{}", upload_result),
    }

    Ok(())
}
