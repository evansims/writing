use anyhow::{Context, Result};
use common_config::load_config;
use common_fs::{create_dir_all};
use common_models::Config;
use slug::slugify;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use std::path::{PathBuf};

/// Options for renaming a topic
pub struct TopicRenameOptions {
    pub key: Option<String>,
    pub new_key: Option<String>,
    pub new_name: Option<String>,
    pub new_description: Option<String>,
    pub new_path: Option<String>,
}

/// Rename a topic
///
/// This function renames a topic in the configuration and optionally moves the content.
///
/// # Parameters
///
/// * `options` - Rename options
///
/// # Returns
///
/// Returns the new key of the renamed topic
///
/// # Errors
///
/// Returns an error if the topic cannot be renamed
pub fn rename_topic(options: &TopicRenameOptions) -> Result<String> {
    // Validate options
    let current_key = match &options.key {
        Some(key) if !key.is_empty() => key.clone(),
        _ => return Err(anyhow::anyhow!("Current topic key is required")),
    };
    
    let new_key = match &options.new_key {
        Some(key) if !key.is_empty() => key.clone(),
        _ => current_key.clone(),
    };
    
    // Load config
    let mut config = load_config()?;
    
    // Check if current topic exists
    let topic_config = match config.content.topics.get(&current_key) {
        Some(config) => config.clone(),
        None => return Err(anyhow::anyhow!("Topic not found: {}", current_key)),
    };
    
    // Check if new topic already exists
    if current_key != new_key && config.content.topics.contains_key(&new_key) {
        return Err(anyhow::anyhow!("Topic already exists: {}", new_key));
    }
    
    // Create new topic config
    let mut new_topic_config = topic_config.clone();
    
    if let Some(name) = &options.new_name {
        if !name.is_empty() {
            new_topic_config.name = name.clone();
        }
    }
    
    if let Some(description) = &options.new_description {
        if !description.is_empty() {
            new_topic_config.description = description.clone();
        }
    }
    
    let _directory_changed = if let Some(directory) = &options.new_path {
        if !directory.is_empty() && directory != &new_topic_config.directory {
            let old_directory = new_topic_config.directory.clone();
            new_topic_config.directory = directory.clone();
            
            // Move content
            let base_dir = PathBuf::from(&config.content.base_dir);
            let old_dir = base_dir.join(&old_directory);
            let new_dir = base_dir.join(directory);
            
            if old_dir.exists() {
                if new_dir.exists() {
                    return Err(anyhow::anyhow!("Directory already exists: {}", new_dir.display()));
                }
                
                // Create parent directory if it doesn't exist
                if let Some(parent) = new_dir.parent() {
                    if !parent.exists() {
                        create_dir_all(parent)?;
                    }
                }
                
                // Move content
                move_content(&old_dir, &new_dir)?;
            }
            
            true
        } else {
            false
        }
    } else {
        false
    };
    
    // Update config
    if current_key != new_key {
        config.content.topics.remove(&current_key);
    }
    
    config.content.topics.insert(new_key.clone(), new_topic_config);
    
    // Save config
    let yaml = serde_yaml::to_string(&config)
        .context("Failed to convert configuration to YAML")?;
    
    let content = format!("---\n{}", yaml);
    
    fs::write("config.yaml", content)
        .context("Failed to write configuration file")?;
    
    Ok(new_key)
}

/// Move content from one directory to another
///
/// This function moves content from one directory to another.
///
/// # Parameters
///
/// * `base_dir` - Base content directory
/// * `from_dir` - Source directory
/// * `to_dir` - Target directory
///
/// # Returns
///
/// Returns Ok(()) if successful
///
/// # Errors
///
/// Returns an error if the move fails
fn move_content(from: &Path, to: &Path) -> Result<()> {
    // Create target directory if it doesn't exist
    if !to.exists() {
        create_dir_all(to)?;
    }
    
    // Copy all files and directories
    copy_dir_all(from, to)?;
    
    // Remove source directory
    fs::remove_dir_all(from)?;
    
    Ok(())
}

/// Copy a directory recursively
///
/// This function copies a directory recursively.
///
/// # Parameters
///
/// * `src` - Source directory
/// * `dst` - Target directory
///
/// # Returns
///
/// Returns Ok(()) if successful
///
/// # Errors
///
/// Returns an error if the copy fails
fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    
    Ok(())
}

/// Check if a topic exists in the configuration
pub fn topic_exists(config: &Config, key: &str) -> bool {
    config.content.topics.contains_key(key)
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

/// Generate a key from a name
pub fn generate_key_from_name(name: &str) -> String {
    slugify(name.to_lowercase())
} 