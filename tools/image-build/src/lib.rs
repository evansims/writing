use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use anyhow::{Result, Context};
use common_models::{Config, ImageNaming};
use common_config::load_config;
use image::{ImageFormat, GenericImageView};

/// Options for building responsive images
#[derive(Debug, Clone)]
pub struct BuildImagesOptions {
    pub output_dir: PathBuf,
    pub source_dir: PathBuf,
    pub topic: Option<String>,
    pub article: Option<String>,
    pub force_rebuild: bool,
}

impl Default for BuildImagesOptions {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("build/images"),
            source_dir: PathBuf::from("content"),
            topic: None,
            article: None,
            force_rebuild: false,
        }
    }
}

/// Generate the filename for a processed image based on config pattern
pub fn generate_image_filename(
    config: &Config,
    article_slug: &str,
    image_type: &str,
    width: u32,
    height: u32,
    format: &str,
) -> String {
    // Get the naming pattern, or use a default if it's not set
    let default_naming = ImageNaming {
        pattern: "{slug}-{type}.{format}".to_string(),
        examples: vec![]
    };

    let naming_config = config.images.naming.as_ref().unwrap_or(&default_naming);
    let mut pattern = naming_config.pattern.clone();

    // Replace placeholders in the pattern
    pattern = pattern.replace("{slug}", article_slug);
    pattern = pattern.replace("{type}", image_type);
    pattern = pattern.replace("{width}", &width.to_string());
    pattern = pattern.replace("{height}", &height.to_string());
    pattern = pattern.replace("{format}", format);

    pattern
}

/// Get the topic key for a specific article
pub fn find_topic_for_article(config: &Config, article_slug: &str) -> Result<String> {
    // Find topic for article
    for (topic_key, topic_config) in &config.content.topics {
        let article_dir = PathBuf::from(format!("{}/{}/{}",
            config.content.base_dir,
            topic_config.directory,
            article_slug
        ));
        if article_dir.exists() {
            return Ok(topic_key.clone());
        }
    }

    Err(anyhow::anyhow!("Article not found: {}", article_slug))
}

/// Get article directory path based on its slug and topic
pub fn get_article_dir(config: &Config, article_slug: &str, topic_key: &str) -> Result<PathBuf> {
    if !config.content.topics.contains_key(topic_key) {
        let valid_topics: Vec<String> = config.content.topics.keys()
            .map(|k| k.to_string())
            .collect();
        return Err(anyhow::anyhow!(
            "Invalid topic: {}. Valid topics are: {}",
            topic_key,
            valid_topics.join(", ")
        ));
    }

    let topic_config = &config.content.topics[topic_key];
    Ok(PathBuf::from(format!("{}/{}/{}",
        config.content.base_dir,
        topic_config.directory,
        article_slug
    )))
}

/// Process a single image, generating all formats and sizes
pub fn process_image(
    source_path: &Path,
    article_slug: &str,
    topic_path: &str,
    output_dir: &Path,
    config: &Config,
) -> Result<Vec<PathBuf>> {
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

        // Define supported formats based on features
        let mut formats = vec!["jpg"];

        #[cfg(feature = "basic-formats")]
        formats.push("webp");

        #[cfg(feature = "avif")]
        formats.push("avif");

        // Create an empty HashMap to use as default
        let empty_quality_settings = HashMap::new();
        let empty_format_settings = HashMap::new();

        // Get quality settings if available
        let quality_settings = config.images.quality.as_ref().unwrap_or(&empty_quality_settings);

        // Generate filenames and save in each format
        for format_name in formats {
            // Get quality setting for this format based on format and size
            let _quality = match format_name {
                #[cfg(feature = "basic-formats")]
                "webp" => {
                    let webp_settings = quality_settings.get("webp").unwrap_or(&empty_format_settings);
                    let default_value = if size_key == "thumbnail" { 75 } else { 80 };
                    let key = if size_key == "thumbnail" { "thumbnail" } else { "standard" };
                    *webp_settings.get(key).unwrap_or(&default_value)
                },
                "jpg" => {
                    let jpg_settings = quality_settings.get("jpg").unwrap_or(&empty_format_settings);
                    let default_value = if size_key == "thumbnail" { 80 } else { 85 };
                    let key = if size_key == "thumbnail" { "thumbnail" } else { "standard" };
                    *jpg_settings.get(key).unwrap_or(&default_value)
                },
                #[cfg(feature = "avif")]
                "avif" => {
                    let avif_settings = quality_settings.get("avif").unwrap_or(&empty_format_settings);
                    let default_value = if size_key == "thumbnail" { 65 } else { 70 };
                    let key = if size_key == "thumbnail" { "thumbnail" } else { "standard" };
                    *avif_settings.get(key).unwrap_or(&default_value)
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
            match format_name {
                #[cfg(feature = "basic-formats")]
                "webp" => {
                    processed_img.save_with_format(&output_path, ImageFormat::WebP)
                        .context(format!("Failed to save WebP image: {:?}", output_path))?;
                },
                "jpg" => {
                    processed_img.save_with_format(&output_path, ImageFormat::Jpeg)
                        .context(format!("Failed to save JPEG image: {:?}", output_path))?;
                },
                #[cfg(feature = "avif")]
                "avif" => {
                    // For AVIF, we'll use ImageMagick's convert command
                    // First save as a temporary PNG
                    let temp_path = output_path.with_extension("tmp.png");
                    processed_img.save(&temp_path)
                        .context(format!("Failed to save temporary image: {:?}", temp_path))?;

                    // Use convert to create AVIF with optimal settings
                    let status = Command::new("convert")
                        .arg(&temp_path)
                        .arg("-quality")
                        .arg(_quality.to_string())
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
        }
    }

    Ok(generated_files)
}

/// Build images for a specific article
pub fn build_article_images(
    config: &Config,
    article_slug: &str,
    topic_key: &str,
    options: &BuildImagesOptions,
) -> Result<Vec<PathBuf>> {
    let topic_config = &config.content.topics[topic_key];
    let article_dir = get_article_dir(config, article_slug, topic_key)?;

    let source_path = article_dir.join("index.jpg");
    if source_path.exists() {
        process_image(
            &source_path,
            article_slug,
            &topic_config.directory,
            &options.output_dir,
            config,
        )
    } else {
        Err(anyhow::anyhow!("Source image not found: {}", source_path.display()))
    }
}

/// Main function to build images based on options
pub fn build_images(options: &BuildImagesOptions) -> Result<(usize, usize, usize, usize)> {
    // Read configuration
    let config = load_config()?;

    // Create output directory
    fs::create_dir_all(&options.output_dir)
        .context(format!("Failed to create output directory: {:?}", options.output_dir))?;

    // Track statistics
    let mut total_articles = 0;
    let mut total_images = 0;
    let mut processed_images = 0;
    let mut skipped_articles = 0;

    // Process specific article if provided
    if let Some(article_slug) = &options.article {
        let topic_key = if let Some(topic) = &options.topic {
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
            find_topic_for_article(&config, article_slug)?
        };

        total_articles += 1;
        total_images += 1;

        match build_article_images(&config, article_slug, &topic_key, options) {
            Ok(_) => {
                processed_images += 1;
                Ok((total_articles, total_images, processed_images, skipped_articles))
            },
            Err(e) => {
                skipped_articles += 1;
                // Return the error with context about the skipped article
                Err(anyhow::anyhow!("Failed to process article {}: {}. Stats: {} total, {} processed, {} skipped",
                    article_slug, e, total_articles, processed_images, skipped_articles))
            }
        }
    } else {
        // Process all articles or specific topic
        let topics_to_process = if let Some(topic) = &options.topic {
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
                topic_config.directory
            ));

            if !topic_dir.exists() {
                continue;
            }

            // Find all article directories in this topic
            for entry in fs::read_dir(topic_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    let article_slug = path.file_name().unwrap().to_string_lossy().to_string();
                    let source_path = path.join("index.jpg");

                    if source_path.exists() {
                        total_articles += 1;
                        total_images += 1;

                        match process_image(
                            &source_path,
                            &article_slug,
                            &topic_config.directory,
                            &options.output_dir,
                            &config,
                        ) {
                            Ok(_) => {
                                processed_images += 1;
                            },
                            Err(_) => {
                                skipped_articles += 1;
                            }
                        }
                    } else {
                        skipped_articles += 1;
                    }
                }
            }
        }

        Ok((total_articles, total_images, processed_images, skipped_articles))
    }
}