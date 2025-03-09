//! # Common Configuration Operations
//! 
//! This module provides common configuration operations for the writing tools.
//! 
//! ## Features
//! 
//! - Configuration loading from files
//! - Topic management
//! - Configuration validation
//! 
//! ## Example
//! 
//! ```rust
//! use common_config::{load_config, get_topics, validate_topic};
//! 
//! fn list_topics() -> common_errors::Result<()> {
//!     let topics = get_topics()?;
//!     
//!     for topic in topics {
//!         println!("Topic: {} - {}", topic.name, topic.description);
//!     }
//!     
//!     Ok(())
//! }
//! 
//! fn check_topic(key: &str) -> common_errors::Result<()> {
//!     let topic = validate_topic(key)?;
//!     println!("Topic '{}' is valid: {}", key, topic.name);
//!     Ok(())
//! }
//! ```

use common_errors::{Result, WritingError, ResultExt};
use common_models::{Config, TopicConfig};
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Load the configuration file from the current or parent directories
pub fn load_config() -> Result<Config> {
    // Try to find config.yaml in the current directory or parent directories
    let mut current_dir = std::env::current_dir()
        .with_context(|| "Failed to get current directory")?;
    let config_filename = "config.yaml";
    let mut config_path = current_dir.join(config_filename);
    
    // Keep going up the directory tree until we find config.yaml or reach the root
    while !config_path.exists() {
        if !current_dir.pop() {
            // We've reached the root directory and still haven't found config.yaml
            return Err(WritingError::config_error(
                "Could not find config.yaml in the current directory or any parent directory"
            ));
        }
        config_path = current_dir.join(config_filename);
    }
    
    load_config_from_path(&config_path)
}

/// Load configuration from a specific path
pub fn load_config_from_path(path: &Path) -> Result<Config> {
    let config_content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;
    
    let config: Config = serde_yaml::from_str(&config_content)
        .with_context(|| "Failed to parse config file")?;
    
    Ok(config)
}

/// Get all topics from the configuration
pub fn get_topics() -> Result<Vec<TopicConfig>> {
    let config = load_config()?;
    let topics = config.content.topics.values().cloned().collect();
    Ok(topics)
}

/// Get all topic keys from the configuration
pub fn get_topic_keys() -> Result<Vec<String>> {
    let config = load_config()?;
    let topic_keys = config.content.topics.keys().cloned().collect();
    Ok(topic_keys)
}

/// Get a specific topic by key
pub fn get_topic_by_key(key: &str) -> Result<Option<TopicConfig>> {
    let config = load_config()?;
    let topic = config.content.topics.get(key).cloned();
    Ok(topic)
}

/// Get the base directory for content
pub fn get_content_base_dir() -> Result<String> {
    let config = load_config()?;
    Ok(config.content.base_dir)
}

/// Get the site URL from the configuration
pub fn get_site_url() -> Result<Option<String>> {
    let config = load_config()?;
    Ok(config.publication.site.clone())
}

/// Validate that a topic exists in the configuration
pub fn validate_topic(topic_key: &str) -> Result<TopicConfig> {
    let config = load_config()?;
    
    config.content.topics.get(topic_key)
        .cloned()
        .ok_or_else(|| WritingError::topic_error(format!("Topic not found: {}", topic_key)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use common_errors::WritingError;
    use tempfile::{tempdir, NamedTempFile};

    #[test]
    fn test_load_config_from_path_success() {
        // Create a temporary file with valid config
        let mut config_content = String::new();
        config_content.push_str("content:\n");
        config_content.push_str("  base_dir: ./content\n");
        config_content.push_str("  topics:\n");
        config_content.push_str("    blog:\n");
        config_content.push_str("      name: Blog\n");
        config_content.push_str("      description: Blog posts\n");
        config_content.push_str("      path: blog\n");
        config_content.push_str("images:\n");
        config_content.push_str("  formats: [webp, jpg]\n");
        config_content.push_str("  sizes:\n");
        config_content.push_str("    small:\n");
        config_content.push_str("      width: 480\n");
        config_content.push_str("      height: 320\n");
        config_content.push_str("      description: Small image\n");
        config_content.push_str("publication:\n");
        config_content.push_str("  author: Test Author\n");
        config_content.push_str("  copyright: Test Copyright\n");
        config_content.push_str("  site: https://example.com\n");

        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), config_content).unwrap();

        // Test loading config
        let config = load_config_from_path(temp_file.path()).unwrap();
        assert_eq!(config.content.base_dir, "./content");
        assert_eq!(config.publication.author, "Test Author");
        assert!(config.content.topics.contains_key("blog"));
    }

    #[test]
    fn test_load_config_from_path_invalid_yaml() {
        // Create a temporary file with invalid YAML
        let config_content = "invalid yaml content";
        let temp_file = NamedTempFile::new().unwrap();
        fs::write(temp_file.path(), config_content).unwrap();

        // Test loading invalid config
        let result = load_config_from_path(temp_file.path());
        assert!(result.is_err());
        
        // Check error message contains expected content
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("parse config file"), "Error message doesn't mention parsing: {}", err_msg);
        assert!(err_msg.contains("invalid"), "Error message doesn't mention invalid content: {}", err_msg);
    }

    #[test]
    fn test_load_config_from_path_missing_file() {
        // Test loading non-existent file
        let result = load_config_from_path(Path::new("/path/to/nonexistent/config.yaml"));
        assert!(result.is_err());
        
        // Check error message contains expected content
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("No such file"), "Error message doesn't mention file not found: {}", err_msg);
    }

    #[test]
    fn test_validate_topic() {
        // Create a temporary directory and config
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("config.yaml");
        
        // Create config with one topic
        let mut config_content = String::new();
        config_content.push_str("content:\n");
        config_content.push_str("  base_dir: ./content\n");
        config_content.push_str("  topics:\n");
        config_content.push_str("    blog:\n");
        config_content.push_str("      name: Blog\n");
        config_content.push_str("      description: Blog posts\n");
        config_content.push_str("      path: blog\n");
        config_content.push_str("images:\n");
        config_content.push_str("  formats: [webp, jpg]\n");
        config_content.push_str("  sizes:\n");
        config_content.push_str("    small:\n");
        config_content.push_str("      width: 480\n");
        config_content.push_str("      height: 320\n");
        config_content.push_str("      description: Small image\n");
        config_content.push_str("publication:\n");
        config_content.push_str("  author: Test Author\n");
        config_content.push_str("  copyright: Test Copyright\n");
        
        fs::write(&config_path, config_content).unwrap();
        
        // Set current directory to temp dir to simulate running from there
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&temp_dir).unwrap();
        
        // Test valid topic
        let result = validate_topic("blog");
        assert!(result.is_ok());
        
        // Test invalid topic
        let result = validate_topic("nonexistent");
        assert!(result.is_err());
        match result {
            Err(WritingError::TopicError(_)) => {}, // Expected
            _ => panic!("Expected TopicError"),
        }
        
        // Restore original directory
        env::set_current_dir(original_dir).unwrap();
    }
} 