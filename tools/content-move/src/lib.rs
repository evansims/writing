use anyhow::{Context, Result};
use common_config;
use common_fs;
use fs_extra::dir::{copy, CopyOptions};
use std::fs;
use std::path::{Path, PathBuf};

/// Options for content movement
#[derive(Clone)]
pub struct MoveOptions {
    pub slug: Option<String>,
    pub new_slug: Option<String>,
    pub topic: Option<String>,
    pub new_topic: Option<String>,
}

/// Find the directory containing the content
pub fn find_content_dir(slug: &str, topic: Option<&str>) -> Result<(PathBuf, String)> {
    let config = common_config::load_config()?;
    
    if let Some(topic_key) = topic {
        // Validate topic
        if !config.content.topics.contains_key(topic_key) {
            let valid_topics: Vec<String> = config.content.topics.keys()
                .map(|k| k.to_string())
                .collect();
            return Err(anyhow::anyhow!(
                "Invalid topic: {}. Valid topics are: {}", 
                topic_key, 
                valid_topics.join(", ")
            ));
        }
        
        let topic_config = &config.content.topics[topic_key];
        let content_dir = PathBuf::from(&config.content.base_dir)
            .join(&topic_config.path)
            .join(slug);
        
        if content_dir.exists() {
            return Ok((content_dir, topic_key.to_string()));
        }
        
        return Err(anyhow::anyhow!("Content not found: {}", content_dir.display()));
    }
    
    // Search for content in all topics
    for (topic_key, topic_config) in &config.content.topics {
        let content_dir = PathBuf::from(&config.content.base_dir)
            .join(&topic_config.path)
            .join(slug);
        
        if content_dir.exists() {
            return Ok((content_dir, topic_key.clone()));
        }
    }
    
    Err(anyhow::anyhow!("Content not found for slug: {}", slug))
}

/// List all content in the repository
pub fn list_all_content() -> Result<Vec<(String, String, PathBuf)>> {
    let config = common_config::load_config()?;
    let mut content_list = Vec::new();
    
    for (topic_key, topic_config) in &config.content.topics {
        let topic_dir = PathBuf::from(&config.content.base_dir).join(&topic_config.path);
        if !topic_dir.exists() {
            continue;
        }
        
        // Find all subdirectories in the topic directory
        let article_dirs = common_fs::find_dirs_with_depth(&topic_dir, 1, 1)?;
        
        for article_dir in article_dirs {
            let slug = article_dir.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("")
                .to_string();
            
            let index_path = article_dir.join("index.mdx");
            if index_path.exists() {
                content_list.push((topic_key.clone(), slug, article_dir));
            }
        }
    }
    
    Ok(content_list)
}

/// Update content references
pub fn update_content_references(content_path: &Path, old_slug: &str, new_slug: &str) -> Result<()> {
    let content = common_fs::read_file(content_path)?;
    
    // Replace old slug with new slug
    let updated_content = content.replace(old_slug, new_slug);
    
    if content != updated_content {
        common_fs::write_file(content_path, &updated_content)?;
    }
    
    Ok(())
}

/// Move content to a new location and/or rename it
pub fn move_content(options: &MoveOptions) -> Result<(String, String, String, String)> {
    let config = common_config::load_config()?;
    
    // Find content to move
    let (content_dir, current_topic) = match &options.slug {
        Some(slug) => find_content_dir(slug, options.topic.as_deref())?,
        None => {
            // This would be handled in the CLI, but we need a slug
            return Err(anyhow::anyhow!("No slug provided for moving content"));
        }
    };
    
    let current_slug = content_dir.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_string();
    
    // Get new slug
    let new_slug = match &options.new_slug {
        Some(s) => s.clone(),
        None => current_slug.clone(),
    };
    
    // Get new topic
    let new_topic = match &options.new_topic {
        Some(t) => {
            if !config.content.topics.contains_key(t) {
                return Err(anyhow::anyhow!("Topic '{}' not found", t));
            }
            t.clone()
        },
        None => current_topic.clone(),
    };
    
    // If no changes are requested, there's nothing to do
    if current_slug == new_slug && current_topic == new_topic {
        return Ok((current_topic, current_slug, new_topic, new_slug));
    }
    
    // Check if the content already exists at the destination
    let new_topic_config = &config.content.topics[&new_topic];
    let new_content_dir = PathBuf::from(&config.content.base_dir)
        .join(&new_topic_config.path)
        .join(&new_slug);
    
    if new_content_dir.exists() && (current_slug != new_slug || current_topic != new_topic) {
        return Err(anyhow::anyhow!("Content already exists at destination: {}/{}", new_topic, new_slug));
    }
    
    // If we're just changing the slug (same topic), rename the directory
    if current_topic == new_topic {
        // Rename the directory
        fs::rename(&content_dir, &new_content_dir)
            .with_context(|| format!("Failed to rename content directory from '{}' to '{}'", 
                content_dir.display(), new_content_dir.display()))?;
    } else {
        // We're moving to a different topic, so copy the directory and then remove the original
        let mut options = CopyOptions::new();
        options.overwrite = false;
        
        // Create parent directory if it doesn't exist
        let new_parent_dir = PathBuf::from(&config.content.base_dir)
            .join(&new_topic_config.path);
        
        if !new_parent_dir.exists() {
            common_fs::create_dir_all(&new_parent_dir)?;
        }
        
        // Copy the directory
        copy(&content_dir, &new_parent_dir, &options)
            .with_context(|| format!("Failed to copy content from '{}' to '{}'", 
                content_dir.display(), new_parent_dir.display()))?;
        
        // Rename the copied directory if needed
        let copied_dir = new_parent_dir.join(&current_slug);
        if current_slug != new_slug {
            fs::rename(&copied_dir, &new_content_dir)
                .with_context(|| format!("Failed to rename copied content directory from '{}' to '{}'", 
                    copied_dir.display(), new_content_dir.display()))?;
        }
        
        // Remove the original directory
        common_fs::delete_dir_all(&content_dir)?;
    }
    
    // Update references to the content in other content files
    update_content_references(&new_content_dir, &current_slug, &new_slug)?;
    
    Ok((current_topic, current_slug, new_topic, new_slug))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_update_content_references() {
        // Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let test_file_path = temp_dir.path().join("test_file.md");

        // Create test content
        let content = r#"
This is a test file that links to [old-slug](../old-slug/) and references old-slug multiple times.
Let's see if old-slug gets replaced correctly.
"#;
        
        // Write test content to file
        let mut file = fs::File::create(&test_file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        
        // Update references
        update_content_references(&test_file_path, "old-slug", "new-slug").unwrap();
        
        // Read updated content
        let updated_content = fs::read_to_string(&test_file_path).unwrap();
        
        // Check if references were updated
        assert!(!updated_content.contains("old-slug"));
        assert!(updated_content.contains("new-slug"));
    }
} 