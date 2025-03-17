use anyhow::Result;
use common_config::load_config;
use common_fs::{create_dir_all, write_file};
use common_models::{Frontmatter, TopicConfig};
use std::path::PathBuf;

pub struct NewOptions {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub topic: Option<String>,
    pub description: Option<String>,
    pub template: Option<String>,
    pub tags: Option<Vec<String>>,
    pub draft: Option<bool>,
}

/// Create new content
///
/// This function creates new content in the specified topic.
///
/// # Parameters
///
/// * `options` - New content options
///
/// # Returns
///
/// Returns the path to the created content
///
/// # Errors
///
/// Returns an error if the content cannot be created
pub fn create_content(options: &NewOptions) -> Result<PathBuf> {
    // Validate options
    let slug = options.slug.as_ref().ok_or_else(|| anyhow::anyhow!("Content slug is required"))?;
    let topic = options.topic.as_ref().ok_or_else(|| anyhow::anyhow!("Topic is required"))?;
    let title = options.title.as_ref().ok_or_else(|| anyhow::anyhow!("Title is required"))?;
    
    // Load config
    let config = load_config()?;
    
    // Check if topic exists
    let topic_config = config.content.topics.get(topic)
        .ok_or_else(|| anyhow::anyhow!("Topic not found: {}", topic))?;
    
    // Create content directory
    let content_dir = PathBuf::from(&config.content.base_dir)
        .join(&topic_config.directory)
        .join(slug);
    
    if content_dir.exists() {
        return Err(anyhow::anyhow!("Content already exists: {}", slug));
    }
    
    create_dir_all(&content_dir)?;
    
    // Create content file
    let default_template = String::from("default");
    let template_name = options.template.as_ref().unwrap_or(&default_template);
    let mut template = common_templates::load_template(template_name)?;
    
    // Create frontmatter
    let topics = vec![topic.clone()];
    
    let frontmatter = Frontmatter {
        title: title.clone(),
        published: Some(chrono::Local::now().format("%Y-%m-%d").to_string()),
        updated: Some(chrono::Local::now().format("%Y-%m-%d").to_string()),
        slug: Some(slug.clone()),
        tagline: options.description.clone(),
        tags: options.tags.clone(),
        topics: Some(topics),
        draft: options.draft,
        featured_image: None,
    };
    
    // Convert frontmatter to YAML
    let frontmatter_yaml = serde_yaml::to_string(&frontmatter)?;
    
    // Create content with frontmatter
    let template_content = template.get_content()?;
    let content = format!("---\n{}---\n\n{}", frontmatter_yaml, template_content);
    
    // Write content to file
    let content_file = content_dir.join("index.md");
    write_file(&content_file, &content)?;
    
    Ok(content_file)
}

/// List available templates
///
/// This function lists all available templates.
///
/// # Returns
///
/// Returns a list of templates
///
/// # Errors
///
/// Returns an error if the templates cannot be listed
pub fn list_templates() -> Result<Vec<common_templates::Template>> {
    Ok(common_templates::list_templates()?)
}

/// Get available topics
///
/// This function lists all available topics.
///
/// # Returns
///
/// Returns a list of topics
///
/// # Errors
///
/// Returns an error if the topics cannot be listed
pub fn get_available_topics() -> Result<Vec<(String, TopicConfig)>> {
    let config = load_config()?;
    
    let topics: Vec<(String, TopicConfig)> = config.content.topics
        .iter()
        .map(|(key, config)| (key.clone(), config.clone()))
        .collect();
    
    Ok(topics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_available_topics() -> Result<()> {
        // This test just verifies that the function doesn't panic
        let topics = get_available_topics()?;
        
        // We should have at least one topic
        assert!(!topics.is_empty());
        
        Ok(())
    }
}
