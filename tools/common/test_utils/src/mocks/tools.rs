//! Mock implementations of tool traits for testing
//!
//! This module provides mock implementations of all tool traits for testing.

use std::path::{Path, PathBuf};
use mockall::predicate;
use mockall::mock;
use common_errors::Result;
use common_traits::tools::*;

/// Mock implementation of the ContentCreator trait
#[mockall::automock]
pub trait ContentCreatorMock: ContentCreator {}

/// Mock implementation of the ContentEditor trait
#[mockall::automock]
pub trait ContentEditorMock: ContentEditor {}

/// Mock implementation of the ContentValidator trait
#[mockall::automock]
pub trait ContentValidatorMock: ContentValidator {}

/// Mock implementation of the ContentSearcher trait
#[mockall::automock]
pub trait ContentSearcherMock: ContentSearcher {}

/// Mock implementation of the ContentMover trait
#[mockall::automock]
pub trait ContentMoverMock: ContentMover {}

/// Mock implementation of the ContentDeleter trait
#[mockall::automock]
pub trait ContentDeleterMock: ContentDeleter {}

/// A simple implementation of ContentCreator for testing
pub struct TestContentCreator;

impl ContentCreator for TestContentCreator {
    fn create_content(&self, options: &ContentOptions) -> Result<PathBuf> {
        // Basic validation
        if options.title.is_none() && options.slug.is_none() {
            return Err(common_errors::WritingError::invalid_input(
                "Either title or slug must be provided",
            ));
        }

        if options.topic.is_none() {
            return Err(common_errors::WritingError::invalid_input(
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
}

/// A simple implementation of ContentEditor for testing
pub struct TestContentEditor;

impl ContentEditor for TestContentEditor {
    fn edit_content(&self, options: &EditOptions) -> Result<PathBuf> {
        // Basic validation
        if options.slug.is_none() {
            return Err(common_errors::WritingError::invalid_input(
                "Slug must be provided",
            ));
        }

        if options.topic.is_none() {
            return Err(common_errors::WritingError::invalid_input(
                "Topic must be provided",
            ));
        }

        if options.field.is_none() {
            return Err(common_errors::WritingError::invalid_input(
                "Field must be provided",
            ));
        }

        let slug = options.slug.as_ref().unwrap();
        let topic = options.topic.as_ref().unwrap();

        // Return a path to the edited content
        Ok(PathBuf::from(format!("content/{}/{}.md", topic, slug)))
    }
}

/// A simple implementation of ContentValidator for testing
pub struct TestContentValidator;

impl ContentValidator for TestContentValidator {
    fn validate_content(&self, options: &ValidationOptions) -> Result<Vec<String>> {
        // Basic validation
        if options.slug.is_none() && options.topic.is_none() {
            return Err(common_errors::WritingError::invalid_input(
                "Either slug or topic must be provided",
            ));
        }

        // Return empty list - no validation issues
        Ok(vec![])
    }

    fn fix_validation_issues(&self, options: &ValidationOptions) -> Result<Vec<String>> {
        // Basic validation
        if options.slug.is_none() && options.topic.is_none() {
            return Err(common_errors::WritingError::invalid_input(
                "Either slug or topic must be provided",
            ));
        }

        // Return list of fixed issues
        Ok(vec!["Fixed formatting".to_string()])
    }
}

/// A simple implementation of ContentSearcher for testing
pub struct TestContentSearcher;

impl ContentSearcher for TestContentSearcher {
    fn search_content(&self, options: &SearchOptions) -> Result<Vec<SearchResult>> {
        // Basic validation
        if options.query.is_none() && options.tags.is_none() {
            return Err(common_errors::WritingError::invalid_input(
                "Either query or tags must be provided",
            ));
        }

        // Return empty search results
        Ok(vec![
            SearchResult {
                path: PathBuf::from("content/blog/test-article.md"),
                title: "Test Article".to_string(),
                snippet: "This is a test article...".to_string(),
                score: 0.95,
            }
        ])
    }
}

/// A simple implementation of ContentMover for testing
pub struct TestContentMover;

impl ContentMover for TestContentMover {
    fn move_content(&self, options: &MoveOptions) -> Result<PathBuf> {
        // Basic validation
        if options.source_slug.is_none() {
            return Err(common_errors::WritingError::invalid_input(
                "Source slug must be provided",
            ));
        }

        if options.source_topic.is_none() {
            return Err(common_errors::WritingError::invalid_input(
                "Source topic must be provided",
            ));
        }

        let dest_slug = options.dest_slug.as_ref().unwrap_or_else(||
            options.source_slug.as_ref().unwrap()
        );

        let dest_topic = options.dest_topic.as_ref().unwrap_or_else(||
            options.source_topic.as_ref().unwrap()
        );

        // Return a path to the moved content
        Ok(PathBuf::from(format!("content/{}/{}.md", dest_topic, dest_slug)))
    }
}

/// A simple implementation of ContentDeleter for testing
pub struct TestContentDeleter;

impl ContentDeleter for TestContentDeleter {
    fn delete_content(&self, options: &DeleteOptions) -> Result<()> {
        // Basic validation
        if options.slug.is_none() {
            return Err(common_errors::WritingError::invalid_input(
                "Slug must be provided",
            ));
        }

        if options.topic.is_none() {
            return Err(common_errors::WritingError::invalid_input(
                "Topic must be provided",
            ));
        }

        // Return success
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