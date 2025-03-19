//! # Mock Markdown Implementation
//!
//! This module provides a mock implementation of markdown operations for testing.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use common_errors::Result;
use common_models::Frontmatter;
use common_errors::WritingError;

/// A mock implementation of markdown operations
#[derive(Debug, Clone, Default)]
pub struct MockMarkdown {
    frontmatter_extraction_results: Arc<Mutex<HashMap<String, Result<(Frontmatter, String)>>>>,
    frontmatter_combination_results: Arc<Mutex<HashMap<String, String>>>,
}

impl MockMarkdown {
    /// Create a new mock markdown implementation
    pub fn new() -> Self {
        Self {
            frontmatter_extraction_results: Arc::new(Mutex::new(HashMap::new())),
            frontmatter_combination_results: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Set the result of extracting frontmatter from a specific content string
    pub fn set_extract_frontmatter_result(&mut self, content: &str, result: Result<(Frontmatter, String)>) {
        self.frontmatter_extraction_results.lock().unwrap().insert(content.to_string(), result);
    }

    /// Extract frontmatter from content
    pub fn extract_frontmatter(&self, content: &str) -> Result<(Frontmatter, String)> {
        let results = self.frontmatter_extraction_results.lock().unwrap();

        if let Some(result) = results.get(content) {
            // We need to clone the Result, but WritingError doesn't implement Clone
            // Let's recreate the result instead
            if let Ok((fm, content_str)) = result {
                Ok((fm.clone(), content_str.clone()))
            } else {
                // If it was an error, recreate a similar error
                Err(WritingError::content_parsing_error("Mock parsing error"))
            }
        } else {
            // Default behavior if no mock response is set
            let frontmatter = Frontmatter {
                title: "Mock Title".to_string(),
                published_at: Some("2023-01-01".to_string()),
                updated_at: Some("2023-01-01".to_string()),
                slug: Some("mock-title".to_string()),
                description: None,
                tags: None,
                topics: None,
                featured_image_path: None,
                is_draft: Some(false),
            };
            Ok((frontmatter, content.to_string()))
        }
    }

    /// Set the result of combining frontmatter with content
    pub fn set_combine_frontmatter_result(&mut self, key: &str, result: &str) {
        let combined_key = format!("{}::{}", key, key); // Use a simple key format
        self.frontmatter_combination_results.lock().unwrap().insert(combined_key, result.to_string());
    }

    /// Combine frontmatter with content
    pub fn combine_frontmatter(&self, frontmatter: &Frontmatter, content: &str) -> String {
        let key = format!("{}::{}", frontmatter.title, content);
        let results = self.frontmatter_combination_results.lock().unwrap();

        if let Some(result) = results.get(&key) {
            result.clone()
        } else {
            // Default behavior if no mock response is set
            format!("---\ntitle: {}\npublished: {}\n---\n{}",
                    frontmatter.title,
                    frontmatter.published_at.as_deref().unwrap_or("2023-01-01"),
                    content)
        }
    }
}

/// Trait for markdown operations
pub trait MarkdownOperations {
    /// Extract frontmatter from content
    fn extract_frontmatter(&self, content: &str) -> Result<(Frontmatter, String)>;

    /// Combine frontmatter with content
    fn combine_frontmatter(&self, frontmatter: &Frontmatter, content: &str) -> String;
}

// Implement the trait for the mock
impl MarkdownOperations for MockMarkdown {
    fn extract_frontmatter(&self, content: &str) -> Result<(Frontmatter, String)> {
        self.extract_frontmatter(content)
    }

    fn combine_frontmatter(&self, frontmatter: &Frontmatter, content: &str) -> String {
        self.combine_frontmatter(frontmatter, content)
    }
}