/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

mod common;

// this is so boring
#[test]
fn test_config_validation() {
    // Valid config for catbox
    let args = ["-s", "catbox", "-f", "png"];
    assert!(common::parse_args(&args).is_ok());

    // Invalid format for imgur
    let args = ["-s", "imgur", "-f", "webp", "--uid", "test_id"];
    assert!(common::parse_args(&args).is_err());
}
