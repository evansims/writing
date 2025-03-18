//! Tests for configuration module using mocks
//!
//! This module contains tests that use mocks to verify the behavior of the configuration
//! module in various scenarios, especially edge cases and error conditions.

use std::collections::HashMap;
use common_errors::{Result, WritingError};
use common_models::{Config, ContentConfig, ImageConfig, PublicationConfig, TopicConfig, ImageSize};
use common_test_utils::{MockConfigLoader, with_mock};
use crate::{validate_topic, get_topic_by_key};
use mockall::predicate;

/// Helper function to create a test config with customizable topics
fn create_test_config(topics: Vec<(&str, &str, &str)>) -> Config {
    let mut topic_map = HashMap::new();

    for (key, name, description) in topics {
        topic_map.insert(
            key.to_string(),
            TopicConfig {
                name: name.to_string(),
                description: description.to_string(),
                directory: key.to_string(),
            }
        );
    }

    Config {
        content: ContentConfig {
            base_dir: "/content".to_string(),
            topics: topic_map,
            tags: None,
        },
        images: ImageConfig {
            formats: vec!["jpg".to_string()],
            format_descriptions: None,
            sizes: HashMap::new(),
            naming: None,
            quality: None,
        },
        publication: PublicationConfig {
            author: "Test Author".to_string(),
            copyright: "Test Copyright".to_string(),
            site_url: None,
        },
    }
}

// Tests for validate_topic function with various scenarios
#[test]
fn test_validate_topic_with_valid_topic() {
    with_mock!(MockConfigLoader, mock_config => {
        // Setup expected config
        let config = create_test_config(vec![
            ("blog", "Blog", "Blog posts"),
            ("tutorials", "Tutorials", "Tutorial content"),
        ]);

        // Configure mock to return our test config
        mock_config.expect_load_config()
            .returning(move || Ok(config.clone()));

        // Apply mock by setting it as the global config loader
        // This is a simplified approach - in a real implementation,
        // you would inject the mock through a trait interface

        // Test valid topic
        let result = validate_topic("blog");
        assert!(result.is_ok());
        let topic = result.unwrap();
        assert_eq!(topic.name, "Blog");
        assert_eq!(topic.description, "Blog posts");
    });
}

#[test]
fn test_validate_topic_with_invalid_topic() {
    with_mock!(MockConfigLoader, mock_config => {
        // Setup expected config
        let config = create_test_config(vec![
            ("blog", "Blog", "Blog posts"),
        ]);

        // Configure mock to return our test config
        mock_config.expect_load_config()
            .returning(move || Ok(config.clone()));

        // Test invalid topic
        let result = validate_topic("nonexistent");
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            WritingError::TopicError(_) => {
                // Expected error type
                assert!(err.to_string().contains("nonexistent"));
            },
            _ => panic!("Expected TopicError, got: {:?}", err),
        }
    });
}

#[test]
fn test_validate_topic_with_empty_topics() {
    with_mock!(MockConfigLoader, mock_config => {
        // Setup config with no topics
        let config = create_test_config(vec![]);

        // Configure mock to return our test config
        mock_config.expect_load_config()
            .returning(move || Ok(config.clone()));

        // Test with any topic
        let result = validate_topic("blog");
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            WritingError::TopicError(_) => {
                // Expected error type
                assert!(err.to_string().contains("blog"));
            },
            _ => panic!("Expected TopicError, got: {:?}", err),
        }
    });
}

#[test]
fn test_validate_topic_with_config_load_error() {
    with_mock!(MockConfigLoader, mock_config => {
        // Configure mock to return an error
        mock_config.expect_load_config()
            .returning(|| Err(WritingError::config_error("Failed to load config")));

        // Test with any topic
        let result = validate_topic("blog");
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            WritingError::ConfigError(_) => {
                // Expected error type
                assert!(err.to_string().contains("Failed to load config"));
            },
            _ => panic!("Expected ConfigError, got: {:?}", err),
        }
    });
}

// Tests for get_topic_by_key function
#[test]
fn test_get_topic_by_key_with_existing_topic() {
    with_mock!(MockConfigLoader, mock_config => {
        // Setup expected config
        let config = create_test_config(vec![
            ("blog", "Blog", "Blog posts"),
            ("tutorials", "Tutorials", "Tutorial content"),
        ]);

        // Configure mock to return our test config
        mock_config.expect_load_config()
            .returning(move || Ok(config.clone()));

        // Test getting an existing topic
        let result = get_topic_by_key("blog");
        assert!(result.is_ok());
        let topic_option = result.unwrap();
        assert!(topic_option.is_some());
        let topic = topic_option.unwrap();
        assert_eq!(topic.name, "Blog");
        assert_eq!(topic.description, "Blog posts");
    });
}

#[test]
fn test_get_topic_by_key_with_nonexistent_topic() {
    with_mock!(MockConfigLoader, mock_config => {
        // Setup expected config
        let config = create_test_config(vec![
            ("blog", "Blog", "Blog posts"),
        ]);

        // Configure mock to return our test config
        mock_config.expect_load_config()
            .returning(move || Ok(config.clone()));

        // Test getting a nonexistent topic
        let result = get_topic_by_key("nonexistent");
        assert!(result.is_ok());
        let topic_option = result.unwrap();
        assert!(topic_option.is_none());
    });
}

#[test]
fn test_get_topic_by_key_with_config_load_error() {
    with_mock!(MockConfigLoader, mock_config => {
        // Configure mock to return an error
        mock_config.expect_load_config()
            .returning(|| Err(WritingError::config_error("Failed to load config")));

        // Test with any topic
        let result = get_topic_by_key("blog");
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            WritingError::ConfigError(_) => {
                // Expected error type
                assert!(err.to_string().contains("Failed to load config"));
            },
            _ => panic!("Expected ConfigError, got: {:?}", err),
        }
    });
}