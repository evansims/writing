use anyhow::Result;
use common_test_utils::TestFixture;
use common_models::{Config, TopicConfig};
use std::collections::HashMap;
use std::path::Path;
use topic_delete::{delete_topic, has_content, topic_exists, get_topic_keys_except, TopicDeleteOptions};
use serial_test::serial;

#[test]
fn test_topic_exists_returns_true_when_topic_exists() {
    // Arrange
    let mut config = Config::default();
    let mut topics = HashMap::new();
    topics.insert(
        "test-topic".to_string(),
        TopicConfig {
            name: "Test Topic".to_string(),
            description: "A test topic".to_string(),
            directory: "test-topic".to_string(),
        },
    );
    config.content.topics = topics;

    // Act & Assert
    assert!(topic_exists(&config, "test-topic"));
}

#[test]
fn test_topic_exists_returns_false_when_topic_does_not_exist() {
    // Arrange
    let config = Config::default();

    // Act & Assert
    assert!(!topic_exists(&config, "nonexistent-topic"));
}

#[test]
fn test_get_topic_keys_except_excludes_provided_key() {
    // Arrange
    let mut config = Config::default();
    let mut topics = HashMap::new();
    topics.insert(
        "topic1".to_string(),
        TopicConfig {
            name: "Topic 1".to_string(),
            description: "First topic".to_string(),
            directory: "topic1".to_string(),
        },
    );
    topics.insert(
        "topic2".to_string(),
        TopicConfig {
            name: "Topic 2".to_string(),
            description: "Second topic".to_string(),
            directory: "topic2".to_string(),
        },
    );
    topics.insert(
        "topic3".to_string(),
        TopicConfig {
            name: "Topic 3".to_string(),
            description: "Third topic".to_string(),
            directory: "topic3".to_string(),
        },
    );
    config.content.topics = topics;

    // Act
    let keys = get_topic_keys_except(&config, "topic2");

    // Assert
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&"topic1".to_string()));
    assert!(keys.contains(&"topic3".to_string()));
    assert!(!keys.contains(&"topic2".to_string()));
}

#[test]
fn test_has_content_returns_true_when_directory_has_files() -> Result<()> {
    // Arrange
    let fixture = TestFixture::new()?;
    let base_dir = fixture.path().to_string_lossy().to_string();
    let topic_dir = "test-topic";
    let full_path = format!("{}/{}", base_dir, topic_dir);

    // Create directory with a file
    std::fs::create_dir_all(&full_path)?;
    std::fs::write(format!("{}/test-file.txt", full_path), "test content")?;

    // Act & Assert
    assert!(has_content(&base_dir, topic_dir));

    Ok(())
}

#[test]
fn test_has_content_returns_false_when_directory_is_empty() -> Result<()> {
    // Arrange
    let fixture = TestFixture::new()?;
    let base_dir = fixture.path().to_string_lossy().to_string();
    let topic_dir = "empty-topic";
    let full_path = format!("{}/{}", base_dir, topic_dir);

    // Create empty directory
    std::fs::create_dir_all(&full_path)?;

    // Act & Assert
    assert!(!has_content(&base_dir, topic_dir));

    Ok(())
}

#[test]
fn test_has_content_returns_false_when_directory_does_not_exist() -> Result<()> {
    // Arrange
    let fixture = TestFixture::new()?;
    let base_dir = fixture.path().to_string_lossy().to_string();
    let topic_dir = "nonexistent-topic";

    // Act & Assert
    assert!(!has_content(&base_dir, topic_dir));

    Ok(())
}

#[test]
#[serial]
fn test_delete_topic_validates_empty_key() -> Result<()> {
    // Arrange - Create a config file
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

    // Create a basic config
    let config = Config::default();

    // Save the config to the fixture path
    let config_str = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, &config_str)?;

    // Set the config path environment variable
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_EMPTY_KEY_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Create options with no key
    let options = TopicDeleteOptions {
        key: None,
        target: None,
        force: false,
    };

    // Act
    let result = delete_topic(&options);

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Topic key is required"));

    Ok(())
}

#[test]
#[serial]
fn test_delete_topic_validates_nonexistent_topic() -> Result<()> {
    // Arrange - Create a config file
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

    // Create a basic config
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

    // Set the config path environment variable
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_NONEXISTENT_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Create options with a nonexistent topic key
    let options = TopicDeleteOptions {
        key: Some("nonexistent-topic".to_string()),
        target: None,
        force: true,
    };

    // Act
    let result = delete_topic(&options);

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("not found"));

    Ok(())
}

#[test]
#[serial]
fn test_delete_topic_successfully_deletes_topic() -> Result<()> {
    // Arrange - Create a config file
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

    // Create content directory
    let content_dir = fixture_path.join("content");
    std::fs::create_dir_all(&content_dir)?;
    let topic_dir = content_dir.join("test-topic");
    std::fs::create_dir_all(&topic_dir)?;

    // Create a basic config
    let mut config = Config::default();
    let mut topics = HashMap::new();
    topics.insert(
        "test-topic".to_string(),
        TopicConfig {
            name: "Test Topic".to_string(),
            description: "A test topic".to_string(),
            directory: "test-topic".to_string(),
        },
    );
    config.content.topics = topics;
    config.content.base_dir = "content".to_string();

    // Save the config to the fixture path
    let config_str = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, &config_str)?;

    // Set the config path environment variable
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_SUCCESS_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Create options to delete the topic
    let options = TopicDeleteOptions {
        key: Some("test-topic".to_string()),
        target: None,
        force: true,
    };

    // Act
    let result = delete_topic(&options);

    // Verify the update by reloading the config
    let updated_config = common_config::load_config_from_path(&config_path)?;

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_ok());
    assert!(!updated_config.content.topics.contains_key("test-topic"));

    // Verify directory still exists (delete_topic shouldn't delete the directory by default)
    assert!(Path::new(&fixture_path).join("content").join("test-topic").exists());

    Ok(())
}

#[test]
#[serial]
fn test_delete_topic_with_content_migration() -> Result<()> {
    // Arrange - Create a config file
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

    // Create content directory with two topics
    let content_dir = fixture_path.join("content");
    std::fs::create_dir_all(&content_dir)?;

    // Source topic with a file
    let source_dir = content_dir.join("source-topic");
    std::fs::create_dir_all(&source_dir)?;
    std::fs::write(source_dir.join("test-file.txt"), "test content")?;

    // Target topic
    let target_dir = content_dir.join("target-topic");
    std::fs::create_dir_all(&target_dir)?;

    // Create a basic config
    let mut config = Config::default();
    let mut topics = HashMap::new();
    topics.insert(
        "source-topic".to_string(),
        TopicConfig {
            name: "Source Topic".to_string(),
            description: "Source topic to delete".to_string(),
            directory: "source-topic".to_string(),
        },
    );
    topics.insert(
        "target-topic".to_string(),
        TopicConfig {
            name: "Target Topic".to_string(),
            description: "Target for content migration".to_string(),
            directory: "target-topic".to_string(),
        },
    );
    config.content.topics = topics;
    config.content.base_dir = "content".to_string();

    // Save the config to the fixture path
    let config_str = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, &config_str)?;

    // Set the config path environment variable
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_MIGRATION_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Create options to delete the topic and migrate content
    let options = TopicDeleteOptions {
        key: Some("source-topic".to_string()),
        target: Some("target-topic".to_string()),
        force: true,
    };

    // Act
    let result = delete_topic(&options);

    // Verify the update by reloading the config
    let updated_config = common_config::load_config_from_path(&config_path)?;

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_ok());
    assert!(!updated_config.content.topics.contains_key("source-topic"));

    // Target topic should still exist
    assert!(updated_config.content.topics.contains_key("target-topic"));

    // The migrated file should be in the target directory
    let target_file = Path::new(&fixture_path).join("content").join("target-topic").join("test-file.txt");
    assert!(target_file.exists());

    Ok(())
}