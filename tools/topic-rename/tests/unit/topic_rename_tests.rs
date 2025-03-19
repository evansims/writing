use anyhow::Result;
use common_test_utils::TestFixture;
use common_models::{Config, TopicConfig, ContentConfig};
use std::collections::HashMap;
use std::path::Path;
use topic_rename::{rename_topic, TopicRenameOptions};
use serial_test::serial;
use temp_env;

// Define a RenameOptions struct for our tests
struct RenameOptions {
    old_key: String,
    new_key: String,
    update_references: bool,
}

impl From<&RenameOptions> for TopicRenameOptions {
    fn from(options: &RenameOptions) -> Self {
        TopicRenameOptions {
            key: Some(options.old_key.clone()),
            new_key: Some(options.new_key.clone()),
            new_name: None,
            new_description: None,
            new_path: None,
        }
    }
}

#[test]
#[serial]
fn test_rename_topic_validates_empty_key() -> Result<()> {
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
    let options = TopicRenameOptions {
        key: None,
        new_key: Some("new-key".to_string()),
        new_name: Some("New Name".to_string()),
        new_description: Some("New description".to_string()),
        new_path: None,
    };

    // Act
    let result = rename_topic(&options);

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Current topic key is required"));

    Ok(())
}

#[test]
#[serial]
fn test_rename_topic_validates_nonexistent_topic() -> Result<()> {
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
    let options = TopicRenameOptions {
        key: Some("nonexistent-topic".to_string()),
        new_key: Some("new-key".to_string()),
        new_name: Some("New Name".to_string()),
        new_description: Some("New description".to_string()),
        new_path: None,
    };

    // Act
    let result = rename_topic(&options);

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Topic not found"));

    Ok(())
}

#[test]
#[serial]
fn test_rename_topic_validates_duplicate_new_key() -> Result<()> {
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
    topics.insert(
        "another-topic".to_string(),
        TopicConfig {
            name: "Another Topic".to_string(),
            description: "Another existing topic".to_string(),
            directory: "another-topic".to_string(),
        },
    );
    config.content.topics = topics;
    config.content.base_dir = "content".to_string();

    // Save the config to the fixture path
    let config_str = serde_yaml::to_string(&config)?;
    std::fs::write(&config_path, &config_str)?;

    // Set the config path environment variable
    let unique_env_var = format!("CONFIG_PATH_UNIQUE_DUPLICATE_{}", std::process::id());
    std::env::set_var(&unique_env_var, config_path.to_string_lossy().to_string());
    std::env::set_var("CONFIG_PATH", config_path.to_string_lossy().to_string());

    // Switch to the fixture directory
    let original_dir = std::env::current_dir()?;
    std::env::set_current_dir(&fixture_path)?;

    // Create options with a duplicate new key
    let options = TopicRenameOptions {
        key: Some("existing-topic".to_string()),
        new_key: Some("another-topic".to_string()), // This key already exists
        new_name: Some("New Name".to_string()),
        new_description: Some("New description".to_string()),
        new_path: None,
    };

    // Act
    let result = rename_topic(&options);

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Topic already exists"));

    Ok(())
}

#[test]
#[serial]
fn test_rename_topic_successfully_renames_with_new_key() -> Result<()> {
    // Arrange - Create a config file
    let fixture = TestFixture::new()?;
    let fixture_path = fixture.path().to_path_buf();
    let config_path = fixture_path.join("config.yaml");

    // Create content directory
    let content_dir = fixture_path.join("content");
    std::fs::create_dir_all(&content_dir)?;
    let topic_dir = content_dir.join("old-topic");
    std::fs::create_dir_all(&topic_dir)?;

    // Create a basic config
    let mut config = Config::default();
    let mut topics = HashMap::new();
    topics.insert(
        "old-topic".to_string(),
        TopicConfig {
            name: "Old Topic".to_string(),
            description: "An old topic".to_string(),
            directory: "old-topic".to_string(),
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

    // Create options to rename the topic
    let options = TopicRenameOptions {
        key: Some("old-topic".to_string()),
        new_key: Some("new-topic".to_string()),
        new_name: Some("New Topic".to_string()),
        new_description: Some("A new topic".to_string()),
        new_path: Some("new-topic".to_string()),
    };

    // Act
    let result = rename_topic(&options);

    // Verify the update by reloading the config
    let updated_config = common_config::load_config_from_path(&config_path)?;

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "new-topic");

    // Old topic key should be removed
    assert!(!updated_config.content.topics.contains_key("old-topic"));

    // New topic key should be added
    assert!(updated_config.content.topics.contains_key("new-topic"));

    // Check new topic config
    let new_topic = updated_config.content.topics.get("new-topic").unwrap();
    assert_eq!(new_topic.name, "New Topic");
    assert_eq!(new_topic.description, "A new topic");
    assert_eq!(new_topic.directory, "new-topic");

    // Verify directory was moved
    assert!(Path::new(&fixture_path).join("content").join("new-topic").exists());
    assert!(!Path::new(&fixture_path).join("content").join("old-topic").exists());

    Ok(())
}

#[test]
#[serial]
fn test_rename_topic_updates_only_provided_fields() -> Result<()> {
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
    config.content.base_dir = "content".to_string();

    // Create the content directory
    let content_dir = fixture_path.join("content");
    std::fs::create_dir_all(&content_dir)?;
    let topic_dir = content_dir.join("test-topic");
    std::fs::create_dir_all(&topic_dir)?;

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
    let options = TopicRenameOptions {
        key: Some("test-topic".to_string()),
        new_key: None,
        new_name: Some("Updated Name".to_string()),
        new_description: None,
        new_path: None,
    };

    // Act
    let result = rename_topic(&options);

    // Verify the update by reloading the config
    let updated_config = common_config::load_config_from_path(&config_path)?;
    let updated_topic = updated_config.content.topics.get("test-topic").unwrap();

    // Cleanup - Switch back to the original directory
    std::env::set_current_dir(original_dir)?;
    std::env::remove_var("CONFIG_PATH");
    std::env::remove_var(&unique_env_var);

    // Assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test-topic"); // Key should remain unchanged
    assert_eq!(updated_topic.name, "Updated Name"); // Should be updated
    assert_eq!(updated_topic.description, "Original description"); // Should remain unchanged
    assert_eq!(updated_topic.directory, "test-topic"); // Should remain unchanged

    Ok(())
}

#[test]
fn test_rename_topic_changes_key_correctly() -> Result<()> {
    // Arrange - Set up environment variable to avoid touching real config file
    temp_env::with_var("CONFIG_PATH", Some("temp_config.yaml"), || -> Result<()> {
        // Create a config with existing topic
        let mut config = Config::default();
        let mut topics = HashMap::new();
        topics.insert(
            "old-key".to_string(),
            TopicConfig {
                name: "Old Topic".to_string(),
                description: "An existing topic".to_string(),
                directory: "old-topic".to_string(),
            },
        );
        config.content.topics = topics;

        // Write config to temporary file
        let config_content = serde_yaml::to_string(&config)?;
        std::fs::write("temp_config.yaml", config_content)?;

        // Create options for renaming
        let options = RenameOptions {
            old_key: "old-key".to_string(),
            new_key: "new-key".to_string(),
            update_references: false, // Don't update references for this test
        };

        // Act
        let result = rename_topic(&(&options).into());

        // Assert
        assert!(result.is_ok());

        // Verify the config was updated correctly
        let updated_config = common_config::load_config()?;

        // Old key should be gone
        assert!(!updated_config.content.topics.contains_key("old-key"));

        // New key should exist
        assert!(updated_config.content.topics.contains_key("new-key"));

        // Topic details should be preserved
        let topic = &updated_config.content.topics["new-key"];
        assert_eq!(topic.name, "Old Topic");
        assert_eq!(topic.description, "An existing topic");
        assert_eq!(topic.directory, "old-topic");

        // Clean up
        std::fs::remove_file("temp_config.yaml")?;

        Ok(())
    })?;

    Ok(())
}

#[test]
fn test_rename_topic_fails_if_old_key_does_not_exist() -> Result<()> {
    // Arrange - Set up environment variable to avoid touching real config file
    temp_env::with_var("CONFIG_PATH", Some("temp_config.yaml"), || -> Result<()> {
        // Create a config with no matching topic
        let config = Config::default();

        // Write config to temporary file
        let config_content = serde_yaml::to_string(&config)?;
        std::fs::write("temp_config.yaml", config_content)?;

        // Create options for renaming
        let options = RenameOptions {
            old_key: "nonexistent-key".to_string(),
            new_key: "new-key".to_string(),
            update_references: false,
        };

        // Act
        let result = rename_topic(&(&options).into());

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));

        // Clean up
        std::fs::remove_file("temp_config.yaml")?;

        Ok(())
    })?;

    Ok(())
}

#[test]
fn test_rename_topic_fails_if_new_key_already_exists() -> Result<()> {
    // Arrange - Set up environment variable to avoid touching real config file
    temp_env::with_var("CONFIG_PATH", Some("temp_config.yaml"), || -> Result<()> {
        // Create a config with both old and new keys
        let mut config = Config::default();
        let mut topics = HashMap::new();

        // Add old topic
        topics.insert(
            "old-key".to_string(),
            TopicConfig {
                name: "Old Topic".to_string(),
                description: "An existing topic".to_string(),
                directory: "old-topic".to_string(),
            },
        );

        // Add new topic (already exists)
        topics.insert(
            "new-key".to_string(),
            TopicConfig {
                name: "New Topic".to_string(),
                description: "Another existing topic".to_string(),
                directory: "new-topic".to_string(),
            },
        );

        config.content.topics = topics;

        // Write config to temporary file
        let config_content = serde_yaml::to_string(&config)?;
        std::fs::write("temp_config.yaml", config_content)?;

        // Create options for renaming
        let options = RenameOptions {
            old_key: "old-key".to_string(),
            new_key: "new-key".to_string(),
            update_references: false,
        };

        // Act
        let result = rename_topic(&(&options).into());

        // Assert
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("already exists"));

        // Clean up
        std::fs::remove_file("temp_config.yaml")?;

        Ok(())
    })?;

    Ok(())
}

#[test]
fn test_rename_topic_updates_tags_references() -> Result<()> {
    // Arrange - Set up environment variable to avoid touching real config file
    temp_env::with_var("CONFIG_PATH", Some("temp_config.yaml"), || -> Result<()> {
        // Create a config with existing topic and tags
        let mut config = Config::default();

        // Add topic
        let mut topics = HashMap::new();
        topics.insert(
            "old-key".to_string(),
            TopicConfig {
                name: "Old Topic".to_string(),
                description: "An existing topic".to_string(),
                directory: "old-topic".to_string(),
            },
        );
        config.content.topics = topics;

        // Add tags for the topic
        let mut tags = HashMap::new();
        tags.insert(
            "old-key".to_string(),
            vec!["tag1".to_string(), "tag2".to_string()],
        );
        config.content.tags = Some(tags);

        // Write config to temporary file
        let config_content = serde_yaml::to_string(&config)?;
        std::fs::write("temp_config.yaml", config_content)?;

        // Create options for renaming with reference updates
        let options = RenameOptions {
            old_key: "old-key".to_string(),
            new_key: "new-key".to_string(),
            update_references: true,
        };

        // Act
        let result = rename_topic(&(&options).into());

        // Assert
        assert!(result.is_ok());

        // Verify the tags references were updated
        let updated_config = common_config::load_config()?;

        // Tags should be moved to new key
        let tags_map = updated_config.content.tags.unwrap();
        assert!(!tags_map.contains_key("old-key"));
        assert!(tags_map.contains_key("new-key"));

        // Tags content should be preserved
        let tags = &tags_map["new-key"];
        assert_eq!(tags.len(), 2);
        assert!(tags.contains(&"tag1".to_string()));
        assert!(tags.contains(&"tag2".to_string()));

        // Clean up
        std::fs::remove_file("temp_config.yaml")?;

        Ok(())
    })?;

    Ok(())
}

#[test]
fn test_rename_topic_with_content_references() -> Result<()> {
    // Arrange
    let fixture = TestFixture::new()?;
    let base_path = fixture.path();
    let content_dir = base_path.join("content");
    std::fs::create_dir_all(&content_dir)?;

    // Create a test content file with topic reference
    let test_content = r#"---
title: Test Article
topic: old-key
date: 2023-01-01
---
This is test content referencing the old-key topic.
"#;

    let content_file = content_dir.join("test-article.md");
    std::fs::write(&content_file, test_content)?;

    // Set up config with the topic
    temp_env::with_var("CONFIG_PATH", Some(base_path.join("config.yaml").to_string_lossy().to_string()), || -> Result<()> {
        // Create a config with existing topic
        let mut config = Config::default();
        config.content.base_dir = content_dir.to_string_lossy().to_string();

        // Add topic
        let mut topics = HashMap::new();
        topics.insert(
            "old-key".to_string(),
            TopicConfig {
                name: "Old Topic".to_string(),
                description: "An existing topic".to_string(),
                directory: "old-key".to_string(),
            },
        );
        config.content.topics = topics;

        // Write config
        let config_content = serde_yaml::to_string(&config)?;
        std::fs::write(base_path.join("config.yaml"), config_content)?;

        // Create options for renaming with reference updates
        let options = RenameOptions {
            old_key: "old-key".to_string(),
            new_key: "new-key".to_string(),
            update_references: true,
        };

        // Act
        let result = rename_topic(&(&options).into());

        // Assert
        assert!(result.is_ok());

        // Verify content file was updated with new topic reference
        let updated_content = std::fs::read_to_string(&content_file)?;
        assert!(updated_content.contains("topic: new-key"));
        assert!(!updated_content.contains("topic: old-key"));

        Ok(())
    })?;

    Ok(())
}

#[test]
fn test_rename_topic_preserves_directory_path() -> Result<()> {
    // Arrange - Set up environment variable to avoid touching real config file
    temp_env::with_var("CONFIG_PATH", Some("temp_config.yaml"), || -> Result<()> {
        // Create a config with existing topic
        let mut config = Config::default();
        let mut topics = HashMap::new();
        topics.insert(
            "old-key".to_string(),
            TopicConfig {
                name: "Old Topic".to_string(),
                description: "An existing topic".to_string(),
                directory: "custom/path/to/topic".to_string(),
            },
        );
        config.content.topics = topics;

        // Write config to temporary file
        let config_content = serde_yaml::to_string(&config)?;
        std::fs::write("temp_config.yaml", config_content)?;

        // Create options for renaming
        let options = RenameOptions {
            old_key: "old-key".to_string(),
            new_key: "new-key".to_string(),
            update_references: false,
        };

        // Act
        let result = rename_topic(&(&options).into());

        // Assert
        assert!(result.is_ok());

        // Verify directory path was preserved
        let updated_config = common_config::load_config()?;
        let topic = &updated_config.content.topics["new-key"];
        assert_eq!(topic.directory, "custom/path/to/topic");

        // Clean up
        std::fs::remove_file("temp_config.yaml")?;

        Ok(())
    })?;

    Ok(())
}