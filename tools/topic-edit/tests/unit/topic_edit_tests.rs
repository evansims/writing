use anyhow::Result;
use common_test_utils::TestFixture;
use common_models::{Config, TopicConfig};
use std::collections::HashMap;
use topic_edit::{edit_topic, topic_exists, get_topic_keys, TopicEditOptions};
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
fn test_get_topic_keys_returns_all_topic_keys() {
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
    config.content.topics = topics;

    // Act
    let keys = get_topic_keys(&config);

    // Assert
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&"topic1".to_string()));
    assert!(keys.contains(&"topic2".to_string()));
}