use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Write, BufWriter};
use anyhow::Result;
use common_config::load_config;
use common_errors::{WritingError, OptionValidationExt};
use image::{GenericImageView, DynamicImage};
use image::imageops::FilterType;
use thiserror::Error;

/// Supported output formats for image optimization
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Jpeg,
    #[cfg(feature = "webp")]
    WebP,
}

impl OutputFormat {
    /// Get the file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Jpeg => "jpg",
            #[cfg(feature = "webp")]
            OutputFormat::WebP => "webp",
        }
    }

    /// Create an OutputFormat from a string
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "jpg" | "jpeg" => Ok(OutputFormat::Jpeg),
            #[cfg(feature = "webp")]
            "webp" => Ok(OutputFormat::WebP),
            _ => Err(anyhow::anyhow!(OptimizeError::UnsupportedFormat(s.to_string()))),
        }
    }
}

/// Error types specific to image optimization
#[derive(Error, Debug)]
pub enum OptimizeError {
    #[error("Unsupported image format: {0}")]
    UnsupportedFormat(String),

    #[error("Failed to process image: {0}")]
    ProcessingError(String),

    #[error("Article not found: {0}")]
    ArticleNotFound(String),

    #[error("Invalid dimensions: width={0}, height={1}")]
    InvalidDimensions(u32, u32),
}

/// Image size variants for responsive images
#[derive(Debug, Clone, Copy)]
pub enum SizeVariant {
    Original,
    Large(u32),    // Default: 1200px width
    Medium(u32),   // Default: 800px width
    Small(u32),    // Default: 400px width
    Thumbnail(u32), // Default: 200px width
}

impl SizeVariant {
    pub fn name(&self) -> &'static str {
        match self {
            SizeVariant::Original => "original",
            SizeVariant::Large(_) => "large",
            SizeVariant::Medium(_) => "medium",
            SizeVariant::Small(_) => "small",
            SizeVariant::Thumbnail(_) => "thumbnail",
        }
    }

    pub fn width(&self) -> Option<u32> {
        match self {
            SizeVariant::Original => None,
            SizeVariant::Large(w) => Some(*w),
            SizeVariant::Medium(w) => Some(*w),
            SizeVariant::Small(w) => Some(*w),
            SizeVariant::Thumbnail(w) => Some(*w),
        }
    }
}

/// Options for image optimization
#[derive(Debug, Clone)]
pub struct OptimizeOptions {
    /// Source image path
    pub source: PathBuf,
    /// Article slug
    pub article: Option<String>,
    /// Topic (optional, will search if not provided)
    pub topic: Option<String>,
    /// Output formats to generate
    pub formats: Vec<OutputFormat>,
    /// Size variants to generate
    pub sizes: Vec<SizeVariant>,
    /// Quality level (0-100)
    pub quality: u8,
    /// Whether to preserve original image metadata
    pub preserve_metadata: bool,
}

impl Default for OptimizeOptions {
    fn default() -> Self {
        Self {
            source: PathBuf::new(),
            article: None,
            topic: None,
            formats: vec![OutputFormat::Jpeg],
            sizes: vec![SizeVariant::Original],
            quality: 85,
            preserve_metadata: false,
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
            topic_config.directory,
            article_slug
        ));
        if dir.exists() {
            return Ok(dir);
        }
    }

    Err(OptimizeError::ArticleNotFound(article_slug.to_string()).into())
}

/// Get the article directory based on the article slug and optional topic
pub fn get_article_directory(options: &OptimizeOptions) -> Result<PathBuf> {
    let config = load_config()
        .map_err(|e| WritingError::config_error(format!("Failed to load config: {}", e)))?;

    if let Some(topic_name) = &options.topic {
        // Find the topic in the config and validate it exists
        let topic_config = config.content.topics.get(topic_name)
            .validate_with(|| WritingError::topic_error(format!("Topic not found: {}", topic_name)))?;

        let article_dir = PathBuf::from(format!("{}/{}/{}",
            config.content.base_dir,
            topic_config.directory,
            options.article.as_ref().unwrap()
        ));

        // Check if directory exists
        if !article_dir.exists() {
            return Err(WritingError::content_not_found(format!(
                "Article '{}' not found in topic '{}'",
                options.article.as_ref().unwrap(),
                topic_name
            )).into());
        }

        return Ok(article_dir);
    } else {
        // Try to find the article directory by slug
        find_article_directory(&options.article.as_ref().unwrap())
    }
}

/// Resize an image to the specified width while maintaining aspect ratio
fn resize_image(img: &DynamicImage, width: u32) -> DynamicImage {
    let (orig_width, orig_height) = img.dimensions();

    // Calculate new height maintaining aspect ratio
    let height = (orig_height as f64 * (width as f64 / orig_width as f64)).round() as u32;

    img.resize(width, height, image::imageops::FilterType::Lanczos3)
}

/// Save image in JPEG format with high quality compression
fn save_as_jpeg(img: &DynamicImage, path: &Path, quality: u8) -> Result<u64> {
    // Use the built-in JPEG encoder from the image crate
    let file = std::fs::File::create(path)?;
    let mut writer = std::io::BufWriter::new(file);

    // Convert quality from 0-100 to 1-100 (image crate uses 1-100)
    let quality = std::cmp::max(1, quality);

    // Encode the image as JPEG
    img.write_to(&mut writer, image::ImageOutputFormat::Jpeg(quality))?;

    // Get the file size
    let file_size = writer.into_inner()?.metadata()?.len();

    Ok(file_size)
}

/// Save image in WebP format
#[cfg(feature = "webp")]
fn save_as_webp(img: &DynamicImage, path: &Path, quality: u8) -> Result<u64> {
    // Convert to RGBA
    let rgba = img.to_rgba8();
    let (width, height) = img.dimensions();

    // Encode as WebP
    let encoder = webp::Encoder::from_rgba(&rgba, width, height);
    let webp_data = encoder.encode(quality as f32 / 100.0);

    // Write to file
    let mut file = BufWriter::new(fs::File::create(path)?);
    file.write_all(&webp_data)?;

    Ok(webp_data.len() as u64)
}

/// Save image in the specified format
fn save_in_format(img: &DynamicImage, path: &Path, format: OutputFormat, quality: u8) -> Result<u64> {
    match format {
        OutputFormat::Jpeg => save_as_jpeg(img, path, quality),
        #[cfg(feature = "webp")]
        OutputFormat::WebP => save_as_webp(img, path, quality),
    }
}

/// Results of image optimization
#[derive(Debug)]
pub struct OptimizationResult {
    pub original_size: u64,
    pub format_results: Vec<FormatResult>,
}

/// Results for a specific format
#[derive(Debug)]
pub struct FormatResult {
    pub format: OutputFormat,
    pub size_results: Vec<SizeResult>,
}

/// Results for a specific size variant
#[derive(Debug)]
pub struct SizeResult {
    pub variant: SizeVariant,
    pub dimensions: (u32, u32),
    pub file_size: u64,
    pub path: PathBuf,
}

/// Main function to optimize an image for an article
pub fn optimize_image(options: &OptimizeOptions) -> Result<OptimizationResult> {
    // Validate source image exists
    if !options.source.exists() {
        return Err(WritingError::file_not_found(&options.source).into());
    }

    // Get the source image file size
    let original_size = std::fs::metadata(&options.source)
        .map_err(|e| WritingError::other(format!("Failed to read source file metadata: {}", e)))?
        .len();

    // Load the source image
    let img = image::open(&options.source)
        .map_err(|e| WritingError::format_error(format!("Failed to open image: {}", e)))?;

    // Get the image dimensions
    let (width, height) = img.dimensions();

    // Validate dimensions
    if width == 0 || height == 0 {
        return Err(WritingError::invalid_argument(format!(
            "Invalid image dimensions: {}x{}", width, height
        )).into());
    }

    // Get the article directory
    let article_dir = get_article_directory(options)?;

    // Create the images directory
    let images_dir = article_dir.join("images");
    std::fs::create_dir_all(&images_dir)
        .map_err(|e| WritingError::other(format!("Failed to create images directory: {}", e)))?;

    // Create results container
    let mut result = OptimizationResult {
        original_size,
        format_results: Vec::new(),
    };

    // Process for each format
    for format in &options.formats {
        // Apply functional transformation to create size_results
        let size_results = options.sizes.iter()
            .filter(|&size| !matches!(size, SizeVariant::Original) || *format != OutputFormat::Jpeg)
            .map(|size| {
                // Process the image according to size variant
                let processed_img = match size {
                    SizeVariant::Original => img.clone(),
                    SizeVariant::Large(width) |
                    SizeVariant::Medium(width) |
                    SizeVariant::Small(width) |
                    SizeVariant::Thumbnail(width) => resize_image(&img, *width),
                };

                // Create filename with size variant and format
                let filename = if matches!(size, SizeVariant::Original) {
                    format!("original.{}", format.extension())
                } else {
                    format!("{}.{}", size.name(), format.extension())
                };

                // Define target path
                let target_path = images_dir.join(&filename);

                // Save the processed image and return result
                let file_size = save_in_format(&processed_img, &target_path, *format, options.quality)
                    .expect("Failed to save image"); // Handle error appropriately in real code

                SizeResult {
                    variant: *size,
                    dimensions: processed_img.dimensions(),
                    file_size,
                    path: target_path,
                }
            })
            .collect::<Vec<_>>();

        // Add special handling for index.jpg
        let all_size_results = if *format == OutputFormat::Jpeg {
            // Define target path for main index.jpg
            let index_path = article_dir.join(format!("index.{}", format.extension()));

            // For index.jpg, always use the original dimensions
            let dimensions = img.dimensions();
            let file_size = save_in_format(&img, &index_path, *format, options.quality)?;

            // Combine with the original index file
            let mut combined = vec![
                SizeResult {
                    variant: SizeVariant::Original,
                    dimensions,
                    file_size,
                    path: index_path,
                }
            ];
            combined.extend(size_results);
            combined
        } else {
            size_results
        };

        result.format_results.push(FormatResult {
            format: *format,
            size_results: all_size_results,
        });
    }

    Ok(result)
}

/// Generates default size variants for responsive images
pub fn default_size_variants() -> Vec<SizeVariant> {
    // Use more functional approach with iterator instead of vec! macro
    [
        SizeVariant::Original,
        SizeVariant::Large(1200),
        SizeVariant::Medium(800),
        SizeVariant::Small(400),
        SizeVariant::Thumbnail(200),
    ].into_iter().collect()
}

/// Get the default output formats
pub fn default_formats() -> Vec<OutputFormat> {
    let mut formats = vec![OutputFormat::Jpeg];

    #[cfg(feature = "webp")]
    formats.push(OutputFormat::WebP);

    formats
}

/// Optimize images in content
///
/// This function optimizes images in content based on the provided options.
///
/// # Parameters
///
/// * `options` - Optimization options
///
/// # Returns
///
/// Returns the number of images optimized
///
/// # Errors
///
/// Returns an error if the optimization fails
pub fn optimize_images(options: &OptimizeOptions) -> Result<usize> {
    let config = load_config()?;
    let mut optimized_count = 0;

    // Determine the quality to use
    let quality = options.quality;

    // Get the maximum dimensions
    let max_width = options.sizes.iter().map(|s| s.width()).max().unwrap_or(Some(1200));
    let max_height = options.sizes.iter().map(|s| s.width()).max().unwrap_or(Some(1200));

    // Determine the output format
    let format = options.formats.iter().next().cloned().unwrap_or(OutputFormat::Jpeg);

    // If article is specified, optimize images for that article
    if options.article.is_some() {
        let article_slug = options.article.as_ref().unwrap();
        if let Some(topic_key) = &options.topic {
            // Optimize images for a specific content in a specific topic
            if let Some(topic_config) = config.content.topics.get(topic_key) {
                let content_dir = PathBuf::from(&config.content.base_dir)
                    .join(&topic_config.directory)
                    .join(article_slug);

                if !content_dir.exists() {
                    return Err(anyhow::anyhow!("Content not found: {}/{}", topic_key, article_slug));
                }

                let images_dir = content_dir.join("images");

                if images_dir.exists() {
                    optimized_count += optimize_images_in_dir(
                        &images_dir,
                        quality,
                        max_width,
                        max_height,
                        format,
                    )?;
                }
            } else {
                return Err(anyhow::anyhow!("Topic not found: {}", topic_key));
            }
        } else {
            // Optimize images for a specific content in any topic
            for (_topic_key, topic_config) in &config.content.topics {
                let content_dir = PathBuf::from(&config.content.base_dir)
                    .join(&topic_config.directory)
                    .join(article_slug);

                if content_dir.exists() {
                    let images_dir = content_dir.join("images");

                    if images_dir.exists() {
                        optimized_count += optimize_images_in_dir(
                            &images_dir,
                            quality,
                            max_width,
                            max_height,
                            format,
                        )?;
                    }

                    break;
                }
            }
        }
    } else if let Some(topic_key) = &options.topic {
        // Optimize images for all content in a specific topic
        if let Some(topic_config) = config.content.topics.get(topic_key) {
            let topic_dir = PathBuf::from(&config.content.base_dir)
                .join(&topic_config.directory);

            if !topic_dir.exists() {
                return Err(anyhow::anyhow!("Topic directory not found: {}", topic_dir.display()));
            }

            // Find all content directories in the topic
            for entry in fs::read_dir(topic_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    let images_dir = path.join("images");

                    if images_dir.exists() {
                        optimized_count += optimize_images_in_dir(
                            &images_dir,
                            quality,
                            max_width,
                            max_height,
                            format,
                        )?;
                    }
                }
            }
        } else {
            return Err(anyhow::anyhow!("Topic not found: {}", topic_key));
        }
    } else {
        // Optimize images for all content in all topics
        for (_topic_key, topic_config) in &config.content.topics {
            let topic_dir = PathBuf::from(&config.content.base_dir)
                .join(&topic_config.directory);

            if !topic_dir.exists() {
                continue;
            }

            // Find all content directories in the topic
            for entry in fs::read_dir(topic_dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    let images_dir = path.join("images");

                    if images_dir.exists() {
                        optimized_count += optimize_images_in_dir(
                            &images_dir,
                            quality,
                            max_width,
                            max_height,
                            format,
                        )?;
                    }
                }
            }
        }
    }

    Ok(optimized_count)
}

/// Optimize images in a directory
///
/// This function optimizes all images in a directory.
///
/// # Parameters
///
/// * `dir` - Directory containing images
/// * `quality` - JPEG/WebP quality (0-100)
/// * `max_width` - Maximum width
/// * `max_height` - Maximum height
/// * `format` - Output format (webp, jpeg, png)
///
/// # Returns
///
/// Returns the number of images optimized
///
/// # Errors
///
/// Returns an error if the optimization fails
fn optimize_images_in_dir(
    dir: &Path,
    quality: u8,
    max_width: Option<u32>,
    max_height: Option<u32>,
    format: OutputFormat,
) -> Result<usize> {
    let mut optimized_count = 0;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let extension = path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
                .to_lowercase();

            if ["jpg", "jpeg", "png", "gif", "webp"].contains(&extension.as_str()) {
                // Load the image
                let img = image::open(&path)?;

                // Resize if necessary
                let img = resize_image_with_max_dimensions(&img, max_width, max_height);

                // Determine the output path
                let output_path = match format {
                    OutputFormat::Jpeg => path.with_extension("jpg"),
                    OutputFormat::WebP => path.with_extension("webp"),
                };

                // Save the optimized image
                let _file_size = save_in_format(&img, &output_path, format, quality)?;

                optimized_count += 1;
            }
        }
    }

    Ok(optimized_count)
}

/// Resize an image with maximum width and height constraints
fn resize_image_with_max_dimensions(img: &DynamicImage, max_width: Option<u32>, max_height: Option<u32>) -> DynamicImage {
    let width = img.width();
    let height = img.height();

    // Calculate aspect ratio
    let aspect_ratio = width as f32 / height as f32;

    // Calculate new dimensions
    let (new_width, new_height) = if width > height {
        let new_width = max_width.unwrap_or(width);
        let new_height = (new_width as f32 / aspect_ratio) as u32;

        if new_height > max_height.unwrap_or(height) {
            let new_height = max_height.unwrap_or(height);
            let new_width = (new_height as f32 * aspect_ratio) as u32;
            (new_width, new_height)
        } else {
            (new_width, new_height)
        }
    } else {
        let new_height = max_height.unwrap_or(height);
        let new_width = (new_height as f32 * aspect_ratio) as u32;

        if new_width > max_width.unwrap_or(width) {
            let new_width = max_width.unwrap_or(width);
            let new_height = (new_width as f32 / aspect_ratio) as u32;
            (new_width, new_height)
        } else {
            (new_width, new_height)
        }
    };

    // Resize the image
    img.resize_exact(new_width, new_height, FilterType::Lanczos3)
}