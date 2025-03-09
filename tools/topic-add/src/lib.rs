use anyhow::{Context, Result};
use common_config::load_config;
use common_fs::create_dir_all;
use common_models::{Config, TopicConfig};
use slug::slugify;
use std::fs;
use std::path::Path;

/// Options for adding a new topic
#[derive(Debug, Clone)]
pub struct TopicAddOptions {
    /// Topic key
    pub key: Option<String>,
    /// Topic name
    pub name: Option<String>,
    /// Topic description
    pub description: Option<String>,
    /// Topic path (directory name)
    pub path: Option<String>,
}

impl Default for TopicAddOptions {
    fn default() -> Self {
        Self {
            key: None,
            name: None,
            description: None,
            path: None,
        }
    }
}

/// Generate a key from a topic name
pub fn generate_key_from_name(name: &str) -> String {
    slugify(&name.to_lowercase())
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
    let config_content = serde_yaml::to_string(config)
        .context("Failed to serialize config")?;
    
    // Add a comment at the top of the file
    let config_content = format!("# Writing Repository Configuration\n\n{}", config_content);
    
    fs::write(config_path, config_content)
        .context(format!("Failed to write config file: {}", config_path))?;
    
    Ok(())
}

/// Add a new topic to the configuration
/// 
/// Returns the key of the newly added topic
pub fn add_topic(options: &TopicAddOptions) -> Result<String> {
    // Read the current configuration
    let mut config = load_config()?;
    
    // Get topic name
    let name = match &options.name {
        Some(n) => n.clone(),
        None => return Err(anyhow::anyhow!("Topic name is required")),
    };
    
    // Generate key from name if not provided
    let key = match &options.key {
        Some(k) => k.clone(),
        None => generate_key_from_name(&name),
    };
    
    // Check if topic already exists
    if topic_exists(&config, &key) {
        return Err(anyhow::anyhow!("Topic with key '{}' already exists", key));
    }
    
    // Get description
    let description = match &options.description {
        Some(d) => d.clone(),
        None => return Err(anyhow::anyhow!("Topic description is required")),
    };
    
    // Get path
    let path = match &options.path {
        Some(p) => p.clone(),
        None => key.clone(),
    };
    
    // Create the topic configuration
    let topic_config = TopicConfig {
        name,
        description,
        path: path.clone(),
    };
    
    // Add the topic to the configuration
    config.content.topics.insert(key.clone(), topic_config);
    
    // Create the topic directory
    create_topic_directory(&config.content.base_dir, &path)?;
    
    // Write the updated configuration
    write_config(&config)?;
    
    Ok(key)
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