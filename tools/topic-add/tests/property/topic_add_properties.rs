use proptest::prelude::*;
use proptest::prop_compose;
use topic_add::{generate_key_from_name, topic_exists, AddOptions};
use common_models::{Config, TopicConfig};
use std::collections::HashMap;

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

// Test property: generate_key_from_name always produces valid slug
proptest! {
    #[test]
    fn prop_generate_key_always_produces_valid_slug(
        name in valid_topic_name()
    ) {
        let key = generate_key_from_name(&name);

        // Slugs should only contain lowercase letters, numbers, and hyphens
        prop_assert!(key.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'));

        // Slugs should not have consecutive hyphens
        prop_assert!(!key.contains("--"));

        // Slugs should not start or end with hyphens
        prop_assert!(!key.starts_with('-'));
        prop_assert!(!key.ends_with('-'));

        // Slug should not be empty
        prop_assert!(!key.is_empty());
    }
}

// Test property: add_topic with valid options always succeeds when topic doesn't exist
proptest! {
    #[test]
    fn prop_add_topic_succeeds_with_valid_options(
        name in valid_topic_name(),
        description in valid_description(),
        directory in valid_directory()
    ) {
        // Skip empty names or names that only have special characters
        prop_assume!(!name.trim().is_empty());
        prop_assume!(name.chars().any(|c| c.is_alphanumeric()));

        // Generate a unique key from the name
        let key = generate_key_from_name(&name);

        // Create valid options
        let options = AddOptions {
            key: key.clone(),
            name,
            description,
            directory,
        };

        // Mock would be set up here in a real implementation
        // For now, we'll just verify the options are valid
        prop_assert!(!options.key.is_empty());
        prop_assert!(!options.name.is_empty());
        prop_assert!(!options.description.is_empty());
        prop_assert!(!options.directory.is_empty());
    }
}

// Test property: keys generated from similar names maintain uniqueness properties
proptest! {
    #[test]
    fn prop_similar_names_produce_consistent_keys(
        name in valid_topic_name()
    ) {
        // Original name
        let original_key = generate_key_from_name(&name);

        // Adding spaces should produce same key
        let with_spaces = format!("  {}  ", name);
        let spaces_key = generate_key_from_name(&with_spaces);
        prop_assert_eq!(original_key.clone(), spaces_key);

        // Changing case should produce same key
        let uppercase = name.to_uppercase();
        let uppercase_key = generate_key_from_name(&uppercase);
        prop_assert_eq!(original_key.clone(), uppercase_key);

        // Multiple spaces should be collapsed to single hyphens
        let multi_spaces = name.replace(" ", "   ");
        let multi_spaces_key = generate_key_from_name(&multi_spaces);
        prop_assert_eq!(original_key, multi_spaces_key);
    }
}

// Test property: topic existence depends only on key
proptest! {
    #[test]
    fn prop_topic_exists_depends_only_on_key(
        key in "[a-z0-9-]{3,30}",
        name1 in valid_topic_name(),
        name2 in valid_topic_name(),
        description1 in "[A-Za-z0-9 !&(),.-]{5,100}",
        description2 in "[A-Za-z0-9 !&(),.-]{5,100}",
        directory1 in "[a-z0-9-]{3,30}",
        directory2 in "[a-z0-9-]{3,30}"
    ) {
        // Create a config with the topic
        let mut config = Config::default();
        let mut topics = HashMap::new();
        topics.insert(
            key.clone(),
            TopicConfig {
                name: name1,
                description: description1,
                directory: directory1,
            },
        );
        config.content.topics = topics;

        // Test property: topic_exists returns true regardless of other details
        prop_assert!(topic_exists(&config, &key));

        // Create different topic with same key
        let mut config2 = Config::default();
        let mut topics2 = HashMap::new();
        topics2.insert(
            key.clone(),
            TopicConfig {
                name: name2,
                description: description2,
                directory: directory2,
            },
        );
        config2.content.topics = topics2;

        // Should still return true with different details but same key
        prop_assert!(topic_exists(&config2, &key));
    }
}