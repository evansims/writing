use anyhow::{Context, Result};
use common_config::load_config;
use common_models::Config;
use image::{GenericImageView, ImageFormat};
use std::fs;
use std::path::{Path, PathBuf};

/// Options for image optimization
#[derive(Debug, Clone)]
pub struct OptimizeOptions {
    /// Source image path
    pub source: PathBuf,
    /// Article slug
    pub article: String,
    /// Topic (optional, will search if not provided)
    pub topic: Option<String>,
}

impl Default for OptimizeOptions {
    fn default() -> Self {
        Self {
            source: PathBuf::new(),
            article: String::new(),
            topic: None,
        }
    }
}

/// Find an article directory based on the slug
pub fn find_article_directory(article_slug: &str) -> Result<PathBuf> {
    let config = load_config()?;
    
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

/// Get the article directory based on the article slug and optional topic
pub fn get_article_directory(options: &OptimizeOptions) -> Result<PathBuf> {
    let config = load_config()?;
    
    match &options.topic {
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
            Ok(PathBuf::from(format!("{}/{}/{}", 
                config.content.base_dir, 
                topic_path, 
                options.article
            )))
        },
        None => find_article_directory(&options.article),
    }
}

/// Optimize a source image and save it to the target path
/// 
/// Returns a tuple with:
/// - Source image dimensions (width, height)
/// - Source file size in bytes
/// - Target file size in bytes
pub fn optimize_source_image(source_path: &Path, target_path: &Path) -> Result<((u32, u32), u64, u64)> {
    // Open source image
    let img = image::open(source_path)
        .context(format!("Failed to open image: {:?}", source_path))?;
    
    // Get image dimensions
    let dimensions = img.dimensions();
    
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
    
    Ok((dimensions, source_size, target_size))
}

/// Main function to optimize an image for an article
pub fn optimize_image(options: &OptimizeOptions) -> Result<PathBuf> {
    // Check if source image exists
    if !options.source.exists() {
        return Err(anyhow::anyhow!("Source image not found: {:?}", options.source));
    }
    
    // Find article directory
    let article_dir = get_article_directory(options)?;
    
    // Create directory if it doesn't exist
    if !article_dir.exists() {
        fs::create_dir_all(&article_dir)
            .context(format!("Failed to create directory: {:?}", article_dir))?;
    }
    
    // Define target path for index.jpg
    let target_path = article_dir.join("index.jpg");
    
    // Optimize the source image
    optimize_source_image(&options.source, &target_path)?;
    
    Ok(target_path)
} 