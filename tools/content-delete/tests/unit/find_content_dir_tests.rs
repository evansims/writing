use content_delete::find_content_dir;
use common_test_utils::fixtures::TestFixture;
use common_test_utils::mocks::{MockFileSystem, MockConfigLoader};
use std::path::PathBuf;
use std::collections::HashMap;
use mockall::predicate;

#[test]
fn test_find_content_dir_with_topic() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let content_dir = fixture.path().join("content/blog/test-article");

    // Mock file existence checks
    mock_fs.expect_exists()
        .with(predicate::eq(content_dir.clone()))
        .returning(|_| true);

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
    fixture.register_fs(Box::new(mock_fs));
    fixture.register_config_loader(Box::new(mock_config));

    // Act - Call function with topic specified
    let result = find_content_dir("test-article", Some("blog"));

    // Assert - Verify result
    assert!(result.is_ok(), "Failed to find content dir: {:?}", result.err());
    let (found_dir, found_topic) = result.unwrap();
    assert_eq!(found_topic, "blog");

    // Check that the paths match (considering normalization)
    let expected_path = content_dir;
    let normalized_found = PathBuf::from(&found_dir);

    // Compare the final components which should be the article directory
    assert_eq!(
        expected_path.file_name().unwrap(),
        normalized_found.file_name().unwrap()
    );
}

#[test]
fn test_find_content_dir_without_topic() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let blog_content_dir = fixture.path().join("content/blog/test-article");
    let podcast_content_dir = fixture.path().join("content/podcast/some-article");

    // Mock file existence checks - first check for blog fails, second for podcast succeeds
    mock_fs.expect_exists()
        .with(predicate::eq(blog_content_dir.clone()))
        .returning(|_| true);

    mock_fs.expect_exists()
        .with(predicate::eq(podcast_content_dir.clone()))
        .returning(|_| false);

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
    fixture.register_fs(Box::new(mock_fs));
    fixture.register_config_loader(Box::new(mock_config));

    // Act - Call function without topic specified (should search all topics)
    let result = find_content_dir("test-article", None);

    // Assert - Verify result
    assert!(result.is_ok(), "Failed to find content dir: {:?}", result.err());
    let (found_dir, found_topic) = result.unwrap();
    assert_eq!(found_topic, "blog");

    // Check that the paths match (considering normalization)
    let expected_path = blog_content_dir;
    let normalized_found = PathBuf::from(found_dir);

    // Compare the final components which should be the article directory
    assert_eq!(
        expected_path.file_name().unwrap(),
        normalized_found.file_name().unwrap()
    );
}

#[test]
fn test_find_content_dir_nonexistent_content() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let content_dir = fixture.path().join("content/blog/nonexistent-article");

    // Mock file existence checks
    mock_fs.expect_exists()
        .with(predicate::eq(content_dir.clone()))
        .returning(|_| false);

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
    fixture.register_fs(Box::new(mock_fs));
    fixture.register_config_loader(Box::new(mock_config));

    // Act - Call function with nonexistent content
    let result = find_content_dir("nonexistent-article", Some("blog"));

    // Assert - Verify result
    assert!(result.is_err(), "Should have failed to find content");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("not found") || err.contains("Content not found"));
}

#[test]
fn test_find_content_dir_invalid_topic() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();

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
    fixture.register_config_loader(Box::new(mock_config));

    // Act - Call function with invalid topic
    let result = find_content_dir("test-article", Some("nonexistent-topic"));

    // Assert - Verify result
    assert!(result.is_err(), "Should have failed to find content with invalid topic");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Invalid topic") || err.contains("not found"));
}

#[test]
fn test_find_content_dir_same_slug_different_topics() {
    // Arrange - Create a fixture and mocks
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let blog_content_dir = fixture.path().join("content/blog/test-article");
    let podcast_content_dir = fixture.path().join("content/podcast/test-article");

    // Mock file existence checks
    mock_fs.expect_exists()
        .with(predicate::eq(blog_content_dir.clone()))
        .returning(|_| true);

    mock_fs.expect_exists()
        .with(predicate::eq(podcast_content_dir.clone()))
        .returning(|_| true);

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
        .times(3) // once for blog, once for podcast, once for no topic
        .returning(move || Ok(config.clone()));

    // Register mocks with the fixture
    fixture.register_fs(Box::new(mock_fs));
    fixture.register_config_loader(Box::new(mock_config));

    // Act & Assert - Find with blog topic
    let blog_result = find_content_dir("test-article", Some("blog"));
    assert!(blog_result.is_ok(), "Failed to find blog content: {:?}", blog_result.err());
    let (blog_dir_found, blog_topic) = blog_result.unwrap();
    assert_eq!(blog_topic, "blog");

    // Act & Assert - Find with podcast topic
    let podcast_result = find_content_dir("test-article", Some("podcast"));
    assert!(podcast_result.is_ok(), "Failed to find podcast content: {:?}", podcast_result.err());
    let (podcast_dir_found, podcast_topic) = podcast_result.unwrap();
    assert_eq!(podcast_topic, "podcast");

    // Act & Assert - Find without topic (should find the first match)
    let result = find_content_dir("test-article", None);
    assert!(result.is_ok(), "Failed to find any content: {:?}", result.err());

    // Note: The exact topic found depends on the implementation's search order
    // but it should find one of the topics
    let (_, found_topic) = result.unwrap();
    assert!(found_topic == "blog" || found_topic == "podcast",
           "Found unexpected topic: {}", found_topic);
}