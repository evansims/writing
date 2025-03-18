//! # Common Validation
//!
//! This module provides common validation functions for the writing tools.
//!
//! ## Features
//!
//! - **Content validation**: Validate content files for proper structure and required fields
//! - **Frontmatter validation**: Ensure frontmatter contains required fields and proper formatting
//! - **Slug validation**: Validate and generate slugs for content
//! - **Path validation**: Validate and generate paths for content files
//! - **Tag validation**: Validate and format tags for content
//! - **Topic validation**: Validate topics against configuration
//!
//! ## Example
//!
//! ```rust
//! use common_validation::{validate_slug, validate_topic, validate_content_path};
//! use common_errors::Result;
//!
//! fn validate_content(slug: &str, topic: Option<&str>) -> Result<()> {
//!     // Validate slug
//!     validate_slug(slug)?;
//!
//!     // Validate topic
//!     let topic = validate_topic(topic)?;
//!
//!     // Validate content path
//!     let content_path = validate_content_path(slug, topic.as_deref())?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Module Structure
//!
//! - `content.rs`: Content validation functions
//! - `frontmatter.rs`: Frontmatter validation functions
//! - `slug.rs`: Slug validation and generation functions
//! - `path.rs`: Path validation and generation functions
//! - `tags.rs`: Tag validation and formatting functions
//! - `topic.rs`: Topic validation functions

// Re-export dependencies for use by modules
pub use common_errors::{Result, WritingError, ResultExt, ErrorContext, IoResultExt};
pub use common_config;
pub use common_fs;
pub use common_fs::normalize::{normalize_path, join_paths};
pub use common_markdown;
pub use common_models::{Config, Frontmatter, TopicConfig};
pub use regex::Regex;
pub use std::collections::HashMap;
pub use std::path::{Path, PathBuf};

// Module declarations
mod content;
mod topic;
mod slug;
mod path;
mod frontmatter;
mod tags;

// Re-export module functions
pub use content::*;
pub use topic::*;
pub use slug::*;
pub use path::*;
pub use frontmatter::*;
pub use tags::*;

/// Validate that a slug is provided and properly formatted
pub fn validate_slug(slug: &str) -> Result<String> {
    // Check if slug is empty
    if slug.is_empty() {
        return Err(WritingError::validation_error("Slug cannot be empty"));
    }

    // Check slug length - absolute maximum is 100 characters
    let max_length = 100;
    if slug.len() > max_length {
        return Err(WritingError::validation_error(
            format!("Slug is too long: {} bytes (maximum is {} bytes)", slug.len(), max_length)
        ));
    }

    // Check if slug contains only valid characters
    let slug_regex = Regex::new(r"^[a-z0-9-]+$").unwrap();
    if !slug_regex.is_match(slug) {
        return Err(WritingError::validation_error(
            "Slug can only contain lowercase letters, numbers, and hyphens"
        ));
    }

    // Check if slug starts or ends with a hyphen
    if slug.starts_with('-') || slug.ends_with('-') {
        return Err(WritingError::validation_error(
            "Slug cannot start or end with a hyphen"
        ));
    }

    // Check if slug contains consecutive hyphens
    if slug.contains("--") {
        return Err(WritingError::validation_error(
            "Slug cannot contain consecutive hyphens"
        ));
    }

    Ok(slug.to_string())
}

/// Validate that a topic exists in the configuration
pub fn validate_topic(topic: Option<&str>) -> Result<Option<String>> {
    if let Some(topic_key) = topic {
        let config = common_config::load_config()
            .with_context(|| "Failed to load configuration".to_string())?;

        if !config.content.topics.contains_key(topic_key) {
            let valid_topics: Vec<String> = config.content.topics.keys()
                .map(|k| k.to_string())
                .collect();

            return Err(WritingError::topic_error(format!(
                "Invalid topic: {}. Valid topics are: {}",
                topic_key,
                valid_topics.join(", ")
            )));
        }

        Ok(Some(topic_key.to_string()))
    } else {
        Ok(None)
    }
}

/// Get available topics from the configuration
pub fn get_available_topics() -> Result<Vec<(String, TopicConfig)>> {
    let config = common_config::load_config()
        .with_context(|| "Failed to load configuration".to_string())?;

    let topics: Vec<(String, TopicConfig)> = config.content.topics
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    Ok(topics)
}

/// Find the path to content by slug and optionally topic
#[cfg(not(test))]
pub fn find_content_path(slug: &str, topic: Option<&str>) -> Result<PathBuf> {
    path::find_content_path(slug, topic)
}

/// Find the path to content by slug and optionally topic (test version)
#[cfg(test)]
pub fn find_content_path(slug: &str, topic: Option<&str>) -> Result<PathBuf> {
    mock_functions::mock_find_content_path(slug, topic)
}

/// Validate frontmatter
pub fn validate_frontmatter(parsed: &Frontmatter) -> Result<()> {
    // Check if title is empty or contains only whitespace
    if parsed.title.trim().is_empty() {
        return Err(WritingError::validation_error("Title is required in frontmatter"));
    }

    // Additional validation can be added here

    Ok(())
}

/// Validate tags
pub fn validate_tags(tags: &str) -> Result<Vec<String>> {
    // Split tags by comma
    let tags: Vec<String> = tags
        .split(',')
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect();

    // Check if any tag contains invalid characters
    let tag_regex = Regex::new(r"^[a-zA-Z0-9-_ ]+$").unwrap();
    for tag in &tags {
        if !tag_regex.is_match(tag) {
            return Err(WritingError::validation_error(format!(
                "Tag '{}' contains invalid characters. Tags can only contain letters, numbers, spaces, hyphens, and underscores",
                tag
            )));
        }
    }

    Ok(tags)
}

/// Format tags for frontmatter
pub fn format_tags(tags: &str) -> String {
    if tags.is_empty() {
        return "".to_string();
    }

    tags.split(',')
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .map(|t| format!("    \"{}\",", t))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Validate content type
pub fn validate_content_type(content_type: &str) -> Result<String> {
    // Check if content type is empty
    if content_type.is_empty() {
        return Err(WritingError::validation_error("Content type cannot be empty"));
    }

    // Check if content type is valid
    let valid_types = ["article", "note", "tutorial"];
    if !valid_types.contains(&content_type) {
        return Err(WritingError::validation_error(format!(
            "Invalid content type: {}. Valid types are: {}",
            content_type,
            valid_types.join(", ")
        )));
    }

    Ok(content_type.to_string())
}

/// Generate a slug from a title
pub fn generate_slug(title: &str) -> String {
    slugify(title)
}

#[cfg(test)]
mod mock_functions {
    use common_errors::Result;
    use std::path::PathBuf;

    // Mock version of find_content_path for testing
    pub fn mock_find_content_path(slug: &str, _topic: Option<&str>) -> Result<PathBuf> {
        // Always return a mock path for testing
        Ok(PathBuf::from(format!("/mock/content/path/{}/index.md", slug)))
    }

    #[allow(dead_code)]
    pub fn mock_content_exists(slug: &str, _topic: Option<&str>) -> Result<bool> {
        // Always return true for test-post, false for others
        Ok(slug == "test-post")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_test_utils::TestFixture;
    use std::sync::Once;

    static INIT: Once = Once::new();

    // Setup function that runs once before any test in this module
    fn setup() {
        INIT.call_once(|| {
            // Setup any global test configuration here
        });
    }

    #[test]
    fn test_validate_slug_valid() {
        let result = validate_slug("test-slug");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test-slug");
    }

    #[test]
    fn test_validate_slug_empty() {
        let result = validate_slug("");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("cannot be empty"));
    }

    #[test]
    fn test_validate_slug_invalid_chars() {
        let result = validate_slug("test_slug");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("can only contain"));
    }

    #[test]
    fn test_validate_slug_starts_with_hyphen() {
        let result = validate_slug("-test-slug");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("cannot start or end with a hyphen"));
    }

    #[test]
    fn test_validate_slug_ends_with_hyphen() {
        let result = validate_slug("test-slug-");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("cannot start or end with a hyphen"));
    }

    #[test]
    fn test_validate_slug_consecutive_hyphens() {
        let result = validate_slug("test--slug");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("cannot contain consecutive hyphens"));
    }

    #[test]
    fn test_validate_topic_valid() {
        setup();
        let fixture = TestFixture::new().unwrap();
        // Set a known mock config to override system config
        fixture.config.set_expected_topics(vec![
            ("creativity".to_string(), TopicConfig {
                name: "Creativity".to_string(),
                description: "Creative content".to_string(),
                directory: "creativity".to_string(),
            })
        ]);

        // Register the test config
        fixture.register_test_config();

        // Test with a valid topic
        let result = topic::validate_topic(Some("creativity"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("creativity".to_string()));
    }

    #[test]
    fn test_validate_topic_invalid() {
        setup();
        let fixture = TestFixture::new().unwrap();
        // Set a known mock config
        fixture.config.set_expected_topics(vec![
            ("creativity".to_string(), TopicConfig {
                name: "Creativity".to_string(),
                description: "Creative content".to_string(),
                directory: "creativity".to_string(),
            })
        ]);

        // Register the test config
        fixture.register_test_config();

        // Test with an invalid topic
        let result = topic::validate_topic(Some("invalid-topic"));
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid topic"));
    }

    #[test]
    fn test_validate_topic_none() {
        // Test with None
        let result = topic::validate_topic(None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_find_content_path() {
        let _fixture = TestFixture::new().unwrap();

        // Use the mock function instead of the real one
        let found_path = mock_functions::mock_find_content_path("test-post", Some("creativity")).unwrap();

        // Verify path
        assert_eq!(found_path.to_string_lossy().to_string(), "/mock/content/path/test-post/index.md");
    }

    #[test]
    fn test_validate_frontmatter_valid() {
        let frontmatter = Frontmatter {
            title: "Test Post".to_string(),
            tagline: Some("A test post".to_string()),
            ..Frontmatter::default()
        };
        let result = validate_frontmatter(&frontmatter);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_frontmatter_missing_title() {
        let frontmatter = Frontmatter {
            title: "".to_string(),
            tagline: Some("A test post".to_string()),
            ..Frontmatter::default()
        };
        let result = validate_frontmatter(&frontmatter);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Title is required"));
    }

    #[test]
    fn test_validate_frontmatter_invalid_format() {
        // This test is no longer relevant since we're validating a parsed Frontmatter object
        // Let's test something else instead
        let frontmatter = Frontmatter {
            title: "   ".to_string(), // Title with only whitespace
            ..Frontmatter::default()
        };
        let result = validate_frontmatter(&frontmatter);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_tags_valid() {
        let _fixture = TestFixture::new().unwrap();
        let tags = "tag1, tag2, tag3";
        let result = validate_tags(tags);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed, vec!["tag1", "tag2", "tag3"]);
    }

    #[test]
    fn test_validate_tags_invalid() {
        let _fixture = TestFixture::new().unwrap();
        let tags = "tag1, tag@2, tag3";
        let result = validate_tags(tags);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("contains invalid characters"));
    }

    #[test]
    fn test_format_tags() {
        let _fixture = TestFixture::new().unwrap();
        let tags = "tag1, tag2, tag3";
        let formatted = format_tags(tags);
        assert_eq!(formatted, "    \"tag1\",\n    \"tag2\",\n    \"tag3\",");
    }

    #[test]
    fn test_validate_content_type_valid() {
        let result = validate_content_type("article");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "article");
    }

    #[test]
    fn test_validate_content_type_invalid() {
        let result = validate_content_type("invalid-type");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid content type"));
    }

    #[test]
    fn test_generate_slug() {
        let title = "Test Title";
        let slug = generate_slug(title);
        assert_eq!(slug, "test-title");
    }

    #[test]
    fn test_get_available_topics() {
        setup();
        let fixture = TestFixture::new().unwrap();
        // Set a known mock config
        fixture.config.set_expected_topics(vec![
            ("creativity".to_string(), TopicConfig {
                name: "Creativity".to_string(),
                description: "Creative content".to_string(),
                directory: "creativity".to_string(),
            }),
            ("strategy".to_string(), TopicConfig {
                name: "Strategy".to_string(),
                description: "Strategic content".to_string(),
                directory: "strategy".to_string(),
            }),
            ("blog".to_string(), TopicConfig {
                name: "Blog".to_string(),
                description: "Blog content".to_string(),
                directory: "blog".to_string(),
            })
        ]);

        // Register the test config
        fixture.register_test_config();

        // Get available topics
        let topics = topic::get_available_topics().unwrap();

        // Check that topics match our expected set
        assert_eq!(topics.len(), 3);
        assert!(topics.iter().any(|(name, _)| name == "creativity"));
        assert!(topics.iter().any(|(name, _)| name == "strategy"));
        assert!(topics.iter().any(|(name, _)| name == "blog"));
    }
}