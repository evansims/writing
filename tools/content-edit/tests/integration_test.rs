//! Integration tests for content-edit
//!
//! These tests verify that the content-edit functionality works correctly
//! when used as a library by external code.

use content_edit::{
    EditableContent,
    save_edited_content,
    extract_frontmatter_from_string,
    split_frontmatter_and_body,
};
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

/// Test the full content editing workflow
#[test]
fn test_content_editing_workflow() {
    // Create a temporary file for testing
    let mut temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();

    // Write test content to the file
    let original_content = r#"---
title: "Integration Test Post"
date: "2023-01-01"
---

# Integration Test Post

This is a test for integration testing."#;

    temp_file.write_all(original_content.as_bytes()).unwrap();

    // Create an EditableContent instance manually
    let content = EditableContent {
        path: file_path.clone(),
        title: "Integration Test Post".to_string(),
        slug: "integration-test".to_string(),
        topic: "test".to_string(),
    };

    // Edit the content
    let edited_content = r#"---
title: "Updated Integration Test Post"
date: "2023-01-01"
---

# Updated Integration Test Post

This post has been edited during integration testing."#;

    // Save the edited content
    save_edited_content(&content.path, edited_content).unwrap();

    // Read the file to verify the changes
    let saved_content = fs::read_to_string(&content.path).unwrap();

    // Check that the content was properly updated
    assert!(saved_content.contains("title: \"Updated Integration Test Post\""));
    assert!(saved_content.contains("date: \"2023-01-01\""));
    assert!(saved_content.contains("# Updated Integration Test Post"));
    assert!(saved_content.contains("This post has been edited during integration testing."));
}

/// Test that we can split frontmatter and body correctly
#[test]
fn test_splitting_and_extraction() {
    let content = r#"---
title: "Integration Test Post"
date: "2023-01-01"
tags: ["integration", "test"]
---

# Integration Test Post

This is a test for integration testing."#;

    // Test splitting
    let (frontmatter, body) = split_frontmatter_and_body(content).unwrap();

    // Check frontmatter
    assert_eq!(frontmatter.title, "Integration Test Post");

    // Check body
    assert!(body.contains("# Integration Test Post"));
    assert!(body.contains("This is a test for integration testing."));

    // Test extraction
    let yaml = extract_frontmatter_from_string(content).unwrap();

    // Check extracted fields
    assert_eq!(yaml.get("title").unwrap().as_str().unwrap(), "Integration Test Post");
    assert_eq!(yaml.get("date").unwrap().as_str().unwrap(), "2023-01-01");

    // Check tags array
    let tags = yaml.get("tags").unwrap().as_sequence().unwrap();
    assert_eq!(tags[0].as_str().unwrap(), "integration");
    assert_eq!(tags[1].as_str().unwrap(), "test");
}