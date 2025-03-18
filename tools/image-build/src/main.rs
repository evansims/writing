use anyhow::Result;
use clap::Parser;
use colored::*;
use image_build::{BuildImagesOptions, build_images};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Generate optimized images from source images for the build process")]
struct Args {
    /// Output directory for optimized images
    #[arg(short, long, default_value = "build/images")]
    output_dir: PathBuf,

    /// Source directory containing content
    #[arg(short, long, default_value = "content")]
    source_dir: PathBuf,

    /// Source image filename (default: index.jpg)
    #[arg(short, long, default_value = "index.jpg")]
    source_filename: String,

    /// Specific article to process (optional)
    #[arg(short, long)]
    article: Option<String>,

    /// Specific topic to process (optional)
    #[arg(short, long)]
    topic: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Convert args to options
    let options = BuildImagesOptions {
        output_dir: args.output_dir,
        source_dir: args.source_dir,
        article: args.article,
        topic: args.topic,
        force_rebuild: false,
    };

    println!("{}", "Scanning for source images...".yellow().bold());

    // Build images using the library function
    match build_images(&options) {
        Ok((total_articles, total_images, processed_images, skipped_articles)) => {
            // Print summary
            println!("\n{}", "Summary:".yellow().bold());
            println!("  Total articles scanned: {}", total_articles.to_string().cyan().bold());
            println!("  Total source images found: {}", total_images.to_string().cyan().bold());
            println!("  Images processed: {}", processed_images.to_string().green().bold());
            println!("  Articles skipped (no source image): {}", skipped_articles.to_string().red().bold());

            println!("\n{}", "Image build complete!".green().bold());
            Ok(())
        },
        Err(e) => {
            eprintln!("{} {}", "Error:".red().bold(), e);
            Err(e)
        }
    }
}