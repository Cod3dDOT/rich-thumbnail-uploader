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
mod files;
mod image_processor;
mod models;
mod uploaders;

use std::process::ExitCode;

use crate::config::cli::{CLI, CLIAction};
use crate::config::config::Config;
use crate::errors::AppError;

fn main() -> ExitCode {
	return match CLI::parse_args() {
		Ok(CLIAction::Run(config)) => match run_application(config) {
			Ok(_) => ExitCode::SUCCESS,
			Err(e) => {
				eprintln!("Error: {e}");
				ExitCode::FAILURE
			}
		},
		Ok(CLIAction::ShowHelp) => {
			CLI::print_help();
			ExitCode::SUCCESS
		}
		Ok(CLIAction::ShowVersion) => {
			CLI::print_version();
			ExitCode::SUCCESS
		}
		Err(e) => {
			eprintln!("Error: {e}");
			ExitCode::FAILURE
		}
	};
}

fn run_application(config: Config) -> Result<(), AppError> {
	let input_file = files::read_input_path()?;

	let thumbnail = image_processor::create_thumbnail(
		&input_file,
		image_processor::ImageProcessingOptions {
			size: config.image_dimensions,
			format: config.image_format,
		},
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
