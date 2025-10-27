/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

fn main() {
	println!("cargo:rerun-if-changed=resources/app.ico");
	println!("cargo:rerun-if-changed=resources/manifest.rc");
	println!("cargo:rerun-if-changed=resources/manifest.manifest");

	println!("cargo:rerun-if-changed=Cargo.toml");
	println!("cargo:rerun-if-changed=build.rs");

	let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

	if target_os != "windows" {
		return;
	}

	embed_resource::compile("resources/app.rc", embed_resource::NONE)
		.manifest_optional()
		.unwrap();

	let res = winresource::WindowsResource::new();
	res.compile().unwrap();
}
