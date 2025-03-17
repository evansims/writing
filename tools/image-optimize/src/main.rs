use anyhow::Result;
use clap::{Parser, ValueEnum};
use colored::*;
use image_optimize::{
    OptimizeOptions, OutputFormat, SizeVariant,
    optimize_image, default_formats, default_size_variants
};
use std::path::PathBuf;

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum FormatOption {
    Jpeg,
    WebP,
    // Avif, // Temporarily removed
    All,
}

impl FormatOption {
    fn to_output_formats(&self) -> Vec<OutputFormat> {
        match self {
            FormatOption::Jpeg => vec![OutputFormat::Jpeg],
            FormatOption::WebP => vec![OutputFormat::WebP],
            // FormatOption::Avif => vec![OutputFormat::Avif], // Temporarily removed
            FormatOption::All => default_formats(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum SizeOption {
    Original,
    Large,
    Medium,
    Small,
    Thumbnail,
    All,
}

impl SizeOption {
    fn to_size_variants(&self) -> Vec<SizeVariant> {
        match self {
            SizeOption::Original => vec![SizeVariant::Original],
            SizeOption::Large => vec![SizeVariant::Large(1200)],
            SizeOption::Medium => vec![SizeVariant::Medium(800)],
            SizeOption::Small => vec![SizeVariant::Small(400)],
            SizeOption::Thumbnail => vec![SizeVariant::Thumbnail(200)],
            SizeOption::All => default_size_variants(),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about = "Optimize source images for articles with multiple format support")]
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
    
    /// Output formats to generate
    #[arg(short, long, value_enum, default_value = "all")]
    formats: FormatOption,
    
    /// Size variants to generate
    #[arg(short, long, value_enum, default_value = "all")]
    sizes: SizeOption,
    
    /// Quality level (0-100)
    #[arg(short, long, default_value = "85")]
    quality: u8,
    
    /// Preserve original image metadata
    #[arg(long, default_value = "false")]
    preserve_metadata: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Convert args to library options
    let options = OptimizeOptions {
        source: PathBuf::from(&args.source),
        article: Some(args.article.clone()),
        topic: args.topic.clone(),
        formats: args.formats.to_output_formats(),
        sizes: args.sizes.to_size_variants(),
        quality: args.quality,
        preserve_metadata: args.preserve_metadata,
    };
    
    println!("{} {}", "Optimizing image for article:".yellow().bold(), options.article.as_ref().unwrap_or(&"all".to_string()));
    println!("  {} {}", "Source:".green().bold(), options.source.display());
    
    // Generate formats string for display
    let formats_str = options.formats.iter()
        .map(|f| format!("{:?}", f))
        .collect::<Vec<_>>()
        .join(", ");
    println!("  {} {}", "Formats:".cyan().bold(), formats_str);
    
    // Generate sizes string for display
    let sizes_str = options.sizes.iter()
        .map(|s| s.name())
        .collect::<Vec<_>>()
        .join(", ");
    println!("  {} {}", "Sizes:".cyan().bold(), sizes_str);
    
    println!("  {} {}%", "Quality:".cyan().bold(), options.quality);
    
    // Optimize the image using the library function
    let result = optimize_image(&options)?;
    
    // Print summary of results
    println!("\n{} Image optimized successfully", "Success:".green().bold());
    println!("  {} {:.2} MB", "Original size:".cyan().bold(), result.original_size as f64 / 1_048_576.0);
    
    // Display results for each format
    for format_result in &result.format_results {
        println!("\n  {} {:?}", "Format:".yellow().bold(), format_result.format);
        
        for size_result in &format_result.size_results {
            let ratio = if result.original_size > 0 {
                (size_result.file_size as f64 / result.original_size as f64) * 100.0
            } else {
                0.0
            };
            
            println!("    {} {}: {}x{}, {:.2} MB ({:.1}%)",
                size_result.variant.name().cyan().bold(),
                size_result.path.display(),
                size_result.dimensions.0,
                size_result.dimensions.1,
                size_result.file_size as f64 / 1_048_576.0,
                ratio
            );
        }
    }
    
    println!("\nRun './writing images build --article={}' to generate HTML references for these images.", options.article.as_ref().unwrap_or(&"all".to_string()));
    
    Ok(())
} 