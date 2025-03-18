//! List content implementation
//!
//! This module provides functionality to list all content in the repository.

use anyhow::Result;
use std::path::PathBuf;
use common_errors::ResultExt;
use common_fs::normalize::join_paths;

use crate::errors::ContentEditError;
use crate::models::EditableContent;

/// List all content in the repository.
///
/// This function retrieves a list of all content items in the repository,
/// including their paths, topics, titles, and slugs.
///
/// # Returns
///
/// Returns a list of `EditableContent` objects representing all content items.
///
/// # Errors
///
/// Returns an error if:
/// * The configuration cannot be loaded
/// * A directory cannot be read
/// * Content cannot be parsed
///
/// # Examples
///
/// ```no_run
/// use content_edit::list_all_content;
///
/// let content_list = list_all_content().unwrap();
/// println!("Found {} content items", content_list.len());
///
/// for (i, content) in content_list.iter().enumerate() {
///     println!("{}. {} (topic: {}, slug: {})",
///         i + 1, content.title, content.topic, content.slug);
/// }
/// ```
pub fn list_all_content() -> Result<Vec<EditableContent>, ContentEditError> {
    let config = common_config::load_config()
        .with_context(|| "Failed to load configuration".to_string())
        .map_err(|e| ContentEditError::Configuration {
            reason: format!("Failed to load configuration: {}", e)
        })?;

    let mut content_list = Vec::new();

    for (topic_key, topic_config) in &config.content.topics {
        let base_dir = PathBuf::from(&config.content.base_dir);
        let topic_dir = join_paths(base_dir, &topic_config.directory);

        if !topic_dir.exists() {
            continue;
        }

        // Find all subdirectories in the topic directory
        let article_dirs = common_fs::find_dirs_with_depth(&topic_dir, 1, 1)
            .map_err(|e| ContentEditError::FileSystem {
                error: std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
            })?;

        for article_dir in article_dirs {
            let slug = article_dir.file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| ContentEditError::InvalidPath {
                    path: article_dir.clone(),
                    reason: format!("Invalid directory name in topic '{}'", topic_key)
                })?
                .to_string();

            // Look for index.md or index.mdx
            let index_md = article_dir.join("index.md");
            let index_mdx = article_dir.join("index.mdx");

            let content_path = if index_md.exists() {
                index_md
            } else if index_mdx.exists() {
                index_mdx
            } else {
                continue;
            };

            // Extract title from frontmatter
            let content = common_fs::read_file(&content_path)
                .map_err(|e| ContentEditError::FileSystem {
                    error: std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
                })?;

            let title = match common_markdown::extract_frontmatter_and_content(&content) {
                Ok((fm, _)) => {
                    if fm.title.is_empty() {
                        slug.clone()
                    } else {
                        fm.title
                    }
                },
                _ => slug.clone(),
            };

            content_list.push(EditableContent::new(
                content_path,
                topic_key.clone(),
                title,
                slug
            ));
        }
    }

    Ok(content_list)
}