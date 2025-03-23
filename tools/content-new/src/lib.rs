use anyhow::Result;
use common_config::load_config;
use common_fs::{create_dir_all, write_file};
use common_models::{Frontmatter, TopicConfig};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum ContentNewError {
    /// Topic not found
    TopicNotFound(String),
    /// Slug already exists
    SlugAlreadyExists(String),
    /// Invalid slug
    InvalidSlug(String),
    /// Required field missing
    MissingRequiredField(String),
    /// IO error
    IoError(std::io::Error),
}

impl fmt::Display for ContentNewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentNewError::TopicNotFound(topic) => write!(f, "Topic not found: {}", topic),
            ContentNewError::SlugAlreadyExists(slug) => {
                write!(f, "Content with slug already exists: {}", slug)
            }
            ContentNewError::InvalidSlug(slug) => write!(f, "Invalid slug: {}", slug),
            ContentNewError::MissingRequiredField(field) => {
                write!(f, "Missing required field: {}", field)
            }
            ContentNewError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl Error for ContentNewError {}

impl From<std::io::Error> for ContentNewError {
    fn from(err: std::io::Error) -> Self {
        ContentNewError::IoError(err)
    }
}

impl From<ContentNewError> for anyhow::Error {
    fn from(err: ContentNewError) -> Self {
        anyhow::anyhow!(err.to_string())
    }
}

pub struct NewOptions {
    pub slug: String,
    pub title: String,
    pub topic: String,
    pub description: Option<String>,
    pub template: Option<String>,
    pub tags: Option<Vec<String>>,
    pub draft: Option<bool>,
    pub publish: bool,
    pub subtitle: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub aliases: Option<Vec<String>>,
}

/// Create new content
///
/// This function creates a new piece of content based on the provided options.
///
/// # Parameters
///
/// * `options` - Creation options
///
/// # Returns
///
/// Returns the path to the created content
///
/// # Errors
///
/// Returns an error if the creation fails
pub fn create_new_content(options: &NewOptions) -> Result<PathBuf> {
    let config = load_config()?;

    // Get the topic configuration
    let topic_config = config
        .content
        .topics
        .get(&options.topic)
        .ok_or_else(|| ContentNewError::TopicNotFound(options.topic.clone()))?;

    // Verify the slug doesn't already exist in other topics
    for (topic_key, topic_conf) in &config.content.topics {
        if topic_key == &options.topic {
            continue;
        }

        let topic_dir = PathBuf::from(&config.content.base_dir).join(&topic_conf.directory);
        let article_dir = topic_dir.join(&options.slug);

        if article_dir.exists() {
            return Err(ContentNewError::SlugAlreadyExists(options.slug.clone()).into());
        }
    }

    // Create the content
    let topic_dir = PathBuf::from(&config.content.base_dir).join(&topic_config.directory);
    let article_dir = topic_dir.join(&options.slug);

    // Create the directory if it doesn't exist
    if !article_dir.exists() {
        std::fs::create_dir_all(&article_dir)?;
    }

    // Create the content file path using the slug name format
    let content_file = article_dir.join(format!("{}.md", options.slug));

    // Check if the file already exists
    if content_file.exists() {
        return Err(ContentNewError::SlugAlreadyExists(options.slug.clone()).into());
    }

    // Create the frontmatter
    let mut frontmatter = HashMap::new();

    // Add title to frontmatter
    frontmatter.insert("title".to_string(), options.title.clone());

    // Add optional description if provided
    if let Some(description) = &options.description {
        frontmatter.insert("description".to_string(), description.clone());
    }

    // Add published status
    frontmatter.insert("published".to_string(), "true".to_string());

    // Add draft status
    let is_draft = options.draft.unwrap_or(false);
    frontmatter.insert("draft".to_string(), is_draft.to_string());

    // Add optional subtitle if provided
    if let Some(subtitle) = &options.subtitle {
        frontmatter.insert("subtitle".to_string(), subtitle.clone());
    }

    // Add optional author if provided
    if let Some(author) = &options.author {
        frontmatter.insert("author".to_string(), author.clone());
    }

    // Add date (either provided or current)
    if let Some(date) = &options.date {
        frontmatter.insert("date".to_string(), date.clone());
    } else {
        let now = chrono::Local::now();
        frontmatter.insert("date".to_string(), now.format("%Y-%m-%d").to_string());
    }

    // Add aliases if provided
    if let Some(aliases) = &options.aliases {
        if !aliases.is_empty() {
            let aliases_string = format!("[{}]", aliases.join(", "));
            frontmatter.insert("aliases".to_string(), aliases_string);
        }
    }

    // Add tags if provided
    if let Some(tags) = &options.tags {
        if !tags.is_empty() {
            let tags_string = format!("[{}]", tags.join(", "));
            frontmatter.insert("tags".to_string(), tags_string);
        }
    }

    // Create the content
    let mut content = String::new();

    // Add the frontmatter
    content.push_str("---\n");
    for (key, value) in &frontmatter {
        content.push_str(&format!("{}: {}\n", key, value));
    }
    content.push_str("---\n\n");

    // Add the content
    content.push_str(&format!("# {}\n\n", options.title));
    if let Some(subtitle) = &options.subtitle {
        content.push_str(&format!("_{}_\n\n", subtitle));
    }
    content.push_str("Your content here...\n");

    // Write the content to the file
    std::fs::write(&content_file, content)?;

    // Return the path to the created content
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

    let topics: Vec<(String, TopicConfig)> = config
        .content
        .topics
        .iter()
        .map(|(key, config)| (key.clone(), config.clone()))
        .collect();

    Ok(topics)
}
