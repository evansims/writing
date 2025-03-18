//! # Mock Trait Definitions
//!
//! This module provides trait definitions that can be used for dependency injection
//! and mocking in tests.

use mockall::automock;
use common_errors::Result;
use common_models::{Config, ContentMeta, Frontmatter, TopicConfig};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

/// Re-exporting existing traits for completeness
pub use super::{FileSystem, ConfigLoader, ContentOperations, CommandExecutor, MarkdownOperations};

/// A trait for environment operations
#[automock]
pub trait Environment {
    /// Get the current working directory
    fn current_dir(&self) -> Result<PathBuf>;

    /// Get an environment variable
    fn get_env(&self, key: &str) -> Option<String>;

    /// Set an environment variable
    fn set_env(&self, key: &str, value: &str) -> Result<()>;
}

/// A trait for date/time operations
#[automock]
pub trait TimeProvider {
    /// Get the current date as YYYY-MM-DD
    fn today(&self) -> String;

    /// Get the current datetime in ISO 8601 format
    fn now(&self) -> String;

    /// Format a date string
    fn format_date(&self, date: &str, format: &str) -> Result<String>;
}

/// A trait for template rendering
#[automock]
pub trait TemplateRenderer {
    /// Render a template with context
    fn render(&self, template: &str, context: &[(&str, &str)]) -> Result<String>;

    /// Load a template from a file
    fn load_template(&self, template_path: &Path) -> Result<String>;
}

/// A trait for validation operations
#[automock]
pub trait Validator {
    /// Validate a slug
    fn validate_slug(&self, slug: &str) -> Result<()>;

    /// Validate a title
    fn validate_title(&self, title: &str) -> Result<()>;

    /// Validate frontmatter
    fn validate_frontmatter(&self, frontmatter: &Frontmatter) -> Result<()>;

    /// Validate a path
    fn validate_path(&self, path: &Path) -> Result<()>;
}

/// A trait for caching operations
#[automock]
pub trait Cache {
    /// Get a value from the cache
    fn get(&self, key: &str) -> Option<String>;

    /// Set a value in the cache
    fn set(&self, key: &str, value: &str) -> Result<()>;

    /// Check if a key exists in the cache
    fn exists(&self, key: &str) -> bool;

    /// Clear the cache
    fn clear(&self) -> Result<()>;
}

/// A trait for testing network operations
#[automock]
pub trait NetworkClient {
    /// Perform a GET request
    fn get(&self, url: &str) -> Result<String>;

    /// Perform a POST request
    fn post(&self, url: &str, body: &str) -> Result<String>;

    /// Check if a URL is accessible
    fn is_accessible(&self, url: &str) -> bool;
}

/// A trait for media processing operations
#[automock]
pub trait MediaProcessor {
    /// Optimize an image
    fn optimize_image(&self, source: &Path, target: &Path) -> Result<()>;

    /// Resize an image
    fn resize_image(&self, source: &Path, target: &Path, width: u32, height: u32) -> Result<()>;

    /// Extract image metadata
    fn extract_image_metadata(&self, path: &Path) -> Result<HashMap<String, String>>;
}

/// A trait for dependency injection management
#[derive(Default)]
pub struct DependencyContainer {
    // Map of dependencies by type ID
    dependencies: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>,
}

impl DependencyContainer {
    /// Create a new dependency container
    pub fn new() -> Self {
        Self {
            dependencies: std::collections::HashMap::new(),
        }
    }

    /// Register a dependency
    pub fn register<T: 'static>(&mut self, instance: T) {
        let type_id = std::any::TypeId::of::<T>();
        self.dependencies.insert(type_id, Box::new(instance));
    }

    /// Get a dependency
    pub fn get<T: 'static>(&self) -> Option<&T> {
        let type_id = std::any::TypeId::of::<T>();
        self.dependencies.get(&type_id)
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}