//! # Configuration Module
//!
//! This module provides configuration handling for the write tool,
//! including lazy loading and caching of configuration.

use anyhow::Result;
use common_config::cache::ConfigCache;
use common_models::{Config, TopicConfig};
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::time::Duration;

/// Global configuration cache instance with a 5-minute cache duration
#[allow(dead_code)]
static CONFIG_CACHE: Lazy<ConfigCache> = Lazy::new(|| {
    ConfigCache::new(Duration::from_secs(300), true)
});

/// Lazy configuration container
///
/// This struct lazily loads configuration only when needed, improving
/// performance by avoiding unnecessary file system operations.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct LazyConfig {
    /// The cached configuration
    config: Option<Arc<Config>>,
}

impl LazyConfig {
    /// Create a new lazy configuration
    pub fn new() -> Self {
        Self { config: None }
    }

    /// Get the configuration, loading it if needed
    pub fn get(&mut self) -> Result<Arc<Config>> {
        if let Some(config) = &self.config {
            // Return cached config if available
            return Ok(config.clone());
        }

        // Load the configuration
        let config = CONFIG_CACHE.get_config()
            .map_err(|e| anyhow::anyhow!("Failed to load configuration: {}", e))?;

        // Cache the configuration
        let config = Arc::new(config);
        self.config = Some(config.clone());

        Ok(config)
    }

    /// Clear the cached configuration, forcing a reload on next access
    pub fn clear(&mut self) {
        self.config = None;
        CONFIG_CACHE.clear();
    }
}

/// Global instance of lazy configuration
#[allow(dead_code)]
static LAZY_CONFIG: Lazy<std::sync::Mutex<LazyConfig>> = Lazy::new(|| {
    std::sync::Mutex::new(LazyConfig::new())
});

/// Get the configuration, loading it lazily
#[allow(dead_code)]
pub fn get_config() -> Result<Arc<Config>> {
    let mut config = LAZY_CONFIG.lock()
        .map_err(|_| anyhow::anyhow!("Failed to acquire configuration lock"))?;
    config.get()
}

/// Clear the configuration cache, forcing a reload on next access
#[allow(dead_code)]
pub fn clear_config_cache() {
    if let Ok(mut config) = LAZY_CONFIG.lock() {
        config.clear();
    }
}

/// Get all topics from the configuration
#[allow(dead_code)]
pub fn get_topics() -> Result<Vec<String>> {
    let config = get_config()?;
    Ok(config.content.topics.keys().cloned().collect())
}

/// Get all topic names from the configuration
#[allow(dead_code)]
pub fn get_topic_names() -> Result<Vec<String>> {
    let config = get_config()?;
    let topic_names = config.content.topics.values()
        .map(|t| t.name.clone())
        .collect();
    Ok(topic_names)
}

/// Get a topic by its key
#[allow(dead_code)]
pub fn get_topic_by_key(key: &str) -> Result<Option<TopicConfig>> {
    let config = get_config()?;
    Ok(config.content.topics.get(key).cloned())
}

/// Get the site URL from the configuration
#[allow(dead_code)]
pub fn get_site_url() -> Result<Option<String>> {
    let config = get_config()?;
    Ok(config.publication.site_url.clone())
}

/// Validate a topic key exists
#[allow(dead_code)]
pub fn validate_topic(key: &str) -> Result<TopicConfig> {
    common_config::validate_topic(key)
        .map_err(|e| anyhow::anyhow!("Invalid topic '{}': {}", key, e))
}