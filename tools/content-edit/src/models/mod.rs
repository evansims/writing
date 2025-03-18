//! Data models for the content-edit module.
//!
//! This file contains the data structures used by the content-edit module.

use std::fmt;
use std::path::PathBuf;

/// Options for content editing
#[derive(Debug, Clone)]
pub struct EditOptions {
    /// The slug of the content to edit
    pub slug: Option<String>,
    /// The topic containing the content
    pub topic: Option<String>,
    /// Whether to edit only the frontmatter
    pub frontmatter_only: bool,
    /// Whether to edit only the content
    pub content_only: bool,
}

impl EditOptions {
    /// Creates a new set of edit options
    pub fn new(
        slug: Option<String>,
        topic: Option<String>,
        frontmatter_only: bool,
        content_only: bool,
    ) -> Self {
        Self {
            slug,
            topic,
            frontmatter_only,
            content_only,
        }
    }

    /// Creates edit options for editing the entire content
    pub fn for_full_edit(slug: &str, topic: Option<String>) -> Self {
        Self {
            slug: Some(slug.to_string()),
            topic,
            frontmatter_only: false,
            content_only: false,
        }
    }

    /// Creates edit options for editing only the frontmatter
    pub fn for_frontmatter(slug: &str, topic: Option<String>) -> Self {
        Self {
            slug: Some(slug.to_string()),
            topic,
            frontmatter_only: true,
            content_only: false,
        }
    }

    /// Creates edit options for editing only the content body
    pub fn for_content_body(slug: &str, topic: Option<String>) -> Self {
        Self {
            slug: Some(slug.to_string()),
            topic,
            frontmatter_only: false,
            content_only: true,
        }
    }
}

impl fmt::Display for EditOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Edit options: slug={}, topic={}, frontmatter_only={}, content_only={}",
            self.slug.as_deref().unwrap_or("None"),
            self.topic.as_deref().unwrap_or("None"),
            self.frontmatter_only,
            self.content_only
        )
    }
}

/// Represents content that can be edited
#[derive(Debug, Clone)]
pub struct EditableContent {
    /// The path to the content file
    pub path: PathBuf,
    /// The topic that contains the content
    pub topic: String,
    /// The title of the content
    pub title: String,
    /// The slug of the content
    pub slug: String,
}

impl EditableContent {
    /// Creates a new editable content
    pub fn new(path: PathBuf, topic: String, title: String, slug: String) -> Self {
        Self {
            path,
            topic,
            title,
            slug,
        }
    }
}

impl fmt::Display for EditableContent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (topic: {}, slug: {})",
            self.title,
            self.topic,
            self.slug
        )
    }
}