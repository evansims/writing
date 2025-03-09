use anyhow::{Context, Result};
use common_config::{load_config};
use common_fs::{create_dir_all, path_exists};
use common_models::{Config, TopicConfig};
use serde_yaml;
use slug::slugify;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Options for renaming a topic
#[derive(Debug)]
pub struct TopicRenameOptions {
    /// Current topic key
    pub key: Option<String>,
    /// New topic key
    pub new_key: Option<String>,
    /// New topic name
    pub new_name: Option<String>,
    /// New topic path
    pub new_path: Option<String>,
}

impl Default for TopicRenameOptions {
    fn default() -> Self {
        Self {
            key: None,
            new_key: None,
            new_name: None,
            new_path: None,
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
pub fn move_content(base_dir: &str, old_path: &str, new_path: &str) -> Result<()> {
    let old_dir = format!("{}/{}", base_dir, old_path);
    let new_dir = format!("{}/{}", base_dir, new_path);
    
    // Create the new directory if it doesn't exist
    if !Path::new(&new_dir).exists() {
        create_dir_all(Path::new(&new_dir))
            .context(format!("Failed to create directory: {}", new_dir))?;
    }
    
    // Check if the old directory exists
    if !Path::new(&old_dir).exists() {
        return Ok(());
    }
    
    // Move each article directory from old path to new path
    for entry in WalkDir::new(&old_dir)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let article_name = entry.file_name().to_string_lossy().to_string();
        let old_article_path = format!("{}/{}", old_dir, article_name);
        let new_article_path = format!("{}/{}", new_dir, article_name);
        
        // Move the article directory
        fs::rename(&old_article_path, &new_article_path)
            .context(format!("Failed to move {} to {}", old_article_path, new_article_path))?;
    }
    
    // Remove the old directory if it's empty
    if let Ok(entries) = fs::read_dir(&old_dir) {
        if entries.count() == 0 {
            fs::remove_dir(&old_dir)
                .context(format!("Failed to remove directory: {}", old_dir))?;
        }
    }
    
    Ok(())
}

/// Generate a key from a name
pub fn generate_key_from_name(name: &str) -> String {
    slugify(&name.to_lowercase())
}

/// Rename a topic in the configuration and move its content
pub fn rename_topic(options: &TopicRenameOptions) -> Result<String> {
    // Load config
    let mut config = load_config()?;
    
    // Get the current topic key
    let current_key = match &options.key {
        Some(k) => k.clone(),
        None => return Err(anyhow::anyhow!("No topic key provided")),
    };
    
    // Check if the topic exists
    if !topic_exists(&config, &current_key) {
        return Err(anyhow::anyhow!("Topic with key '{}' not found", current_key));
    }
    
    // Get the current topic configuration
    let current_topic = config.content.topics.get(&current_key).unwrap().clone();
    
    // Determine the new key (use current if not specified)
    let new_key = match &options.new_key {
        Some(k) => k.clone(),
        None => {
            // If a new name is provided but no new key, generate from name
            if let Some(new_name) = &options.new_name {
                generate_key_from_name(new_name)
            } else {
                current_key.clone()
            }
        }
    };
    
    // Check if the new key already exists (and is different from current)
    if new_key != current_key && config.content.topics.contains_key(&new_key) {
        return Err(anyhow::anyhow!("Topic with key '{}' already exists", new_key));
    }
    
    // Determine the new name (use current if not specified)
    let new_name = match &options.new_name {
        Some(n) => n.clone(),
        None => current_topic.name.clone(),
    };
    
    // Determine the new path (use current if not specified)
    let new_path = match &options.new_path {
        Some(p) => p.clone(),
        None => current_topic.path.clone(),
    };
    
    // Create a new topic configuration
    let new_topic = TopicConfig {
        name: new_name,
        description: current_topic.description.clone(),
        path: new_path.clone(),
    };
    
    // Check if we need to move content (only if path changed)
    if current_topic.path != new_path {
        move_content(&config.content.base_dir, &current_topic.path, &new_path)?;
    }
    
    // Update the configuration
    config.content.topics.remove(&current_key);
    config.content.topics.insert(new_key.clone(), new_topic);
    
    // Update tags if the topic key changed
    if current_key != new_key {
        if let Some(tags) = &mut config.content.tags {
            if let Some(topic_tags) = tags.remove(&current_key) {
                tags.insert(new_key.clone(), topic_tags);
            }
        }
    }
    
    // Write the updated configuration
    write_config(&config)?;
    
    Ok(new_key)
} 