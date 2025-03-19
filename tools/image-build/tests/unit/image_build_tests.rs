use anyhow::Result;
use common_test_utils::TestFixture;
use image_build::{BuildImagesOptions, generate_image_filename, find_topic_for_article, get_article_dir};
use std::path::{Path, PathBuf};
use serial_test::serial;

#[test]
fn test_build_images_options_default() {
    let options = BuildImagesOptions::default();

    assert_eq!(options.output_dir, PathBuf::from("build/images"));
    assert_eq!(options.source_dir, PathBuf::from("content"));
    assert!(options.topic.is_none());
    assert!(options.article.is_none());
    assert!(!options.force_rebuild);
}

#[test]
#[serial]
fn test_generate_image_filename_with_default_pattern() -> Result<()> {
    // Arrange - Create a config file
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

    // Create a basic config
    let mut config = common_models::Config::default();
    config.content.base_dir = "content".to_string();

    // Create default pattern (not setting creates default)

    // Generate filename
    let result = generate_image_filename(
        &config,
        "test-article",
        "header",
        800,
        600,
        "jpg"
    );

    // Assert
    assert_eq!(result, "test-article-header.jpg");

    Ok(())
}

#[test]
#[serial]
fn test_generate_image_filename_with_custom_pattern() -> Result<()> {
    // Arrange - Create a config file
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

    // Create a basic config with custom naming pattern
    let mut config = common_models::Config::default();
    config.content.base_dir = "content".to_string();

    // Set custom naming pattern
    let naming = common_models::ImageNaming {
        pattern: "{slug}/{type}_{width}x{height}.{format}".to_string(),
        examples: vec![]
    };
    config.images.naming = Some(naming);

    // Generate filename
    let result = generate_image_filename(
        &config,
        "test-article",
        "header",
        800,
        600,
        "jpg"
    );

    // Assert
    assert_eq!(result, "test-article/header_800x600.jpg");

    Ok(())
}

#[test]
#[serial]
fn test_find_topic_for_article() -> Result<()> {
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
    topics.insert(
        "tutorials".to_string(),
        common_models::TopicConfig {
            name: "Tutorials".to_string(),
            description: "Tutorial articles".to_string(),
            directory: "tutorials".to_string(),
        },
    );
    config.content.topics = topics;

    // Save the config to fixture path
    let config_str = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, &config_str)?;

    // Create content directory with article in the blog topic
    let content_dir = fixture_path.join("content");
    std::fs::create_dir_all(&content_dir)?;
    let blog_dir = content_dir.join("blog");
    std::fs::create_dir_all(&blog_dir)?;
    let article_dir = blog_dir.join("test-article");
    std::fs::create_dir_all(&article_dir)?;

    // Set the config path environment variable
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_FIND_TOPIC_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Act
    let result = find_topic_for_article(&config, "test-article");

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "blog");

    Ok(())
}

#[test]
#[serial]
fn test_find_topic_for_nonexistent_article() -> Result<()> {
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

    // Create content directory but no article
    let content_dir = fixture_path.join("content");
    std::fs::create_dir_all(&content_dir)?;
    let blog_dir = content_dir.join("blog");
    std::fs::create_dir_all(&blog_dir)?;

    // Set the config path environment variable
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_FIND_NONEXISTENT_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Act
    let result = find_topic_for_article(&config, "nonexistent-article");

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Article not found"));

    Ok(())
}

#[test]
#[serial]
fn test_get_article_dir_with_valid_topic() -> Result<()> {
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
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_GET_ARTICLE_DIR_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Act
    let result = get_article_dir(&config, "test-article", "blog");

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_ok());
    let path = result.unwrap();
    assert_eq!(path.file_name().unwrap().to_string_lossy(), "test-article");
    assert!(path.to_string_lossy().contains("content/blog/test-article"));

    Ok(())
}

#[test]
#[serial]
fn test_get_article_dir_with_invalid_topic() -> Result<()> {
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
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_INVALID_TOPIC_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Act
    let result = get_article_dir(&config, "test-article", "nonexistent-topic");

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Invalid topic"));

    Ok(())
}