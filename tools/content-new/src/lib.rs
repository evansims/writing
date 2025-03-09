use common_errors::{Result, WritingError, ResultExt};
use chrono::Local;
use common_config;
use common_fs;
use common_templates;
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
    pub template: Option<String>,
    pub introduction: Option<String>,
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
        return Err(WritingError::topic_error(format!(
            "Invalid topic: {}. Valid topics are: {}", 
            options.topic, 
            valid_topics.join(", ")
        )));
    }
    
    // Generate slug from title
    let slug = slugify(&options.title);
    
    // Get topic path from config
    let topic_path = &config.content.topics[&options.topic].path;
    
    // Create content directory
    let content_dir = format!("{}/{}/{}", config.content.base_dir, topic_path, slug);
    let content_dir_path = Path::new(&content_dir);
    common_fs::create_dir_all(content_dir_path)?;
    
    // Load template
    let mut template = match &options.template {
        Some(template_name) => {
            common_templates::load_template(template_name)
                .with_context(|| format!("Failed to load template: {}", template_name))?
        },
        None => {
            common_templates::load_template_for_content_type(&options.content_type)
                .with_context(|| format!("Failed to load template for content type: {}", options.content_type))?
        }
    };
    
    // Format tags
    let formatted_tags = if options.tags.is_empty() {
        "".to_string()
    } else {
        options.tags
            .split(',')
            .map(|t| t.trim())
            .filter(|t| !t.is_empty())
            .map(|t| format!("    \"{}\",", t))
            .collect::<Vec<_>>()
            .join("\n")
    };
    
    // Prepare variables for template
    let today = if options.draft {
        "DRAFT".to_string()
    } else {
        Local::now().format("%Y-%m-%d").to_string()
    };
    
    let draft_value = if options.draft { "true" } else { "false" };
    
    let introduction = options.introduction.unwrap_or_else(|| 
        "Start with a compelling introduction that hooks the reader and outlines what they'll learn.".to_string()
    );
    
    let variables = [
        ("title", options.title.as_str()),
        ("tagline", options.tagline.as_str()),
        ("slug", slug.as_str()),
        ("topic", options.topic.as_str()),
        ("tags", formatted_tags.as_str()),
        ("date", today.as_str()),
        ("draft", draft_value),
        ("introduction", introduction.as_str()),
        ("content_type", options.content_type.as_str()),
    ];
    
    // Render the template with variables
    let content = template.render(&variables)
        .with_context(|| "Failed to render template")?;
    
    // Write content file
    let content_path = format!("{}/index.mdx", content_dir);
    let content_path_obj = Path::new(&content_path);
    common_fs::write_file(content_path_obj, &content)?;
    
    Ok(content_path)
}

/// List available templates
pub fn list_templates() -> Result<Vec<common_templates::Template>> {
    common_templates::list_templates()
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

/// Create a new template
pub fn create_template(name: &str, content_type: &str, content: &str) -> Result<common_templates::Template> {
    common_templates::create_template(name, content_type, content)
} 