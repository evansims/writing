use content_delete::{delete_content, DeleteOptions};
use proptest::prelude::*;
use common_test_utils::fixtures::TestFixture;
use common_test_utils::mocks::{MockFileSystem, MockConfigLoader};
use std::collections::HashMap;
use mockall::predicate;

// Simple strategy for valid slugs
fn valid_slug_strategy() -> impl Strategy<Value = String> {
    r"[a-z0-9][a-z0-9\-]{1,50}".prop_map(String::from)
}

// Simple strategy for invalid slugs
fn invalid_slug_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        "[A-Z]+".prop_map(String::from),
        r"[a-z]+\s+[a-z]+".prop_map(String::from),
        r"[^a-zA-Z0-9\-]+".prop_map(String::from),
        "".prop_map(String::from)
    ]
}

// Simple strategy for valid topics
fn valid_topic_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("blog".to_string()),
        Just("podcast".to_string()),
        Just("notes".to_string())
    ]
}

// Simple strategy for invalid topics
fn invalid_topic_strategy() -> impl Strategy<Value = String> {
    r"[^a-zA-Z0-9\-]+".prop_map(String::from)
}

proptest! {
    #[test]
    fn test_delete_content_with_valid_inputs(
        slug in valid_slug_strategy(),
        topic in valid_topic_strategy(),
    ) {
        // Arrange - Create a fixture and mocks
        let fixture = TestFixture::new().unwrap();
        let mut mock_fs = MockFileSystem::new();

        // Define test paths
        let content_dir = fixture.path().join(format!("content/{}/{}", topic, slug));

        // Mock file existence checks
        mock_fs.expect_exists()
            .with(predicate::eq(content_dir.clone()))
            .returning(|_| true);

        mock_fs.expect_remove_dir_all()
            .with(predicate::eq(content_dir.clone()))
            .returning(|_| Ok(()));

        // Create a mock config for topics
        let mut topics = HashMap::new();
        topics.insert(topic.clone(), common_config::TopicConfig {
            name: topic.clone(),
            directory: topic.clone(),
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

        // Create delete options with force flag to ensure deletion happens
        let options = DeleteOptions {
            slug: Some(slug.clone()),
            topic: Some(topic.clone()),
            force: true,
        };

        // Delete the content
        let result = delete_content(&options);

        // Verify deletion was successful
        prop_assert!(result.is_ok(), "Deletion should succeed with valid inputs");
    }

    #[test]
    fn test_delete_content_fails_with_invalid_slug(
        slug in invalid_slug_strategy(),
        topic in valid_topic_strategy(),
    ) {
        // Arrange - Create a fixture and mocks
        let fixture = TestFixture::new().unwrap();
        let mut mock_fs = MockFileSystem::new();

        // Define test paths - we expect this path to be checked
        let content_dir = fixture.path().join(format!("content/{}/{}", topic, slug));

        // Mock file existence checks - content should not exist
        mock_fs.expect_exists()
            .with(predicate::eq(content_dir.clone()))
            .returning(|_| false);

        // Create a mock config for topics
        let mut topics = HashMap::new();
        topics.insert(topic.clone(), common_config::TopicConfig {
            name: topic.clone(),
            directory: topic.clone(),
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

        // Create delete options with invalid slug
        let options = DeleteOptions {
            slug: Some(slug),
            topic: Some(topic),
            force: true,
        };

        // Try to delete content with invalid slug
        let result = delete_content(&options);

        // Should fail with error indicating content not found
        prop_assert!(result.is_err(), "Deletion should fail with invalid slug");
        let err = result.unwrap_err().to_string();
        prop_assert!(
            err.contains("not found") ||
            err.contains("invalid") ||
            err.contains("Invalid slug"),
            "Error should indicate content not found or invalid slug"
        );
    }

    #[test]
    fn test_delete_content_fails_with_invalid_topic(
        slug in valid_slug_strategy(),
        topic in invalid_topic_strategy(),
    ) {
        // Arrange - Create a fixture and mocks
        let fixture = TestFixture::new().unwrap();

        // Create a mock config with only valid topics
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
        fixture.register_config_loader(Box::new(mock_config));

        // Create delete options with invalid topic
        let options = DeleteOptions {
            slug: Some(slug),
            topic: Some(topic),
            force: true,
        };

        // Try to delete content with invalid topic
        let result = delete_content(&options);

        // Should fail with error indicating topic not found
        prop_assert!(result.is_err(), "Deletion should fail with invalid topic");
        let err = result.unwrap_err().to_string();
        prop_assert!(
            err.contains("not found") ||
            err.contains("invalid") ||
            err.contains("Invalid topic"),
            "Error should indicate topic not found or invalid topic"
        );
    }

    #[test]
    fn test_delete_content_without_slug_always_fails(
        topic in prop::option::of(valid_topic_strategy()),
        force in prop::bool::ANY,
    ) {
        // Create delete options without slug
        let options = DeleteOptions {
            slug: None,
            topic,
            force,
        };

        // Try to delete content without slug
        let result = delete_content(&options);

        // Should always fail with error indicating slug is required
        prop_assert!(result.is_err(), "Deletion should fail without slug");
        let err = result.unwrap_err().to_string();
        prop_assert!(
            err.contains("Slug is required") ||
            err.contains("slug is required") ||
            err.contains("Missing slug"),
            "Error should indicate slug is required"
        );
    }
}