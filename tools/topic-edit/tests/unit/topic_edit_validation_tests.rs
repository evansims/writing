use anyhow::Result;
use common_test_utils::TestFixture;
use common_models::{Config, TopicConfig};
use std::collections::HashMap;
use std::path::PathBuf;
use topic_edit::{edit_topic, TopicEditOptions};
use serial_test::serial;

#[test]
#[serial]
fn test_edit_topic_validates_empty_key() -> Result<()> {
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
    let options = TopicEditOptions {
        key: None,
        name: Some("New Name".to_string()),
        description: Some("New description".to_string()),
    };

    // Act
    let result = edit_topic(&options);

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    println!("Actual error: {}", error);
    assert!(error.contains("No topic key provided"));

    Ok(())
}

#[test]
#[serial]
fn test_edit_topic_validates_nonexistent_topic() -> Result<()> {
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
    let options = TopicEditOptions {
        key: Some("nonexistent-topic".to_string()),
        name: Some("New Name".to_string()),
        description: Some("New description".to_string()),
    };

    // Act
    let result = edit_topic(&options);

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
fn test_edit_topic_successfully_updates_name_and_description() -> Result<()> {
    // Arrange - Create a config file
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

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

    // Create options to update the topic
    let options = TopicEditOptions {
        key: Some("test-topic".to_string()),
        name: Some("Updated Topic Name".to_string()),
        description: Some("Updated topic description".to_string()),
    };

    // Act
    let result = edit_topic(&options);

    // Verify the update by reloading the config
    let updated_config = common_config::load_config_from_path(&config_path)?;
    let updated_topic = updated_config.content.topics.get("test-topic").unwrap();

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_ok());
    assert_eq!(updated_topic.name, "Updated Topic Name");
    assert_eq!(updated_topic.description, "Updated topic description");

    Ok(())
}

#[test]
#[serial]
fn test_edit_topic_updates_only_provided_fields() -> Result<()> {
    // Arrange - Create a config file
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

    // Create a basic config
    let mut config = Config::default();
    let mut topics = HashMap::new();
    topics.insert(
        "test-topic".to_string(),
        TopicConfig {
            name: "Original Name".to_string(),
            description: "Original description".to_string(),
            directory: "test-topic".to_string(),
        },
    );
    config.content.topics = topics;

    // Save the config to the fixture path
    let config_str = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, &config_str)?;

    // Set the config path environment variable
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_PARTIAL_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Create options with only name update
    let options = TopicEditOptions {
        key: Some("test-topic".to_string()),
        name: Some("Updated Name".to_string()),
        description: None,
    };

    // Act
    let result = edit_topic(&options);

    // Verify the update by reloading the config
    let updated_config = common_config::load_config_from_path(&config_path)?;
    let updated_topic = updated_config.content.topics.get("test-topic").unwrap();

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_ok());
    assert_eq!(updated_topic.name, "Updated Name"); // Should be updated
    assert_eq!(updated_topic.description, "Original description"); // Should remain unchanged

    Ok(())
}