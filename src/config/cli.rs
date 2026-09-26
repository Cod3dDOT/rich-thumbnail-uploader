/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use pico_args::Arguments;

use crate::{
	config::{Config, UASTRING, help::HELP},
	errors::AppError,
};

pub(crate) enum CLIAction {
	ShowHelp,
	ShowVersion,
	Run(Config),
}

pub(crate) struct Cli;

impl Cli {
	pub(crate) fn parse_args() -> Result<CLIAction, AppError> {
		let mut pargs = Arguments::from_env();

		if pargs.contains(["-h", "--help"]) {
			return Ok(CLIAction::ShowHelp);
		}

		if pargs.contains(["-v", "--version"]) {
			return Ok(CLIAction::ShowVersion);
		}

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
