use anyhow::{Context, Result};
use common_config;
use common_fs;
use common_markdown;
use common_models::{Config, Frontmatter};
use std::path::{Path, PathBuf};

/// Options for content editing
pub struct EditOptions {
    pub slug: Option<String>,
    pub topic: Option<String>,
    pub frontmatter_only: bool,
    pub content_only: bool,
}

/// Find the path to content by slug and optionally topic
pub fn find_content_path(slug: &str, topic: Option<&str>) -> Result<PathBuf> {
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
        let content_path = PathBuf::from(&config.content.base_dir)
            .join(&topic_config.path)
            .join(slug)
            .join("index.mdx");
        
        if content_path.exists() {
            return Ok(content_path);
        }
        
        return Err(anyhow::anyhow!("Content not found: {}", content_path.display()));
    }
    
    // Search for content in all topics
    for (_, topic_config) in &config.content.topics {
        let content_path = PathBuf::from(&config.content.base_dir)
            .join(&topic_config.path)
            .join(slug)
            .join("index.mdx");
        
        if content_path.exists() {
            return Ok(content_path);
        }
    }
    
    Err(anyhow::anyhow!("Content not found with slug: {}", slug))
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
                content_list.push((topic_key.clone(), slug, index_path));
            }
        }
    }
    
    Ok(content_list)
}

/// Edit content by slug with the provided options
/// Returns a tuple with the content path and the content to be edited
pub fn edit_content(options: &EditOptions) -> Result<(PathBuf, String)> {
    let slug = match &options.slug {
        Some(s) => s.clone(),
        None => {
            // If no slug is provided, list all content for selection
            let content_list = list_all_content()?;
            
            if content_list.is_empty() {
                return Err(anyhow::anyhow!("No content found"));
            }
            
            // Can't return a selection here, as it requires user interaction
            // This would be handled in the main.rs file
            return Err(anyhow::anyhow!("No slug provided"));
        }
    };
    
    let content_path = find_content_path(&slug, options.topic.as_deref())?;
    
    // Read the content file
    let content = common_fs::read_file(&content_path)?;
    
    // Extract frontmatter and content
    let (frontmatter, markdown_content) = common_markdown::extract_frontmatter_and_content(&content)?;
    
    // Determine which parts to edit based on options
    let edit_frontmatter = !options.content_only;
    let edit_content = !options.frontmatter_only;
    
    // Create updated content
    let updated_content = if edit_frontmatter && edit_content {
        // Edit the entire file
        content
    } else if edit_frontmatter {
        // Edit only frontmatter
        let frontmatter_yaml = serde_yaml::to_string(&frontmatter)?;
        format!("---\n{}---\n\n{}", frontmatter_yaml, markdown_content)
    } else {
        // Edit only content
        let frontmatter_yaml = serde_yaml::to_string(&frontmatter)?;
        format!("---\n{}---\n\n{}", frontmatter_yaml, markdown_content)
    };
    
    // Return the content path and content to be edited
    Ok((content_path, updated_content))
}

/// Save edited content back to the file
pub fn save_edited_content(content_path: &Path, edited_content: &str) -> Result<()> {
    // Validate that the edited content has valid frontmatter
    if !edited_content.starts_with("---") {
        return Err(anyhow::anyhow!("Invalid content: missing frontmatter delimiter"));
    }
    
    let parts: Vec<&str> = edited_content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(anyhow::anyhow!("Invalid content: missing frontmatter"));
    }
    
    // Try to parse frontmatter to ensure it's valid
    let _: Frontmatter = serde_yaml::from_str(parts[1].trim())
        .context("Invalid frontmatter format")?;
    
    // Save the content
    common_fs::write_file(content_path, edited_content)?;
    
    Ok(())
} 