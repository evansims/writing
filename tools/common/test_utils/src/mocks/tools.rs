//! Mock implementations of tool traits for testing
//!
//! This module provides mock implementations of all tool traits for testing.

use std::path::{Path, PathBuf};
use mockall::predicate;
use mockall::mock;
use common_errors::Result;
use common_traits::tools::*;

/// A mock implementation of ContentCreator
pub trait ContentCreatorMock: ContentCreator {}

/// A mock implementation of ContentEditor
pub trait ContentEditorMock: ContentEditor {}

/// A mock implementation of ContentValidator
pub trait ContentValidatorMock: ContentValidator {}

/// A mock implementation of ContentSearcher
pub trait ContentSearcherMock: ContentSearcher {}

/// A mock implementation of ContentMover
pub trait ContentMoverMock: ContentMover {}

/// A mock implementation of ContentDeleter
pub trait ContentDeleterMock: ContentDeleter {}

/// A simple struct representing a search result for testing purposes
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf,
    pub title: String,
    pub snippet: String,
    pub score: f32,
}

/// A simple implementation of ContentCreator for testing
pub struct TestContentCreator;

impl ContentCreator for TestContentCreator {
    fn create_content(&self, options: &ContentOptions) -> Result<PathBuf> {
        // Basic validation
        if options.title.is_none() && options.slug.is_none() {
            return Err(common_errors::WritingError::invalid_argument(
                "Either title or slug must be provided",
            ));
        }

        if options.topic.is_none() {
            return Err(common_errors::WritingError::invalid_argument(
                "Topic must be provided",
            ));
        }

        // Generate a slug if not provided
        let slug = match &options.slug {
            Some(s) => s.clone(),
            None => {
                let title = options.title.as_ref().unwrap();
                title.to_lowercase().replace(' ', "-")
            }
        };

        let topic = options.topic.as_ref().unwrap();

        // Return a path to the created content
        Ok(PathBuf::from(format!("content/{}/{}.md", topic, slug)))
    }

    fn list_templates(&self) -> Result<Vec<String>> {
        // Return a list of template names
        Ok(vec!["article".to_string(), "note".to_string(), "page".to_string()])
    }

    fn get_available_topics(&self) -> Result<Vec<(String, String)>> {
        // Return a list of (topic_key, topic_name) pairs
        Ok(vec![
            ("blog".to_string(), "Blog Posts".to_string()),
            ("notes".to_string(), "Quick Notes".to_string()),
            ("pages".to_string(), "Static Pages".to_string()),
        ])
    }
}

/// A simple implementation of ContentEditor for testing
pub struct TestContentEditor;

impl ContentEditor for TestContentEditor {
    fn edit_content(&self, options: &EditOptions) -> Result<PathBuf> {
        // Basic validation
        if options.slug.is_none() {
            return Err(common_errors::WritingError::invalid_argument(
                "Slug must be provided",
            ));
        }

        if options.topic.is_none() {
            return Err(common_errors::WritingError::invalid_argument(
                "Topic must be provided",
            ));
        }

        if options.field.is_none() {
            return Err(common_errors::WritingError::invalid_argument(
                "Field must be provided",
            ));
        }

        let slug = options.slug.as_ref().unwrap();
        let topic = options.topic.as_ref().unwrap();

        // Return a path to the edited content
        Ok(PathBuf::from(format!("content/{}/{}.md", topic, slug)))
    }

    fn update_frontmatter_field(&self, slug: &str, topic: Option<&str>, field: &str, value: &str) -> Result<()> {
        // Basic validation
        if slug.is_empty() {
            return Err(common_errors::WritingError::invalid_argument(
                "Slug must be provided",
            ));
        }

        // For testing purposes, just return success
        Ok(())
    }

    fn get_frontmatter_fields(&self, slug: &str, topic: Option<&str>) -> Result<std::collections::HashMap<String, String>> {
        // Basic validation
        if slug.is_empty() {
            return Err(common_errors::WritingError::invalid_argument(
                "Slug must be provided",
            ));
        }

        // Return some sample frontmatter fields
        let mut fields = std::collections::HashMap::new();
        fields.insert("title".to_string(), "Test Article".to_string());
        fields.insert("date".to_string(), "2023-05-15".to_string());
        fields.insert("draft".to_string(), "false".to_string());

        Ok(fields)
    }
}

/// A simple implementation of ContentValidator for testing
pub struct TestContentValidator;

impl ContentValidator for TestContentValidator {
    fn validate_content(&self, options: &ValidationOptions) -> Result<Vec<String>> {
        // Basic validation
        if options.slug.is_none() && options.topic.is_none() {
            return Err(common_errors::WritingError::invalid_argument(
                "Either slug or topic must be provided",
            ));
        }

        // Return empty list - no validation issues
        Ok(vec![])
    }

    fn fix_validation_issues(&self, options: &ValidationOptions) -> Result<Vec<String>> {
        // Basic validation
        if options.slug.is_none() && options.topic.is_none() {
            return Err(common_errors::WritingError::invalid_argument(
                "Either slug or topic must be provided",
            ));
        }

        // Return list of fixed issues
        Ok(vec!["Fixed formatting".to_string()])
    }

    fn get_validation_types(&self) -> Vec<String> {
        // Return available validation types
        vec![
            "spelling".to_string(),
            "links".to_string(),
            "frontmatter".to_string(),
            "formatting".to_string(),
        ]
    }
}

/// A simple implementation of ContentSearcher for testing
pub struct TestContentSearcher;

impl ContentSearcher for TestContentSearcher {
    fn search_content(&self, options: &SearchOptions) -> Result<Vec<PathBuf>> {
        // Basic validation
        if options.query.is_empty() && options.tags.is_none() {
            return Err(common_errors::WritingError::invalid_argument(
                "Either query or tags must be provided",
            ));
        }

        // Return paths to matching content
        Ok(vec![
            PathBuf::from("content/blog/test-article.md"),
        ])
    }

    fn build_search_index(&self, _include_drafts: bool) -> Result<()> {
        // For testing purposes, just return success
        Ok(())
    }
}

/// A simple implementation of ContentMover for testing
pub struct TestContentMover;

impl ContentMover for TestContentMover {
    fn move_content(&self, options: &MoveOptions) -> Result<PathBuf> {
        // Basic validation
        if options.slug.is_none() {
            return Err(common_errors::WritingError::invalid_argument(
                "Source slug must be provided",
            ));
        }

        if options.from_topic.is_none() {
            return Err(common_errors::WritingError::invalid_argument(
                "Source topic must be provided",
            ));
        }

        let dest_slug = options.new_slug.as_ref().unwrap_or_else(||
            options.slug.as_ref().unwrap()
        );

        let dest_topic = options.to_topic.as_ref().unwrap_or_else(||
            options.from_topic.as_ref().unwrap()
        );

        // Return a path to the moved content
        Ok(PathBuf::from(format!("content/{}/{}.md", dest_topic, dest_slug)))
    }

    fn validate_move(&self, options: &MoveOptions) -> Result<()> {
        // Basic validation
        if options.slug.is_none() {
            return Err(common_errors::WritingError::invalid_argument(
                "Source slug must be provided",
            ));
        }

        if options.from_topic.is_none() {
            return Err(common_errors::WritingError::invalid_argument(
                "Source topic must be provided",
            ));
        }

        Ok(())
    }
}

/// A simple implementation of the ContentDeleter trait for testing
pub struct TestContentDeleter;

impl ContentDeleter for TestContentDeleter {
    fn can_delete(&self, slug: &str, topic: Option<&str>) -> Result<bool> {
        // For testing purposes, we'll just return true
        Ok(true)
    }

    fn delete_content(&self, slug: &str, topic: Option<&str>, force: bool) -> Result<()> {
        // Basic validation
        if slug.is_empty() {
            return Err(common_errors::WritingError::invalid_argument(
                "Slug must be provided",
            ));
        }

        // For testing purposes, we'll just return success
        Ok(())
    }
}

/// Create a set of test tool implementations
pub fn create_test_tools() -> (
    TestContentCreator,
    TestContentEditor,
    TestContentValidator,
    TestContentSearcher,
    TestContentMover,
    TestContentDeleter
) {
    (
        TestContentCreator,
        TestContentEditor,
        TestContentValidator,
        TestContentSearcher,
        TestContentMover,
        TestContentDeleter
    )
}