//! Frontmatter implementation
//!
//! This module provides functionality to extract and manipulate frontmatter.

use std::path::Path;
use common_models::Frontmatter;

use crate::errors::ContentEditError;

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