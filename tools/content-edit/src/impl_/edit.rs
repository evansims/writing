//! Edit content implementation
//!
//! This module provides functionality to edit content in the repository.

use anyhow::Result;
use std::path::Path;
use common_models::Frontmatter;

use crate::errors::ContentEditError;
use crate::models::{EditOptions, EditableContent};
use super::find::find_content_path;
use super::frontmatter::split_frontmatter_and_body;

/// Edit content with the given options.
///
/// This function retrieves content for editing based on the provided options.
/// It returns an `EditableContent` object containing information about the content.
///
/// # Arguments
///
/// * `options` - Options for editing content, including slug, topic, and mode
///
/// # Returns
///
/// Returns an `EditableContent` object containing information about the content.
///
/// # Errors
///
/// Returns an error if:
/// * The configuration cannot be loaded
/// * The content cannot be found
/// * The content cannot be read
/// * The content path is invalid
///
/// # Examples
///
/// ```no_run
/// use content_edit::{EditOptions, edit_content};
///
/// // Create options for editing content
/// let options = EditOptions::new(
///     Some("example-post".to_string()),
///     Some("blog".to_string()),
///     false,
///     false
/// );
///
/// // Get the content to edit
/// let content = edit_content(&options).unwrap();
/// println!("Editing '{}' in topic '{}'", content.title, content.topic);
/// ```
pub fn edit_content(options: &EditOptions) -> Result<EditableContent, ContentEditError> {
    let _config = common_config::load_config()
        .map_err(|e| ContentEditError::Configuration {
            reason: format!("Failed to load config: {}", e)
        })?;

    let slug = options.slug
        .as_ref()
        .ok_or_else(|| ContentEditError::Validation {
            reason: "Slug is required for editing content".to_string()
        })?;

    let content_path = find_content_path(slug, options.topic.as_deref())?;

    let content = common_fs::read_file(&content_path)
        .map_err(|e| ContentEditError::FileSystem {
            error: std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
        })?;

    // Get the title from frontmatter
    let title = match common_markdown::extract_frontmatter_and_content(&content) {
        Ok((fm, _)) => fm.title,
        _ => "Untitled".to_string(),
    };

    // Get the topic from the path
    let topic = match content_path
        .parent() // Get the index.md parent directory
        .and_then(|p| p.parent()) // Get the topic directory
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str()) {
            Some(name) => name.to_string(),
            None => return Err(ContentEditError::InvalidPath {
                path: content_path.clone(),
                reason: "Unable to determine topic from path".to_string()
            }),
        };

    // Get the slug from the path
    let path_slug = match content_path
        .parent() // Get the index.md parent directory
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str()) {
            Some(name) => name.to_string(),
            None => slug.clone(), // Fall back to the provided slug if path parsing fails
        };

    Ok(EditableContent::new(
        content_path,
        topic,
        title,
        path_slug
    ))
}

/// Save edited content to a file.
///
/// This function saves the edited content to the specified path.
/// It can handle full content edits, frontmatter-only edits, and body-only edits.
///
/// # Arguments
///
/// * `content_path` - The path to the content file
/// * `edited_content` - The edited content to save
///
/// # Returns
///
/// Returns `Ok(())` if the content was saved successfully.
///
/// # Errors
///
/// Returns an error if:
/// * The original content cannot be read
/// * The frontmatter cannot be parsed or serialized
/// * The content cannot be saved
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use content_edit::save_edited_content;
///
/// // Save full content edit
/// let content_path = Path::new("content/blog/example/index.md");
/// let edited_content = "---\ntitle: \"Example\"\n---\n\n# Example\n\nThis is an example.";
/// save_edited_content(content_path, edited_content).unwrap();
///
/// // Save frontmatter-only edit
/// let frontmatter_only = "---\ntitle: \"Updated Example\"\n---";
/// save_edited_content(content_path, frontmatter_only).unwrap();
///
/// // Save body-only edit
/// let body_only = "# Updated Example\n\nThis content has been updated.";
/// save_edited_content(content_path, body_only).unwrap();
/// ```
pub fn save_edited_content(content_path: &Path, edited_content: &str) -> Result<(), ContentEditError> {
    // Check if we're editing only frontmatter or only content
    let is_frontmatter_only = edited_content.starts_with("---\n") &&
                             edited_content.trim_end().ends_with("---");

    let is_content_only = !edited_content.contains("---");

    if is_frontmatter_only || is_content_only {
        // Read the original content
        let original_content = common_fs::read_file(content_path)
            .map_err(|e| ContentEditError::FileSystem {
                error: std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
            })?;

        // Split the original content
        let (frontmatter, body) = split_frontmatter_and_body(&original_content)
            .map_err(|e| ContentEditError::InvalidFormat {
                reason: format!("Failed to parse original content: {}", e)
            })?;

        // Merge the edited part with the original
        if is_frontmatter_only {
            // Parse the edited frontmatter
            let edited_frontmatter = edited_content.trim_start_matches("---\n").trim_end_matches("---").trim();

            // Write the merged content
            common_fs::write_file(content_path, &format!("---\n{}---\n\n{}", edited_frontmatter, body))
                .map_err(|e| ContentEditError::FileSystem {
                    error: std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
                })?;
        } else {
            // Edited content is body only
            let frontmatter_yaml = serde_yaml::to_string(&frontmatter)
                .map_err(|e| ContentEditError::InvalidFormat {
                    reason: format!("Failed to serialize frontmatter: {}", e)
                })?;

            // Write the merged content
            common_fs::write_file(content_path, &format!("---\n{}---\n\n{}", frontmatter_yaml, edited_content))
                .map_err(|e| ContentEditError::FileSystem {
                    error: std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
                })?;
        }

        Ok(())
    } else {
        // Full content edit, just write it directly
        common_fs::write_file(content_path, edited_content)
            .map_err(|e| ContentEditError::FileSystem {
                error: std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
            })?;

        Ok(())
    }
}

/// Update content with new frontmatter and/or content.
///
/// This function updates a content file with new frontmatter and/or body content.
///
/// # Arguments
///
/// * `path` - The path to the content file
/// * `frontmatter` - Optional new frontmatter
/// * `content` - Optional new body content
///
/// # Returns
///
/// Returns `Ok(())` if the content was updated successfully.
///
/// # Errors
///
/// Returns an error if:
/// * The file cannot be read
/// * The frontmatter cannot be parsed or serialized
/// * The content cannot be saved
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use content_edit::update_content;
/// use common_models::Frontmatter;
///
/// let path = Path::new("content/blog/example/index.md");
///
/// // Create a new frontmatter
/// let mut frontmatter = Frontmatter::default();
/// frontmatter.title = "Updated Example".to_string();
///
/// // Update just the frontmatter
/// update_content(path, Some(frontmatter), None).unwrap();
///
/// // Update just the body
/// let new_body = "# Updated Example\n\nThis content has been updated.";
/// update_content(path, None, Some(new_body)).unwrap();
///
/// // Update both frontmatter and body
/// let mut frontmatter = Frontmatter::default();
/// frontmatter.title = "Completely Updated Example".to_string();
/// let new_body = "# Completely Updated\n\nThis content has been completely updated.";
/// update_content(path, Some(frontmatter), Some(new_body)).unwrap();
/// ```
pub fn update_content(path: &Path, frontmatter: Option<Frontmatter>, content: Option<&str>) -> Result<(), ContentEditError> {
    let current_content = common_fs::read_file(path)
        .map_err(|e| ContentEditError::FileSystem {
            error: std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
        })?;

    let (current_frontmatter, current_body) = split_frontmatter_and_body(&current_content)?;

    let new_frontmatter = frontmatter.unwrap_or(current_frontmatter);
    let new_body = content.unwrap_or(&current_body);

    let yaml = serde_yaml::to_string(&new_frontmatter)
        .map_err(|e| ContentEditError::InvalidFormat {
            reason: format!("Failed to serialize frontmatter: {}", e)
        })?;

    let updated_content = format!("---\n{}---\n\n{}", yaml, new_body);

    common_fs::write_file(path, &updated_content)
        .map_err(|e| ContentEditError::FileSystem {
            error: std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
        })?;

    Ok(())
}