use anyhow::Result;
use common_config;
use common_fs;
use common_markdown;
use std::path::{Path, PathBuf};

/// Options for content deletion
pub struct DeleteOptions {
    pub slug: Option<String>,
    pub topic: Option<String>,
    pub force: bool,
}

/// Find the directory containing the content to delete
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

/// Extract the title from a content file
pub fn extract_title_from_content(content_path: &Path) -> Result<String> {
    let content = common_fs::read_file(content_path)?;
    
    // Use common_markdown to extract frontmatter
    if let Ok((frontmatter, _)) = common_markdown::extract_frontmatter_and_content(&content) {
        return Ok(frontmatter.title);
    }
    
    // Fallback to extracting title manually
    for line in content.lines() {
        if line.starts_with("title:") {
            return Ok(line.trim_start_matches("title:").trim().trim_matches('"').to_string());
        }
    }
    
    Ok("Untitled".to_string())
}

/// Delete content using the provided options
pub fn delete_content(options: &DeleteOptions) -> Result<(String, String, String)> {
    // Find content to delete
    let (content_dir, topic) = match &options.slug {
        Some(slug) => find_content_dir(slug, options.topic.as_deref())?,
        None => {
            // This would be handled in the CLI, but we return an error here
            return Err(anyhow::anyhow!("No slug provided for deletion"));
        }
    };
    
    let slug = content_dir.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_string();
    
    // Get content title for confirmation
    let content_file = content_dir.join("index.mdx");
    let title = extract_title_from_content(&content_file)?;
    
    // Delete content directory
    common_fs::delete_dir_all(&content_dir)?;
    
    Ok((topic, slug, title))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_extract_title_from_content() {
        // Create a temporary file with test content
        let mut file = NamedTempFile::new().unwrap();
        let content = r#"---
title: "Test Article"
published: "2023-01-01"
tags:
  - test
---

# Test Content

This is a test article."#;
        
        file.write_all(content.as_bytes()).unwrap();
        
        // Test extracting the title
        let title = extract_title_from_content(file.path()).unwrap();
        assert_eq!(title, "Test Article");
    }
} 