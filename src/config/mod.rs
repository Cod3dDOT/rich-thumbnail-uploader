/*
 * SPDX-FileCopyrightText: 2025 cod3ddot@proton.me
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */

mod cli;
mod help;

use crate::uploaders::UploadServiceIdentifier;

pub const UASTRING: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, PartialEq)]
pub enum SupportedOutputFormat {
    Png,
    WebP,
}

impl SupportedOutputFormat {
    pub fn to_image_format(&self) -> image::ImageFormat {
        match self {
            SupportedOutputFormat::Png => image::ImageFormat::Png,
            SupportedOutputFormat::WebP => image::ImageFormat::WebP,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "png" => Some(Self::Png),
            "webp" => Some(Self::WebP),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            SupportedOutputFormat::Png => "png",
            SupportedOutputFormat::WebP => "webp",
        }
    }
}

pub struct Config {
    pub service: UploadServiceIdentifier,
    pub client_id: Option<String>,
    pub image_format: SupportedOutputFormat,
    pub image_dimensions: u32,
    pub user_agent: &'static str,
}
