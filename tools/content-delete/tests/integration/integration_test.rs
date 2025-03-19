use common_test_utils::fixtures::TestFixture;
use common_test_utils::mocks::{MockFileSystem, MockConfigLoader};
use common_traits::tools::ContentDeleter;
use content_delete::ContentDeleterImpl;
use std::collections::HashMap;
use mockall::predicate;

#[test]
fn test_delete_content() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let content_dir = fixture.path().join("content/blog/test-post");
    let index_file = content_dir.join("index.mdx");

    // Mock file existence checks
    mock_fs.expect_dir_exists()
        .with(predicate::eq(content_dir.clone()))
        .returning(|_| Ok(true));

    mock_fs.expect_read_file()
        .with(predicate::eq(index_file.clone()))
        .returning(move |_| {
            Ok(r#"---
title: "Test Post"
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

    // Create a ContentDeleterImpl instance
    let deleter = ContentDeleterImpl::new();

    // Act - Delete content with force flag
    let result = deleter.delete_content("test-post", Some("blog"), true);

    // Assert - Check the result
    assert!(result.is_ok(), "Failed to delete content: {:?}", result.err());
}

#[test]
fn test_delete_nonexistent_content() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let content_dir = fixture.path().join("content/blog/nonexistent-post");

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

    // Create a ContentDeleterImpl instance
    let deleter = ContentDeleterImpl::new();

    // Act - Try to delete nonexistent content
    let result = deleter.delete_content("nonexistent-post", Some("blog"), true);

    // Assert - Check the result
    assert!(result.is_err(), "Should have failed to delete nonexistent content");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("not found") || err.contains("Content not found"));
}

#[test]
fn test_can_delete_check() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let content_dir = fixture.path().join("content/blog/test-post");

    // Mock file existence checks
    mock_fs.expect_dir_exists()
        .with(predicate::eq(content_dir.clone()))
        .returning(|_| Ok(true));

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

    // Create a ContentDeleterImpl instance
    let deleter = ContentDeleterImpl::new();

    // Act - Check if content can be deleted
    let result = deleter.can_delete("test-post", Some("blog"));

    // Assert - Check the result
    assert!(result.is_ok(), "Failed to check deletion possibility: {:?}", result.err());
    assert!(result.unwrap(), "Should be able to delete existing content");
}

#[test]
fn test_can_delete_nonexistent() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let content_dir = fixture.path().join("content/blog/nonexistent-post");

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

    // Create a ContentDeleterImpl instance
    let deleter = ContentDeleterImpl::new();

    // Act - Check if nonexistent content can be deleted
    let result = deleter.can_delete("nonexistent-post", Some("blog"));

    // Assert - Check the result
    assert!(result.is_ok(), "Failed to check deletion possibility: {:?}", result.err());
    assert!(!result.unwrap(), "Should not be able to delete nonexistent content");
}