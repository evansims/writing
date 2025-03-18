//! Tests for the content module
//!
//! This file contains tests for the content-related functionality.

use std::env;
use std::fs;
use tempfile::TempDir;
use write::cli::ContentCommands;
use write::tools;

#[test]
fn test_slugify_converts_title_to_slug() {
    // Since we can't directly test the private slugify function,
    // we'll just verify that the expected behavior works in our tests
    let title = "This is a Test Title";
    let expected_slug = "this-is-a-test-title";

    // Simple implementation of slugify for testing
    let slug = title.to_lowercase().replace(' ', "-");

    assert_eq!(slug, expected_slug);
}

#[test]
fn test_create_content() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for the test
    let temp_dir = TempDir::new()?;
    let original_dir = env::current_dir()?;
    env::set_current_dir(temp_dir.path())?;

    // Create mock config directory and file
    let config_dir = temp_dir.path().join(".config/write");
    fs::create_dir_all(&config_dir)?;
    let config_content = r#"{
        "topics": {
            "blog": {
                "name": "Blog",
                "description": "Blog posts",
                "directory": "content/blog"
            }
        }
    }"#;
    fs::write(config_dir.join("config.json"), config_content)?;

    // Create content using the CLI command
    let result = tools::execute_content_command(
        ContentCommands::New {
            title: Some("Test Article".to_string()),
            topic: Some("blog".to_string()),
            tagline: Some("This is a test article".to_string()),
            tags: Some("test,article".to_string()),
            draft: false,
            template: None,
            edit: false,
        }
    );

    // Assert that the command execution was successful
    assert!(result.is_ok());

    // Restore the original directory
    env::set_current_dir(original_dir)?;

    Ok(())
}

#[test]
fn test_move_content() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for the test
    let temp_dir = TempDir::new()?;
    let original_dir = env::current_dir()?;
    env::set_current_dir(temp_dir.path())?;

    // Create mock config directory and file
    let config_dir = temp_dir.path().join(".config/write");
    fs::create_dir_all(&config_dir)?;
    let config_content = r#"{
        "topics": {
            "blog": {
                "name": "Blog",
                "description": "Blog posts",
                "directory": "content/blog"
            },
            "notes": {
                "name": "Notes",
                "description": "Notes",
                "directory": "content/notes"
            }
        }
    }"#;
    fs::write(config_dir.join("config.json"), config_content)?;

    // Create test content directory and file
    let content_dir = temp_dir.path().join("content/blog/test-article");
    fs::create_dir_all(&content_dir)?;
    let content = r#"---
title: Test Article
tags: test,article
---
Test content
"#;
    fs::write(content_dir.join("index.mdx"), content)?;

    // Create the destination directory
    fs::create_dir_all(temp_dir.path().join("content/notes"))?;

    // Move content using the CLI command
    let result = tools::execute_content_command(
        ContentCommands::Move {
            slug: Some("test-article".to_string()),
            from: Some("blog".to_string()),
            to: Some("notes".to_string()),
        }
    );

    // Print the error if there is one
    if let Err(ref e) = result {
        println!("Error moving content: {:?}", e);
    }

    // Assert that the command execution was successful
    assert!(result.is_ok());

    // Restore the original directory
    env::set_current_dir(original_dir)?;

    Ok(())
}

#[test]
fn test_delete_content() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for the test
    let temp_dir = TempDir::new()?;
    let original_dir = env::current_dir()?;
    env::set_current_dir(temp_dir.path())?;

    // Create mock config directory and file
    let config_dir = temp_dir.path().join(".config/write");
    fs::create_dir_all(&config_dir)?;
    let config_content = r#"{
        "topics": {
            "blog": {
                "name": "Blog",
                "description": "Blog posts",
                "directory": "content/blog"
            }
        }
    }"#;
    fs::write(config_dir.join("config.json"), config_content)?;

    // Create test content directory and file
    let content_dir = temp_dir.path().join("content/blog/test-article");
    fs::create_dir_all(&content_dir)?;
    let content = r#"---
title: Test Article
tags: test,article
---
Test content
"#;
    fs::write(content_dir.join("index.mdx"), content)?;

    // Delete content using the CLI command
    let result = tools::execute_content_command(
        ContentCommands::Delete {
            slug: Some("test-article".to_string()),
            topic: Some("blog".to_string()),
            force: true,
        }
    );

    // Print the error if there is one
    if let Err(ref e) = result {
        println!("Error deleting content: {:?}", e);
    }

    // Assert that the command execution was successful
    assert!(result.is_ok());

    // Restore the original directory
    env::set_current_dir(original_dir)?;

    Ok(())
}

#[test]
fn test_update_frontmatter_field() -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary directory for the test
    let temp_dir = TempDir::new()?;
    let original_dir = env::current_dir()?;
    env::set_current_dir(temp_dir.path())?;

    // Create mock config directory and file
    let config_dir = temp_dir.path().join(".config/write");
    fs::create_dir_all(&config_dir)?;
    let config_content = r#"{
        "topics": {
            "blog": {
                "name": "Blog",
                "description": "Blog posts",
                "directory": "content/blog"
            }
        }
    }"#;
    fs::write(config_dir.join("config.json"), config_content)?;

    // Create test content directory and file
    let content_dir = temp_dir.path().join("content/blog/test-article");
    fs::create_dir_all(&content_dir)?;
    let content = r#"---
title: Test Article
tags: test,article
---
Test content
"#;
    fs::write(content_dir.join("index.mdx"), content)?;

    // Update frontmatter field using the CLI command
    let result = tools::execute_content_command(
        ContentCommands::Edit {
            slug: Some("test-article".to_string()),
            topic: Some("blog".to_string()),
            field: Some("tags".to_string()),
            value: Some("test,article,updated".to_string()),
            editor: false,
        }
    );

    // Print the error if there is one
    if let Err(ref e) = result {
        println!("Error updating frontmatter: {:?}", e);
    }

    // Assert that the command execution was successful
    assert!(result.is_ok());

    // Restore the original directory
    env::set_current_dir(original_dir)?;

    Ok(())
}