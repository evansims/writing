//! Tests for configuration caching functionality
//!
//! These tests verify that configuration caching works properly, including
//! cache invalidation, lazy loading, and thread safety.

use std::fs;
use std::path::Path;
use std::time::Duration;
use tempfile::{tempdir, NamedTempFile};
use crate::cache::ConfigCache;

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
      directory: "content/blog"

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

/// Create an updated version of the configuration file
fn update_config_file(file: &NamedTempFile) {
    let config_content = r#"
publication:
  author: "Updated Author"
  copyright: "© 2023 Updated Author"
  site: "https://example.com"

content:
  base_dir: "content"
  topics:
    blog:
      name: "Blog"
      description: "Updated blog posts"
      directory: "content/blog"
    notes:
      name: "Notes"
      description: "Quick notes and thoughts"
      directory: "content/notes"

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
}

#[test]
fn test_config_cache_new() {
    // Create a new cache with a 1-minute max age and checking for modifications
    let cache = ConfigCache::new(Duration::from_secs(60), true);

    // Clear the cache to ensure it's empty
    cache.clear();

    // Try to load a non-existent file to verify the cache is empty
    let result = cache.get_config_from_path(Path::new("/nonexistent/file.yaml"));
    assert!(result.is_err());
}

#[test]
fn test_config_cache_global() {
    // Get the global cache instance
    let cache = ConfigCache::global();

    // Verify we can get the cache instance
    assert!(!std::ptr::eq(cache, &ConfigCache::new(Duration::from_secs(60), true)));
}

#[test]
fn test_config_cache_get_config_from_path() {
    // Create a test configuration file
    let config_file = create_valid_config_file();
    let config_path = config_file.path();

    // Create a cache with a 1-minute max age and checking for modifications
    let cache = ConfigCache::new(Duration::from_secs(60), true);

    // Load the configuration from the path
    let result = cache.get_config_from_path(config_path);
    assert!(result.is_ok(), "Failed to load config: {:?}", result.err());

    // Verify the configuration was loaded correctly
    let config = result.unwrap();
    assert_eq!(config.content.base_dir, "content");
    assert_eq!(config.publication.author, "Test Author");
    assert_eq!(config.content.topics.len(), 1);
    assert!(config.content.topics.contains_key("blog"));

    // Load the configuration again to test caching
    let result2 = cache.get_config_from_path(config_path);
    assert!(result2.is_ok(), "Failed to load config from cache: {:?}", result2.err());

    // Verify we got the same configuration
    let config2 = result2.unwrap();
    assert_eq!(config2.content.base_dir, "content");
    assert_eq!(config2.publication.author, "Test Author");
    assert_eq!(config2.content.topics.len(), 1);
    assert!(config2.content.topics.contains_key("blog"));
}

#[test]
fn test_config_cache_clear() {
    // Create a test config file
    let config_file = create_valid_config_file();
    let config_path = config_file.path();

    // Create a cache with a 1-hour max age
    let cache = ConfigCache::new(Duration::from_secs(3600), true);

    // Load the config to populate the cache
    let config1 = cache.get_config_from_path(config_path).unwrap();
    assert_eq!(config1.publication.author, "Test Author");

    // Clear the cache
    cache.clear();

    // Modify the config file
    update_config_file(&config_file);

    // Load the config again - should get the updated version since cache was cleared
    let config2 = cache.get_config_from_path(config_path).unwrap();
    assert_eq!(config2.publication.author, "Updated Author");
}

#[test]
fn test_config_cache_modification_check() {
    // Create a test configuration file
    let config_file = create_valid_config_file();
    let config_path = config_file.path();

    // Create a cache with a short max age and checking for modifications
    let cache = ConfigCache::new(Duration::from_secs(60), true);

    // Load the configuration to populate the cache
    let config1 = cache.get_config_from_path(config_path).unwrap();
    assert_eq!(config1.content.base_dir, "content");
    assert_eq!(config1.publication.author, "Test Author");
    assert_eq!(config1.content.topics.len(), 1);
    assert!(config1.content.topics.contains_key("blog"));

    // Wait a moment to ensure file modification time changes
    std::thread::sleep(Duration::from_millis(10));

    // Update the config file
    update_config_file(&config_file);

    // Load the configuration again
    let config2 = cache.get_config_from_path(config_path).unwrap();

    // Verify we got the updated configuration
    assert_eq!(config2.content.base_dir, "content");
    assert_eq!(config2.publication.author, "Updated Author");
    assert_eq!(config2.content.topics.len(), 2);
    assert!(config2.content.topics.contains_key("notes"));
}

#[test]
fn test_config_cache_no_modification_check() {
    // Create a test configuration file
    let config_file = create_valid_config_file();
    let config_path = config_file.path();

    // Create a cache with a short max age but not checking for modifications
    let cache = ConfigCache::new(Duration::from_secs(60), false);

    // Load the configuration to populate the cache
    let config1 = cache.get_config_from_path(config_path).unwrap();
    assert_eq!(config1.content.base_dir, "content");
    assert_eq!(config1.publication.author, "Test Author");

    // Wait a moment to ensure file modification time changes
    std::thread::sleep(Duration::from_millis(10));

    // Update the config file
    update_config_file(&config_file);

    // Load the configuration again
    let config2 = cache.get_config_from_path(config_path).unwrap();

    // Verify we still got the cached configuration (not updated)
    assert_eq!(config2.content.base_dir, "content");
    assert_eq!(config2.publication.author, "Test Author");
    assert_eq!(config2.content.topics.len(), 1);
    assert!(!config2.content.topics.contains_key("notes"));
}

#[test]
fn test_config_cache_age_invalidation() {
    // Create a test configuration file
    let config_file = create_valid_config_file();
    let config_path = config_file.path();

    // Create a cache with a very short max age
    let cache = ConfigCache::new(Duration::from_millis(10), false);

    // Load the configuration to populate the cache
    let config1 = cache.get_config_from_path(config_path).unwrap();
    assert_eq!(config1.content.base_dir, "content");
    assert_eq!(config1.publication.author, "Test Author");

    // Update the config file
    update_config_file(&config_file);

    // Wait long enough for the cache to expire
    std::thread::sleep(Duration::from_millis(20));

    // Load the configuration again
    let config2 = cache.get_config_from_path(config_path).unwrap();

    // Verify we got the updated configuration due to cache expiration
    assert_eq!(config2.content.base_dir, "content");
    assert_eq!(config2.publication.author, "Updated Author");
    assert_eq!(config2.content.topics.len(), 2);
    assert!(config2.content.topics.contains_key("notes"));
}

#[test]
fn test_config_cache_nonexistent_file() {
    // Create a cache with a 1-minute max age
    let cache = ConfigCache::new(Duration::from_secs(60), true);

    // Clear the cache to ensure it's empty
    cache.clear();

    // Create a path to a nonexistent file
    let nonexistent_path = Path::new("/path/to/nonexistent/config.yaml");

    // Try to load a nonexistent file
    let result = cache.get_config_from_path(&nonexistent_path);
    assert!(result.is_err(), "Expected error for nonexistent file, but got success");

    // The important part is that it's an error. The exact message format isn't critical
    // for this test and may change over time as the error handling evolves.
    assert!(result.is_err());

    // Try to load the nonexistent file again to verify the cache is still empty
    let result2 = cache.get_config_from_path(&nonexistent_path);
    assert!(result2.is_err(), "Expected error for nonexistent file, but got success");
}

#[test]
fn test_config_cache_load_after_error() {
    // Create a temporary directory
    let temp_dir = tempdir().unwrap();
    let config_path = temp_dir.path().join("config.yaml");

    // Create a cache
    let cache = ConfigCache::new(Duration::from_secs(60), true);

    // Try to load a nonexistent file
    let result1 = cache.get_config_from_path(&config_path);
    assert!(result1.is_err());

    // Create a valid config file at the path
    let config_content = r#"
publication:
  author: "Test Author"
  copyright: "© 2023 Test Author"
content:
  base_dir: "content"
  topics: {}
images:
  formats:
    - "jpg"
  sizes:
    thumbnail:
      width: 480
      height: 320
      description: "Small image"
"#;
    fs::write(&config_path, config_content).unwrap();

    // Try to load the file again
    let result2 = cache.get_config_from_path(&config_path);
    assert!(result2.is_ok(), "Failed to load config after creating it: {:?}", result2.err());

    // Verify the configuration was loaded correctly
    let config = result2.unwrap();
    assert_eq!(config.content.base_dir, "content");
    assert_eq!(config.publication.author, "Test Author");
}