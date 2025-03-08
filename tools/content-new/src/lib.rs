use anyhow::{Context, Result};
use chrono::Local;
use colored::*;
use common_config;
use common_fs;
use common_markdown;
use common_models::{Config, TopicConfig};
use slug::slugify;
use std::path::{Path, PathBuf};

/// Configuration for creating new content
pub struct ContentOptions {
    pub title: String,
    pub topic: String,
    pub tagline: String,
    pub tags: String,
    pub content_type: String,
    pub draft: bool,
}

/// Get the path to the template for a specific content type
pub fn get_template_path(content_type: &str) -> PathBuf {
    match content_type {
        "article" => PathBuf::from("templates/article-template.mdx"),
        "note" => PathBuf::from("templates/note-template.mdx"),
        _ => PathBuf::from("templates/article-template.mdx"), // Default to article template
    }
}

/// Create new content with the given options
pub fn create_content(options: ContentOptions) -> Result<String> {
    // Load configuration
    let config = common_config::load_config()?;
    
    // Validate topic
    if !config.content.topics.contains_key(&options.topic) {
        let valid_topics: Vec<String> = config.content.topics.keys()
            .map(|k| k.to_string())
            .collect();
        return Err(anyhow::anyhow!(
            "Invalid topic: {}. Valid topics are: {}", 
            options.topic, 
            valid_topics.join(", ")
        ));
    }
    
    // Generate slug from title
    let slug = slugify(&options.title);
    
    // Get topic path from config
    let topic_path = &config.content.topics[&options.topic].path;
    
    // Create content directory
    let content_dir = format!("{}/{}/{}", config.content.base_dir, topic_path, slug);
    let content_dir_path = Path::new(&content_dir);
    common_fs::create_dir_all(content_dir_path)?;
    
    // Read template
    let template_path = get_template_path(&options.content_type);
    let template = common_fs::read_file(&template_path)
        .context(format!("Failed to read template: {:?}", template_path))?;
    
    // Format tags
    let formatted_tags = options.tags
        .split(',')
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .map(|t| format!("    \"{}\",", t))
        .collect::<Vec<_>>()
        .join("\n");
    
    // Replace placeholders in template
    let today = if options.draft {
        "DRAFT".to_string()
    } else {
        Local::now().format("%Y-%m-%d").to_string()
    };
    
    let content_content = template
        .replace("Article Title", &options.title)
        .replace("A compelling single-sentence description of the article.", &options.tagline)
        .replace("article-slug", &slug)
        .replace("\"primary-topic\", \"secondary-topic\"", &format!("\"{}\"", options.topic))
        .replace("    \"tag1\",\n    \"tag2\",\n    \"tag3\",", &formatted_tags)
        .replace("YYYY-MM-DD", &today);
    
    // Write content file
    let content_path = format!("{}/index.mdx", content_dir);
    let content_path_obj = Path::new(&content_path);
    common_fs::write_file(content_path_obj, &content_content)?;
    
    Ok(content_path)
}

/// Get all available topics from the configuration
pub fn get_available_topics() -> Result<Vec<(String, TopicConfig)>> {
    let config = common_config::load_config()?;
    
    let topics: Vec<(String, TopicConfig)> = config.content.topics
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    
    Ok(topics)
} 