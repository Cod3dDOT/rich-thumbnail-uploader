/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

fn main() {
    println!("cargo:rerun-if-changed=resources/app.ico");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=build.rs");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os != "windows" {
        return;
    }

    let mut res = winresource::WindowsResource::new();
    res.set_icon("./resources/app.ico");

    let app_name = std::env::var("CARGO_PKG_NAME").unwrap();

    let manifest = embed_manifest::new_manifest(app_name.as_str())
        .requested_execution_level(embed_manifest::manifest::ExecutionLevel::AsInvoker)
        .max_version_tested(embed_manifest::manifest::MaxVersionTested::Windows11)
        .to_string();

    res.set_manifest(&manifest);
    res.compile().expect("Unable to compile resources");
}
