use anyhow::Result;
use std::env;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

// Import the topic function
use crate::tools::topic::add_topic;

pub fn run_debug_test() -> Result<()> {
    // Set up a temporary directory
    let temp_dir = tempdir()?;
    let current_dir = env::current_dir()?;

    println!("Current dir: {:?}", current_dir);
    println!("Temp dir: {:?}", temp_dir.path());

    // Change to the temp directory
    env::set_current_dir(temp_dir.path())?;

    // Add a new topic
    add_topic(
        Some("blog".to_string()),
        Some("Blog".to_string()),
        Some("Personal blog posts".to_string()),
        None,
    )?;

    // Check if directory exists
    let expected_path = temp_dir.path().join("content/blog");
    println!("Checking if path exists: {:?}", expected_path);
    println!("Path exists: {}", expected_path.exists());

    // List all content directories
    if Path::new("content").exists() {
        println!("Content dir exists");
        for entry in fs::read_dir("content")? {
            let entry = entry?;
            println!("Found: {:?}", entry.path());
        }
    } else {
        println!("Content dir does not exist");
    }

    // Restore the current directory
    env::set_current_dir(current_dir)?;

    Ok(())
}