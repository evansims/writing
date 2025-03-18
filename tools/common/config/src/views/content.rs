//! # Content Configuration View
//!
//! This module provides a view of the configuration specific to content management.

use common_errors::{Result, WritingError, ResultExt};
use common_models::{Config, TopicConfig};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::ConfigView;
use crate::load_config_from_path;

/// View for content-related configuration
pub struct ContentView {
    /// The underlying configuration
    config: Config,
}

impl ContentView {
    /// Create a new content view using the default configuration
    ///
    /// # Returns
    ///
    /// A new `ContentView` instance
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    pub fn new() -> Result<Self> {
        let config = crate::load_config()?;
        Ok(Self { config })
    }

    /// Create a new content view from a specific configuration path
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the configuration file
    ///
    /// # Returns
    ///
    /// A new `ContentView` instance
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    pub fn from_path(path: &Path) -> Result<Self> {
        let config = load_config_from_path(path)
            .with_context(|| format!("Failed to load config from path: {}", path.display()))?;
        Ok(Self { config })
    }

    /// Get the base directory for content
    ///
    /// # Returns
    ///
    /// The base directory as a string
    pub fn base_dir(&self) -> &str {
        &self.config.content.base_dir
    }

    /// Get the base directory for content as a path
    ///
    /// # Returns
    ///
    /// The base directory as a `PathBuf`
    pub fn base_dir_path(&self) -> PathBuf {
        PathBuf::from(&self.config.content.base_dir)
    }

    /// Get all topics from the configuration
    ///
    /// # Returns
    ///
    /// A map of topic keys to topic configurations
    pub fn topics(&self) -> &HashMap<String, TopicConfig> {
        &self.config.content.topics
    }

    /// Get all topic keys from the configuration
    ///
    /// # Returns
    ///
    /// A vector of topic keys
    pub fn topic_keys(&self) -> Vec<String> {
        self.config.content.topics.keys()
            .cloned()
            .collect()
    }

    /// Get a specific topic configuration
    ///
    /// # Arguments
    ///
    /// * `key` - The topic key
    ///
    /// # Returns
    ///
    /// The topic configuration if found, or `None` if not found
    pub fn topic(&self, key: &str) -> Option<&TopicConfig> {
        self.config.content.topics.get(key)
    }

    /// Get a specific topic configuration and validate it exists
    ///
    /// # Arguments
    ///
    /// * `key` - The topic key
    ///
    /// # Returns
    ///
    /// The topic configuration if found
    ///
    /// # Errors
    ///
    /// Returns an error if the topic is not found
    pub fn validate_topic(&self, key: &str) -> Result<&TopicConfig> {
        self.topic(key).ok_or_else(|| WritingError::topic_error(format!("Topic not found: {}", key)))
    }

    /// Get the path for a specific topic
    ///
    /// # Arguments
    ///
    /// * `key` - The topic key
    ///
    /// # Returns
    ///
    /// The topic path if found, or `None` if not found
    pub fn get_topic_path(&self, key: &str) -> Option<&str> {
        self.topic(key).map(|t| t.directory.as_str())
    }

    /// Get the absolute path for a specific topic
    ///
    /// # Arguments
    ///
    /// * `key` - The topic key
    ///
    /// # Returns
    ///
    /// The absolute topic path if found, or `None` if not found
    pub fn get_topic_absolute_path(&self, key: &str) -> Option<PathBuf> {
        self.get_topic_path(key).map(|p| {
            let mut path = self.base_dir_path();
            path.push(p);
            path
        })
    }
}

impl ConfigView for ContentView {
    fn config(&self) -> &Config {
        &self.config
    }

    fn from_config(config: Config) -> Self {
        Self { config }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_config() -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        write!(file, r#"
content:
  base_dir: "./content"
  topics:
    blog:
      directory: "blog"
      name: "Blog"
      description: "Blog Posts"
    articles:
      directory: "articles"
      name: "Articles"
      description: "Technical Articles"
images:
  formats:
    - "webp"
    - "jpg"
  sizes:
    small:
      width: 400
      height: 300
    medium:
      width: 800
      height: 600
publication:
  author: "Test Author"
  copyright: "© 2023"
  site: "https://example.com"
"#).unwrap();
        file.flush().unwrap();
        file
    }

    #[test]
    fn test_content_view() {
        let config_file = create_test_config();
        let view = ContentView::from_path(config_file.path()).unwrap();

        assert_eq!(view.base_dir(), "./content");

        let topics = view.topics();
        assert_eq!(topics.len(), 2);
        assert!(topics.contains_key("blog"));
        assert!(topics.contains_key("articles"));

        let keys = view.topic_keys();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"blog".to_string()));
        assert!(keys.contains(&"articles".to_string()));

        let blog = view.topic("blog").unwrap();
        assert_eq!(blog.name, "Blog");
        // Note: path is renamed to directory
        assert_eq!(blog.directory, "blog");

        let path = view.get_topic_path("blog").unwrap();
        assert_eq!(path, "blog");

        assert!(view.validate_topic("blog").is_ok());
        assert!(view.validate_topic("unknown").is_err());
    }
}