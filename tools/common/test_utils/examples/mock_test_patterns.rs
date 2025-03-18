//! # Mock Test Patterns
//!
//! This file provides examples of common test patterns using mocks.

use std::path::{Path, PathBuf};
use mockall::predicate;
use common_errors::Result;
use common_traits::tools::*;
use common_test_utils::mocks::*;

/// The system under test - a component that depends on external components
struct ContentProcessor {
    creator: Box<dyn ContentCreator>,
    validator: Box<dyn ContentValidator>,
    filesystem: Box<dyn FileSystem>,
    config_loader: Box<dyn ConfigLoader>,
}

impl ContentProcessor {
    /// Create a new ContentProcessor with dependencies injected
    fn new(
        creator: Box<dyn ContentCreator>,
        validator: Box<dyn ContentValidator>,
        filesystem: Box<dyn FileSystem>,
        config_loader: Box<dyn ConfigLoader>,
    ) -> Self {
        Self {
            creator,
            validator,
            filesystem,
            config_loader,
        }
    }

    /// Process content creation and validation
    fn process_content(&self, title: &str, topic: &str) -> Result<PathBuf> {
        // Load configuration to check if topic is valid
        let config = self.config_loader.load_config()?;
        if !config.topics.contains(&topic.to_string()) {
            return Err(common_errors::WritingError::invalid_input(
                &format!("Topic '{}' is not configured", topic)
            ));
        }

        // Create content with the creator
        let options = ContentOptions {
            slug: None,
            title: Some(title.to_string()),
            topic: Some(topic.to_string()),
            description: None,
            template: None,
            tags: None,
            draft: Some(false),
        };

        let content_path = self.creator.create_content(&options)?;

        // Validate the created content
        let validation_options = ValidationOptions {
            slug: Some(content_path.file_stem().unwrap().to_string_lossy().to_string()),
            topic: Some(topic.to_string()),
            validation_types: None,
            check_external_links: false,
            external_link_timeout: None,
            dictionary: None,
            include_drafts: true,
            verbose: false,
            fix: true,
        };

        let validation_issues = self.validator.validate_content(&validation_options)?;

        // If there are validation issues, fix them
        if !validation_issues.is_empty() {
            self.validator.fix_validation_issues(&validation_options)?;
        }

        // Check that the file actually exists on the filesystem
        if !self.filesystem.file_exists(&content_path)? {
            return Err(common_errors::WritingError::not_found(
                &format!("Content file not found: {}", content_path.display())
            ));
        }

        Ok(content_path)
    }
}

/// Pattern 1: Basic mocking with mock expectations
#[test]
fn test_basic_mocking_with_expectations() {
    // Arrange: Create mock instances
    let mut mock_creator = MockContentCreatorMock::new();
    let mut mock_validator = MockContentValidatorMock::new();
    let mut mock_fs = MockFileSystem::new();
    let mut mock_config = MockConfigLoader::new();

    // Set up expectations for each mock

    // Config loader should return a valid config with the topic
    let mut config = common_config::Config::default();
    config.topics = vec!["blog".to_string()];
    mock_config.expect_load_config()
        .times(1)
        .return_const(Ok(config));

    // Creator should create content successfully
    mock_creator.expect_create_content()
        .with(predicate::function(|options: &ContentOptions| {
            options.title == Some("Test Article".to_string()) &&
            options.topic == Some("blog".to_string())
        }))
        .times(1)
        .returning(|_| Ok(PathBuf::from("content/blog/test-article.md")));

    // Validator should not find any issues
    mock_validator.expect_validate_content()
        .times(1)
        .returning(|_| Ok(vec![]));

    // File system should confirm the file exists
    mock_fs.expect_file_exists()
        .with(predicate::eq(PathBuf::from("content/blog/test-article.md")))
        .times(1)
        .returning(|_| Ok(true));

    // Create the system under test with mocks
    let processor = ContentProcessor::new(
        Box::new(mock_creator),
        Box::new(mock_validator),
        Box::new(mock_fs),
        Box::new(mock_config),
    );

    // Act: Call the method being tested
    let result = processor.process_content("Test Article", "blog");

    // Assert: Verify the result
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), PathBuf::from("content/blog/test-article.md"));
}

/// Pattern 2: Testing error conditions
#[test]
fn test_error_conditions() {
    // Arrange: Create mock instances
    let mut mock_creator = MockContentCreatorMock::new();
    let mut mock_validator = MockContentValidatorMock::new();
    let mut mock_fs = MockFileSystem::new();
    let mut mock_config = MockConfigLoader::new();

    // Config loader should return a config without the requested topic
    let config = common_config::Config::default(); // Empty topics list
    mock_config.expect_load_config()
        .times(1)
        .return_const(Ok(config));

    // None of the other mocks should be called

    // Create the system under test with mocks
    let processor = ContentProcessor::new(
        Box::new(mock_creator),
        Box::new(mock_validator),
        Box::new(mock_fs),
        Box::new(mock_config),
    );

    // Act: Call the method being tested
    let result = processor.process_content("Test Article", "blog");

    // Assert: Verify the error
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Topic 'blog' is not configured"));
}

/// Pattern 3: Sequence of method calls
#[test]
fn test_sequence_of_method_calls() {
    // Arrange: Create mock instances with a sequence of calls
    let mut mock_creator = MockContentCreatorMock::new();
    let mut mock_validator = MockContentValidatorMock::new();
    let mut mock_fs = MockFileSystem::new();
    let mut mock_config = MockConfigLoader::new();

    // Config loader should return a valid config with the topic
    let mut config = common_config::Config::default();
    config.topics = vec!["blog".to_string()];
    mock_config.expect_load_config()
        .times(1)
        .return_const(Ok(config));

    // Creator should create content successfully
    mock_creator.expect_create_content()
        .times(1)
        .returning(|_| Ok(PathBuf::from("content/blog/test-article.md")));

    // Validator should find issues and then fix them
    mock_validator.expect_validate_content()
        .times(1)
        .returning(|_| Ok(vec!["Missing description".to_string()]));

    mock_validator.expect_fix_validation_issues()
        .times(1)
        .returning(|_| Ok(vec!["Fixed: Added description".to_string()]));

    // File system should confirm the file exists
    mock_fs.expect_file_exists()
        .times(1)
        .returning(|_| Ok(true));

    // Create the system under test with mocks
    let processor = ContentProcessor::new(
        Box::new(mock_creator),
        Box::new(mock_validator),
        Box::new(mock_fs),
        Box::new(mock_config),
    );

    // Act: Call the method being tested
    let result = processor.process_content("Test Article", "blog");

    // Assert: Verify the result
    assert!(result.is_ok());
}

/// Pattern 4: Using context-specific predicates
#[test]
fn test_context_specific_predicates() {
    // Arrange: Create mock instances
    let mut mock_creator = MockContentCreatorMock::new();
    let mut mock_validator = MockContentValidatorMock::new();
    let mut mock_fs = MockFileSystem::new();
    let mut mock_config = MockConfigLoader::new();

    // Config loader should return a valid config with the topic
    let mut config = common_config::Config::default();
    config.topics = vec!["blog".to_string()];
    mock_config.expect_load_config()
        .times(1)
        .return_const(Ok(config));

    // Creator should create content successfully
    mock_creator.expect_create_content()
        .with(predicate::function(|options: &ContentOptions| {
            // Complex custom predicate logic
            if options.title.is_none() || options.topic.is_none() {
                return false;
            }

            let title = options.title.as_ref().unwrap();
            let topic = options.topic.as_ref().unwrap();

            title.contains("Test") && topic == "blog" && options.draft == Some(false)
        }))
        .times(1)
        .returning(|_| Ok(PathBuf::from("content/blog/test-article.md")));

    // Validator expectations
    mock_validator.expect_validate_content()
        .times(1)
        .returning(|_| Ok(vec![]));

    // File system should confirm the file exists
    mock_fs.expect_file_exists()
        .times(1)
        .returning(|_| Ok(true));

    // Create the system under test with mocks
    let processor = ContentProcessor::new(
        Box::new(mock_creator),
        Box::new(mock_validator),
        Box::new(mock_fs),
        Box::new(mock_config),
    );

    // Act: Call the method being tested
    let result = processor.process_content("Test Article", "blog");

    // Assert: Verify the result
    assert!(result.is_ok());
}

/// Pattern 5: Using test data factories
#[test]
fn test_with_test_data_factories() {
    // Create a config loader with test data
    let mut config_loader = create_test_config_loader();

    // Create test implementations
    let (creator, _, validator, _, _, _) = create_test_tools();

    // Create an in-memory file system with test files
    let mut fs = create_test_fs();

    // Creating a real implementation with test doubles
    let processor = ContentProcessor::new(
        Box::new(creator),
        Box::new(validator),
        Box::new(fs),
        Box::new(config_loader),
    );

    // Act: Call the method with test data
    let result = processor.process_content("Test Article", "blog");

    // Assert: Verify the result
    assert!(result.is_ok());
}

/// Pattern 6: Verifying side effects
#[test]
fn test_verifying_side_effects() {
    // Arrange: Create mock instances
    let mut mock_creator = MockContentCreatorMock::new();
    let mut mock_validator = MockContentValidatorMock::new();
    let mut mock_fs = MockFileSystem::new();
    let mut mock_config = MockConfigLoader::new();

    // Config loader should return a valid config with the topic
    let mut config = common_config::Config::default();
    config.topics = vec!["blog".to_string()];
    mock_config.expect_load_config()
        .times(1)
        .return_const(Ok(config.clone()));

    // Track what content options were passed to create_content
    let captured_options = std::sync::Arc::new(std::sync::Mutex::new(None));
    let captured_options_for_assert = captured_options.clone();

    mock_creator.expect_create_content()
        .times(1)
        .returning(move |options| {
            // Capture the options for later verification
            let mut options_storage = captured_options.lock().unwrap();
            *options_storage = Some(options.clone());
            Ok(PathBuf::from("content/blog/test-article.md"))
        });

    // Other expectations
    mock_validator.expect_validate_content()
        .times(1)
        .returning(|_| Ok(vec![]));

    mock_fs.expect_file_exists()
        .times(1)
        .returning(|_| Ok(true));

    // Create the system under test with mocks
    let processor = ContentProcessor::new(
        Box::new(mock_creator),
        Box::new(mock_validator),
        Box::new(mock_fs),
        Box::new(mock_config),
    );

    // Act: Call the method being tested
    let result = processor.process_content("Test Article", "blog");

    // Assert: Verify the result
    assert!(result.is_ok());

    // Verify captured options
    let captured = captured_options_for_assert.lock().unwrap();
    let options = captured.as_ref().expect("Options should have been captured");
    assert_eq!(options.title.as_ref().unwrap(), "Test Article");
    assert_eq!(options.topic.as_ref().unwrap(), "blog");
    assert_eq!(options.draft, Some(false));
}

/// Example main function to run all tests
fn main() {
    println!("Running mock test patterns...");

    // Run all the test patterns
    test_basic_mocking_with_expectations();
    test_error_conditions();
    test_sequence_of_method_calls();
    test_context_specific_predicates();
    test_with_test_data_factories();
    test_verifying_side_effects();

    println!("All mock test patterns completed successfully!");
}