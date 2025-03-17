//! # Context-Specific Configuration Views
//!
//! This module provides context-specific views of the configuration for different tools.
//!
//! ## Features
//!
//! - Tool-specific configuration views
//! - Simplified access to configuration properties
//! - Cached configuration access
//!
//! ## Example
//!
//! ```rust
//! use common_config::views::{ContentView, ImageView};
//!
//! fn use_content_view() -> common_errors::Result<()> {
//!     let view = ContentView::new()?;
//!     
//!     println!("Base directory: {}", view.base_dir());
//!     println!("Topics: {:?}", view.topic_keys());
//!     
//!     Ok(())
//! }
//!
//! fn use_image_view() -> common_errors::Result<()> {
//!     let view = ImageView::new()?;
//!     
//!     println!("Formats: {:?}", view.formats());
//!     println!("Sizes: {:?}", view.size_keys());
//!     
//!     Ok(())
//! }
//! ```

use common_errors::{Result, WritingError};
use common_models::{Config, TopicConfig, ImageSize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Base trait for configuration views
pub trait ConfigView {
    /// Get the underlying configuration
    fn config(&self) -> &Config;
    
    /// Create a new view from a specific configuration
    fn from_config(config: Config) -> Self where Self: Sized;
}

/// View for content-related configuration
pub struct ContentView {
    /// The underlying configuration
    config: Config,
}

impl ContentView {
    /// Create a new content view
    ///
    /// # Returns
    ///
    /// A new content view
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ContentView;
    ///
    /// let view = ContentView::new()?;
    /// ```
    pub fn new() -> Result<Self> {
        let config = super::load_config()?;
        Ok(ContentView { config })
    }
    
    /// Create a new content view from a specific path
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the configuration file
    ///
    /// # Returns
    ///
    /// A new content view
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ContentView;
    /// use std::path::Path;
    ///
    /// let view = ContentView::from_path(Path::new("config.yaml"))?;
    /// ```
    pub fn from_path(path: &Path) -> Result<Self> {
        let config = super::load_config_from_path(path)?;
        Ok(ContentView { config })
    }
    
    /// Get the base directory for content
    ///
    /// # Returns
    ///
    /// The base directory for content
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ContentView;
    ///
    /// let view = ContentView::new()?;
    /// println!("Base directory: {}", view.base_dir());
    /// ```
    pub fn base_dir(&self) -> &str {
        &self.config.content.base_dir
    }
    
    /// Get the base directory as a path
    ///
    /// # Returns
    ///
    /// The base directory as a path
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ContentView;
    ///
    /// let view = ContentView::new()?;
    /// println!("Base directory: {}", view.base_dir_path().display());
    /// ```
    pub fn base_dir_path(&self) -> PathBuf {
        PathBuf::from(&self.config.content.base_dir)
    }
    
    /// Get all topics
    ///
    /// # Returns
    ///
    /// A reference to the topics map
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ContentView;
    ///
    /// let view = ContentView::new()?;
    /// println!("Topics: {:?}", view.topics());
    /// ```
    pub fn topics(&self) -> &HashMap<String, TopicConfig> {
        &self.config.content.topics
    }
    
    /// Get all topic keys
    ///
    /// # Returns
    ///
    /// A vector of topic keys
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ContentView;
    ///
    /// let view = ContentView::new()?;
    /// println!("Topic keys: {:?}", view.topic_keys());
    /// ```
    pub fn topic_keys(&self) -> Vec<String> {
        self.config.content.topics.keys().cloned().collect()
    }
    
    /// Get a specific topic by key
    ///
    /// # Arguments
    ///
    /// * `key` - The topic key
    ///
    /// # Returns
    ///
    /// The topic configuration, or `None` if the topic doesn't exist
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ContentView;
    ///
    /// let view = ContentView::new()?;
    /// if let Some(topic) = view.topic("blog") {
    ///     println!("Blog topic: {}", topic.name);
    /// }
    /// ```
    pub fn topic(&self, key: &str) -> Option<&TopicConfig> {
        self.config.content.topics.get(key)
    }
    
    /// Validate that a topic exists
    ///
    /// # Arguments
    ///
    /// * `key` - The topic key
    ///
    /// # Returns
    ///
    /// The topic configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the topic doesn't exist
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ContentView;
    ///
    /// let view = ContentView::new()?;
    /// let topic = view.validate_topic("blog")?;
    /// println!("Blog topic: {}", topic.name);
    /// ```
    pub fn validate_topic(&self, key: &str) -> Result<&TopicConfig> {
        self.config.content.topics.get(key)
            .ok_or_else(|| WritingError::topic_error(format!("Topic not found: {}", key)))
    }
    
    /// Get the path to a topic directory
    ///
    /// This function returns the path to a topic directory by its key.
    ///
    /// # Parameters
    ///
    /// * `key` - The key of the topic
    ///
    /// # Returns
    ///
    /// Returns the path to the topic directory if found, None otherwise
    pub fn get_topic_path(&self, key: &str) -> Option<&str> {
        self.config.content.topics.get(key).map(|t| t.directory.as_str())
    }
    
    /// Get the absolute path to a topic directory
    ///
    /// This function returns the absolute path to a topic directory by its key.
    ///
    /// # Parameters
    ///
    /// * `key` - The key of the topic
    ///
    /// # Returns
    ///
    /// Returns the absolute path to the topic directory if found, None otherwise
    pub fn get_topic_absolute_path(&self, key: &str) -> Option<PathBuf> {
        self.config.content.topics.get(key).map(|t| {
            PathBuf::from(&self.config.content.base_dir).join(&t.directory)
        })
    }
}

impl ConfigView for ContentView {
    fn config(&self) -> &Config {
        &self.config
    }
    
    fn from_config(config: Config) -> Self {
        ContentView { config }
    }
}

/// View for image-related configuration
pub struct ImageView {
    /// The underlying configuration
    config: Config,
}

impl ImageView {
    /// Create a new image view
    ///
    /// # Returns
    ///
    /// A new image view
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ImageView;
    ///
    /// let view = ImageView::new()?;
    /// ```
    pub fn new() -> Result<Self> {
        let config = super::load_config()?;
        Ok(ImageView { config })
    }
    
    /// Create a new image view from a specific path
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the configuration file
    ///
    /// # Returns
    ///
    /// A new image view
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ImageView;
    /// use std::path::Path;
    ///
    /// let view = ImageView::from_path(Path::new("config.yaml"))?;
    /// ```
    pub fn from_path(path: &Path) -> Result<Self> {
        let config = super::load_config_from_path(path)?;
        Ok(ImageView { config })
    }
    
    /// Get the supported image formats
    ///
    /// # Returns
    ///
    /// A reference to the supported image formats
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ImageView;
    ///
    /// let view = ImageView::new()?;
    /// println!("Formats: {:?}", view.formats());
    /// ```
    pub fn formats(&self) -> &Vec<String> {
        &self.config.images.formats
    }
    
    /// Get all image sizes
    ///
    /// # Returns
    ///
    /// A reference to the image sizes map
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ImageView;
    ///
    /// let view = ImageView::new()?;
    /// println!("Sizes: {:?}", view.sizes());
    /// ```
    pub fn sizes(&self) -> &HashMap<String, ImageSize> {
        &self.config.images.sizes
    }
    
    /// Get all image size keys
    ///
    /// # Returns
    ///
    /// A vector of image size keys
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ImageView;
    ///
    /// let view = ImageView::new()?;
    /// println!("Size keys: {:?}", view.size_keys());
    /// ```
    pub fn size_keys(&self) -> Vec<String> {
        self.config.images.sizes.keys().cloned().collect()
    }
    
    /// Get a specific image size by key
    ///
    /// # Arguments
    ///
    /// * `key` - The image size key
    ///
    /// # Returns
    ///
    /// The image size configuration, or `None` if the size doesn't exist
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ImageView;
    ///
    /// let view = ImageView::new()?;
    /// if let Some(size) = view.size("small") {
    ///     println!("Small size: {}x{}", size.width, size.height);
    /// }
    /// ```
    pub fn size(&self, key: &str) -> Option<&ImageSize> {
        self.config.images.sizes.get(key)
    }
    
    /// Validate that an image size exists
    ///
    /// # Arguments
    ///
    /// * `key` - The image size key
    ///
    /// # Returns
    ///
    /// The image size configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the image size doesn't exist
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::ImageView;
    ///
    /// let view = ImageView::new()?;
    /// let size = view.validate_size("small")?;
    /// println!("Small size: {}x{}", size.width, size.height);
    /// ```
    pub fn validate_size(&self, key: &str) -> Result<&ImageSize> {
        self.config.images.sizes.get(key)
            .ok_or_else(|| WritingError::config_error(format!("Image size not found: {}", key)))
    }
}

impl ConfigView for ImageView {
    fn config(&self) -> &Config {
        &self.config
    }
    
    fn from_config(config: Config) -> Self {
        ImageView { config }
    }
}

/// View for publication-related configuration
pub struct PublicationView {
    /// The underlying configuration
    config: Config,
}

impl PublicationView {
    /// Create a new publication view
    ///
    /// # Returns
    ///
    /// A new publication view
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::PublicationView;
    ///
    /// let view = PublicationView::new()?;
    /// ```
    pub fn new() -> Result<Self> {
        let config = super::load_config()?;
        Ok(PublicationView { config })
    }
    
    /// Create a new publication view from a specific path
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the configuration file
    ///
    /// # Returns
    ///
    /// A new publication view
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration cannot be loaded
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::PublicationView;
    /// use std::path::Path;
    ///
    /// let view = PublicationView::from_path(Path::new("config.yaml"))?;
    /// ```
    pub fn from_path(path: &Path) -> Result<Self> {
        let config = super::load_config_from_path(path)?;
        Ok(PublicationView { config })
    }
    
    /// Get the author
    ///
    /// # Returns
    ///
    /// The author
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::PublicationView;
    ///
    /// let view = PublicationView::new()?;
    /// println!("Author: {}", view.author());
    /// ```
    pub fn author(&self) -> &str {
        &self.config.publication.author
    }
    
    /// Get the copyright
    ///
    /// # Returns
    ///
    /// The copyright
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::PublicationView;
    ///
    /// let view = PublicationView::new()?;
    /// println!("Copyright: {}", view.copyright());
    /// ```
    pub fn copyright(&self) -> &str {
        &self.config.publication.copyright
    }
    
    /// Get the site URL
    ///
    /// # Returns
    ///
    /// The site URL, or `None` if it's not set
    ///
    /// # Examples
    ///
    /// ```
    /// use common_config::views::PublicationView;
    ///
    /// let view = PublicationView::new()?;
    /// if let Some(site) = view.site() {
    ///     println!("Site: {}", site);
    /// }
    /// ```
    pub fn site(&self) -> Option<&str> {
        self.config.publication.site.as_deref()
    }
}

impl ConfigView for PublicationView {
    fn config(&self) -> &Config {
        &self.config
    }
    
    fn from_config(config: Config) -> Self {
        PublicationView { config }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;
    
    fn create_test_config() -> NamedTempFile {
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
        
        temp_file
    }
    
    #[test]
    fn test_content_view() {
        let temp_file = create_test_config();
        
        // Create a content view from the test config
        let view = ContentView::from_path(temp_file.path()).unwrap();
        
        // Test the view methods
        assert_eq!(view.base_dir(), "./content");
        assert_eq!(view.topic_keys(), vec!["blog"]);
        assert!(view.topics().contains_key("blog"));
        
        // Test topic methods
        let topic = view.topic("blog").unwrap();
        assert_eq!(topic.name, "Blog");
        assert_eq!(topic.description, "Blog posts");
        assert_eq!(topic.directory, "blog");
        
        // Test validate_topic
        let topic = view.validate_topic("blog").unwrap();
        assert_eq!(topic.name, "Blog");
        
        // Test topic_path
        let path = view.get_topic_path("blog").unwrap();
        assert_eq!(path, "blog");
        
        // Test topic_full_path
        let full_path = view.get_topic_absolute_path("blog").unwrap();
        assert_eq!(full_path, PathBuf::from("./content/blog"));
    }
    
    #[test]
    fn test_image_view() {
        let temp_file = create_test_config();
        
        // Create an image view from the test config
        let view = ImageView::from_path(temp_file.path()).unwrap();
        
        // Test the view methods
        assert_eq!(view.formats(), &vec!["webp".to_string(), "jpg".to_string()]);
        assert_eq!(view.size_keys(), vec!["small"]);
        assert!(view.sizes().contains_key("small"));
        
        // Test size methods
        let size = view.size("small").unwrap();
        assert_eq!(size.width, 480);
        assert_eq!(size.height, 320);
        assert_eq!(size.description, "Small image");
        
        // Test validate_size
        let size = view.validate_size("small").unwrap();
        assert_eq!(size.width, 480);
    }
    
    #[test]
    fn test_publication_view() {
        let temp_file = create_test_config();
        
        // Create a publication view from the test config
        let view = PublicationView::from_path(temp_file.path()).unwrap();
        
        // Test the view methods
        assert_eq!(view.author(), "Test Author");
        assert_eq!(view.copyright(), "Test Copyright");
        assert_eq!(view.site().unwrap(), "https://example.com");
    }
} 