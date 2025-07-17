/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

use miniserde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ImgurResponse {
    pub data: ImgurData,
    pub success: bool,
}

#[derive(Deserialize, Debug)]
pub struct ImgurData {
    pub link: String,
}
