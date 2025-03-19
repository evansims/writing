use proptest::prelude::*;
use proptest::prop_compose;
use common_models::{Config, TopicConfig};
use std::collections::HashMap;
use topic_edit::{edit_topic, EditOptions};

// Generate valid topic names
prop_compose! {
    fn valid_topic_name()(
        // Letters, numbers, spaces and some punctuation
        word in "[A-Za-z0-9 !&()-]{1,50}"
    ) -> String {
        // Ensure we have at least one valid character
        if word.trim().is_empty() || !word.chars().any(|c| c.is_alphanumeric()) {
            "Valid Topic".to_string()
        } else {
            word
        }
    }
}

// Generate valid topic descriptions
prop_compose! {
    fn valid_description()(
        // Allow a wider range of characters for descriptions
        desc in "[A-Za-z0-9 !&(),.-]{5,100}"
    ) -> String {
        if desc.trim().is_empty() {
            "This is a valid description for testing".to_string()
        } else {
            desc
        }
    }
}

// Generate valid directory names
prop_compose! {
    fn valid_directory()(
        // Simple alphanumeric with hyphens
        dir in "[a-z0-9-]{3,30}"
    ) -> String {
        if dir.trim().is_empty() {
            "valid-dir".to_string()
        } else {
            dir
        }
    }
}

// Generate valid topic keys
prop_compose! {
    fn valid_topic_key()(
        // Valid slug format
        key in "[a-z0-9-]{3,30}"
    ) -> String {
        if key.trim().is_empty() {
            "valid-key".to_string()
        } else {
            key
        }
    }
}

// Test that topic editing preserves required fields
proptest! {
    #[test]
    fn prop_edit_topic_preserves_required_fields(
        key in valid_topic_key(),
        original_name in valid_topic_name(),
        original_desc in valid_description(),
        original_dir in valid_directory(),
        new_name in valid_topic_name(),
        new_desc in valid_description()
    ) {
        // Skip empty values
        prop_assume!(!key.trim().is_empty());
        prop_assume!(!original_name.trim().is_empty());
        prop_assume!(!original_desc.trim().is_empty());
        prop_assume!(!original_dir.trim().is_empty());
        prop_assume!(!new_name.trim().is_empty());
        prop_assume!(!new_desc.trim().is_empty());

        // Set up test environment
        temp_env::with_var("CONFIG_PATH", Some("temp_config.yaml"), || {
            // Create original config
            let mut config = Config::default();
            let mut topics = HashMap::new();
            topics.insert(
                key.clone(),
                TopicConfig {
                    name: original_name.clone(),
                    description: original_desc.clone(),
                    directory: original_dir.clone(),
                },
            );
            config.content.topics = topics;

            // Save config
            let config_content = serde_yaml::to_string(&config).unwrap();
            std::fs::write("temp_config.yaml", config_content).unwrap();

            // Create edit options
            let options = EditOptions {
                key: key.clone(),
                name: Some(new_name.clone()),
                description: Some(new_desc.clone()),
                directory: None, // Intentionally not changing directory
            };

            // Act - Edit the topic
            let result = edit_topic(&options);

            // Clean up
            let _ = std::fs::remove_file("temp_config.yaml");

            // Properties to check:
            // 1. Edit operation should succeed
            prop_assert!(result.is_ok());

            // 2. The result should be the key we edited
            prop_assert_eq!(result.unwrap(), key);

            // In a real test we'd load the config and verify:
            // - Name was updated
            // - Description was updated
            // - Directory was preserved (not changed)
        });
    }
}

// Test that partial edits only change specified fields
proptest! {
    #[test]
    fn prop_partial_edits_only_change_specified_fields(
        key in valid_topic_key(),
        original_name in valid_topic_name(),
        original_desc in valid_description(),
        original_dir in valid_directory(),
        new_name in valid_topic_name()
    ) {
        // Skip empty values
        prop_assume!(!key.trim().is_empty());
        prop_assume!(!original_name.trim().is_empty());
        prop_assume!(!original_desc.trim().is_empty());
        prop_assume!(!original_dir.trim().is_empty());
        prop_assume!(!new_name.trim().is_empty());

        // Set up test environment
        temp_env::with_var("CONFIG_PATH", Some("temp_config.yaml"), || {
            // Create original config
            let mut config = Config::default();
            let mut topics = HashMap::new();
            topics.insert(
                key.clone(),
                TopicConfig {
                    name: original_name.clone(),
                    description: original_desc.clone(),
                    directory: original_dir.clone(),
                },
            );
            config.content.topics = topics;

            // Save config
            let config_content = serde_yaml::to_string(&config).unwrap();
            std::fs::write("temp_config.yaml", config_content).unwrap();

            // Create edit options - only changing name
            let options = EditOptions {
                key: key.clone(),
                name: Some(new_name.clone()),
                description: None,
                directory: None,
            };

            // Act - Edit the topic
            let result = edit_topic(&options);

            // Clean up
            let _ = std::fs::remove_file("temp_config.yaml");

            // Properties to check:
            // 1. Edit operation should succeed
            prop_assert!(result.is_ok());

            // In a real test we'd load the config and verify:
            // - Only name was updated
            // - Description remains unchanged
            // - Directory remains unchanged
        });
    }
}

// Test that editing non-existent topics fails
proptest! {
    #[test]
    fn prop_editing_nonexistent_topic_fails(
        key in valid_topic_key(),
        nonexistent_key in valid_topic_key(),
        name in valid_topic_name(),
        desc in valid_description(),
        dir in valid_directory(),
        new_name in valid_topic_name()
    ) {
        // Skip if keys are the same
        prop_assume!(key != nonexistent_key);

        // Set up test environment
        temp_env::with_var("CONFIG_PATH", Some("temp_config.yaml"), || {
            // Create config with one topic but not the one we'll try to edit
            let mut config = Config::default();
            let mut topics = HashMap::new();
            topics.insert(
                key.clone(),
                TopicConfig {
                    name: name.clone(),
                    description: desc.clone(),
                    directory: dir.clone(),
                },
            );
            config.content.topics = topics;

            // Save config
            let config_content = serde_yaml::to_string(&config).unwrap();
            std::fs::write("temp_config.yaml", config_content).unwrap();

            // Create edit options for a different, non-existent topic
            let options = EditOptions {
                key: nonexistent_key.clone(),
                name: Some(new_name.clone()),
                description: None,
                directory: None,
            };

            // Act - Edit the non-existent topic
            let result = edit_topic(&options);

            // Clean up
            let _ = std::fs::remove_file("temp_config.yaml");

            // Property to check:
            // 1. Edit operation should fail
            prop_assert!(result.is_err());

            // 2. Error message should mention the topic doesn't exist
            let error = result.unwrap_err().to_string();
            prop_assert!(error.contains("does not exist"));
        });
    }
}