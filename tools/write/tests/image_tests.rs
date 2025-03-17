//! Tests for the image module
//!
//! This file contains tests for the image-related functionality.

use anyhow::Result;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

// Import the module under test
use write::tools::*;
use write::cli;

/// Set up a test environment with a temporary directory
fn setup() -> Result<tempfile::TempDir> {
    let temp_dir = tempdir()?;
    
    // Create content directory structure
    let content_dir = temp_dir.path().join("content");
    let blog_dir = content_dir.join("blog");
    fs::create_dir_all(&blog_dir)?;
    
    // Create an article for testing
    let article_dir = blog_dir.join("test-article");
    fs::create_dir_all(&article_dir)?;
    
    // Create a test article file
    let article_content = r#"---
title: Test Article
date: 2023-01-01
tags: test,article
---

# Test Article

This is a test article with an image:

![Test Image](assets/test-image.jpg)
"#;
    
    fs::write(article_dir.join("index.mdx"), article_content)?;
    
    // Create assets directory
    let assets_dir = article_dir.join("assets");
    fs::create_dir_all(&assets_dir)?;
    
    // Create a dummy image file
    let image_data = vec![
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01,
        0x01, 0x01, 0x00, 0x60, 0x00, 0x60, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43,
        0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    ];
    fs::write(assets_dir.join("test-image.jpg"), image_data)?;
    
    Ok(temp_dir)
}

#[test]
fn test_build_images() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Create a mock configuration file to avoid interactive prompts
    let config_dir = temp_dir.path().join(".config/write");
    fs::create_dir_all(&config_dir)?;
    
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
    
    // Create the public directory
    fs::create_dir_all(temp_dir.path().join("public"))?;
    
    // Build images using the CLI command
    let cmd = cli::ImageCommands::Build {
        topic: Some("blog".to_string()),
        rebuild: false,
    };
    
    let result = execute_image_command(cmd);
    
    // Check that the command execution was successful
    assert!(result.is_ok());
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_optimize_images() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Optimize images for the blog topic using the CLI command
    let cmd = cli::ImageCommands::Optimize {
        topic: Some("blog".to_string()),
        reoptimize: false,
    };
    
    execute_image_command(cmd)?;
    
    // Since the implementation is a stub, we just test that the function doesn't crash
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_optimize_image() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Path to the test image
    let image_path = "content/blog/test-article/assets/test-image.jpg";
    
    // Optimize a single image
    optimize_image(
        image_path.to_string(),
        "test-article".to_string(),
        Some("blog".to_string()),
        Some(vec!["webp".to_string(), "avif".to_string()]),
        Some(vec!["300".to_string(), "600".to_string()]),
        Some(85),
        Some(false),
    )?;
    
    // Since the implementation is a stub, we just test that the function doesn't crash
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
} 