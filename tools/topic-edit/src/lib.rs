use anyhow::{Context, Result};
use common_config::{load_config, get_topic_by_key};
use common_models::{Config, TopicConfig};
use serde_yaml;
use std::fs;

/// Options for editing a topic
#[derive(Debug)]
pub struct TopicEditOptions {
    /// Topic key to edit
    pub key: Option<String>,
    /// New name for the topic
    pub name: Option<String>,
    /// New description for the topic
    pub description: Option<String>,
}

impl Default for TopicEditOptions {
    fn default() -> Self {
        Self {
            key: None,
            name: None,
            description: None,
        }
    }
}

/// Check if a topic exists in the configuration
pub fn topic_exists(config: &Config, key: &str) -> bool {
    config.content.topics.contains_key(key)
}

/// Write the configuration to the filesystem
fn write_config(config: &Config) -> Result<()> {
    // Convert to YAML with a header comment
    let yaml = serde_yaml::to_string(config)
        .context("Failed to convert configuration to YAML")?;
    
    let content = format!("# Writing Configuration\n{}", yaml);
    
    // Write to file
    fs::write("config.yaml", content)
        .context("Failed to write configuration file")?;
    
    Ok(())
}

/// Get all topic keys from the configuration
pub fn get_topic_keys(config: &Config) -> Vec<String> {
    config.content.topics.keys().cloned().collect()
}

/// Edit a topic in the configuration
pub fn edit_topic(options: &TopicEditOptions) -> Result<String> {
    // Load config
    let mut config = load_config()?;
    
    // Get the topic key to update
    let key = match &options.key {
        Some(k) => k.clone(),
        None => return Err(anyhow::anyhow!("No topic key provided")),
    };
    
    // Check if the topic exists
    if !topic_exists(&config, &key) {
        return Err(anyhow::anyhow!("Topic with key '{}' not found", key));
    }
    
    // Get the current topic configuration
    let mut topic_config = config.content.topics.get(&key).unwrap().clone();
    
    // Update the name if provided
    if let Some(name) = &options.name {
        topic_config.name = name.clone();
    }
    
    // Update the description if provided
    if let Some(description) = &options.description {
        topic_config.description = description.clone();
    }
    
    // Update the topic configuration
    config.content.topics.insert(key.clone(), topic_config);
    
    // Write the updated configuration
    write_config(&config)?;
    
    Ok(key)
} 