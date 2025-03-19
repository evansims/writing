//! Unit tests for the ContentEditorImpl

use common_test_utils::fixtures::TestFixture;
use common_test_utils::mocks::{MockFileSystem, MockConfigLoader};
use common_traits::tools::{ContentEditor, EditOptions};
use content_edit::ContentEditorImpl;
use std::path::{Path, PathBuf};
use mockall::predicate;
use std::collections::HashMap;

/// Test the edit_content method
#[test]
fn test_edit_content_with_valid_options() {
    // Arrange: Set up test fixture
    let test_fixture = TestFixture::builder()
        .with_content_directory()
        .build()
        .unwrap();

    // Create mock FileSystem
    let mut mock_fs = MockFileSystem::new();

    // Set expectations for finding content path
    let content_path = PathBuf::from("content/blog/test-post/index.mdx");
    let content_dir = content_path.parent().unwrap().to_path_buf();

    mock_fs.expect_dir_exists()
        .with(predicate::eq(content_path.clone()))
        .returning(|_| Ok(true));

    // Set expectations for reading content
    let content = r#"---
title: "Test Post"
date: "2023-01-01"
---

# Test Post Content
"#;

    mock_fs.expect_read_file()
        .with(predicate::eq(content_path.clone()))
        .returning(move |_| Ok(content.to_string()));

    // Create mock ConfigLoader
    let mut mock_config = MockConfigLoader::new();

    // Set expectations for loading config
    let config = common_models::Config {
        content: common_models::ContentConfig {
            base_dir: "content".to_string(),
            topics: {
                let mut topics = HashMap::new();
                topics.insert("blog".to_string(), common_models::TopicConfig {
                    name: "Blog".to_string(),
                    directory: "blog".to_string(),
                    template: "blog".to_string(),
                });
                topics
            },
            templates: HashMap::new(),
        },
        ..Default::default()
    };

    mock_config.expect_load_config()
        .returning(move || Ok(config.clone()));

    // Create test dependencies (dependency injection)
    common_fs::set_impl(Box::new(mock_fs));
    common_config::set_impl(Box::new(mock_config));

    // Create SUT (system under test)
    let editor = ContentEditorImpl::new();

    // Act: Call the method being tested
    let edit_options = EditOptions {
        slug: Some("test-post".to_string()),
        topic: Some("blog".to_string()),
        field: None,
        value: None,
        editor: false,
    };
    let result = editor.edit_content(&edit_options);

    // Assert: Verify the result
    assert!(result.is_ok());
    if let Ok(path) = result {
        assert_eq!(path, content_path);
    }
}

/// Test the update_frontmatter_field method
#[test]
fn test_update_frontmatter_field() {
    // Arrange: Set up test fixture
    let test_fixture = TestFixture::builder()
        .with_content_directory()
        .build()
        .unwrap();

    // Create mock FileSystem
    let mut mock_fs = MockFileSystem::new();

    // Set expectations for finding content path
    let content_path = PathBuf::from("content/blog/test-post/index.mdx");
    let content_dir = content_path.parent().unwrap().to_path_buf();

    mock_fs.expect_dir_exists()
        .with(predicate::eq(content_path.clone()))
        .returning(|_| Ok(true));

    // Set expectations for reading content
    let content = r#"---
title: "Test Post"
date: "2023-01-01"
---

# Test Post Content
"#;

    mock_fs.expect_read_file()
        .with(predicate::eq(content_path.clone()))
        .returning(move |_| Ok(content.to_string()));

    // Set expectations for writing updated content
    mock_fs.expect_write_file()
        .with(
            predicate::eq(content_path.clone()),
            predicate::function(|content: &String| content.contains("status: published"))
        )
        .returning(|_, _| Ok(()));

    // Create mock ConfigLoader
    let mut mock_config = MockConfigLoader::new();

    // Set expectations for loading config
    let config = common_models::Config {
        content: common_models::ContentConfig {
            base_dir: "content".to_string(),
            topics: {
                let mut topics = HashMap::new();
                topics.insert("blog".to_string(), common_models::TopicConfig {
                    name: "Blog".to_string(),
                    directory: "blog".to_string(),
                    template: "blog".to_string(),
                });
                topics
            },
            templates: HashMap::new(),
        },
        ..Default::default()
    };

    mock_config.expect_load_config()
        .returning(move || Ok(config.clone()));

    // Create test dependencies (dependency injection)
    common_fs::set_impl(Box::new(mock_fs));
    common_config::set_impl(Box::new(mock_config));

    // Create SUT (system under test)
    let editor = ContentEditorImpl::new();

    // Act: Call the method being tested
    let result = editor.update_frontmatter_field("test-post", Some("blog"), "status", "published");

    // Assert: Verify the result
    assert!(result.is_ok());
}

/// Test the get_frontmatter_fields method
#[test]
fn test_get_frontmatter_fields() {
    // Arrange: Set up test fixture
    let test_fixture = TestFixture::builder()
        .with_content_directory()
        .build()
        .unwrap();

    // Create mock FileSystem
    let mut mock_fs = MockFileSystem::new();

    // Set expectations for finding content path
    let content_path = PathBuf::from("content/blog/test-post/index.mdx");
    let content_dir = content_path.parent().unwrap().to_path_buf();

    mock_fs.expect_dir_exists()
        .with(predicate::eq(content_path.clone()))
        .returning(|_| Ok(true));

    // Set expectations for reading content
    let content = r#"---
title: "Test Post"
date: "2023-01-01"
description: "A test post"
draft: false
tags:
  - test
  - example
---

# Test Post Content
"#;

    mock_fs.expect_read_file()
        .with(predicate::eq(content_path.clone()))
        .returning(move |_| Ok(content.to_string()));

    // Create mock ConfigLoader
    let mut mock_config = MockConfigLoader::new();

    // Set expectations for loading config
    let config = common_models::Config {
        content: common_models::ContentConfig {
            base_dir: "content".to_string(),
            topics: {
                let mut topics = HashMap::new();
                topics.insert("blog".to_string(), common_models::TopicConfig {
                    name: "Blog".to_string(),
                    directory: "blog".to_string(),
                    template: "blog".to_string(),
                });
                topics
            },
            templates: HashMap::new(),
        },
        ..Default::default()
    };

    mock_config.expect_load_config()
        .returning(move || Ok(config.clone()));

    // Create test dependencies (dependency injection)
    common_fs::set_impl(Box::new(mock_fs));
    common_config::set_impl(Box::new(mock_config));

    // Create SUT (system under test)
    let editor = ContentEditorImpl::new();

    // Act: Call the method being tested
    let result = editor.get_frontmatter_fields("test-post", Some("blog"));

    // Assert: Verify the result
    assert!(result.is_ok());
    if let Ok(fields) = result {
        assert_eq!(fields.get("title").unwrap(), "Test Post");
        assert_eq!(fields.get("date").unwrap(), "2023-01-01");
        assert_eq!(fields.get("description").unwrap(), "A test post");
        assert_eq!(fields.get("draft").unwrap(), "false");
        assert!(fields.get("tags").is_some()); // Tags are converted to JSON
    }
}

/// Test error handling for non-existent content
#[test]
fn test_edit_content_with_nonexistent_content() {
    // Arrange: Set up test fixture
    let test_fixture = TestFixture::builder()
        .with_content_directory()
        .build()
        .unwrap();

    // Create mock FileSystem
    let mut mock_fs = MockFileSystem::new();

    // Set expectations for finding content path - this will fail
    mock_fs.expect_dir_exists()
        .returning(|_| Ok(false));

    // Create mock ConfigLoader
    let mut mock_config = MockConfigLoader::new();

    // Set expectations for loading config
    let config = common_models::Config {
        content: common_models::ContentConfig {
            base_dir: "content".to_string(),
            topics: {
                let mut topics = HashMap::new();
                topics.insert("blog".to_string(), common_models::TopicConfig {
                    name: "Blog".to_string(),
                    directory: "blog".to_string(),
                    template: "blog".to_string(),
                });
                topics
            },
            templates: HashMap::new(),
        },
        ..Default::default()
    };

    mock_config.expect_load_config()
        .returning(move || Ok(config.clone()));

    // Create test dependencies (dependency injection)
    common_fs::set_impl(Box::new(mock_fs));
    common_config::set_impl(Box::new(mock_config));

    // Create SUT (system under test)
    let editor = ContentEditorImpl::new();

    // Act: Call the method being tested
    let edit_options = EditOptions {
        slug: Some("nonexistent-post".to_string()),
        topic: Some("blog".to_string()),
        field: None,
        value: None,
        editor: false,
    };
    let result = editor.edit_content(&edit_options);

    // Assert: Verify the result
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.to_string().contains("not found"));
}