use anyhow::{Context, Result};
use common_config::load_config;
use common_fs::create_dir_all;
use common_models::{Config, TopicConfig};
use serde_yaml;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Options for deleting a topic
#[derive(Debug)]
pub struct TopicDeleteOptions {
    /// Topic key to delete
    pub key: Option<String>,
    /// Target topic key for content migration
    pub target: Option<String>,
    /// Force deletion without confirmation
    pub force: bool,
}

impl Default for TopicDeleteOptions {
    fn default() -> Self {
        Self {
            key: None,
            target: None,
            force: false,
        }
    }
}

/// Check if a topic exists in the configuration
pub fn topic_exists(config: &Config, key: &str) -> bool {
    config.content.topics.contains_key(key)
}

/// Get all topic keys from the configuration except for a specific key
pub fn get_topic_keys_except(config: &Config, except_key: &str) -> Vec<String> {
    config.content.topics.keys()
        .filter(|&k| k != except_key)
        .cloned()
        .collect()
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

/// Check if a directory has content
pub fn has_content(base_dir: &str, path: &str) -> bool {
    let full_path = format!("{}/{}", base_dir, path);
    if !Path::new(&full_path).exists() {
        return false;
    }
    
    let entries = WalkDir::new(&full_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
        .count();
    
    entries > 0
}

/// Move content from one directory to another
pub fn move_content(base_dir: &str, source_path: &str, target_path: &str) -> Result<()> {
    let source_dir = format!("{}/{}", base_dir, source_path);
    let target_dir = format!("{}/{}", base_dir, target_path);
    
    // Create the target directory if it doesn't exist
    if !Path::new(&target_dir).exists() {
        create_dir_all(Path::new(&target_dir))
            .context(format!("Failed to create directory: {}", target_dir))?;
    }
    
    // Check if the source directory exists
    if !Path::new(&source_dir).exists() {
        return Ok(());
    }
    
    // Move each article directory from source path to target path
    for entry in WalkDir::new(&source_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let article_name = entry.file_name().to_string_lossy().to_string();
        let source_article_path = format!("{}/{}", source_dir, article_name);
        let target_article_path = format!("{}/{}", target_dir, article_name);
        
        // Move the article directory
        fs::rename(&source_article_path, &target_article_path)
            .context(format!("Failed to move {} to {}", source_article_path, target_article_path))?;
    }
    
    // Remove the source directory if it's empty
    if let Ok(entries) = fs::read_dir(&source_dir) {
        if entries.count() == 0 {
            fs::remove_dir(&source_dir)
                .context(format!("Failed to remove directory: {}", source_dir))?;
        }
    }
    
    Ok(())
}

/// Delete a topic and optionally migrate its content
pub fn delete_topic(options: &TopicDeleteOptions) -> Result<String> {
    // Load config
    let mut config = load_config()?;
    
    // Get the topic key to delete
    let topic_key = match &options.key {
        Some(k) => k.clone(),
        None => return Err(anyhow::anyhow!("No topic key provided")),
    };
    
    // Check if the topic exists
    if !topic_exists(&config, &topic_key) {
        return Err(anyhow::anyhow!("Topic with key '{}' not found", topic_key));
    }
    
    // Get the topic configuration
    let topic_config = config.content.topics.get(&topic_key).unwrap().clone();
    
    // Check if the topic has content
    let topic_has_content = has_content(&config.content.base_dir, &topic_config.path);
    
    // If the topic has content, we need to migrate it
    if topic_has_content {
        // Get the target topic for migration
        let target_key = match &options.target {
            Some(t) => {
                // Check if the target topic exists
                if !topic_exists(&config, t) {
                    return Err(anyhow::anyhow!("Target topic with key '{}' not found", t));
                }
                t.clone()
            },
            None => return Err(anyhow::anyhow!("No target topic specified for content migration")),
        };
        
        // Get the target topic configuration
        let target_config = config.content.topics.get(&target_key).unwrap().clone();
        
        // Move the content from source to target
        move_content(&config.content.base_dir, &topic_config.path, &target_config.path)?;
    }
    
    // Remove the topic directory if it exists
    let topic_dir = format!("{}/{}", config.content.base_dir, topic_config.path);
    if Path::new(&topic_dir).exists() {
        if let Ok(entries) = fs::read_dir(&topic_dir) {
            if entries.count() == 0 {
                fs::remove_dir(&topic_dir)
                    .context(format!("Failed to remove directory: {}", topic_dir))?;
            }
        }
    }
    
    // Remove the topic from the configuration
    config.content.topics.remove(&topic_key);
    
    // Remove the topic's tags if they exist
    if let Some(tags) = &mut config.content.tags {
        tags.remove(&topic_key);
    }
    
    // Write the updated configuration
    write_config(&config)?;
    
    Ok(topic_key)
} 