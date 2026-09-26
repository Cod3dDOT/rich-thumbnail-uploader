/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */
#![windows_subsystem = "console"]

// Only built and tested for Windows/MSVC.
#[cfg(not(all(target_env = "msvc", target_os = "windows")))]
compile_error!("Platform not supported!");

mod config;
mod errors;
mod files;
mod image;
mod uploaders;

use std::process::ExitCode;

use crate::{
	config::cli::{CLIAction, Cli},
	errors::AppError,
};

fn main() -> ExitCode {
	match Cli::parse_args() {
		Ok(CLIAction::Run(config)) => match run_application(config) {
			Ok(_) => ExitCode::SUCCESS,
			Err(e) => {
				eprintln!("Error: {e}");
				ExitCode::FAILURE
			}
		},
		Ok(CLIAction::ShowHelp) => {
			Cli::print_help();
			ExitCode::SUCCESS
		}
		Ok(CLIAction::ShowVersion) => {
			Cli::print_version();
			ExitCode::SUCCESS
		}
		Err(e) => {
			eprintln!("Error: {e}");
			ExitCode::FAILURE
		}
	}
}

fn run_application(config: config::Config) -> Result<(), AppError> {
	let input_file = files::read_input_path()?;

	let thumbnail = image::thumbnail::create_thumbnail(
		&input_file,
		image::thumbnail::ThumbnailOptions::new(config.image_dimensions, config.image_format)
			.with_quality(config.image_quality),
	)?;

	let upload_result = uploaders::upload(
		config.service,
		&thumbnail,
		config.client_id.as_deref().unwrap_or(""),
		config.user_agent,
		config.timeout_seconds,
	)?;

	println!("{upload_result}");
	Ok(())
}
