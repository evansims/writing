//! Find content implementation
//!
//! This module provides functionality to find content by slug and topic.

use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use super::frontmatter::extract_frontmatter_from_string;
use crate::errors::ContentEditError;

/// Find the path to content by slug and optionally topic.
///
/// This function searches for content in the repository based on the provided slug.
/// If a topic is specified, it only looks in that topic directory.
/// Otherwise, it searches all topic directories.
///
/// # Arguments
///
/// * `slug` - The slug of the content to find
/// * `topic` - Optional topic to narrow the search
///
/// # Returns
///
/// Returns the path to the content file if found, or an error if not found.
///
/// # Errors
///
/// Returns an error if:
/// * The content directory does not exist
/// * The topic directory does not exist (if a topic is specified)
/// * The content is not found
///
/// # Examples
///
/// ```no_run
/// use content_edit::find_content_path;
///
/// // Find content with a specific slug in a specific topic
/// let path = find_content_path("example-post", Some("blog")).unwrap();
/// println!("Found at: {:?}", path);
///
/// // Find content with a specific slug in any topic
/// let path = find_content_path("example-post", None).unwrap();
/// println!("Found at: {:?}", path);
/// ```
pub fn find_content_path(slug: &str, topic: Option<&str>) -> Result<PathBuf, ContentEditError> {
    let content_dir = PathBuf::from("content");
    if !content_dir.exists() {
        return Err(ContentEditError::Validation {
            reason: "Content directory not found".to_string(),
        });
    }

    // If topic is provided, only look in that topic directory
    if let Some(topic_name) = topic {
        let topic_dir = content_dir.join(topic_name);
        if !topic_dir.exists() {
            return Err(ContentEditError::ContentNotFound {
                slug: slug.to_string(),
                topic: Some(topic_name.to_string()),
            });
        }

        // Check for the matching-name file with .md extension
        let content_path_md = topic_dir.join(slug).join(format!("{}.md", slug));
        if content_path_md.exists() {
            return Ok(content_path_md);
        }

        // Check for the matching-name file with .mdx extension
        let content_path_mdx = topic_dir.join(slug).join(format!("{}.mdx", slug));
        if content_path_mdx.exists() {
            return Ok(content_path_mdx);
        }

        return Err(ContentEditError::ContentNotFound {
            slug: slug.to_string(),
            topic: Some(topic_name.to_string()),
        });
    }

    // If no topic is provided, look in all topic directories
    let mut content_list: Vec<(String, String, PathBuf)> = Vec::new();

    for topic_dir in fs::read_dir(&content_dir)? {
        let topic_dir = topic_dir?.path();
        if topic_dir.is_dir() {
            let topic_key = topic_dir
                .file_name()
                .ok_or_else(|| ContentEditError::InvalidPath {
                    path: topic_dir.clone(),
                    reason: "Invalid directory name (not UTF-8)".to_string(),
                })?
                .to_string_lossy()
                .to_string();

            // Check for the matching-name file with .md extension
            let content_path_md = topic_dir.join(slug).join(format!("{}.md", slug));
            if content_path_md.exists() {
                return Ok(content_path_md);
            }

            // Check for the matching-name file with .mdx extension
            let content_path_mdx = topic_dir.join(slug).join(format!("{}.mdx", slug));
            if content_path_mdx.exists() {
                return Ok(content_path_mdx);
            }

            // Scan the content directory to try to find a match by title
            for entry in fs::read_dir(&topic_dir)? {
                let entry = entry?.path();
                if !entry.is_dir() {
                    continue;
                }

                let entry_slug = entry
                    .file_name()
                    .ok_or_else(|| ContentEditError::InvalidPath {
                        path: entry.clone(),
                        reason: "Invalid directory name (not UTF-8)".to_string(),
                    })?
                    .to_string_lossy()
                    .to_string();

                // Only check for the file with matching name
                let content_path = entry.join(format!("{}.md", entry_slug));

                if !content_path.exists() {
                    continue;
                }

                let content = fs::read_to_string(&content_path)?;
                let frontmatter = extract_frontmatter_from_string(&content)?;

                let title = match frontmatter.get("title") {
                    Some(serde_yaml::Value::String(t)) => t.to_string(),
                    _ => entry_slug,
                };

                content_list.push((topic_key.clone(), title, content_path));
            }
        }
    }

    Err(ContentEditError::ContentNotFound {
        slug: slug.to_string(),
        topic: None,
    })
}
