use clap::{Parser, ValueEnum};
use image::ImageFormat;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process;

use crate::errors::AppError;
use crate::uploaders::UploadServiceIdentifier;

#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    about,
    version,
    author,
    override_usage = concat!(env!("CARGO_PKG_NAME"), " [OPTIONS]")
)]
pub struct Cli {
    /// Dimensions to crop the image to
    #[arg(short, long, default_value = "256")]
    pub dims: u32,

    /// Upload service to use
    #[arg(short, long, value_enum, default_value_t = UploadServiceIdentifier::Imgur)]
    pub service: UploadServiceIdentifier,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Url)]
    pub output: OutputFormat,

    /// Optional uid (overrides provided client id for imgur / sets user hash for catbox)
    #[arg(short, long)]
    pub uid: Option<String>,

    /// Preffered image format, only affects catbox
    #[arg(short, long, value_enum, default_value_t = SupportedImageFormat::Png)]
    pub format: SupportedImageFormat,
}

impl std::fmt::Display for UploadServiceIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Url,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum SupportedImageFormat {
    Png,
    Webp,
}

impl SupportedImageFormat {
    pub fn to_image_format(&self) -> ImageFormat {
        match self {
            SupportedImageFormat::Png => ImageFormat::Png,
            SupportedImageFormat::Webp => ImageFormat::WebP,
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            SupportedImageFormat::Png => "png",
            SupportedImageFormat::Webp => "webp",
        }
    }
}

pub fn read_filepath() -> Result<PathBuf, AppError> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let filepath = match lines.next() {
        Some(line) => line?,
        None => {
            eprintln!("Error: Expected file path from stdin");
            process::exit(1);
        }
    };

    Ok(PathBuf::new().join(filepath.trim().to_string()))
}
