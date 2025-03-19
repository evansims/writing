//! Property-based tests for content-edit
//!
//! These tests verify properties of the content-edit functionality using
//! randomly generated inputs.

use proptest::prelude::*;
use content_edit::{ContentEditorImpl, EditOptions as InternalEditOptions};
use common_test_utils::fixtures::TestFixture;
use common_test_utils::mocks::{MockFileSystem, MockConfigLoader};
use common_traits::tools::{ContentEditor, EditOptions};
use std::path::{Path, PathBuf};
use mockall::predicate;
use std::collections::HashMap;

/// Strategies for generating test inputs
mod strategies {
    use super::*;

    /// Strategy for generating valid slugs
    pub fn valid_slug() -> impl Strategy<Value = String> {
        r"[a-z0-9][a-z0-9\-]{0,50}".prop_map(|s| s.to_string())
    }

    /// Strategy for generating valid topics
    pub fn valid_topic() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("blog".to_string()),
            Just("articles".to_string()),
            Just("notes".to_string()),
            Just("pages".to_string()),
            Just("docs".to_string()),
        ]
    }

    /// Strategy for generating valid frontmatter fields
    pub fn valid_field() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("title".to_string()),
            Just("date".to_string()),
            Just("description".to_string()),
            Just("status".to_string()),
            Just("draft".to_string()),
            Just("tags".to_string()),
            Just("author".to_string()),
        ]
    }

    /// Strategy for generating valid field values
    pub fn valid_value() -> impl Strategy<Value = String> {
        prop_oneof![
            // String values
            "[a-zA-Z0-9 ]{1,20}".prop_map(|s| s.to_string()),
            // Boolean values
            prop_oneof![Just("true".to_string()), Just("false".to_string())],
            // Date values
            Just("2023-01-01".to_string()),
            // Numeric values
            (1i32..1000).prop_map(|n| n.to_string()),
        ]
    }

    /// Strategy for generating valid edit options
    pub fn valid_edit_options() -> impl Strategy<Value = EditOptions> {
        (valid_slug(), option::of(valid_topic()), option::of(valid_field()), option::of(valid_value()))
            .prop_map(|(slug, topic, field, value)| {
                EditOptions {
                    slug: Some(slug),
                    topic,
                    field,
                    value,
                    editor: false,
                }
            })
    }

    /// Strategy for generating valid content
    pub fn valid_content() -> impl Strategy<Value = String> {
        r#"---
title: "Test Post"
date: "2023-01-01"
description: "A test post"
draft: false
---

# Test Post Content

This is a test post.
"#.prop_map(|s| s.to_string())
    }
}

/// Test property: update_frontmatter_field preserves other fields
proptest! {
    #[test]
    fn prop_update_frontmatter_preserves_other_fields(
        slug in strategies::valid_slug(),
        topic in strategies::valid_topic(),
        field in strategies::valid_field(),
        value in strategies::valid_value(),
        content in strategies::valid_content(),
    ) {
        // Arrange: Set up the environment
        let test_fixture = TestFixture::builder()
            .with_content_directory()
            .build()
            .unwrap();

        // Create mock dependencies
        let mut mock_fs = MockFileSystem::new();
        let mut mock_config = MockConfigLoader::new();

        let content_path = PathBuf::from(format!("content/{}/{}/index.mdx", topic, slug));

        // Set up mock expectations
        mock_fs.expect_dir_exists()
            .returning(|_| Ok(true));

        mock_fs.expect_read_file()
            .returning(move |_| Ok(content.clone()));

        mock_fs.expect_write_file()
            .returning(|_, c| {
                // The property we're testing: all existing fields should still be present
                // except the one we're updating
                let content = c.to_string();

                // Original fields that should be preserved
                prop_assert!(content.contains("title:"));
                prop_assert!(content.contains("date:"));
                prop_assert!(content.contains("description:"));

                // The updated field should be present with the new value
                // (though we can't easily check the value itself here)
                prop_assert!(content.contains(&format!("{}: ", field)) ||
                           content.contains(&format!("{}: ", field.to_lowercase())));

                Ok(())
            });

        let config = common_models::Config {
            content: common_models::ContentConfig {
                base_dir: "content".to_string(),
                topics: {
                    let mut topics = HashMap::new();
                    topics.insert(topic.clone(), common_models::TopicConfig {
                        name: topic.clone(),
                        directory: topic.clone(),
                        template: "default".to_string(),
                    });
                    topics
                },
                templates: HashMap::new(),
            },
            ..Default::default()
        };

        mock_config.expect_load_config()
            .returning(move || Ok(config.clone()));

        // Set up dependencies
        common_fs::set_impl(Box::new(mock_fs));
        common_config::set_impl(Box::new(mock_config));

        // Create SUT
        let editor = ContentEditorImpl::new();

        // Act: Call the method being tested
        let result = editor.update_frontmatter_field(&slug, Some(&topic), &field, &value);

        // Assert: The operation should succeed
        prop_assert!(result.is_ok());
    }
}

/// Test property: edit_content with invalid slug should return error
proptest! {
    #[test]
    fn prop_edit_content_with_invalid_slug_errors(
        topic in strategies::valid_topic(),
    ) {
        // Arrange: Set up the environment
        let test_fixture = TestFixture::builder()
            .with_content_directory()
            .build()
            .unwrap();

        // Create mock dependencies
        let mut mock_fs = MockFileSystem::new();
        let mut mock_config = MockConfigLoader::new();

        // Set up mock expectations - file system returns false for exists
        mock_fs.expect_dir_exists()
            .returning(|_| Ok(false));

        let config = common_models::Config {
            content: common_models::ContentConfig {
                base_dir: "content".to_string(),
                topics: {
                    let mut topics = HashMap::new();
                    topics.insert(topic.clone(), common_models::TopicConfig {
                        name: topic.clone(),
                        directory: topic.clone(),
                        template: "default".to_string(),
                    });
                    topics
                },
                templates: HashMap::new(),
            },
            ..Default::default()
        };

        mock_config.expect_load_config()
            .returning(move || Ok(config.clone()));

        // Set up dependencies
        common_fs::set_impl(Box::new(mock_fs));
        common_config::set_impl(Box::new(mock_config));

        // Create SUT
        let editor = ContentEditorImpl::new();

        // Act: Call the method with deliberately invalid slug
        let options = EditOptions {
            slug: Some("this-slug-does-not-exist".to_string()),
            topic: Some(topic.clone()),
            field: None,
            value: None,
            editor: false,
        };

        let result = editor.edit_content(&options);

        // Assert: The operation should fail
        prop_assert!(result.is_err());
    }
}

/// Test property: get_frontmatter_fields returns correct field types
proptest! {
    #[test]
    fn prop_get_frontmatter_fields_correct_types(
        slug in strategies::valid_slug(),
        topic in strategies::valid_topic(),
        title in "[a-zA-Z0-9 ]{1,20}".prop_map(|s| s.to_string()),
        description in "[a-zA-Z0-9 ]{1,20}".prop_map(|s| s.to_string()),
        is_draft in proptest::bool::ANY,
    ) {
        // Arrange: Set up the environment
        let test_fixture = TestFixture::builder()
            .with_content_directory()
            .build()
            .unwrap();

        // Create mock dependencies
        let mut mock_fs = MockFileSystem::new();
        let mut mock_config = MockConfigLoader::new();

        let content_path = PathBuf::from(format!("content/{}/{}/index.mdx", topic, slug));

        // Create test content with the generated values
        let content = format!(r#"---
title: "{}"
date: "2023-01-01"
description: "{}"
draft: {}
---

# Test Content
"#, title, description, is_draft);

        // Set up mock expectations
        mock_fs.expect_dir_exists()
            .returning(|_| Ok(true));

        mock_fs.expect_read_file()
            .returning(move |_| Ok(content.clone()));

        let config = common_models::Config {
            content: common_models::ContentConfig {
                base_dir: "content".to_string(),
                topics: {
                    let mut topics = HashMap::new();
                    topics.insert(topic.clone(), common_models::TopicConfig {
                        name: topic.clone(),
                        directory: topic.clone(),
                        template: "default".to_string(),
                    });
                    topics
                },
                templates: HashMap::new(),
            },
            ..Default::default()
        };

        mock_config.expect_load_config()
            .returning(move || Ok(config.clone()));

        // Set up dependencies
        common_fs::set_impl(Box::new(mock_fs));
        common_config::set_impl(Box::new(mock_config));

        // Create SUT
        let editor = ContentEditorImpl::new();

        // Act: Call the method being tested
        let result = editor.get_frontmatter_fields(&slug, Some(&topic));

        // Assert: Check the field types
        prop_assert!(result.is_ok());
        if let Ok(fields) = result {
            // String fields should match the input
            prop_assert_eq!(fields.get("title").unwrap(), &title);
            prop_assert_eq!(fields.get("description").unwrap(), &description);

            // Date field should be in the correct format
            prop_assert_eq!(fields.get("date").unwrap(), "2023-01-01");

            // Boolean field should be "true" or "false" string
            prop_assert_eq!(fields.get("draft").unwrap(), &is_draft.to_string());
        }
    }
}