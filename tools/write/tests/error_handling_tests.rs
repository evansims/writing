//! Tests for error handling in the write tool
//!
//! This file contains integration tests focusing on error cases for different modules.

use anyhow::Result;
use std::fs;
use tempfile::tempdir;

// Import the modules under test
use write::tools::*;
use write::cli;

/// Set up a test environment with a temporary directory
fn setup() -> Result<tempfile::TempDir> {
    let temp_dir = tempdir()?;
    
    // Create basic config directory structure
    let config_dir = temp_dir.path().join(".config/write");
    fs::create_dir_all(&config_dir)?;
    
    // Create a minimal configuration file
    let config_content = r#"{
        "topics": {
            "blog": {
                "name": "Blog",
                "description": "Blog posts",
                "directory": "content/blog"
            }
        },
        "content": {
            "base_dir": "content"
        }
    }"#;
    
    fs::write(config_dir.join("config.json"), config_content)?;
    
    // Create content directory structure
    let content_dir = temp_dir.path().join("content");
    let blog_dir = content_dir.join("blog");
    fs::create_dir_all(&blog_dir)?;
    
    Ok(temp_dir)
}

#[test]
fn test_content_not_found_error() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Attempt to access non-existent content
    let cmd = cli::ContentCommands::Edit {
        slug: Some("non-existent-post".to_string()),
        topic: Some("blog".to_string()),
        field: Some("title".to_string()),
        value: Some("Updated Title".to_string()),
        editor: false,
    };
    
    let result = execute_content_command(cmd);
    
    // Check that the command failed with the expected error
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("not found") || err.contains("does not exist"));
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_invalid_topic_error() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Attempt to use a non-existent topic
    let cmd = cli::ContentCommands::New {
        title: Some("Test Post".to_string()),
        topic: Some("non-existent-topic".to_string()),
        tagline: None,
        tags: None,
        draft: false,
        template: None,
        edit: false,
    };
    
    let result = execute_content_command(cmd);
    
    // Check that the command failed with the expected error
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("topic") && (err.contains("not found") || err.contains("does not exist")));
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_invalid_build_target_error() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Attempt to build a non-existent topic
    let cmd = cli::BuildCommands::Content {
        topic: Some("non-existent-topic".to_string()),
        rebuild: false,
    };
    
    let result = execute_build_command(cmd);
    
    // Check that the command failed with the expected error
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("topic") && (err.contains("not found") || err.contains("invalid")));
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_invalid_frontmatter_error() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Create test content with invalid frontmatter
    let article_dir = temp_dir.path().join("content/blog/test-article");
    fs::create_dir_all(&article_dir)?;
    
    let article_content = r#"---
title: Test Article
date: invalid-date-format
tags: test,article
---

# Test Article

This is a test article with invalid frontmatter.
"#;
    
    fs::write(article_dir.join("index.mdx"), article_content)?;
    
    // Attempt to build content
    let cmd = cli::BuildCommands::Content {
        topic: Some("blog".to_string()),
        rebuild: false,
    };
    
    let result = execute_build_command(cmd);
    
    // Check that the command failed due to invalid frontmatter
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("frontmatter") || err.contains("metadata") || err.contains("date"));
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_topic_already_exists_error() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Attempt to add a topic that already exists
    let cmd = cli::TopicCommands::Add {
        key: Some("blog".to_string()),
        name: Some("Blog".to_string()),
        description: Some("Blog posts".to_string()),
        directory: None,
    };
    
    let result = execute_topic_command(cmd);
    
    // Check that the command failed with the expected error
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("already exists"));
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_topic_delete_with_content_error() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Create test content
    let article_dir = temp_dir.path().join("content/blog/test-article");
    fs::create_dir_all(&article_dir)?;
    
    let article_content = r#"---
title: Test Article
date: 2023-01-01
tags: test,article
---

# Test Article

This is a test article.
"#;
    
    fs::write(article_dir.join("index.mdx"), article_content)?;
    
    // Attempt to delete a topic that has content without force flag
    let cmd = cli::TopicCommands::Delete {
        key: Some("blog".to_string()),
        force: false,
    };
    
    let result = execute_topic_command(cmd);
    
    // Check that the command failed with the expected error
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("contains content") || err.contains("not empty"));
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_topic_rename_to_existing_error() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Create a second topic in the config
    let config_file = temp_dir.path().join(".config/write/config.json");
    let config_content = r#"{
        "topics": {
            "blog": {
                "name": "Blog",
                "description": "Blog posts",
                "directory": "content/blog"
            },
            "notes": {
                "name": "Notes",
                "description": "Quick notes",
                "directory": "content/notes"
            }
        },
        "content": {
            "base_dir": "content"
        }
    }"#;
    
    fs::write(config_file, config_content)?;
    
    // Create the notes directory
    fs::create_dir_all(temp_dir.path().join("content/notes"))?;
    
    // Attempt to rename a topic to an existing name
    let cmd = cli::TopicCommands::Rename {
        from: Some("blog".to_string()),
        to: Some("notes".to_string()),
    };
    
    let result = execute_topic_command(cmd);
    
    // Check that the command failed with the expected error
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("already exists") || err.contains("duplicate"));
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_stats_command_with_no_content() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // No content files are created
    
    // Attempt to generate stats directly with the execute_stats_command function
    let result = execute_stats_command(
        None,
        Some("blog".to_string()),
        false,
        "date",
        false
    );
    
    // This might not be an error, but should show warning or empty stats
    // Here we test that it completes without error but logs appropriate output
    assert!(result.is_ok());
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_image_optimization_with_missing_image() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Create a test article with frontmatter that references an image
    let content_dir = temp_dir.path().join("content");
    let blog_dir = content_dir.join("blog");
    let article_dir = blog_dir.join("test-article");
    fs::create_dir_all(&article_dir)?;
    
    let article_content = r#"---
title: Test Article
image: ./assets/test-image.jpg
---

This is a test article with a missing image.
"#;
    
    fs::write(article_dir.join("index.mdx"), article_content)?;
    
    // Create assets directory but don't add the image
    let assets_dir = article_dir.join("assets");
    fs::create_dir_all(&assets_dir)?;
    
    // Attempt to optimize images
    let cmd = cli::ImageCommands::Optimize {
        topic: Some("blog".to_string()),
        reoptimize: false,
    };
    
    let result = execute_image_command(cmd);
    
    // Check that the command reported the missing image
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("missing") || err.contains("not found"));
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_image_optimization_with_invalid_path() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Create a test article with frontmatter that references an image
    let content_dir = temp_dir.path().join("content");
    let blog_dir = content_dir.join("blog");
    let article_dir = blog_dir.join("test-article");
    fs::create_dir_all(&article_dir)?;
    
    let article_content = r#"---
title: Test Article
image: ./assets/test-image.jpg
---

This is a test article with a missing image.
"#;
    
    fs::write(article_dir.join("index.mdx"), article_content)?;
    
    // Create assets directory but don't add the image
    let assets_dir = article_dir.join("assets");
    fs::create_dir_all(&assets_dir)?;
    
    // Attempt to optimize images
    let cmd = cli::ImageCommands::Optimize {
        topic: Some("blog".to_string()),
        reoptimize: false,
    };
    
    let result = execute_image_command(cmd);
    
    // Check that the command failed with the expected error
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("format") || err.contains("image") || err.contains("invalid"));
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
} 