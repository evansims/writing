use anyhow::Result;
use common_fs::normalize::{normalize_path, join_paths};
use common_errors::{WritingError, ErrorContext, IoResultExt};
use common_cli::{Command, ContentCommand, DisplayResult};
use common_traits::tools::ContentDeleter;
use clap::Parser;
use std::path::{Path, PathBuf};
use colored::*;
use std::fs;

/// CLI arguments for the content-delete command
#[derive(Parser, Debug)]
#[command(author, version, about = "Delete existing content")]
pub struct DeleteArgs {
    /// Content slug to delete
    #[arg(short, long)]
    pub slug: Option<String>,

    /// Topic (optional, will search if not provided)
    #[arg(short, long)]
    pub topic: Option<String>,

    /// Force delete without confirmation
    #[arg(short, long)]
    pub force: bool,
}

/// Command for deleting content
pub struct DeleteCommand {
    args: DeleteArgs,
}

/// Result of the delete operation
#[derive(Debug)]
pub struct DeleteResult {
    pub topic: String,
    pub slug: String,
    pub title: String,
}

impl DisplayResult for DeleteResult {
    fn to_display(&self) -> String {
        format!("{} Content deleted: {}/{} ({})",
            "SUCCESS:".green().bold(),
            self.topic, self.slug, self.title
        )
    }
}

impl Command for DeleteCommand {
    type Args = DeleteArgs;
    type Output = DeleteResult;

    fn new(args: Self::Args) -> Self {
        DeleteCommand { args }
    }

    fn execute(&self) -> Result<Self::Output> {
        // If no slug is provided, this would typically involve UI interaction
        // which we would delegate to the main.rs CLI interface
        if self.args.slug.is_none() {
            return Err(WritingError::validation_error("No slug provided. Use the CLI to select content interactively.").into());
        }

        let slug = self.validate_slug(&self.args.slug)?;
        let topic = self.validate_topic(&self.args.topic)?;

        // Find the content directory
        let (content_dir, topic_name) = find_content_dir(&slug, topic.as_deref())?;

        // Get content title for confirmation
        let content_file = join_paths(&content_dir, "index.mdx");
        let title = extract_title_from_content(&content_file)?;

        // Delete content directory with enhanced context
        std::fs::remove_dir_all(&content_dir)
            .with_enhanced_context(|| {
                ErrorContext::new("delete content directory")
                    .with_file(&content_dir)
                    .with_details("Unable to remove directory")
            })?;

        Ok(DeleteResult {
            topic: topic_name,
            slug,
            title,
        })
    }

    fn handle_result(result: Self::Output) {
        result.print();
    }
}

impl ContentCommand for DeleteCommand {}

/// Options for content deletion
#[derive(Default)]
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
        let base_dir = PathBuf::from(&config.content.base_dir);
        let topic_path = &topic_config.directory;

        // Use join_paths to properly handle path components
        let content_dir = join_paths(&base_dir, join_paths(topic_path, slug));

        if content_dir.exists() {
            return Ok((normalize_path(content_dir), topic_key.to_string()));
        }

        return Err(anyhow::anyhow!("Content not found: {}", content_dir.display()));
    }

    // Search for content in all topics
    for (topic_key, topic_config) in &config.content.topics {
        let base_dir = PathBuf::from(&config.content.base_dir);
        let topic_path = &topic_config.directory;

        // Use join_paths to properly handle path components
        let content_dir = join_paths(&base_dir, join_paths(topic_path, slug));

        if content_dir.exists() {
            return Ok((normalize_path(content_dir), topic_key.clone()));
        }
    }

    Err(anyhow::anyhow!("Content not found for slug: {}", slug))
}

/// List all content in the repository
pub fn list_all_content() -> Result<Vec<(String, String, PathBuf)>> {
    let config = common_config::load_config()?;
    let mut content_list = Vec::new();

    for (topic_key, topic_config) in &config.content.topics {
        let base_dir = PathBuf::from(&config.content.base_dir);
        let topic_dir = join_paths(&base_dir, &topic_config.directory);

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

            let index_path = join_paths(&article_dir, "index.mdx");
            if index_path.exists() {
                content_list.push((topic_key.clone(), slug, normalize_path(article_dir)));
            }
        }
    }

    Ok(content_list)
}

/// Extract the title from a content file
pub fn extract_title_from_content(content_path: &Path) -> Result<String> {
    // Normalize the path before reading the file
    let normalized_path = normalize_path(content_path);
    let content = common_fs::read_file(&normalized_path)?;

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

/// Delete content with the given options
///
/// This function deletes content based on the provided options.
///
/// # Parameters
///
/// * `options` - Delete options
///
/// # Returns
///
/// Returns the path to the deleted content
///
/// # Errors
///
/// Returns an error if the deletion fails
pub fn delete_content(options: &DeleteOptions) -> Result<String> {
    // Validate options
    let slug = options.slug.as_deref()
        .ok_or_else(|| WritingError::validation_error("Slug is required for deleting content"))?;

    // Load configuration
    let config = common_config::load_config()
        .map_err(|e| WritingError::config_error(format!("Failed to load config: {}", e)))?;

    // If topic is provided, look in that topic directory
    if let Some(topic_key) = &options.topic {
        let topic_config = config.content.topics.get(topic_key)
            .ok_or_else(|| WritingError::topic_error(format!("Topic '{}' not found", topic_key)))?;

        let topic_dir = join_paths(&config.content.base_dir, &topic_config.directory);
        let content_dir = topic_dir.join(slug);

        if !content_dir.exists() {
            return Err(WritingError::content_not_found(format!("Content with slug '{}' not found in topic '{}'", slug, topic_key)).into());
        }

        // Delete the content directory
        fs::remove_dir_all(&content_dir)
            .map_err(|e| WritingError::validation_error(format!("Failed to delete content directory: {}", e)))?;

        Ok(content_dir.to_string_lossy().to_string())
    } else {
        // No topic provided, search all topics
        for topic_config in config.content.topics.values() {
            let topic_dir = join_paths(&config.content.base_dir, &topic_config.directory);
            let content_dir = topic_dir.join(slug);

            if content_dir.exists() {
                // Delete the content directory
                fs::remove_dir_all(&content_dir)
                    .map_err(|e| WritingError::validation_error(format!("Failed to delete content directory: {}", e)))?;

                return Ok(content_dir.to_string_lossy().to_string());
            }
        }

        Err(WritingError::content_not_found(format!("Content with slug '{}' not found in any topic", slug)).into())
    }
}

// Add ContentDeleterImpl struct to implement ContentDeleter trait
/// Implementation of ContentDeleter trait for the content-delete tool
pub struct ContentDeleterImpl;

impl ContentDeleterImpl {
    /// Create a new ContentDeleterImpl
    pub fn new() -> Self {
        ContentDeleterImpl
    }
}

impl ContentDeleter for ContentDeleterImpl {
    /// Check if content can be safely deleted
    fn can_delete(&self, slug: &str, topic: Option<&str>) -> Result<bool, WritingError> {
        // Check if the content exists by trying to find its directory
        match find_content_dir(slug, topic) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Delete content
    fn delete_content(&self, slug: &str, topic: Option<&str>, force: bool) -> Result<(), WritingError> {
        // Create DeleteOptions from parameters
        let options = DeleteOptions {
            slug: Some(slug.to_string()),
            topic: topic.map(String::from),
            force,
        };

        // Use the existing delete_content function
        delete_content(&options)
            .map(|_| ())
            .map_err(|e| WritingError::other(e.to_string()))
    }
}
