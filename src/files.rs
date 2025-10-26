/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use crate::errors::AppError;

pub(crate) fn read_input_path() -> Result<std::path::PathBuf, AppError> {
	let mut input = String::new();
	std::io::stdin()
		.read_line(&mut input)
		.map_err(|e| AppError::IO(e))?;

	let trimmed = input.trim();
	if trimmed.is_empty() {
		return Err(AppError::Config("Expected input file path".into()));
	}

	return std::path::PathBuf::from(trimmed)
		.canonicalize()
		.map_err(AppError::from);
}
