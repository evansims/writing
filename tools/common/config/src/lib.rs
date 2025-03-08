use anyhow::{Context, Result};
use common_models::{Config, TopicConfig};
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Load the configuration file from the current or parent directories
pub fn load_config() -> Result<Config> {
    // Try to find config.yaml in the current directory or parent directories
    let mut current_dir = std::env::current_dir()?;
    let config_filename = "config.yaml";
    let mut config_path = current_dir.join(config_filename);
    
    // Keep going up the directory tree until we find config.yaml or reach the root
    while !config_path.exists() {
        if !current_dir.pop() {
            // We've reached the root directory and still haven't found config.yaml
            return Err(anyhow::anyhow!("Could not find config.yaml in the current directory or any parent directory"));
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