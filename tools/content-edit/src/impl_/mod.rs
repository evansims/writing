//! Implementation details for the content-edit module.
//!
//! This file contains the implementation of the public API functions.
//! It should not be directly used by external code.

use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;
use common_errors::ResultExt;
use common_fs::normalize::join_paths;
use common_models::Frontmatter;

use crate::errors::ContentEditError;
use crate::models::{EditOptions, EditableContent};

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
            reason: "Content directory not found".to_string()
        });
    }

    // If topic is provided, only look in that topic directory
    if let Some(topic_name) = topic {
        let topic_dir = content_dir.join(topic_name);
        if !topic_dir.exists() {
            return Err(ContentEditError::ContentNotFound {
                slug: slug.to_string(),
                topic: Some(topic_name.to_string())
            });
        }

        let content_path = topic_dir.join(slug).join("index.md");
        if content_path.exists() {
            return Ok(content_path);
        }

        let content_path_mdx = topic_dir.join(slug).join("index.mdx");
        if content_path_mdx.exists() {
            return Ok(content_path_mdx);
        }

        return Err(ContentEditError::ContentNotFound {
            slug: slug.to_string(),
            topic: Some(topic_name.to_string())
        });
    }

    // If no topic is provided, look in all topic directories
    let mut content_list: Vec<(String, String, PathBuf)> = Vec::new();

    for topic_dir in fs::read_dir(&content_dir)? {
        let topic_dir = topic_dir?.path();
        if topic_dir.is_dir() {
            let topic_key = topic_dir.file_name()
                .ok_or_else(|| ContentEditError::InvalidPath {
                    path: topic_dir.clone(),
                    reason: "Invalid directory name (not UTF-8)".to_string()
                })?
                .to_string_lossy()
                .to_string();

            let content_path = topic_dir.join(slug).join("index.md");
            if content_path.exists() {
                return Ok(content_path);
            }

            let content_path_mdx = topic_dir.join(slug).join("index.mdx");
            if content_path_mdx.exists() {
                return Ok(content_path_mdx);
            }

            // Scan the content directory to try to find a match by title
            for entry in fs::read_dir(&topic_dir)? {
                let entry = entry?.path();
                if !entry.is_dir() {
                    continue;
                }

                let content_path = entry.join("index.md");
                if !content_path.exists() {
                    continue;
                }

                let content = fs::read_to_string(&content_path)?;
                let frontmatter = extract_frontmatter_from_string(&content)?;

                let title = match frontmatter.get("title") {
                    Some(serde_yaml::Value::String(t)) => t.to_string(),
                    _ => entry.file_name()
                        .ok_or_else(|| ContentEditError::InvalidPath {
                            path: entry.clone(),
                            reason: "Invalid directory name (not UTF-8)".to_string()
                        })?
                        .to_string_lossy()
                        .to_string(),
                };

                content_list.push((topic_key.clone(), title, content_path));
            }
        }
    }

    Err(ContentEditError::ContentNotFound {
        slug: slug.to_string(),
        topic: None
    })
}

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

/// Extract frontmatter from a string.
///
/// This function extracts the YAML frontmatter from a content string.
///
/// # Arguments
///
/// * `content` - The content string containing frontmatter
///
/// # Returns
///
/// Returns the parsed YAML frontmatter as a `serde_yaml::Value`.
///
/// # Errors
///
/// Returns an error if:
/// * The content does not contain frontmatter
/// * The frontmatter format is invalid
/// * The frontmatter cannot be parsed as YAML
///
/// # Examples
///
/// ```no_run
/// use content_edit::extract_frontmatter_from_string;
///
/// let content = r#"---
/// title: "Example Post"
/// date: "2023-01-01"
/// ---
///
/// # Example Post
///
/// This is an example post.
/// "#;
///
/// let frontmatter = extract_frontmatter_from_string(content).unwrap();
/// let title = frontmatter.get("title").unwrap().as_str().unwrap();
/// println!("Title: {}", title);
/// ```
pub fn extract_frontmatter_from_string(content: &str) -> Result<serde_yaml::Value, ContentEditError> {
    let content = content.trim_start();

    if !content.starts_with("---") {
        return Err(ContentEditError::InvalidFormat {
            reason: "Content does not contain frontmatter".to_string()
        });
    }

    // Find the end of the frontmatter
    let rest = &content[3..];
    if let Some(end_index) = rest.find("---") {
        let frontmatter_str = &rest[..end_index].trim();

        // Parse the frontmatter as YAML
        let frontmatter = serde_yaml::from_str(frontmatter_str)
            .map_err(|e| ContentEditError::InvalidFormat {
                reason: format!("Failed to parse frontmatter: {}", e)
            })?;

        Ok(frontmatter)
    } else {
        Err(ContentEditError::InvalidFormat {
            reason: "Invalid frontmatter format".to_string()
        })
    }
}

/// Extract frontmatter from a file.
///
/// This function reads a file and extracts its frontmatter.
///
/// # Arguments
///
/// * `path` - The path to the content file
///
/// # Returns
///
/// Returns the parsed frontmatter as a `Frontmatter` object.
///
/// # Errors
///
/// Returns an error if:
/// * The file cannot be read
/// * The frontmatter format is invalid
/// * The frontmatter cannot be parsed
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use content_edit::extract_frontmatter;
///
/// let path = Path::new("content/blog/example/index.md");
/// let frontmatter = extract_frontmatter(path).unwrap();
/// println!("Title: {}", frontmatter.title);
/// ```
pub fn extract_frontmatter(path: &Path) -> Result<Frontmatter, ContentEditError> {
    let content = common_fs::read_file(path)
        .map_err(|e| ContentEditError::FileSystem {
            error: std::io::Error::new(std::io::ErrorKind::Other, e.to_string())
        })?;

    let (frontmatter, _) = split_frontmatter_and_body(&content)
        .map_err(|e| ContentEditError::InvalidFormat {
            reason: e.to_string()
        })?;

    Ok(frontmatter)
}

/// Split content into frontmatter and body.
///
/// This function splits a content string into its frontmatter and body parts.
///
/// # Arguments
///
/// * `content` - The content string to split
///
/// # Returns
///
/// Returns a tuple containing the frontmatter and body.
///
/// # Errors
///
/// Returns an error if:
/// * The frontmatter format is invalid
/// * The frontmatter cannot be parsed
///
/// # Examples
///
/// ```no_run
/// use content_edit::split_frontmatter_and_body;
///
/// let content = r#"---
/// title: "Example Post"
/// date: "2023-01-01"
/// ---
///
/// # Example Post
///
/// This is an example post.
/// "#;
///
/// let (frontmatter, body) = split_frontmatter_and_body(content).unwrap();
/// println!("Title: {}", frontmatter.title);
/// println!("Body starts with: {}", body.lines().next().unwrap());
/// ```
pub fn split_frontmatter_and_body(content: &str) -> Result<(Frontmatter, String), ContentEditError> {
    let (frontmatter, body) = common_markdown::extract_frontmatter_and_content(content)
        .map_err(|e| ContentEditError::InvalidFormat {
            reason: format!("Failed to extract frontmatter: {}", e)
        })?;

    Ok((frontmatter, body))
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