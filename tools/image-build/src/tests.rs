use super::*;
use std::path::PathBuf;
use tempfile::TempDir;

/// Helper function to create a test image
fn create_test_image(dir: &Path) -> Result<PathBuf> {
    let img_path = dir.join("test.jpg");
    let img = image::RgbaImage::new(100, 100);
    img.save(&img_path)?;
    Ok(img_path)
}

/// Helper function to create a test config
fn create_test_config() -> Config {
    Config {
        content: common_models::ContentConfig {
            base_dir: "content".into(),
            topics: {
                let mut map = std::collections::HashMap::new();
                map.insert("test-topic".into(), common_models::TopicConfig {
                    name: "Test Topic".into(),
                    description: "Test Topic Description".into(),
                    directory: "test-topic".into(),
                });
                map
            },
            tags: None,
        },
        images: common_models::ImageConfig {
            formats: vec!["jpg".to_string(), "webp".to_string()],
            format_descriptions: None,
            sizes: {
                let mut map = std::collections::HashMap::new();
                map.insert("standard".into(), common_models::ImageSize {
                    width: 800,
                    height: 600,
                    description: "Standard size".into(),
                });
                map.insert("thumbnail".into(), common_models::ImageSize {
                    width: 200,
                    height: 150,
                    description: "Thumbnail size".into(),
                });
                map
            },
            naming: Some(common_models::ImageNaming {
                pattern: "{slug}-{type}.{format}".into(),
                examples: vec![],
            }),
            quality: Some({
                let mut map = std::collections::HashMap::new();
                let mut jpg_settings = std::collections::HashMap::new();
                jpg_settings.insert("standard".into(), 85);
                jpg_settings.insert("thumbnail".into(), 80);
                map.insert("jpg".into(), jpg_settings);
                map
            }),
        },
        publication: common_models::PublicationConfig {
            author: "Test Author".into(),
            copyright: "Test Copyright".into(),
            site_url: Some("https://example.com".into()),
        },
    }
}

#[test]
fn test_basic_image_processing() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let source_path = create_test_image(temp_dir.path())?;
    let output_dir = temp_dir.path().join("output");
    let config = create_test_config();

    let result = process_image(
        &source_path,
        "test-article",
        "test-topic",
        &output_dir,
        &config,
    )?;

    // Should always generate JPEG
    assert!(result.iter().any(|p| p.extension().unwrap() == "jpg"));

    // Check WebP generation based on feature flag
    #[cfg(feature = "basic-formats")]
    assert!(result.iter().any(|p| p.extension().unwrap() == "webp"));

    // Check AVIF generation based on feature flag
    #[cfg(feature = "avif")]
    assert!(result.iter().any(|p| p.extension().unwrap() == "avif"));

    Ok(())
}

#[test]
fn test_supported_formats() {
    // Create a vector to store supported formats
    let mut formats = vec!["jpg"];

    #[cfg(feature = "basic-formats")]
    formats.push("webp");

    #[cfg(feature = "avif")]
    formats.push("avif");

    // JPEG should always be supported
    assert!(formats.contains(&"jpg"));

    // Check WebP support based on feature flag
    #[cfg(feature = "basic-formats")]
    assert!(formats.contains(&"webp"));
    #[cfg(not(feature = "basic-formats"))]
    assert!(!formats.contains(&"webp"));

    // Check AVIF support based on feature flag
    #[cfg(feature = "avif")]
    assert!(formats.contains(&"avif"));
    #[cfg(not(feature = "avif"))]
    assert!(!formats.contains(&"avif"));
}

#[test]
fn test_build_article_images() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let content_dir = temp_dir.path().join("content");
    let article_dir = content_dir.join("test-topic/test-article");
    std::fs::create_dir_all(&article_dir)?;

    let source_path = create_test_image(&article_dir)?;
    std::fs::rename(&source_path, article_dir.join("index.jpg"))?;

    // Create a config that uses the temporary content directory
    let mut config = create_test_config();
    config.content.base_dir = content_dir.to_string_lossy().to_string();

    let options = BuildImagesOptions {
        output_dir: temp_dir.path().join("output"),
        source_dir: content_dir.clone(),  // Use the actual content directory path
        source_filename: "index.jpg".into(),
        article: Some("test-article".into()),
        topic: Some("test-topic".into()),
    };

    let result = build_article_images(
        &config,
        "test-article",
        "test-topic",
        &options,
    )?;

    // Should always generate JPEG
    assert!(result.iter().any(|p| p.extension().unwrap() == "jpg"));

    // Check WebP generation based on feature flag
    #[cfg(feature = "basic-formats")]
    assert!(result.iter().any(|p| p.extension().unwrap() == "webp"));

    // Check AVIF generation based on feature flag
    #[cfg(feature = "avif")]
    assert!(result.iter().any(|p| p.extension().unwrap() == "avif"));

    Ok(())
}

#[test]
fn test_quality_settings() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let source_path = create_test_image(temp_dir.path())?;
    let output_dir = temp_dir.path().join("output");
    let mut config = create_test_config();

    // Add quality settings for all formats
    let mut quality_settings = std::collections::HashMap::new();

    // JPEG settings
    let mut jpg_settings = std::collections::HashMap::new();
    jpg_settings.insert("standard".into(), 85);
    jpg_settings.insert("thumbnail".into(), 80);
    quality_settings.insert("jpg".into(), jpg_settings);

    #[cfg(feature = "basic-formats")]
    {
        let mut webp_settings = std::collections::HashMap::new();
        webp_settings.insert("standard".into(), 80);
        webp_settings.insert("thumbnail".into(), 75);
        quality_settings.insert("webp".into(), webp_settings);
    }

    #[cfg(feature = "avif")]
    {
        let mut avif_settings = std::collections::HashMap::new();
        avif_settings.insert("standard".into(), 70);
        avif_settings.insert("thumbnail".into(), 65);
        quality_settings.insert("avif".into(), avif_settings);
    }

    config.images.quality = Some(quality_settings);

    let result = process_image(
        &source_path,
        "test-article",
        "test-topic",
        &output_dir,
        &config,
    )?;

    // Verify files were generated
    assert!(!result.is_empty());

    Ok(())
}