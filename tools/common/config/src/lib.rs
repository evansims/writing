//! # Common Configuration Operations
//!
//! This module provides common configuration operations for the writing tools.
//!
//! ## Features
//!
//! - Configuration loading from files
//! - Topic management
//! - Configuration validation
//! - Configuration caching
//! - Context-specific configuration views
//!
//! ## Example
//!
//! ```rust
//! use common_config::{load_config, get_topics, validate_topic};
//! use common_config::views::ContentView;
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
//!
//! fn use_content_view() -> common_errors::Result<()> {
//!     let view = ContentView::new()?;
//!     println!("Base directory: {}", view.base_dir());
//!     println!("Topics: {:?}", view.topic_keys());
//!     Ok(())
//! }
//! ```

use common_errors::{Result, WritingError, ResultExt};
use common_models::{Config, TopicConfig};
use std::fs;
use std::path::Path;

// Export the cache module
pub mod cache;

// Export the views module
pub mod views;

// Re-export the views for convenience
pub use views::ContentView;
pub use views::ImageView;
pub use views::PublicationView;
pub use views::ConfigView;

#[cfg(test)]
mod tests;

/// Load the configuration file from the current or parent directories
pub fn load_config() -> Result<Config> {
    // Use the global cache instance to get the config
    cache::ConfigCache::global().get_config()
}

/// Load the configuration file from a specific path
pub fn load_config_from_path(path: &Path) -> Result<Config> {
    // Read the file
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    // Parse the YAML
    let config: Config = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

    Ok(config)
}

/// Get all topics from the configuration
pub fn get_topics() -> Result<Vec<TopicConfig>> {
    let config = load_config()?;
    Ok(config.content.topics.values().cloned().collect())
}

/// Get all topic keys from the configuration
pub fn get_topic_keys() -> Result<Vec<String>> {
    let config = load_config()?;
    Ok(config.content.topics.keys().cloned().collect())
}

/// Get a topic by its key
pub fn get_topic_by_key(key: &str) -> Result<Option<TopicConfig>> {
    let config = load_config()?;
    Ok(config.content.topics.get(key).cloned())
}

/// Get the base directory for content
pub fn get_content_base_dir() -> Result<String> {
    let config = load_config()?;
    Ok(config.content.base_dir.clone())
}

/// Get the site URL from the configuration
pub fn get_site_url() -> Result<Option<String>> {
    let config = load_config()?;
    Ok(config.publication.site.clone())
}

/// Validate a topic key and return the topic configuration
pub fn validate_topic(topic_key: &str) -> Result<TopicConfig> {
    let config = load_config()?;

    // Check if the topic exists
    if let Some(topic) = config.content.topics.get(topic_key) {
        // Check if the directory exists
        let topic_dir = Path::new(&config.content.base_dir).join(&topic.directory);
        if !topic_dir.exists() {
            return Err(WritingError::topic_error(
                format!("Topic directory does not exist: {}", topic_dir.display())
            ));
        }

        // Check if the directory is readable
        if let Err(e) = fs::read_dir(&topic_dir) {
            return Err(WritingError::topic_error(
                format!("Topic directory is not readable: {}: {}", topic_dir.display(), e)
            ));
        }

        Ok(topic.clone())
    } else {
        Err(WritingError::topic_error(format!("Topic not found: {}", topic_key)))
    }
}

/// Clear the configuration cache
///
/// This function clears the configuration cache, forcing the next call to
/// `load_config` or `load_config_from_path` to load the configuration from disk.
///
/// # Examples
///
/// ```rust
/// use common_config::clear_config_cache;
///
/// fn refresh_config() -> common_errors::Result<()> {
///     clear_config_cache();
///     let config = common_config::load_config()?;
///     println!("Refreshed config: {}", config.publication.author);
///     Ok(())
/// }
/// ```
pub fn clear_config_cache() {
    cache::ConfigCache::global().clear();
}