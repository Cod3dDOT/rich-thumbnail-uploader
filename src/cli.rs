use clap::{Parser, ValueEnum};
use image::ImageFormat;

#[derive(Parser)]
#[command(
    name = "rich-thumbnail-uploader",
    about = "Thumbnail uploader for the foo_discord_rich plugin",
    version,
    author
)]
pub struct Cli {
    /// Size to resize the image to (maintains aspect ratio)
    #[arg(short, long, default_value = "256")]
    pub size: u32,

    /// Image hosting service to use
    #[arg(short, long, value_enum, default_value_t = ImageService::Imgur)]
    pub service: ImageService,

    /// Output format for the response (URL only or JSON)
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Url)]
    pub output: OutputFormat,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum ImageService {
    Imgur,
    Catbox,
}

impl std::fmt::Display for ImageService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageService::Imgur => write!(f, "imgur"),
            ImageService::Catbox => write!(f, "catbox"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// Only output the URL
    Url,
}

pub fn get_image_format(&service: &ImageService) -> ImageFormat {
    match service {
        ImageService::Imgur => ImageFormat::Png,
        ImageService::Catbox => ImageFormat::Png,
    }
}
