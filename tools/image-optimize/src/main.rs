use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use image::{GenericImageView, ImageFormat};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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

#[derive(Deserialize, Debug)]
struct TopicConfig {
    name: String,
    description: String,
    path: String,
}

#[derive(Deserialize, Debug)]
struct ContentConfig {
    base_dir: String,
    topics: HashMap<String, TopicConfig>,
}

#[derive(Deserialize, Debug)]
struct Config {
    content: ContentConfig,
}

fn read_config() -> Result<Config> {
    let config_content = fs::read_to_string("config.yaml")
        .context("Failed to read config.yaml")?;
    
    let config: Config = serde_yaml::from_str(&config_content)
        .context("Failed to parse config.yaml")?;
    
    Ok(config)
}

fn find_article_directory(config: &Config, article_slug: &str) -> Result<PathBuf> {
    // Search for the article directory in all topics
    for (_, topic_config) in &config.content.topics {
        let dir = PathBuf::from(format!("{}/{}/{}", 
            config.content.base_dir, 
            topic_config.path, 
            article_slug
        ));
        if dir.exists() {
            return Ok(dir);
        }
    }
    
    Err(anyhow::anyhow!("Article directory not found for slug: {}", article_slug))
}

fn optimize_source_image(source_path: &Path, target_path: &Path) -> Result<()> {
    println!("  {} {}", "Source:".green().bold(), source_path.display());
    println!("  {} {}", "Target:".green().bold(), target_path.display());
    
    // Open source image
    let img = image::open(source_path)
        .context(format!("Failed to open image: {:?}", source_path))?;
    
    // Get image dimensions
    let (width, height) = img.dimensions();
    println!("  {} {}x{}", "Dimensions:".cyan().bold(), width, height);
    
    // Check if the image is large enough for high-quality source
    if width < 2400 || height < 1260 {
        println!("  {} The source image is smaller than the recommended size (2400x1260).", "Warning:".yellow().bold());
        println!("  This may result in lower quality images when scaled up.");
    }
    
    // Save the image as a high-quality JPEG
    img.save_with_format(target_path, ImageFormat::Jpeg)
        .context(format!("Failed to save image: {:?}", target_path))?;
    
    // Get file sizes for comparison
    let source_size = fs::metadata(source_path)
        .context(format!("Failed to get metadata for source file: {:?}", source_path))?
        .len();
    
    let target_size = fs::metadata(target_path)
        .context(format!("Failed to get metadata for target file: {:?}", target_path))?
        .len();
    
    println!("  {} {:.2} MB → {:.2} MB", 
        "Size:".cyan().bold(), 
        source_size as f64 / 1_048_576.0,
        target_size as f64 / 1_048_576.0
    );
    
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Read configuration
    let config = read_config()?;
    
    // Check if source image exists
    if !args.source.exists() {
        return Err(anyhow::anyhow!("Source image not found: {:?}", args.source));
    }
    
    // Find article directory
    let article_dir = match &args.topic {
        Some(topic) => {
            // Validate topic
            if !config.content.topics.contains_key(topic) {
                let valid_topics: Vec<String> = config.content.topics.keys()
                    .map(|k| k.to_string())
                    .collect();
                return Err(anyhow::anyhow!(
                    "Invalid topic: {}. Valid topics are: {}", 
                    topic, 
                    valid_topics.join(", ")
                ));
            }
            
            let topic_path = &config.content.topics[topic].path;
            PathBuf::from(format!("{}/{}/{}", 
                config.content.base_dir, 
                topic_path, 
                args.article
            ))
        },
        None => find_article_directory(&config, &args.article)?,
    };
    
    // Create directory if it doesn't exist
    if !article_dir.exists() {
        fs::create_dir_all(&article_dir)
            .context(format!("Failed to create directory: {:?}", article_dir))?;
    }
    
    // Define target path for index.jpg
    let target_path = article_dir.join("index.jpg");
    
    println!("{} {}", "Optimizing source image for article:".yellow().bold(), args.article);
    
    // Optimize the source image
    optimize_source_image(&args.source, &target_path)?;
    
    println!("{} Source image optimized and saved as {}", 
        "Success:".green().bold(), 
        target_path.display()
    );
    println!("Run 'make images article={}' to generate all optimized versions.", args.article);
    
    Ok(())
} 