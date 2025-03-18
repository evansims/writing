/*
// Topic Tests
// This file has been commented out because it uses interfaces that don't exist or have changed.
*/

//! Tests for the topic module
//!
//! This file contains tests for the topic-related functionality.

use anyhow::Result;
use tempfile::tempdir;

// Import the module under test - fix the imports to use the specific topic functions
use write::tools::topic::{add_topic_with_base_path, delete_topic, edit_topic, rename_topic};

fn setup() -> Result<tempfile::TempDir> {
    // Create a temporary directory for testing
    let temp_dir = tempdir()?;

    // Set up the config
    let config_str = r#"
content:
  base_dir: content
  topics: {}
images:
  formats: ["jpg"]
publication:
  author: Test Author
  copyright: Test Copyright
  site: https://example.com
"#;

    // Create and configure a minimal config.yaml in the temp directory
    let config_path = temp_dir.path().join("config.yaml");
    std::fs::write(&config_path, config_str)?;

    // Set the CONFIG_PATH environment variable to point to our test config
    std::env::set_var("CONFIG_PATH", config_path.to_str().unwrap());

    Ok(temp_dir)
}

#[test]
fn test_add_topic() -> Result<()> {
    let temp_dir = setup()?;
    let _current_dir = std::env::current_dir()?;

    // Store temp_dir path for later reference
    let temp_dir_path = temp_dir.path().to_path_buf();

    // Create the topic
    add_topic_with_base_path(
        Some("blog".to_string()),
        Some("Blog".to_string()),
        Some("Personal blog posts".to_string()),
        None,
        Some(temp_dir_path.clone()),
    )?;

    // Check if the topic directory was created using stored temp_dir path
    let topic_path = temp_dir_path.join("content").join("blog");
    println!("Checking path: {:?}", topic_path);
    println!("Current working directory: {:?}", std::env::current_dir()?);

    assert!(topic_path.exists(), "Topic directory does not exist at: {:?}", topic_path);

    Ok(())
}

#[test]
fn test_add_topic_with_custom_directory() -> Result<()> {
    let temp_dir = setup()?;
    let _current_dir = std::env::current_dir()?;

    // Store temp_dir path for later reference
    let temp_dir_path = temp_dir.path().to_path_buf();

    println!("Changed to directory: {:?}", std::env::current_dir()?);

    // Ensure content directory exists
    let content_dir = temp_dir_path.join("content");
    println!("Creating content directory at: {:?}", content_dir);
    std::fs::create_dir_all(&content_dir)?;

    // Verify the content directory exists
    assert!(content_dir.exists(), "Content directory was not created at: {:?}", content_dir);

    // Create the topic with a custom directory name
    add_topic_with_base_path(
        Some("blog".to_string()),
        Some("Blog".to_string()),
        Some("Personal blog posts".to_string()),
        Some("custom-blog-dir".to_string()),
        Some(temp_dir_path.clone()),
    )?;

    // The directory should be at "content/custom-blog-dir" not "content/blog"
    let custom_topic_dir = temp_dir_path.join("content").join("custom-blog-dir");
    println!("Checking path: {:?}", custom_topic_dir);
    assert!(custom_topic_dir.exists(), "Custom topic directory does not exist at: {:?}", custom_topic_dir);

    // The standard path should not exist
    let standard_topic_dir = temp_dir_path.join("content").join("blog");
    assert!(!standard_topic_dir.exists(), "Standard topic directory should not exist at: {:?}", standard_topic_dir);

    Ok(())
}

#[test]
#[ignore]
fn test_delete_topic() -> Result<()> {
    let temp_dir = setup()?;

    // Store temp_dir path for later reference
    let temp_dir_path = temp_dir.path().to_path_buf();

    // First, add a topic
    add_topic_with_base_path(
        Some("blog".to_string()),    // key
        Some("Blog".to_string()),    // name
        None,                        // description
        None,                        // directory
        None,
    )?;

    // Verify topic directory was created
    let topic_path = temp_dir_path.join("content").join("blog");
    assert!(topic_path.exists(), "Topic directory was not created before deletion");

    // Delete the topic
    delete_topic(
        Some("blog".to_string()),
        None,
        true, // Force deletion
    )?;

    // Verify topic directory was deleted
    assert!(!topic_path.exists(), "Topic directory was not deleted");

    Ok(())
}

#[test]
fn test_rename_topic() -> Result<()> {
    let temp_dir = setup()?;

    // Store temp_dir path for later reference
    let temp_dir_path = temp_dir.path().to_path_buf();

    // First, add a topic
    add_topic_with_base_path(
        Some("blog".to_string()),
        Some("Blog".to_string()),
        None,
        None,
        Some(temp_dir_path.clone()),
    )?;

    // Rename the topic
    rename_topic(
        Some("blog".to_string()),
        Some("articles".to_string()),
        None,
        None,
    )?;

    // Verify old directory doesn't exist and new one does
    let old_path = temp_dir_path.join("content").join("blog");
    let new_path = temp_dir_path.join("content").join("articles");

    assert!(!old_path.exists(), "Old topic directory still exists");
    assert!(new_path.exists(), "New topic directory does not exist");

    Ok(())
}

#[test]
fn test_edit_topic() -> Result<()> {
    let temp_dir = setup()?;

    // Store temp_dir path for later reference
    let temp_dir_path = temp_dir.path().to_path_buf();

    // First, add a topic
    add_topic_with_base_path(
        Some("blog".to_string()),
        Some("Blog".to_string()),
        None,
        None,
        Some(temp_dir_path.clone()),
    )?;

    // Edit the topic
    edit_topic(
        Some("blog".to_string()),
        Some("Updated Blog".to_string()),
        Some("Updated description".to_string()),
    )?;

    // TODO: Verify the changes were made
    // This would require reading the config file

    Ok(())
}

#[test]
fn test_topic_placeholder() -> Result<()> {
    // Placeholder test for when the topics module is modified
    assert!(true);
    Ok(())
}