use anyhow::Result;
use common_test_utils::TestFixture;
use topic_add::{generate_key_from_name, topic_exists, create_topic_directory};
use common_models::{Config, TopicConfig};
use std::collections::HashMap;
use std::path::Path;

#[test]
fn test_generate_key_from_name_with_valid_input() {
    // Simple test for key generation
    assert_eq!(generate_key_from_name("Test Topic"), "test-topic");
    assert_eq!(generate_key_from_name("Another Example!"), "another-example");
    assert_eq!(generate_key_from_name("Multi  Space"), "multi-space");
}

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
fn test_create_topic_directory_creates_directory_if_not_exists() -> Result<()> {
    // Arrange
    let fixture = TestFixture::new()?;
    let base_dir = fixture.path().to_string_lossy().to_string();
    let path = "test-topic";
    let dir_path = format!("{}/{}", base_dir, path);

    // Act
    let result = create_topic_directory(&base_dir, path);

    // Assert
    assert!(result.is_ok());
    assert!(Path::new(&dir_path).exists());

    Ok(())
}

#[test]
fn test_create_topic_directory_succeeds_if_directory_already_exists() -> Result<()> {
    // Arrange
    let fixture = TestFixture::new()?;
    let base_dir = fixture.path().to_string_lossy().to_string();
    let path = "existing-topic";
    let dir_path = format!("{}/{}", base_dir, path);

    // Create the directory first
    std::fs::create_dir_all(&dir_path)?;
    assert!(Path::new(&dir_path).exists());

    // Act
    let result = create_topic_directory(&base_dir, path);

    // Assert
    assert!(result.is_ok());

    Ok(())
}