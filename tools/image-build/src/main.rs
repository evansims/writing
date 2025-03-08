use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use image::{GenericImageView, ImageFormat};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
struct ImageSize {
    width: u32,
    height: u32,
    description: String,
}

#[derive(Deserialize, Debug)]
struct ImageQuality {
    standard: u8,
    thumbnail: u8,
}

#[derive(Deserialize, Debug)]
struct ImageNaming {
    pattern: String,
    examples: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct ImagesConfig {
    formats: Vec<HashMap<String, String>>,
    sizes: HashMap<String, ImageSize>,
    naming: ImageNaming,
    quality: HashMap<String, ImageQuality>,
}

#[derive(Deserialize, Debug)]
struct Config {
    content: ContentConfig,
    images: ImagesConfig,
}

fn read_config() -> Result<Config> {
    let config_content = fs::read_to_string("config.yaml")
        .context("Failed to read config.yaml")?;
    
    let config: Config = serde_yaml::from_str(&config_content)
        .context("Failed to parse config.yaml")?;
    
    Ok(config)
}

fn generate_image_filename(
    config: &Config,
    article_slug: &str,
    image_type: &str,
    width: u32,
    height: u32,
    format: &str,
) -> String {
    let mut pattern = config.images.naming.pattern.clone();
    
    // Replace placeholders in the pattern
    pattern = pattern.replace("{slug}", article_slug);
    pattern = pattern.replace("{type}", image_type);
    pattern = pattern.replace("{width}", &width.to_string());
    pattern = pattern.replace("{height}", &height.to_string());
    pattern = pattern.replace("{format}", format);
    
    pattern
}

fn process_image(
    source_path: &Path,
    article_slug: &str,
    topic_path: &str,
    output_dir: &Path,
    config: &Config,
) -> Result<Vec<PathBuf>> {
    println!("  {} {}", "Processing:".green().bold(), source_path.display());
    
    // Create output directory for this article
    let article_output_dir = output_dir.join(topic_path).join(article_slug);
    fs::create_dir_all(&article_output_dir)
        .context(format!("Failed to create output directory: {:?}", article_output_dir))?;
    
    // Open source image
    let img = image::open(source_path)
        .context(format!("Failed to open image: {:?}", source_path))?;
    
    // Track all generated files
    let mut generated_files = Vec::new();
    
    // Process each image size
    for (size_key, size_config) in &config.images.sizes {
        // Prepare the image according to its type
        let processed_img = if size_key == "square" {
            // Create square version
            let (width, height) = img.dimensions();
            let square = if width > height {
                let x = (width - height) / 2;
                img.crop_imm(x, 0, height, height)
            } else {
                let y = (height - width) / 2;
                img.crop_imm(0, y, width, width)
            };
            square.resize(size_config.width, size_config.height, image::imageops::FilterType::Lanczos3)
        } else {
            // For other types, maintain aspect ratio
            let aspect_ratio = img.width() as f32 / img.height() as f32;
            let target_height = (size_config.width as f32 / aspect_ratio) as u32;
            
            img.resize(size_config.width, target_height, image::imageops::FilterType::Lanczos3)
        };
        
        // Generate filenames and save in each format
        for format_map in &config.images.formats {
            for (format_name, _) in format_map {
                // Get quality setting
                let quality = match format_name.as_str() {
                    "webp" => {
                        if size_key == "thumbnail" {
                            config.images.quality.get("webp").map_or(75, |q| q.thumbnail)
                        } else {
                            config.images.quality.get("webp").map_or(80, |q| q.standard)
                        }
                    },
                    "jpg" => {
                        if size_key == "thumbnail" {
                            config.images.quality.get("jpg").map_or(80, |q| q.thumbnail)
                        } else {
                            config.images.quality.get("jpg").map_or(85, |q| q.standard)
                        }
                    },
                    "avif" => {
                        if size_key == "thumbnail" {
                            config.images.quality.get("avif").map_or(65, |q| q.thumbnail)
                        } else {
                            config.images.quality.get("avif").map_or(70, |q| q.standard)
                        }
                    },
                    _ => 80, // Default quality
                };
                
                // Generate filename
                let type_key = size_key.replace("_", "-"); // Convert featured_2x to featured-2x
                let filename = generate_image_filename(
                    config,
                    article_slug,
                    &type_key,
                    size_config.width,
                    size_config.height,
                    format_name,
                );
                
                let output_path = article_output_dir.join(&filename);
                
                // Save the image based on format
                match format_name.as_str() {
                    "webp" => {
                        processed_img.save_with_format(&output_path, ImageFormat::WebP)
                            .context(format!("Failed to save WebP image: {:?}", output_path))?;
                    },
                    "jpg" => {
                        processed_img.save_with_format(&output_path, ImageFormat::Jpeg)
                            .context(format!("Failed to save JPEG image: {:?}", output_path))?;
                    },
                    "avif" => {
                        // For AVIF, we'll use ImageMagick's convert command
                        // First save as a temporary PNG
                        let temp_path = output_path.with_extension("tmp.png");
                        processed_img.save(&temp_path)
                            .context(format!("Failed to save temporary image: {:?}", temp_path))?;
                        
                        // Use convert to create AVIF with optimal settings
                        // -define avif:speed=2 provides a good balance between encoding speed and compression
                        // -define avif:compression=av1 ensures AV1 compression is used
                        let status = std::process::Command::new("convert")
                            .arg(&temp_path)
                            .arg("-quality")
                            .arg(quality.to_string())
                            .arg("-define")
                            .arg("avif:speed=2")
                            .arg("-define")
                            .arg("avif:compression=av1")
                            .arg(&output_path)
                            .status()
                            .context("Failed to run convert command")?;
                        
                        // Remove the temporary file
                        fs::remove_file(&temp_path)
                            .context(format!("Failed to remove temporary file: {:?}", temp_path))?;
                        
                        if !status.success() {
                            return Err(anyhow::anyhow!("Failed to convert image to AVIF"));
                        }
                    },
                    _ => {
                        // Skip unsupported formats
                        continue;
                    }
                }
                
                generated_files.push(output_path.clone());
                println!("    {} {}", "Generated:".cyan().bold(), output_path.display());
            }
        }
    }
    
    Ok(generated_files)
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Read configuration
    let config = read_config()?;
    
    // Create output directory
    fs::create_dir_all(&args.output_dir)
        .context(format!("Failed to create output directory: {:?}", args.output_dir))?;
    
    // Track statistics
    let mut total_articles = 0;
    let mut total_images = 0;
    let mut processed_images = 0;
    let mut skipped_articles = 0;
    
    println!("{}", "Scanning for source images...".yellow().bold());
    
    // Process specific article if provided
    if let Some(article_slug) = &args.article {
        let topic_key = if let Some(topic) = &args.topic {
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
            topic.clone()
        } else {
            // Find topic for article
            let mut found_topic = None;
            for (topic_key, topic_config) in &config.content.topics {
                let article_dir = PathBuf::from(format!("{}/{}/{}", 
                    config.content.base_dir, 
                    topic_config.path, 
                    article_slug
                ));
                if article_dir.exists() {
                    found_topic = Some(topic_key.clone());
                    break;
                }
            }
            
            match found_topic {
                Some(topic) => topic,
                None => return Err(anyhow::anyhow!("Article not found: {}", article_slug)),
            }
        };
        
        let topic_config = &config.content.topics[&topic_key];
        let article_dir = PathBuf::from(format!("{}/{}/{}", 
            config.content.base_dir, 
            topic_config.path, 
            article_slug
        ));
        
        let source_path = article_dir.join(&args.source_filename);
        if source_path.exists() {
            total_articles += 1;
            total_images += 1;
            
            process_image(
                &source_path,
                article_slug,
                &topic_config.path,
                &args.output_dir,
                &config,
            )?;
            
            processed_images += 1;
        } else {
            println!("  {} Source image not found: {}", "Skipping:".red().bold(), source_path.display());
            skipped_articles += 1;
        }
    } else {
        // Process all articles or specific topic
        let topics_to_process = if let Some(topic) = &args.topic {
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
            vec![topic.clone()]
        } else {
            config.content.topics.keys().cloned().collect()
        };
        
        for topic_key in topics_to_process {
            let topic_config = &config.content.topics[&topic_key];
            let topic_dir = PathBuf::from(format!("{}/{}", 
                config.content.base_dir, 
                topic_config.path
            ));
            
            if !topic_dir.exists() {
                println!("  {} Topic directory not found: {}", "Skipping:".red().bold(), topic_dir.display());
                continue;
            }
            
            // Find all article directories in this topic
            for entry in fs::read_dir(topic_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.is_dir() {
                    let article_slug = path.file_name().unwrap().to_string_lossy().to_string();
                    let source_path = path.join(&args.source_filename);
                    
                    if source_path.exists() {
                        total_articles += 1;
                        total_images += 1;
                        
                        process_image(
                            &source_path,
                            &article_slug,
                            &topic_config.path,
                            &args.output_dir,
                            &config,
                        )?;
                        
                        processed_images += 1;
                    } else {
                        println!("  {} Source image not found: {}", "Skipping:".red().bold(), source_path.display());
                        skipped_articles += 1;
                    }
                }
            }
        }
    }
    
    // Print summary
    println!("\n{}", "Summary:".yellow().bold());
    println!("  Total articles scanned: {}", total_articles.to_string().cyan().bold());
    println!("  Total source images found: {}", total_images.to_string().cyan().bold());
    println!("  Images processed: {}", processed_images.to_string().green().bold());
    println!("  Articles skipped (no source image): {}", skipped_articles.to_string().red().bold());
    
    println!("\n{}", "Image build complete!".green().bold());
    println!("Optimized images are available in: {}", args.output_dir.display().to_string().cyan().bold());
    
    Ok(())
} 