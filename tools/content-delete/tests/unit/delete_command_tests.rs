use content_delete::{DeleteCommand, DeleteArgs, DeleteResult, ContentDeleterImpl};
use common_cli::Command;
use common_cli::DisplayResult;
use common_test_utils::fixtures::TestFixture;
use common_test_utils::mocks::MockFileSystem;
use common_test_utils::mocks::MockConfigLoader;
use common_traits::tools::ContentDeleter;
use mockall::predicate;
use std::path::PathBuf;
use std::collections::HashMap;

#[test]
fn test_delete_command_no_slug() {
    // Create command without a slug (which should fail)
    let args = DeleteArgs {
        slug: None,
        topic: Some("blog".to_string()),
        force: false,
    };

    let command = DeleteCommand::new(args);
    let result = command.execute();

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("No slug provided"));
}

#[test]
fn test_delete_command_invalid_topic() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();

    // Create command with invalid topic
    let args = DeleteArgs {
        slug: Some("test-article".to_string()),
        topic: Some("nonexistent-topic".to_string()),
        force: false,
    };

    let command = DeleteCommand::new(args);
    let result = command.execute();

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Invalid topic") || err.contains("not found"));
}

#[test]
fn test_delete_command_success() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Setup test expectations for file system
    let content_dir = fixture.path().join("content/blog/test-article");
    let index_file = content_dir.join("index.mdx");

    // Mock file existence checks
    mock_fs.expect_dir_exists()
        .with(predicate::eq(content_dir.clone()))
        .returning(|_| Ok(true));

    mock_fs.expect_read_file()
        .with(predicate::eq(index_file.clone()))
        .returning(move |_| {
            Ok(r#"---
title: "Test Article"
---
Test content
"#.into())
        });

    mock_fs.expect_remove_dir_all()
        .with(predicate::eq(content_dir.clone()))
        .returning(|_| Ok(()));

    // Create a mock config for topics
    let mut topics = HashMap::new();
    topics.insert("blog".to_string(), common_config::TopicConfig {
        name: "Blog".to_string(),
        directory: "blog".to_string(),
        ..Default::default()
    });

    let config = common_config::Config {
        content: common_config::ContentConfig {
            base_dir: "content".to_string(),
            topics,
            ..Default::default()
        },
        ..Default::default()
    };

    // Setup mock config loader
    let mut mock_config = MockConfigLoader::new();
    mock_config.expect_load_config()
        .returning(move || Ok(config.clone()));

    // Register mocks with the fixture
    fixture.fs = mock_fs;
    fixture.config = mock_config;

    // Create command with valid args and force flag
    let args = DeleteArgs {
        slug: Some("test-article".to_string()),
        topic: Some("blog".to_string()),
        force: true, // Force delete without confirmation
    };

    let command = DeleteCommand::new(args);

    // Act - Execute the command
    let result = command.execute();

    // Assert - Verify the result
    assert!(result.is_ok(), "Command execution failed: {:?}", result.err());
    let delete_result = result.unwrap();

    // Verify the result contains the correct information
    assert_eq!(delete_result.slug, "test-article");
    assert_eq!(delete_result.topic, "blog");
    assert_eq!(delete_result.title, "Test Article");
}

#[test]
fn test_delete_command_nonexistent_content() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Setup test expectations for file system
    let content_dir = fixture.path().join("content/blog/nonexistent-article");

    // Mock file existence checks
    mock_fs.expect_dir_exists()
        .with(predicate::eq(content_dir.clone()))
        .returning(|_| Ok(false));

    // Create a mock config for topics
    let mut topics = HashMap::new();
    topics.insert("blog".to_string(), common_config::TopicConfig {
        name: "Blog".to_string(),
        directory: "blog".to_string(),
        ..Default::default()
    });

    let config = common_config::Config {
        content: common_config::ContentConfig {
            base_dir: "content".to_string(),
            topics,
            ..Default::default()
        },
        ..Default::default()
    };

    // Setup mock config loader
    let mut mock_config = MockConfigLoader::new();
    mock_config.expect_load_config()
        .returning(move || Ok(config.clone()));

    // Register mocks with the fixture
    fixture.fs = mock_fs;
    fixture.config = mock_config;

    // Create command for content that doesn't exist
    let args = DeleteArgs {
        slug: Some("nonexistent-article".to_string()),
        topic: Some("blog".to_string()),
        force: true,
    };

    let command = DeleteCommand::new(args);
    let result = command.execute();

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("not found") || err.contains("Content not found"));
}

#[test]
fn test_delete_result_to_display() {
    // Test that DeleteResult produces correct display output
    let result = DeleteResult {
        topic: "blog".to_string(),
        slug: "test-article".to_string(),
        title: "Test Article".to_string(),
    };

    let display = result.to_display();
    assert!(display.contains("SUCCESS"));
    assert!(display.contains("blog"));
    assert!(display.contains("test-article"));
    assert!(display.contains("Test Article"));
}

#[test]
fn test_content_deleter_impl() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Setup test expectations for file system
    let content_dir = fixture.path().join("content/blog/test-article");

    // Mock file existence checks
    mock_fs.expect_dir_exists()
        .with(predicate::eq(content_dir.clone()))
        .returning(|_| Ok(true));

    mock_fs.expect_remove_dir_all()
        .with(predicate::eq(content_dir.clone()))
        .returning(|_| Ok(()));

    // Create a mock config for topics
    let mut topics = HashMap::new();
    topics.insert("blog".to_string(), common_config::TopicConfig {
        name: "Blog".to_string(),
        directory: "blog".to_string(),
        ..Default::default()
    });

    let config = common_config::Config {
        content: common_config::ContentConfig {
            base_dir: "content".to_string(),
            topics,
            ..Default::default()
        },
        ..Default::default()
    };

    // Setup mock config loader
    let mut mock_config = MockConfigLoader::new();
    mock_config.expect_load_config()
        .times(2) // Once for can_delete, once for delete_content
        .returning(move || Ok(config.clone()));

    // Register mocks with the fixture
    fixture.fs = mock_fs;
    fixture.config = mock_config;

    // Create the ContentDeleterImpl
    let deleter = ContentDeleterImpl::new();

    // Act & Assert - Check can_delete
    let can_delete = deleter.can_delete("test-article", Some("blog"));
    assert!(can_delete.is_ok());
    assert!(can_delete.unwrap());

    // Act & Assert - Delete content
    let result = deleter.delete_content("test-article", Some("blog"), true);
    assert!(result.is_ok(), "Failed to delete content: {:?}", result.err());
}