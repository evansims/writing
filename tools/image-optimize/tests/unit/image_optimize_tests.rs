use anyhow::Result;
use common_test_utils::TestFixture;
use image_optimize::{OptimizeOptions, OutputFormat, SizeVariant, find_article_directory, get_article_directory};
use std::path::{Path, PathBuf};
use serial_test::serial;

#[test]
fn test_output_format_extension() {
    assert_eq!(OutputFormat::Jpeg.extension(), "jpg");

    #[cfg(feature = "webp")]
    assert_eq!(OutputFormat::WebP.extension(), "webp");
}

#[test]
fn test_output_format_from_str() -> Result<()> {
    assert_eq!(OutputFormat::from_str("jpg")?, OutputFormat::Jpeg);
    assert_eq!(OutputFormat::from_str("jpeg")?, OutputFormat::Jpeg);
    assert_eq!(OutputFormat::from_str("JPEG")?, OutputFormat::Jpeg);

    #[cfg(feature = "webp")]
    assert_eq!(OutputFormat::from_str("webp")?, OutputFormat::WebP);

    // Invalid format should return an error
    let result = OutputFormat::from_str("invalid");
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Unsupported image format"));

    Ok(())
}

#[test]
fn test_size_variant_properties() {
    // Test names
    assert_eq!(SizeVariant::Original.name(), "original");
    assert_eq!(SizeVariant::Large(1000).name(), "large");
    assert_eq!(SizeVariant::Medium(500).name(), "medium");
    assert_eq!(SizeVariant::Small(250).name(), "small");
    assert_eq!(SizeVariant::Thumbnail(100).name(), "thumbnail");

    // Test widths
    assert_eq!(SizeVariant::Original.width(), None);
    assert_eq!(SizeVariant::Large(1000).width(), Some(1000));
    assert_eq!(SizeVariant::Medium(500).width(), Some(500));
    assert_eq!(SizeVariant::Small(250).width(), Some(250));
    assert_eq!(SizeVariant::Thumbnail(100).width(), Some(100));
}

#[test]
fn test_optimize_options_default() {
    let options = OptimizeOptions::default();

    assert!(options.source.as_os_str().is_empty());
    assert!(options.article.is_none());
    assert!(options.topic.is_none());
    assert_eq!(options.formats.len(), 1);
    assert_eq!(options.formats[0], OutputFormat::Jpeg);
    assert_eq!(options.sizes.len(), 1);
    match options.sizes[0] {
        SizeVariant::Original => {},
        _ => panic!("Default size variant should be Original"),
    }
    assert_eq!(options.quality, 85);
    assert!(!options.preserve_metadata);
}

#[test]
#[serial]
fn test_find_article_directory_returns_error_for_nonexistent_article() -> Result<()> {
    // Arrange - Create a config file
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

    // Create a basic config
    let mut config = common_models::Config::default();
    config.content.base_dir = "content".to_string();

    // Create topic structure
    let mut topics = std::collections::HashMap::new();
    topics.insert(
        "blog".to_string(),
        common_models::TopicConfig {
            name: "Blog".to_string(),
            description: "Blog posts".to_string(),
            directory: "blog".to_string(),
        },
    );
    config.content.topics = topics;

    // Save the config to fixture path
    let config_str = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, &config_str)?;

    // Create content directory
    let content_dir = fixture_path.join("content");
    std::fs::create_dir_all(&content_dir)?;
    std::fs::create_dir_all(content_dir.join("blog"))?;

    // Set the config path environment variable
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_ARTICLE_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Act
    let result = find_article_directory("nonexistent-article");

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("not found"));

    Ok(())
}

#[test]
#[serial]
fn test_find_article_directory_finds_existing_article() -> Result<()> {
    // Arrange - Create a config file
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

    // Create a basic config
    let mut config = common_models::Config::default();
    config.content.base_dir = "content".to_string();

    // Create topic structure
    let mut topics = std::collections::HashMap::new();
    topics.insert(
        "blog".to_string(),
        common_models::TopicConfig {
            name: "Blog".to_string(),
            description: "Blog posts".to_string(),
            directory: "blog".to_string(),
        },
    );
    config.content.topics = topics;

    // Save the config to fixture path
    let config_str = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, &config_str)?;

    // Create content directory with an article
    let content_dir = fixture_path.join("content");
    std::fs::create_dir_all(&content_dir)?;
    let blog_dir = content_dir.join("blog");
    std::fs::create_dir_all(&blog_dir)?;
    let article_dir = blog_dir.join("test-article");
    std::fs::create_dir_all(&article_dir)?;

    // Set the config path environment variable
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_ARTICLE_EXISTS_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Act
    let result = find_article_directory("test-article");

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_ok());
    let dir = result.unwrap();
    assert_eq!(dir.file_name().unwrap().to_string_lossy(), "test-article");

    Ok(())
}

#[test]
#[serial]
fn test_get_article_directory_with_topic_specified() -> Result<()> {
    // Arrange - Create a config file
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

    // Create a basic config
    let mut config = common_models::Config::default();
    config.content.base_dir = "content".to_string();

    // Create topic structure
    let mut topics = std::collections::HashMap::new();
    topics.insert(
        "blog".to_string(),
        common_models::TopicConfig {
            name: "Blog".to_string(),
            description: "Blog posts".to_string(),
            directory: "blog".to_string(),
        },
    );
    config.content.topics = topics;

    // Save the config to fixture path
    let config_str = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, &config_str)?;

    // Create content directory with an article
    let content_dir = fixture_path.join("content");
    std::fs::create_dir_all(&content_dir)?;
    let blog_dir = content_dir.join("blog");
    std::fs::create_dir_all(&blog_dir)?;
    let article_dir = blog_dir.join("test-article");
    std::fs::create_dir_all(&article_dir)?;

    // Set the config path environment variable
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_WITH_TOPIC_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Create options
    let options = OptimizeOptions {
        source: PathBuf::from("test-image.jpg"),
        article: Some("test-article".to_string()),
        topic: Some("blog".to_string()),
        ..OptimizeOptions::default()
    };

    // Act
    let result = get_article_directory(&options);

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_ok());
    let dir = result.unwrap();
    assert_eq!(dir.file_name().unwrap().to_string_lossy(), "test-article");

    Ok(())
}

#[test]
#[serial]
fn test_get_article_directory_returns_error_for_nonexistent_topic() -> Result<()> {
    // Arrange - Create a config file
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

    // Create a basic config
    let mut config = common_models::Config::default();
    config.content.base_dir = "content".to_string();

    // Create topic structure
    let mut topics = std::collections::HashMap::new();
    topics.insert(
        "blog".to_string(),
        common_models::TopicConfig {
            name: "Blog".to_string(),
            description: "Blog posts".to_string(),
            directory: "blog".to_string(),
        },
    );
    config.content.topics = topics;

    // Save the config to fixture path
    let config_str = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, &config_str)?;

    // Set the config path environment variable
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_NONEXISTENT_TOPIC_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Create options with nonexistent topic
    let options = OptimizeOptions {
        source: PathBuf::from("test-image.jpg"),
        article: Some("test-article".to_string()),
        topic: Some("nonexistent-topic".to_string()),
        ..OptimizeOptions::default()
    };

    // Act
    let result = get_article_directory(&options);

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Topic not found"));

    Ok(())
}