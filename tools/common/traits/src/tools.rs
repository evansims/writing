//! # Tool-Specific Traits
//!
//! This module provides traits that define interfaces for specific tools.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use common_errors::Result;

/// Options for creating new content
#[derive(Debug, Clone)]
pub struct ContentOptions {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub topic: Option<String>,
    pub description: Option<String>,
    pub template: Option<String>,
    pub tags: Option<Vec<String>>,
    pub draft: Option<bool>,
}

/// Options for editing content
#[derive(Debug, Clone)]
pub struct EditOptions {
    pub slug: Option<String>,
    pub topic: Option<String>,
    pub field: Option<String>,
    pub value: Option<String>,
    pub editor: bool,
}

/// Options for moving content
#[derive(Debug, Clone)]
pub struct MoveOptions {
    pub slug: Option<String>,
    pub new_slug: Option<String>,
    pub from_topic: Option<String>,
    pub to_topic: Option<String>,
}

/// Options for validation
#[derive(Debug, Clone)]
pub struct ValidationOptions {
    pub slug: Option<String>,
    pub topic: Option<String>,
    pub validation_types: Option<Vec<String>>,
    pub check_external_links: bool,
    pub external_link_timeout: Option<u64>,
    pub dictionary: Option<String>,
    pub include_drafts: bool,
    pub verbose: bool,
    pub fix: bool,
}

/// Options for searching content
#[derive(Debug, Clone)]
pub struct SearchOptions {
    pub query: String,
    pub topic: Option<String>,
    pub content_type: Option<String>,
    pub tags: Option<String>,
    pub limit: Option<usize>,
    pub include_drafts: bool,
    pub title_only: bool,
    pub index_path: Option<String>,
    pub rebuild: bool,
}

/// Options for building content
#[derive(Debug, Clone)]
pub struct BuildOptions {
    pub output_dir: Option<String>,
    pub slug: Option<String>,
    pub topic: Option<String>,
    pub include_drafts: bool,
    pub skip_html: bool,
    pub skip_json: bool,
    pub skip_rss: bool,
    pub skip_sitemap: bool,
    pub force_rebuild: bool,
    pub verbose: bool,
}

/// Trait for creating new content
pub trait ContentCreator {
    /// Create new content
    fn create_content(&self, options: &ContentOptions) -> Result<PathBuf>;

    /// List available templates
    fn list_templates(&self) -> Result<Vec<String>>;

    /// Get available topics
    fn get_available_topics(&self) -> Result<Vec<(String, String)>>;
}

/// Trait for editing content
pub trait ContentEditor {
    /// Edit content
    fn edit_content(&self, options: &EditOptions) -> Result<PathBuf>;

    /// Update a specific frontmatter field
    fn update_frontmatter_field(&self, slug: &str, topic: Option<&str>, field: &str, value: &str) -> Result<()>;

    /// Get all available fields for a specific content
    fn get_frontmatter_fields(&self, slug: &str, topic: Option<&str>) -> Result<HashMap<String, String>>;
}

/// Trait for moving content
pub trait ContentMover {
    /// Move content from one location to another
    fn move_content(&self, options: &MoveOptions) -> Result<PathBuf>;

    /// Check if a move operation is valid
    fn validate_move(&self, options: &MoveOptions) -> Result<()>;
}

/// Trait for deleting content
pub trait ContentDeleter {
    /// Delete content
    fn delete_content(&self, slug: &str, topic: Option<&str>, force: bool) -> Result<()>;

    /// Check if content can be safely deleted
    fn can_delete(&self, slug: &str, topic: Option<&str>) -> Result<bool>;
}

/// Trait for validating content
pub trait ContentValidator {
    /// Validate content
    fn validate_content(&self, options: &ValidationOptions) -> Result<Vec<String>>;

    /// Get available validation types
    fn get_validation_types(&self) -> Vec<String>;

    /// Attempt to fix validation issues
    fn fix_validation_issues(&self, options: &ValidationOptions) -> Result<Vec<String>>;
}

/// Trait for searching content
pub trait ContentSearcher {
    /// Search for content
    fn search_content(&self, options: &SearchOptions) -> Result<Vec<PathBuf>>;

    /// Build or rebuild the search index
    fn build_search_index(&self, include_drafts: bool) -> Result<()>;
}

/// Trait for building content
pub trait ContentBuilder {
    /// Build content
    fn build_content(&self, options: &BuildOptions) -> Result<()>;

    /// Generate table of contents
    fn generate_toc(&self, topic: Option<String>) -> Result<()>;

    /// Generate LLMs format
    fn generate_llms(&self, include_drafts: bool) -> Result<()>;
}

/// Trait for managing topics
pub trait TopicManager {
    /// Add a new topic
    fn add_topic(&self, key: &str, name: &str, description: &str) -> Result<()>;

    /// Edit a topic
    fn edit_topic(&self, key: &str, name: Option<&str>, description: Option<&str>) -> Result<()>;

    /// Rename a topic (changes the key)
    fn rename_topic(&self, old_key: &str, new_key: &str) -> Result<()>;

    /// Delete a topic
    fn delete_topic(&self, key: &str, force: bool) -> Result<()>;

    /// List all topics
    fn list_topics(&self) -> Result<Vec<(String, String, String)>>;
}

/// Trait for image management
pub trait ImageManager {
    /// Optimize images in a directory
    fn optimize_images(&self, directory: &Path, recursive: bool) -> Result<Vec<PathBuf>>;

    /// Build responsive images
    fn build_responsive_images(&self, source: &Path, destination: &Path, sizes: &[u32]) -> Result<Vec<PathBuf>>;
}

/// A factory trait for creating tool implementations
pub trait ToolFactory {
    /// Create a content creator implementation
    fn create_content_creator(&self) -> Box<dyn ContentCreator + Send + Sync>;

    /// Create a content editor implementation
    fn create_content_editor(&self) -> Box<dyn ContentEditor + Send + Sync>;

    /// Create a content mover implementation
    fn create_content_mover(&self) -> Box<dyn ContentMover + Send + Sync>;

    /// Create a content deleter implementation
    fn create_content_deleter(&self) -> Box<dyn ContentDeleter + Send + Sync>;

    /// Create a content validator implementation
    fn create_content_validator(&self) -> Box<dyn ContentValidator + Send + Sync>;

    /// Create a content searcher implementation
    fn create_content_searcher(&self) -> Box<dyn ContentSearcher + Send + Sync>;

    /// Create a content builder implementation
    fn create_content_builder(&self) -> Box<dyn ContentBuilder + Send + Sync>;

    /// Create a topic manager implementation
    fn create_topic_manager(&self) -> Box<dyn TopicManager + Send + Sync>;

    /// Create an image manager implementation
    fn create_image_manager(&self) -> Box<dyn ImageManager + Send + Sync>;
}