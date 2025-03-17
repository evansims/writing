use anyhow::{Context, Result};
use common_config::load_config;
use common_fs::create_dir_all;
use common_models::{Config, TopicConfig};
use slug::slugify;
use std::fs;
use std::path::Path;

/// Options for adding a topic
#[derive(Clone)]
pub struct AddOptions {
    pub key: String,
    pub name: String,
    pub description: String,
    pub directory: String,
}

/// Add a topic
///
/// This function adds a new topic to the configuration.
///
/// # Parameters
///
/// * `options` - Add options
///
/// # Returns
///
/// Returns the key of the added topic
///
/// # Errors
///
/// Returns an error if the topic cannot be added
pub fn add_topic(options: &AddOptions) -> Result<String> {
    // Validate options
    if options.key.is_empty() {
        return Err(anyhow::anyhow!("Topic key is required"));
    }
    
    if options.name.is_empty() {
        return Err(anyhow::anyhow!("Topic name is required"));
    }
    
    if options.description.is_empty() {
        return Err(anyhow::anyhow!("Topic description is required"));
    }
    
    if options.directory.is_empty() {
        return Err(anyhow::anyhow!("Topic directory is required"));
    }
    
    // Load config
    let mut config = load_config()?;
    
    // Check if topic already exists
    if config.content.topics.contains_key(&options.key) {
        return Err(anyhow::anyhow!("Topic with key '{}' already exists", options.key));
    }
    
    // Create topic directory
    let dir_path = format!("{}/{}", config.content.base_dir, options.directory);
    create_dir_all(Path::new(&dir_path))
        .context(format!("Failed to create topic directory: {}", dir_path))?;
    
    // Add topic to config
    let topic_config = TopicConfig {
        name: options.name.clone(),
        description: options.description.clone(),
        directory: options.directory.clone(),
    };
    
    config.content.topics.insert(options.key.clone(), topic_config);
    
    // Save config
    write_config(&config)?;
    
    Ok(options.key.clone())
}

/// Generate a key from a topic name
pub fn generate_key_from_name(name: &str) -> String {
    slugify(name.to_lowercase())
}

/// Create the topic directory
pub fn create_topic_directory(base_dir: &str, path: &str) -> Result<()> {
    let dir_path = format!("{}/{}", base_dir, path);
    if !Path::new(&dir_path).exists() {
        create_dir_all(Path::new(&dir_path))
            .context(format!("Failed to create topic directory: {}", dir_path))?;
    }
    
    Ok(())
}

/// Check if a topic with the given key already exists
pub fn topic_exists(config: &Config, key: &str) -> bool {
    config.content.topics.contains_key(key)
}

/// Write the configuration to the config file
fn write_config(config: &Config) -> Result<()> {
    let config_path = "config.yaml";
    
    // Convert config to YAML
    let config_content = serde_yaml::to_string(config)
        .context("Failed to serialize config")?;
    
    // Write config to file
    fs::write(config_path, config_content)
        .context(format!("Failed to write config file: {}", config_path))?;
    
    Ok(())
}

/// Add tags to a topic
/// 
/// Returns true if tags were added successfully
pub fn add_tags_to_topic(topic_key: &str, tags: Vec<String>) -> Result<bool> {
    if tags.is_empty() {
        return Ok(false);
    }
    
    // Read the current configuration
    let mut config = load_config()?;
    
    // Check if topic exists
    if !topic_exists(&config, topic_key) {
        return Err(anyhow::anyhow!("Topic with key '{}' does not exist", topic_key));
    }
    
    // Make sure tags map exists
    if config.content.tags.is_none() {
        config.content.tags = Some(std::collections::HashMap::new());
    }
    
    // Add tags for the topic
    if let Some(tags_map) = &mut config.content.tags {
        tags_map.insert(topic_key.to_string(), tags);
    }
    
    // Write the updated configuration
    write_config(&config)?;
    
    Ok(true)
} 