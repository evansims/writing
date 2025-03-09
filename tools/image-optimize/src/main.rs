use anyhow::Result;
use clap::Parser;
use colored::*;
use image_optimize::{OptimizeOptions, optimize_image, optimize_source_image};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Optimize source images for articles")]
struct Args {
    /// Source image path
    #[arg(short, long)]
    source: PathBuf,

    /// Article slug
    #[arg(short, long)]
    article: String,

    /// Topic (optional, will search if not provided)
    #[arg(short, long)]
    topic: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Convert args to library options
    let options = OptimizeOptions {
        source: args.source,
        article: args.article,
        topic: args.topic,
    };
    
    println!("{} {}", "Optimizing source image for article:".yellow().bold(), options.article);
    
    // Optimize the image using the library function
    let target_path = optimize_image(&options)?;
    
    // Get image details for display
    let ((width, height), source_size, target_size) = 
        optimize_source_image(&options.source, &target_path)?;
    
    // Print details
    println!("  {} {}", "Source:".green().bold(), options.source.display());
    println!("  {} {}", "Target:".green().bold(), target_path.display());
    println!("  {} {}x{}", "Dimensions:".cyan().bold(), width, height);
    
    // Check if the image is large enough for high-quality source
    if width < 2400 || height < 1260 {
        println!("  {} The source image is smaller than the recommended size (2400x1260).", "Warning:".yellow().bold());
        println!("  This may result in lower quality images when scaled up.");
    }
    
    println!("  {} {:.2} MB → {:.2} MB", 
        "Size:".cyan().bold(), 
        source_size as f64 / 1_048_576.0,
        target_size as f64 / 1_048_576.0
    );
    
    println!("{} Source image optimized and saved as {}", 
        "Success:".green().bold(), 
        target_path.display()
    );
    println!("Run './writing images build --article={}' to generate all optimized versions.", options.article);
    
    Ok(())
} 