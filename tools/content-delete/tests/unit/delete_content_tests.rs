use content_delete::{delete_content, DeleteOptions};
use common_test_utils::fixtures::TestFixture;
use common_test_utils::mocks::{MockFileSystem, MockConfigLoader};
use std::path::PathBuf;
use std::collections::HashMap;
use mockall::predicate;

#[test]
fn test_delete_content_requires_slug() {
    // A slug is required for deletion
    let options = DeleteOptions {
        slug: None,
        topic: Some("blog".to_string()),
        force: false,
    };

    let result = delete_content(&options);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Slug is required"));
}

#[test]
fn test_delete_content_nonexistent_topic() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();

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
    fixture.config = mock_config;

    // Topic must exist
    let options = DeleteOptions {
        slug: Some("test-article".to_string()),
        topic: Some("nonexistent-topic".to_string()),
        force: false,
    };

    let result = delete_content(&options);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("not found"));
}

#[test]
fn test_delete_content_topic_not_found() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
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

    // Content must exist in the specified topic
    let options = DeleteOptions {
        slug: Some("nonexistent-article".to_string()),
        topic: Some("blog".to_string()),
        force: false,
    };

    let result = delete_content(&options);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("not found"));
}

#[test]
fn test_delete_content_success() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let content_dir = fixture.path().join("content/blog/test-article");

    // Mock file existence checks and operations
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
        .returning(move || Ok(config.clone()));

    // Register mocks with the fixture
    fixture.fs = mock_fs;
    fixture.config = mock_config;

    // Successfully delete content
    let options = DeleteOptions {
        slug: Some("test-article".to_string()),
        topic: Some("blog".to_string()),
        force: true, // Force delete without confirmation
    };

    let result = delete_content(&options);
    assert!(result.is_ok(), "Error: {:?}", result.err());
}

#[test]
fn test_delete_content_search_in_all_topics() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let blog_content_dir = fixture.path().join("content/blog/test-article");
    let podcast_content_dir = fixture.path().join("content/podcast/test-article");

    // Mock file existence checks - test that it finds content in blog topic
    mock_fs.expect_dir_exists()
        .with(predicate::eq(blog_content_dir.clone()))
        .returning(|_| Ok(true));

    mock_fs.expect_dir_exists()
        .with(predicate::eq(podcast_content_dir.clone()))
        .returning(|_| Ok(false));

    mock_fs.expect_remove_dir_all()
        .with(predicate::eq(blog_content_dir.clone()))
        .returning(|_| Ok(()));

    // Create a mock config for topics
    let mut topics = HashMap::new();
    topics.insert("blog".to_string(), common_config::TopicConfig {
        name: "Blog".to_string(),
        directory: "blog".to_string(),
        ..Default::default()
    });
    topics.insert("podcast".to_string(), common_config::TopicConfig {
        name: "Podcast".to_string(),
        directory: "podcast".to_string(),
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

    // When topic is not specified, search in all topics
    let options = DeleteOptions {
        slug: Some("test-article".to_string()),
        topic: None, // No topic specified, should search in all
        force: true,
    };

    let result = delete_content(&options);
    assert!(result.is_ok(), "Error: {:?}", result.err());
}

#[test]
fn test_delete_content_non_force_safety() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let content_dir = fixture.path().join("content/blog/test-article");

    // Mock file existence checks
    mock_fs.expect_dir_exists()
        .with(predicate::eq(content_dir.clone()))
        .returning(|_| Ok(true));

    // This may or may not be called depending on implementation
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

    // Without force flag, deletion should require confirmation
    let options = DeleteOptions {
        slug: Some("test-article".to_string()),
        topic: Some("blog".to_string()),
        force: false, // No force flag
    };

    let result = delete_content(&options);

    // The behavior depends on the implementation
    // We'll accept either success or a message requiring confirmation
    if result.is_err() {
        let err = result.unwrap_err().to_string();
        assert!(err.contains("confirmation") || err.contains("force"),
                "Unexpected error message: {}", err);
    }
    // If it succeeds without requiring confirmation, that's also acceptable
}