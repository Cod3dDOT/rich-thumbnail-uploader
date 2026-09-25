/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

fn main() {
	println!("cargo:rerun-if-changed=resources");
	// winresource reads `[package.metadata.winresource]` from here
	println!("cargo:rerun-if-changed=Cargo.toml");

	if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
		return;
	}

	// Version info comes from Cargo.toml
	winresource::WindowsResource::new()
		.set_icon("resources/app.ico")
		.set_manifest_file("resources/app.manifest")
		.compile()
		.unwrap();
}
