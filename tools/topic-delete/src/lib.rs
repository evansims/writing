use anyhow::{Context, Result};
use common_config::load_config;
use common_models::Config;
use serde_yaml;
use std::fs;
use std::path::Path;

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

/// Check if a topic has content
///
/// This function checks if a topic directory contains any content.
///
/// # Parameters
///
/// * `base_dir` - Base directory for content
/// * `path` - Topic directory
///
/// # Returns
///
/// Returns true if the topic has content, false otherwise
pub fn has_content(base_dir: &str, path: &str) -> bool {
    let full_path = format!("{}/{}", base_dir, path);
    let dir = Path::new(&full_path);
    
    if !dir.exists() || !dir.is_dir() {
        return false;
    }
    
    // Check if the directory has any entries
    match fs::read_dir(dir) {
        Ok(entries) => entries.count() > 0,
        Err(_) => false,
    }
}

/// Move content from one topic to another
///
/// This function moves all content from one topic to another.
///
/// # Parameters
///
/// * `base_dir` - Base directory for content
/// * `source_path` - Source topic directory
/// * `target_path` - Target topic directory
///
/// # Returns
///
/// Returns Ok(()) if successful
///
/// # Errors
///
/// Returns an error if the content cannot be moved
pub fn move_content(base_dir: &str, source_path: &str, target_path: &str) -> Result<()> {
    let source_dir = format!("{}/{}", base_dir, source_path);
    let target_dir = format!("{}/{}", base_dir, target_path);
    
    // Create the target directory if it doesn't exist
    if !Path::new(&target_dir).exists() {
        fs::create_dir_all(&target_dir)
            .context(format!("Failed to create directory: {}", target_dir))?;
    }
    
    // Get all entries in the source directory
    let entries = fs::read_dir(&source_dir)
        .context(format!("Failed to read directory: {}", source_dir))?;
    
    // Move each entry to the target directory
    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();
        
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
            let target_path = format!("{}/{}", target_dir, dir_name);
            
            // Create the target directory
            fs::create_dir_all(&target_path)
                .context(format!("Failed to create directory: {}", target_path))?;
            
            // Move all files from source to target
            let files = fs::read_dir(&path)
                .context(format!("Failed to read directory: {}", path.display()))?;
            
            for file in files {
                let file = file.context("Failed to read directory entry")?;
                let file_path = file.path();
                
                if file_path.is_file() {
                    let file_name = file_path.file_name().unwrap().to_string_lossy().to_string();
                    let target_file = format!("{}/{}", target_path, file_name);
                    
                    fs::copy(&file_path, &target_file)
                        .context(format!("Failed to copy file: {} to {}", file_path.display(), target_file))?;
                }
            }
        }
    }
    
    Ok(())
}

/// Delete a topic
///
/// This function deletes a topic and optionally moves its content to another topic.
///
/// # Parameters
///
/// * `options` - Delete options
///
/// # Returns
///
/// Returns the result of the delete operation
///
/// # Errors
///
/// Returns an error if the topic cannot be deleted
pub fn delete_topic(options: &TopicDeleteOptions) -> Result<String> {
    // Load the configuration
    let mut config = load_config()?;
    
    // Get the topic key
    let topic_key = match &options.key {
        Some(k) => k.clone(),
        None => return Err(anyhow::anyhow!("No topic key specified")),
    };
    
    // Check if the topic exists
    if !topic_exists(&config, &topic_key) {
        return Err(anyhow::anyhow!("Topic with key '{}' not found", topic_key));
    }
    
    // Get the topic configuration
    let topic_config = config.content.topics.get(&topic_key).unwrap().clone();
    
    // Check if the topic has content
    let topic_has_content = has_content(&config.content.base_dir, &topic_config.directory);
    
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
        move_content(&config.content.base_dir, &topic_config.directory, &target_config.directory)?;
    }
    
    // Remove the topic directory if it exists
    let topic_dir = format!("{}/{}", config.content.base_dir, topic_config.directory);
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
    
    Ok(format!("Topic '{}' deleted successfully", topic_key))
} 