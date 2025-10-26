/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use pico_args::Arguments;

use crate::{
	config::{
		config::{Config, UASTRING},
		help::HELP,
	},
	errors::AppError,
};

// CLI parsing result that indicates what action to take
pub(crate) enum CLIAction {
	ShowHelp,
	ShowVersion,
	Run(Config),
}

/// Handles CLI argument parsing and determines the appropriate action
pub(crate) struct CLI;

impl CLI {
	/// Parse CLI arguments and return the appropriate action
	pub(crate) fn parse_args() -> Result<CLIAction, AppError> {
		let mut pargs = Arguments::from_env();

		// Handle meta-actions first
		if pargs.contains(["-h", "--help"]) {
			return Ok(CLIAction::ShowHelp);
		}

		if pargs.contains(["-v", "--version"]) {
			return Ok(CLIAction::ShowVersion);
		}

		// Parse actual configuration
		let config = Config::parse(pargs)?;
		Ok(CLIAction::Run(config))
	}

	pub(crate) fn print_help() {
		println!("{HELP}");
	}

	pub(crate) fn print_version() {
		println!("{UASTRING}");
	}
}
