use anyhow::Result;
use common_test_utils::TestFixture;
use common_models::{Config, TopicConfig};
use std::collections::HashMap;
use topic_add::{add_topic, AddOptions};
use serial_test::serial;
use common_config::ConfigLoader;
use common_test_utils::mocks::{MockConfigLoader, MockFileSystem};
use mockall::{predicate, Sequence};
use topic_add::{add_tags_to_topic};

#[test]
fn test_add_topic_rejects_empty_key() {
    // Arrange
    let options = AddOptions {
        key: "".to_string(),
        name: "Test Topic".to_string(),
        description: "A test topic".to_string(),
        directory: "test-topic".to_string(),
    };

    // Act
    let result = add_topic(&options);

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Topic key is required"
    );
}

#[test]
fn test_add_topic_rejects_empty_name() {
    // Arrange
    let options = AddOptions {
        key: "test-topic".to_string(),
        name: "".to_string(),
        description: "A test topic".to_string(),
        directory: "test-topic".to_string(),
    };

    // Act
    let result = add_topic(&options);

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Topic name is required"
    );
}

#[test]
fn test_add_topic_rejects_empty_description() {
    // Arrange
    let options = AddOptions {
        key: "test-topic".to_string(),
        name: "Test Topic".to_string(),
        description: "".to_string(),
        directory: "test-topic".to_string(),
    };

    // Act
    let result = add_topic(&options);

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Topic description is required"
    );
}

#[test]
fn test_add_topic_rejects_empty_directory() {
    // Arrange
    let options = AddOptions {
        key: "test-topic".to_string(),
        name: "Test Topic".to_string(),
        description: "A test topic".to_string(),
        directory: "".to_string(),
    };

    // Act
    let result = add_topic(&options);

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Topic directory is required"
    );
}

#[test]
fn test_add_topic_rejects_duplicate_key() {
    // Arrange - Set up environment variable to avoid touching real config file
    temp_env::with_var("CONFIG_PATH", Some("temp_config.yaml"), || {
        // Create a config with existing topic
        let mut config = Config::default();
        let mut topics = HashMap::new();
        topics.insert(
            "existing-topic".to_string(),
            TopicConfig {
                name: "Existing Topic".to_string(),
                description: "An existing topic".to_string(),
                directory: "existing-topic".to_string(),
            },
        );
        config.content.topics = topics;

        // Write config to temporary file
        let config_content = serde_yaml::to_string(&config).unwrap();
        std::fs::write("temp_config.yaml", config_content).unwrap();

        // Create options with duplicate key
        let options = AddOptions {
            key: "existing-topic".to_string(),
            name: "New Topic".to_string(),
            description: "A new topic".to_string(),
            directory: "new-topic".to_string(),
        };

        // Act
        let result = add_topic(&options);

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Topic with key 'existing-topic' already exists"
        );

        // Clean up
        std::fs::remove_file("temp_config.yaml").unwrap_or(());
    });
}

#[test]
fn test_add_tags_to_topic_rejects_nonexistent_topic() {
    // Arrange - Set up environment variable to avoid touching real config file
    temp_env::with_var("CONFIG_PATH", Some("temp_config.yaml"), || {
        // Create a config with no topics
        let config = Config::default();

        // Write config to temporary file
        let config_content = serde_yaml::to_string(&config).unwrap();
        std::fs::write("temp_config.yaml", config_content).unwrap();

        // Act
        let result = add_tags_to_topic("nonexistent-topic", vec!["tag1".to_string(), "tag2".to_string()]);

        // Assert
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Topic with key 'nonexistent-topic' does not exist"
        );

        // Clean up
        std::fs::remove_file("temp_config.yaml").unwrap_or(());
    });
}

#[test]
fn test_add_tags_to_topic_handles_empty_tags() {
    // Arrange - Set up environment variable to avoid touching real config file
    temp_env::with_var("CONFIG_PATH", Some("temp_config.yaml"), || {
        // Create a config with existing topic
        let mut config = Config::default();
        let mut topics = HashMap::new();
        topics.insert(
            "existing-topic".to_string(),
            TopicConfig {
                name: "Existing Topic".to_string(),
                description: "An existing topic".to_string(),
                directory: "existing-topic".to_string(),
            },
        );
        config.content.topics = topics;

        // Write config to temporary file
        let config_content = serde_yaml::to_string(&config).unwrap();
        std::fs::write("temp_config.yaml", config_content).unwrap();

        // Act - Add empty tags
        let result = add_tags_to_topic("existing-topic", vec![]);

        // Assert - Should not error but return false (no tags added)
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), false);

        // Clean up
        std::fs::remove_file("temp_config.yaml").unwrap_or(());
    });
}

#[test]
fn test_add_tags_creates_tags_map_if_none_exists() {
    // Arrange - Set up environment variable to avoid touching real config file
    temp_env::with_var("CONFIG_PATH", Some("temp_config.yaml"), || {
        // Create a config with existing topic but no tags map
        let mut config = Config::default();
        let mut topics = HashMap::new();
        topics.insert(
            "existing-topic".to_string(),
            TopicConfig {
                name: "Existing Topic".to_string(),
                description: "An existing topic".to_string(),
                directory: "existing-topic".to_string(),
            },
        );
        config.content.topics = topics;
        config.content.tags = None; // Ensure no tags map exists

        // Write config to temporary file
        let config_content = serde_yaml::to_string(&config).unwrap();
        std::fs::write("temp_config.yaml", config_content).unwrap();

        // Act - Add tags
        let result = add_tags_to_topic("existing-topic", vec!["tag1".to_string(), "tag2".to_string()]);

        // Assert - Should succeed
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);

        // Verify tags were added correctly
        let updated_config = common_config::load_config().unwrap();
        assert!(updated_config.content.tags.is_some());
        let tags = updated_config.content.tags.unwrap();
        assert!(tags.contains_key("existing-topic"));
        assert_eq!(tags.get("existing-topic").unwrap().len(), 2);
        assert!(tags.get("existing-topic").unwrap().contains(&"tag1".to_string()));
        assert!(tags.get("existing-topic").unwrap().contains(&"tag2".to_string()));

        // Clean up
        std::fs::remove_file("temp_config.yaml").unwrap_or(());
    });
}

#[test]
#[serial]
fn test_add_topic_fails_if_topic_already_exists() -> Result<()> {
    // Arrange - Create a config with an existing topic
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

    // Setup a test Config with a topic
    let mut config = Config::default();
    let mut topics = HashMap::new();
    topics.insert(
        "existing-topic".to_string(),
        TopicConfig {
            name: "Existing Topic".to_string(),
            description: "An existing topic".to_string(),
            directory: "existing-topic".to_string(),
        },
    );
    config.content.topics = topics;
    config.content.base_dir = "content".to_string();

    // Save the config to the fixture path
    let config_str = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, &config_str)?;

    // Set the config path environment variable with a unique name to avoid conflicts
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_EXISTS_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Verify config is loaded correctly with existing topic
    let loaded_config = common_config::load_config_from_path(&config_path)?;
    assert!(loaded_config.content.topics.contains_key("existing-topic"),
            "Loaded config should contain 'existing-topic'");

    // Create the content directory and the existing topic directory
    let content_dir = fixture_path.join("content");
    std::fs::create_dir_all(&content_dir)?;
    std::fs::create_dir_all(content_dir.join("existing-topic"))?;

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Create options with the same key as the existing topic
    let options = AddOptions {
        key: "existing-topic".to_string(),
        name: "New Topic".to_string(),
        description: "A new topic".to_string(),
        directory: "new-topic".to_string(),
    };

    // Act
    let result = add_topic(&options);

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_err(), "Expected an error but got success: {:?}", result);
    if let Err(err) = result {
        let error = err.to_string();
        assert!(error.contains("Topic with key 'existing-topic' already exists"),
                "Error message did not contain expected text: {}", error);
    }

    Ok(())
}

#[test]
#[serial]
fn test_add_topic_succeeds_with_valid_options() -> Result<()> {
    // Arrange - Create a config without the topic we're going to add
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

    // Setup a test Config
    let mut config = Config::default();
    let mut topics = HashMap::new();
    topics.insert(
        "blog".to_string(),
        TopicConfig {
            name: "Blog".to_string(),
            description: "Blog posts".to_string(),
            directory: "blog".to_string(),
        },
    );
    config.content.topics = topics;
    config.content.base_dir = "content".to_string();

    // Save the config to the fixture path
    let config_str = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, &config_str)?;

    // Set the config path environment variable with a unique name for this test
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_SUCCEEDS_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Verify config is loaded correctly
    let loaded_config = common_config::load_config_from_path(&config_path)?;
    assert!(loaded_config.content.topics.contains_key("blog"),
            "Loaded config should contain 'blog' topic");

    // Create the content directory
    let content_dir = fixture_path.join("content");
    std::fs::create_dir_all(&content_dir)?;
    std::fs::create_dir_all(content_dir.join("blog"))?;

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Create valid options
    let options = AddOptions {
        key: "new-topic".to_string(),
        name: "New Topic".to_string(),
        description: "A new topic".to_string(),
        directory: "new-topic".to_string(),
    };

    // Act
    let result = add_topic(&options);
    assert!(result.is_ok(), "Expected add_topic to succeed, but got error: {:?}", result);
    let result_key = result.unwrap();
    assert_eq!(result_key, "new-topic", "Expected key to be 'new-topic', got '{}'", result_key);

    // Ensure filesystem operations have completed and verify the topic directory exists
    let topic_dir = fixture_path.join("content").join("new-topic");

    // Verify directory exists with retries to handle any filesystem delays
    let mut attempts = 0;
    const MAX_ATTEMPTS: usize = 3;
    let mut dir_exists = false;

    while attempts < MAX_ATTEMPTS {
        if topic_dir.exists() {
            dir_exists = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
        attempts += 1;
    }

    assert!(dir_exists, "Topic directory was not created at: {:?} after {} attempts",
            topic_dir, MAX_ATTEMPTS);

    // Load the updated config and verify the topic is in it
    let updated_config = std::fs::read_to_string(&config_path)?;
    let parsed_config: Config = serde_yaml::from_str(&updated_config)?;
    assert!(
        parsed_config.content.topics.contains_key("new-topic"),
        "Topic 'new-topic' was not found in the updated config"
    );

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    Ok(())
}