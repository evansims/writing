//! Tests for configuration views functionality
//!
//! These tests verify that configuration views provide proper abstraction
//! and access to configuration data.

use std::fs;
use std::path::Path;
use tempfile::NamedTempFile;
use crate::views::{ConfigView, ContentView, ImageView, PublicationView};
use common_errors::WritingError;
use common_models::{Config, ContentConfig, TopicConfig, PublicationConfig, ImageConfig, ImageSize, ImageNaming};
use std::collections::HashMap;

/// Create a test configuration file with comprehensive content
fn create_test_config_file() -> NamedTempFile {
    let file = NamedTempFile::new().unwrap();

    let config_content = r#"
publication:
  author: "Test Author"
  copyright: "© 2023 Test Author"
  site: "https://example.com"

content:
  base_dir: "content"
  topics:
    blog:
      name: "Blog"
      description: "Blog posts"
      path: "content/blog"
    notes:
      name: "Notes"
      description: "Quick notes and thoughts"
      path: "content/notes"

images:
  formats:
    - "jpg"
    - "png"
    - "webp"
  sizes:
    thumbnail:
      width: 200
      height: 200
      description: "Thumbnail image"
    medium:
      width: 800
      height: 600
      description: "Medium image"
    large:
      width: 1200
      height: 900
      description: "Large image"
"#;

    fs::write(file.path(), config_content).unwrap();
    file
}

/// Create a test configuration object in memory
fn create_test_config() -> Config {
    // Create the publication configuration
    let publication = PublicationConfig {
        author: "Test Author".to_string(),
        copyright: "© 2023 Test Author".to_string(),
        site: Some("https://example.com".to_string()),
    };

    // Create topic configurations
    let mut topics = HashMap::new();

    let blog = TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "content/blog".to_string(),
    };

    let notes = TopicConfig {
        name: "Notes".to_string(),
        description: "Quick notes and thoughts".to_string(),
        directory: "content/notes".to_string(),
    };

    topics.insert("blog".to_string(), blog);
    topics.insert("notes".to_string(), notes);

    // Create tags
    let tags = HashMap::new();

    // Create the content configuration
    let content = ContentConfig {
        base_dir: "content".to_string(),
        topics,
        tags: Some(tags),
    };

    // Create image configurations
    let mut formats = Vec::new();
    formats.push("jpg".to_string());
    formats.push("png".to_string());
    formats.push("webp".to_string());

    let mut sizes = HashMap::new();

    let thumbnail = ImageSize {
        width: 200,
        height: 200,
        description: "Thumbnail image".to_string(),
    };

    let medium = ImageSize {
        width: 800,
        height: 600,
        description: "Medium image".to_string(),
    };

    let large = ImageSize {
        width: 1200,
        height: 900,
        description: "Large image".to_string(),
    };

    sizes.insert("thumbnail".to_string(), thumbnail);
    sizes.insert("medium".to_string(), medium);
    sizes.insert("large".to_string(), large);

    // Create format descriptions
    let mut format_descriptions = HashMap::new();
    format_descriptions.insert("jpg".to_string(), "JPEG image format".to_string());
    format_descriptions.insert("png".to_string(), "PNG image format".to_string());
    format_descriptions.insert("webp".to_string(), "WebP image format".to_string());

    // Create image naming
    let naming = ImageNaming {
        pattern: "{slug}-{size}.{format}".to_string(),
        examples: vec!["post-small.jpg".to_string()],
    };

    let images = ImageConfig {
        formats,
        sizes,
        format_descriptions: Some(format_descriptions),
        naming: Some(naming),
        quality: Some(HashMap::new()),
    };

    // Create the full configuration
    Config {
        publication,
        content,
        images,
    }
}

#[test]
fn test_content_view_from_config() {
    // Create a test configuration
    let config = create_test_config();

    // Create a content view from the configuration
    let view = ContentView::from_config(config);

    // Test the ConfigView trait
    assert_eq!(view.config().publication.author, "Test Author");

    // Test ContentView specific methods
    assert_eq!(view.base_dir(), "content");
    assert_eq!(view.topics().len(), 2);

    let topic_keys = view.topic_keys();
    assert_eq!(topic_keys.len(), 2);
    assert!(topic_keys.contains(&"blog".to_string()));
    assert!(topic_keys.contains(&"notes".to_string()));

    // Test topic retrieval
    let blog = view.topic("blog").unwrap();
    assert_eq!(blog.name, "Blog");
    assert_eq!(blog.description, "Blog posts");
    assert_eq!(blog.directory, "content/blog");

    // Test nonexistent topic
    let nonexistent = view.topic("nonexistent");
    assert!(nonexistent.is_none());

    // Test topic path retrieval
    let blog_path = view.get_topic_path("blog").unwrap();
    assert_eq!(blog_path, "content/blog");

    // Test nonexistent topic path
    let nonexistent_path = view.get_topic_path("nonexistent");
    assert!(nonexistent_path.is_none());

    // Test topic validation
    let result = view.validate_topic("blog");
    assert!(result.is_ok());

    let result = view.validate_topic("nonexistent");
    assert!(result.is_err());
    match result.unwrap_err() {
        WritingError::TopicError(_) => (), // Expected
        other => panic!("Expected TopicError, got {:?}", other),
    }
}

#[test]
fn test_content_view_from_path() {
    // Create a test configuration file
    let config_file = create_test_config_file();
    let config_path = config_file.path();

    // Create a content view from the file path
    let result = ContentView::from_path(config_path);
    assert!(result.is_ok(), "Failed to create ContentView from path: {:?}", result.err());

    let view = result.unwrap();

    // Test the view
    assert_eq!(view.base_dir(), "content");
    assert_eq!(view.topics().len(), 2);

    let topic_keys = view.topic_keys();
    assert_eq!(topic_keys.len(), 2);
    assert!(topic_keys.contains(&"blog".to_string()));
    assert!(topic_keys.contains(&"notes".to_string()));
}

#[test]
fn test_image_view_from_config() {
    // Create a test configuration
    let config = create_test_config();

    // Create an image view from the configuration
    let view = ImageView::from_config(config);

    // Test the ConfigView trait
    assert_eq!(view.config().publication.author, "Test Author");

    // Test ImageView specific methods
    let formats = view.formats();
    assert_eq!(formats.len(), 3);
    assert!(formats.contains(&"jpg".to_string()));
    assert!(formats.contains(&"png".to_string()));
    assert!(formats.contains(&"webp".to_string()));

    // Test size retrieval
    let sizes = view.sizes();
    assert_eq!(sizes.len(), 3);

    let size_keys = view.size_keys();
    assert_eq!(size_keys.len(), 3);
    assert!(size_keys.contains(&"thumbnail".to_string()));
    assert!(size_keys.contains(&"medium".to_string()));
    assert!(size_keys.contains(&"large".to_string()));

    // Test specific size retrieval
    let thumbnail = view.size("thumbnail").unwrap();
    assert_eq!(thumbnail.width, 200);
    assert_eq!(thumbnail.height, 200);
    assert_eq!(thumbnail.description, "Thumbnail image");

    // Test nonexistent size
    let nonexistent = view.size("nonexistent");
    assert!(nonexistent.is_none());

    // Test size validation
    let result = view.validate_size("thumbnail");
    assert!(result.is_ok());

    let result = view.validate_size("nonexistent");
    assert!(result.is_err());
}

#[test]
fn test_publication_view_from_config() {
    // Create a test configuration
    let config = create_test_config();

    // Create a publication view from the configuration
    let view = PublicationView::from_config(config);

    // Test the ConfigView trait
    assert_eq!(view.config().publication.author, "Test Author");

    // Test PublicationView specific methods
    assert_eq!(view.author(), "Test Author");
    assert_eq!(view.copyright(), "© 2023 Test Author");
    assert_eq!(view.site(), Some("https://example.com"));
}

#[test]
fn test_publication_view_from_path() {
    // Create a test configuration file
    let config_file = create_test_config_file();
    let config_path = config_file.path();

    // Create a publication view from the file path
    let result = PublicationView::from_path(config_path);
    assert!(result.is_ok(), "Failed to create PublicationView from path: {:?}", result.err());

    let view = result.unwrap();

    // Test the view
    assert_eq!(view.author(), "Test Author");
    assert_eq!(view.copyright(), "© 2023 Test Author");
    assert_eq!(view.site(), Some("https://example.com"));
}

#[test]
fn test_image_view_from_path() {
    // Create a test configuration file
    let config_file = create_test_config_file();
    let config_path = config_file.path();

    // Create an image view from the file path
    let result = ImageView::from_path(config_path);
    assert!(result.is_ok(), "Failed to create ImageView from path: {:?}", result.err());

    let view = result.unwrap();

    // Test the view
    let formats = view.formats();
    assert_eq!(formats.len(), 3);
    assert!(formats.contains(&"jpg".to_string()));

    // Test size retrieval
    let thumbnail = view.size("thumbnail").unwrap();
    assert_eq!(thumbnail.width, 200);
    assert_eq!(thumbnail.height, 200);
    assert_eq!(thumbnail.description, "Thumbnail image");
}

#[test]
fn test_content_view_base_dir_path() {
    // Create a test configuration
    let config = create_test_config();

    // Create a content view from the configuration
    let view = ContentView::from_config(config);

    // Test base_dir_path
    let path = view.base_dir_path();
    assert_eq!(path.to_str().unwrap(), "content");
}

#[test]
fn test_content_view_get_topic_absolute_path() {
    // Create a test configuration
    let mut config = create_test_config();

    // Ensure we're using absolute paths
    config.content.base_dir = "/content".to_string();
    config.content.topics.get_mut("blog").unwrap().directory = "/content/blog".to_string();

    // Create a content view from the configuration
    let view = ContentView::from_config(config);

    // Test get_topic_absolute_path
    let path = view.get_topic_absolute_path("blog").unwrap();
    assert_eq!(path.to_str().unwrap(), "/content/blog");

    // Test nonexistent topic
    let path = view.get_topic_absolute_path("nonexistent");
    assert!(path.is_none());
}

#[test]
fn test_content_view_base_dir_path_modified() {
    // Modify the config to test with a different base directory
    let mut config = create_test_config();
    config.content.base_dir = "/content".to_string();
    config.content.topics.get_mut("blog").unwrap().directory = "/content/blog".to_string();

    // Create a content view from the modified configuration
    let view = ContentView::from_config(config);

    // Test base_dir_path
    let path = view.base_dir_path();
    assert_eq!(path.to_str().unwrap(), "/content");
}

#[test]
fn test_publication_view() {
    let config_file = create_test_config();
    let view = PublicationView::from_path(config_file.path()).unwrap();

    assert_eq!(view.author(), "Test Author");
    assert_eq!(view.copyright(), "© 2023");
    assert_eq!(view.site_url(), Some("https://example.com"));
}

#[test]
fn test_image_view_sizes() {
    let config_file = create_test_config();
    let view = ImageView::from_path(config_file.path()).unwrap();

    let formats = view.formats();
    assert_eq!(formats.len(), 3);
    assert!(formats.contains(&"jpg".to_string()));
    assert!(formats.contains(&"png".to_string()));
    assert!(formats.contains(&"webp".to_string()));

    let sizes = view.sizes();
    assert_eq!(sizes.len(), 3);

    let size_keys = view.size_keys();
    assert_eq!(size_keys.len(), 3);
    assert!(size_keys.contains(&"thumbnail".to_string()));
    assert!(size_keys.contains(&"medium".to_string()));
    assert!(size_keys.contains(&"large".to_string()));

    let thumbnail = view.size("thumbnail").unwrap();
    assert_eq!(thumbnail.width_px, 200);
    assert_eq!(thumbnail.height_px, 200);
    assert_eq!(thumbnail.description, "Thumbnail size");

    let medium = view.size("medium").unwrap();
    assert_eq!(medium.width_px, 800);
    assert_eq!(medium.height_px, 600);
    assert_eq!(medium.description, "Medium image");

    let large = view.size("large").unwrap();
    assert_eq!(large.width_px, 1200);
    assert_eq!(large.height_px, 900);
    assert_eq!(large.description, "Large image");

    let nonexistent = view.size("nonexistent");
    assert!(nonexistent.is_none());

    let result = view.validate_size("thumbnail");
    assert!(result.is_ok());

    let result = view.validate_size("nonexistent");
    assert!(result.is_err());
}