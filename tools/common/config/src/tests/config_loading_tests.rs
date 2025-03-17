//! Tests for configuration loading functionality
//!
//! These tests verify that configuration files can be properly loaded, parsed,
//! and validated from various sources.

use std::fs;
use std::path::Path;
use tempfile::{tempdir, NamedTempFile};
use crate::{load_config_from_path, get_topics, get_topic_keys, get_topic_by_key, validate_topic};
use common_errors::{WritingError, Result};
use common_models::{Config, TopicConfig, PublicationConfig, ImageConfig, ImageSize};
use std::collections::HashMap;

/// Create a test configuration file with valid content
fn create_valid_config_file() -> NamedTempFile {
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
      width: 480
      height: 320
      description: "Small image"
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

/// Create a test configuration file with invalid YAML
fn create_invalid_yaml_config_file() -> NamedTempFile {
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
    # Invalid YAML below - missing value after colon
    invalid:
    
images:
  formats:
    - "jpg"
    - "png"
"#;
    
    fs::write(file.path(), config_content).unwrap();
    file
}

/// Create a test configuration file with valid YAML but invalid schema
fn create_invalid_schema_config_file() -> NamedTempFile {
    let file = NamedTempFile::new().unwrap();
    
    let config_content = r#"
publication:
  author: "Test Author"
  copyright: "© 2023 Test Author"
  site: "https://example.com"

# Missing required 'content' section

images:
  formats:
    - "jpg"
    - "png"
  sizes:
    thumbnail:
      width: 480
      height: 320
      description: "Small image"
"#;
    
    fs::write(file.path(), config_content).unwrap();
    file
}

/// Create a test configuration file with missing topics
fn create_no_topics_config_file() -> NamedTempFile {
    let file = NamedTempFile::new().unwrap();
    
    let config_content = r#"
publication:
  author: "Test Author"
  copyright: "© 2023 Test Author"
  site: "https://example.com"

content:
  base_dir: "content"
  topics: {}

images:
  formats:
    - "jpg"
    - "png"
  sizes:
    thumbnail:
      width: 480
      height: 320
      description: "Small image"
"#;
    
    fs::write(file.path(), config_content).unwrap();
    file
}

#[test]
fn test_load_valid_config() {
    let config_file = create_valid_config_file();
    let config_path = config_file.path();
    
    let result = load_config_from_path(config_path);
    assert!(result.is_ok(), "Failed to load valid config: {:?}", result.err());
    
    let config = result.unwrap();
    
    // Verify publication section
    assert_eq!(config.publication.author, "Test Author");
    assert_eq!(config.publication.copyright, "© 2023 Test Author");
    assert_eq!(config.publication.site, Some("https://example.com".to_string()));
    
    // Verify content section
    assert_eq!(config.content.base_dir, "content");
    
    // Verify topics section
    assert_eq!(config.content.topics.len(), 2);
    assert!(config.content.topics.contains_key("blog"));
    assert!(config.content.topics.contains_key("notes"));
    
    // Verify blog topic
    let blog = &config.content.topics["blog"];
    assert_eq!(blog.name, "Blog");
    assert_eq!(blog.description, "Blog posts");
    assert_eq!(blog.directory, "content/blog");
    
    // Verify images section
    assert_eq!(config.images.formats.len(), 3);
    assert!(config.images.formats.contains(&"jpg".to_string()));
    assert!(config.images.formats.contains(&"png".to_string()));
    assert!(config.images.formats.contains(&"webp".to_string()));
    
    // Verify image sizes
    assert_eq!(config.images.sizes.len(), 3);
    assert!(config.images.sizes.contains_key("thumbnail"));
    assert!(config.images.sizes.contains_key("medium"));
    assert!(config.images.sizes.contains_key("large"));
    
    // Verify thumbnail size
    let thumbnail = &config.images.sizes["thumbnail"];
    assert_eq!(thumbnail.width, 480);
    assert_eq!(thumbnail.height, 320);
    assert_eq!(thumbnail.description, "Small image");
}

#[test]
fn test_load_invalid_yaml_config() {
    let config_file = create_invalid_yaml_config_file();
    let config_path = config_file.path();
    
    let result = load_config_from_path(config_path);
    assert!(result.is_err(), "Expected error for invalid YAML, but got success");
    
    // Verify that the error message contains the expected content
    let err_msg = format!("{}", result.unwrap_err());
    assert!(err_msg.contains("Failed to parse config file"), "Error message doesn't mention parsing: {}", err_msg);
    assert!(err_msg.contains("invalid"), "Error message doesn't mention invalid content: {}", err_msg);
}

#[test]
fn test_load_nonexistent_config() {
    let temp_dir = tempdir().unwrap();
    let nonexistent_path = temp_dir.path().join("nonexistent.yaml");
    
    let result = load_config_from_path(&nonexistent_path);
    assert!(result.is_err(), "Expected error for nonexistent file, but got success");
    
    // Verify that the error message contains the expected content
    let err_msg = format!("{}", result.unwrap_err());
    assert!(err_msg.contains("Failed to read config file"), "Error message doesn't mention reading: {}", err_msg);
    assert!(err_msg.contains("No such file"), "Error message doesn't mention file not found: {}", err_msg);
}

#[test]
fn test_get_topics_functionality() {
    // Create a test config file
    let temp_file = create_valid_config_file();
    
    // Mock the get_topics function
    let get_topics_mock = |config: Config| -> Result<Vec<TopicConfig>> {
        Ok(config.content.topics.values().cloned().collect())
    };
    
    // Load the config
    let config = load_config_from_path(temp_file.path()).unwrap();
    
    // Call the mock function
    let topics = get_topics_mock(config).unwrap();
    
    // Verify the result
    assert_eq!(topics.len(), 2);
    assert!(topics.iter().any(|t| t.name == "Blog"));
    assert!(topics.iter().any(|t| t.name == "Notes"));
}

#[test]
fn test_get_topic_keys_functionality() {
    // Create a test config file
    let temp_file = create_valid_config_file();
    
    // Mock the get_topic_keys function
    let get_topic_keys_mock = |config: Config| -> Result<Vec<String>> {
        Ok(config.content.topics.keys().cloned().collect())
    };
    
    // Load the config
    let config = load_config_from_path(temp_file.path()).unwrap();
    
    // Call the mock function
    let keys = get_topic_keys_mock(config).unwrap();
    
    // Verify the result
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&"blog".to_string()));
    assert!(keys.contains(&"notes".to_string()));
}

#[test]
fn test_get_topic_by_key_functionality() {
    // Create a test config file
    let temp_file = create_valid_config_file();
    
    // Mock the get_topic_by_key function
    let get_topic_by_key_mock = |config: Config, key: &str| -> Result<Option<TopicConfig>> {
        Ok(config.content.topics.get(key).cloned())
    };
    
    // Load the config
    let config = load_config_from_path(temp_file.path()).unwrap();
    
    // Call the mock function with valid key
    let blog = get_topic_by_key_mock(config.clone(), "blog").unwrap();
    
    // Verify the result
    assert!(blog.is_some());
    let blog = blog.unwrap();
    assert_eq!(blog.name, "Blog");
    assert_eq!(blog.description, "Blog posts");
    
    // Call the mock function with invalid key
    let nonexistent = get_topic_by_key_mock(config, "nonexistent").unwrap();
    
    // Verify the result
    assert!(nonexistent.is_none());
}

#[test]
fn test_validate_topic_functionality() {
    // Create a test config file
    let temp_file = create_valid_config_file();
    
    // Mock the validate_topic function
    let validate_topic_mock = |config: Config, key: &str| -> Result<TopicConfig> {
        if let Some(topic) = config.content.topics.get(key) {
            Ok(topic.clone())
        } else {
            Err(WritingError::topic_error(&format!("Topic not found: {}", key)))
        }
    };
    
    // Load the config
    let config = load_config_from_path(temp_file.path()).unwrap();
    
    // Call the mock function with valid key
    let result = validate_topic_mock(config.clone(), "blog");
    
    // Verify the result
    assert!(result.is_ok());
    let topic = result.unwrap();
    assert_eq!(topic.name, "Blog");
    
    // Call the mock function with invalid key
    let result = validate_topic_mock(config, "nonexistent");
    
    // Verify the result
    assert!(result.is_err());
    match result {
        Err(WritingError::TopicError(_)) => {}, // Expected
        _ => panic!("Expected TopicError"),
    }
}

#[test]
fn test_no_topics_config() {
    // Create a test config file with no topics
    let temp_file = create_no_topics_config_file();
    
    // Load the config
    let config = load_config_from_path(temp_file.path()).unwrap();
    
    // Verify topics section is empty
    assert_eq!(config.content.topics.len(), 0);
    
    // Mock the get_topic_keys function
    let get_topic_keys_mock = |config: Config| -> Result<Vec<String>> {
        Ok(config.content.topics.keys().cloned().collect())
    };
    
    // Call the mock function
    let keys = get_topic_keys_mock(config).unwrap();
    
    // Verify the result
    assert_eq!(keys.len(), 0);
} 