//! # Mocking Examples
//!
//! This file provides examples of how to use mocks effectively in tests.

use std::path::PathBuf;
use mockall::predicate;
use common_errors::Result;
use common_traits::tools::*;
use common_test_utils::mocks::{
    MockContentCreatorMock, MockContentEditorMock, MockContentValidatorMock,
    MockContentSearcherMock, MockContentMoverMock, MockContentDeleterMock,
    MockFileSystem
};

/// Example struct that uses dependency injection
pub struct ContentManager {
    creator: Box<dyn ContentCreator>,
    validator: Box<dyn ContentValidator>,
}

impl ContentManager {
    /// Create a new ContentManager with the given dependencies
    pub fn new(creator: Box<dyn ContentCreator>, validator: Box<dyn ContentValidator>) -> Self {
        Self { creator, validator }
    }

    /// Create validated content
    pub fn create_validated_content(&self, options: &ContentOptions) -> Result<PathBuf> {
        // Create the content first
        let path = self.creator.create_content(options)?;

        // Validate the newly created content
        let validation_options = ValidationOptions {
            slug: options.slug.clone(),
            topic: options.topic.clone(),
            validation_types: None,
            check_external_links: false,
            external_link_timeout: None,
            dictionary: None,
            include_drafts: true,
            verbose: false,
            fix: true,
        };

        let issues = self.validator.validate_content(&validation_options)?;
        if !issues.is_empty() {
            // If validation failed, try to fix issues
            self.validator.fix_validation_issues(&validation_options)?;
        }

        Ok(path)
    }
}

/// Example test using mocks
#[test]
fn test_create_validated_content() {
    // Arrange - Create mock instances
    let mut mock_creator = MockContentCreatorMock::new();
    let mut mock_validator = MockContentValidatorMock::new();

    // Set expectations for the creator mock
    mock_creator.expect_create_content()
        .with(predicate::always()) // Accept any options
        .times(1) // Expect exactly one call
        .returning(|_| Ok(PathBuf::from("content/blog/test-article.md")));

    // Set expectations for the validator mock
    mock_validator.expect_validate_content()
        .with(predicate::always()) // Accept any validation options
        .times(1) // Expect exactly one call
        .returning(|_| Ok(vec![])); // Return no validation issues

    // Mock is not expected to call fix_validation_issues since validate_content returns no issues

    // Create the system under test with the mocks
    let content_manager = ContentManager::new(
        Box::new(mock_creator),
        Box::new(mock_validator),
    );

    // Act - Call the method being tested
    let result = content_manager.create_validated_content(&ContentOptions {
        slug: Some("test-article".to_string()),
        title: Some("Test Article".to_string()),
        topic: Some("blog".to_string()),
        description: None,
        template: None,
        tags: None,
        draft: Some(false),
    });

    // Assert - Verify the result
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), PathBuf::from("content/blog/test-article.md"));
}

/// Example test with validation issues
#[test]
fn test_create_content_with_validation_issues() {
    // Arrange - Create mock instances
    let mut mock_creator = MockContentCreatorMock::new();
    let mut mock_validator = MockContentValidatorMock::new();

    // Set expectations for the creator mock
    mock_creator.expect_create_content()
        .with(predicate::always())
        .times(1)
        .returning(|_| Ok(PathBuf::from("content/blog/test-article.md")));

    // Set expectations for the validator mock - this time with validation issues
    mock_validator.expect_validate_content()
        .with(predicate::always())
        .times(1)
        .returning(|_| Ok(vec!["Missing title".to_string()]));

    // Since validation returned issues, fix_validation_issues should be called
    mock_validator.expect_fix_validation_issues()
        .with(predicate::always())
        .times(1)
        .returning(|_| Ok(vec!["Fixed: Added title".to_string()]));

    // Create the system under test with the mocks
    let content_manager = ContentManager::new(
        Box::new(mock_creator),
        Box::new(mock_validator),
    );

    // Act - Call the method being tested
    let result = content_manager.create_validated_content(&ContentOptions {
        slug: Some("test-article".to_string()),
        title: None, // Missing title will cause validation issues
        topic: Some("blog".to_string()),
        description: None,
        template: None,
        tags: None,
        draft: Some(false),
    });

    // Assert - Verify the result
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), PathBuf::from("content/blog/test-article.md"));
}

/// Example of chaining multiple mocks together
pub struct ContentWorkflow {
    creator: Box<dyn ContentCreator>,
    editor: Box<dyn ContentEditor>,
    validator: Box<dyn ContentValidator>,
}

impl ContentWorkflow {
    /// Create a new ContentWorkflow with the given dependencies
    pub fn new(
        creator: Box<dyn ContentCreator>,
        editor: Box<dyn ContentEditor>,
        validator: Box<dyn ContentValidator>,
    ) -> Self {
        Self { creator, editor, validator }
    }

    /// Complete workflow: create content, edit it, and validate it
    pub fn complete_workflow(&self, title: &str, topic: &str) -> Result<PathBuf> {
        // Step 1: Create content
        let options = ContentOptions {
            slug: None, // Will be generated from title
            title: Some(title.to_string()),
            topic: Some(topic.to_string()),
            description: None,
            template: None,
            tags: None,
            draft: Some(true),
        };

        let path = self.creator.create_content(&options)?;

        // Step 2: Edit content
        let edit_options = EditOptions {
            slug: Some(path.file_stem().unwrap().to_string_lossy().to_string()),
            topic: Some(topic.to_string()),
            field: Some("description".to_string()),
            value: Some("Automatically generated description".to_string()),
            editor: false,
        };

        self.editor.edit_content(&edit_options)?;

        // Step 3: Validate content
        let validation_options = ValidationOptions {
            slug: options.slug,
            topic: options.topic,
            validation_types: None,
            check_external_links: false,
            external_link_timeout: None,
            dictionary: None,
            include_drafts: true,
            verbose: false,
            fix: true,
        };

        self.validator.validate_content(&validation_options)?;

        Ok(path)
    }
}

/// Example test for a complex workflow with multiple mocks
#[test]
fn test_content_workflow() {
    // Arrange - Create all required mocks
    let mut mock_creator = MockContentCreatorMock::new();
    let mut mock_editor = MockContentEditorMock::new();
    let mut mock_validator = MockContentValidatorMock::new();

    // Set expectations for content creation
    mock_creator.expect_create_content()
        .with(predicate::function(|options: &ContentOptions| {
            options.title == Some("Test Article".to_string()) &&
            options.topic == Some("blog".to_string())
        }))
        .times(1)
        .returning(|_| Ok(PathBuf::from("content/blog/test-article.md")));

    // Set expectations for content editing
    mock_editor.expect_edit_content()
        .with(predicate::function(|options: &EditOptions| {
            options.slug == Some("test-article".to_string()) &&
            options.topic == Some("blog".to_string()) &&
            options.field == Some("description".to_string())
        }))
        .times(1)
        .returning(|_| Ok(PathBuf::from("content/blog/test-article.md")));

    // Set expectations for content validation
    mock_validator.expect_validate_content()
        .with(predicate::always())
        .times(1)
        .returning(|_| Ok(vec![]));

    // Create the workflow manager with the mocks
    let workflow = ContentWorkflow::new(
        Box::new(mock_creator),
        Box::new(mock_editor),
        Box::new(mock_validator),
    );

    // Act - Execute the workflow
    let result = workflow.complete_workflow("Test Article", "blog");

    // Assert - Verify the result
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), PathBuf::from("content/blog/test-article.md"));
}

/// Example of using mocks for filesystem operations
pub struct ContentFinder {
    file_system: Box<dyn common_test_utils::mocks::FileSystem>,
}

impl ContentFinder {
    /// Create a new ContentFinder with the given filesystem
    pub fn new(file_system: Box<dyn common_test_utils::mocks::FileSystem>) -> Self {
        Self { file_system }
    }

    /// Find content by slug
    pub fn find_by_slug(&self, slug: &str, topic: &str) -> Result<String> {
        let path = PathBuf::from(format!("content/{}/{}.md", topic, slug));

        if !self.file_system.file_exists(&path)? {
            return Err(common_errors::WritingError::not_found(&format!(
                "Content not found: {}", path.display()
            )));
        }

        let content = self.file_system.read_file(&path)?;
        Ok(content)
    }
}

/// Example test for filesystem mocking
#[test]
fn test_find_by_slug() {
    // Arrange - Create a mock filesystem
    let mut mock_fs = MockFileSystem::new();

    // Set expectations for file existence check
    mock_fs.expect_file_exists()
        .with(predicate::eq(PathBuf::from("content/blog/test-article.md")))
        .times(1)
        .returning(|_| Ok(true));

    // Set expectations for file reading
    mock_fs.expect_read_file()
        .with(predicate::eq(PathBuf::from("content/blog/test-article.md")))
        .times(1)
        .returning(|_| Ok("---\ntitle: Test Article\n---\nThis is test content.".to_string()));

    // Create the finder with the mock filesystem
    let finder = ContentFinder::new(Box::new(mock_fs));

    // Act - Call the method being tested
    let result = finder.find_by_slug("test-article", "blog");

    // Assert - Verify the result
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "---\ntitle: Test Article\n---\nThis is test content.");
}

/// Example of a test that verifies a method is not called
#[test]
fn test_file_not_found() {
    // Arrange - Create a mock filesystem
    let mut mock_fs = MockFileSystem::new();

    // Set expectations for file existence check - return false
    mock_fs.expect_file_exists()
        .with(predicate::eq(PathBuf::from("content/blog/nonexistent.md")))
        .times(1)
        .returning(|_| Ok(false));

    // The read_file method should NOT be called
    // Not setting an expectation for read_file ensures the test fails if it's called

    // Create the finder with the mock filesystem
    let finder = ContentFinder::new(Box::new(mock_fs));

    // Act - Call the method being tested
    let result = finder.find_by_slug("nonexistent", "blog");

    // Assert - Verify the result is an error
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Content not found"));
}

/// Main function to run the examples
fn main() {
    println!("Running mock examples...");

    // These tests are normally run with 'cargo test'
    test_create_validated_content();
    test_create_content_with_validation_issues();
    test_content_workflow();
    test_find_by_slug();
    test_file_not_found();

    println!("All examples completed successfully!");
}