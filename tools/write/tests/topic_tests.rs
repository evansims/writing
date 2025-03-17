//! Tests for the topic module
//!
//! This file contains tests for the topic-related functionality.

use anyhow::Result;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

// Import the module under test
use write::tools::*;

/// Set up a test environment with a temporary directory
fn setup() -> Result<tempfile::TempDir> {
    let temp_dir = tempdir()?;
    let content_dir = temp_dir.path().join("content");
    fs::create_dir_all(&content_dir)?;
    
    Ok(temp_dir)
}

#[test]
fn test_add_topic() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Add a new topic
    add_topic(
        Some("blog".to_string()),
        Some("Blog".to_string()),
        Some("Personal blog posts".to_string()),
        None,
    )?;
    
    // Check if the topic directory was created
    let topic_path = temp_dir.path().join("content/blog");
    assert!(topic_path.exists());
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_add_topic_with_custom_directory() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Add a new topic with a custom directory
    add_topic(
        Some("blog".to_string()),
        Some("Blog".to_string()),
        Some("Personal blog posts".to_string()),
        Some("custom-blog-dir".to_string()),
    )?;
    
    // Check if the custom directory was created
    let topic_path = temp_dir.path().join("content/custom-blog-dir");
    assert!(topic_path.exists());
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_delete_topic() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Create a topic directory
    let topic_path = temp_dir.path().join("content/blog");
    fs::create_dir_all(&topic_path)?;
    
    // Add some content to the topic
    let article_path = topic_path.join("test-article");
    fs::create_dir_all(&article_path)?;
    fs::write(article_path.join("index.mdx"), "Test content")?;
    
    // Delete the topic
    delete_topic(
        Some("blog".to_string()),
        None,
        true, // Force deletion
    )?;
    
    // Check if the topic was deleted
    assert!(!topic_path.exists());
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_rename_topic() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Create a topic directory
    let topic_path = temp_dir.path().join("content/blog");
    fs::create_dir_all(&topic_path)?;
    
    // The rename_topic function is mostly a stub at this point, so we'll just
    // test that it doesn't crash
    rename_topic(
        Some("blog".to_string()),
        Some("articles".to_string()),
        None,
        None,
    )?;
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_edit_topic() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Create a topic directory
    let topic_path = temp_dir.path().join("content/blog");
    fs::create_dir_all(&topic_path)?;
    
    // The edit_topic function is mostly a stub at this point, so we'll just
    // test that it doesn't crash
    edit_topic(
        Some("blog".to_string()),
        Some("Updated Blog".to_string()),
        Some("Updated description".to_string()),
    )?;
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
} 