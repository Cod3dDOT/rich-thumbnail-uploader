mod cli;
mod config;
mod errors;
mod image_processor;
mod models;
mod uploaders;

use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process;

use clap::Parser;

use crate::cli::{Cli, OutputFormat};
use crate::config::Config;
use crate::errors::AppError;
use crate::image_processor::{create_thumbnail, ImageOptions};
use crate::uploaders::get_uploader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let args = Cli::parse();

    // Initialize configuration
    let config = Config::from_env()?;

    let input_file = read_filepath()?;

    // Validate file exists
    if !input_file.exists() {
        return Err(AppError::FileNotFound(input_file.to_string_lossy().to_string()).into());
    }

    // Create image processing options
    let options = ImageOptions {
        size: args.size,
        format: cli::get_image_format(&args.service),
    };

    // Process the image
    let thumbnail = create_thumbnail(&input_file.to_string_lossy(), &options)?;

    // Get the appropriate uploader
    let uploader = get_uploader(args.service.to_string().as_str(), &config)?;

    // Upload the image
    let upload_result = uploader.upload(thumbnail, &options).await?;

    // Output the result based on requested format
    match args.output {
        OutputFormat::Url => println!("{}", upload_result),
    }

    Ok(())
}

fn read_filepath() -> Result<PathBuf, AppError> {
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
