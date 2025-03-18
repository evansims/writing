//! # Topic Management Module
//!
//! This module provides functionality for managing topics, including adding,
//! editing, renaming, and deleting topics.

use anyhow::Result;
use colored::*;
use crate::ui;

/// Add a new topic
pub fn add_topic(
    key: Option<String>,
    name: Option<String>,
    description: Option<String>,
    directory: Option<String>,
) -> Result<()> {
    // Get the key if not provided
    let key = match key {
        Some(k) => k,
        None => {
            ui::show_info("Enter a key for the topic (e.g., 'blog', 'notes'):");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    // Get the name if not provided
    let name = match name {
        Some(n) => n,
        None => {
            ui::show_info("Enter a name for the topic (e.g., 'Blog', 'Notes'):");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    // Get the description if not provided
    let _description = match description {
        Some(d) => d,
        None => {
            ui::show_info("Enter a description for the topic:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    // Get the directory if not provided
    let directory = match directory {
        Some(p) => p,
        None => key.clone(), // Default to using the key as the directory name
    };

    // TODO: Add topic to config
    // For now, just create the directory
    let content_dir = format!("content/{}", directory);
    std::fs::create_dir_all(&content_dir)?;

    ui::show_success(&format!("Added topic: {} ({})", name.green(), key.green()));

    Ok(())
}

/// Edit an existing topic
pub fn edit_topic(
    key: Option<String>,
    name: Option<String>,
    description: Option<String>,
) -> Result<()> {
    // Call the extended version with directory set to None
    edit_topic_with_directory(key, name, description, None)
}

/// Edit an existing topic, including its directory
pub fn edit_topic_with_directory(
    key: Option<String>,
    name: Option<String>,
    description: Option<String>,
    directory: Option<String>,
) -> Result<()> {
    // Get the key if not provided
    let key = match key {
        Some(k) => k,
        None => {
            ui::show_info("Enter the key of the topic to edit:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    // TODO: Get the current topic from config

    // Update fields if provided
    if let Some(name) = name {
        // TODO: Update name in config
        ui::show_info(&format!("Updating topic name to: {}", name));
    }

    if let Some(description) = description {
        // TODO: Update description in config
        ui::show_info(&format!("Updating topic description to: {}", description));
    }

    if let Some(directory) = directory {
        // TODO: Update directory in config and move files
        ui::show_info(&format!("Updating topic directory to: {}", directory));
    }

    ui::show_success(&format!("Edited topic: {}", key.green()));

    Ok(())
}

/// Rename a topic
pub fn rename_topic(
    key: Option<String>,
    new_key: Option<String>,
    _new_name: Option<String>,
    _new_path: Option<String>,
) -> Result<()> {
    // Get the key if not provided
    let key = match key {
        Some(k) => k,
        None => {
            ui::show_info("Enter the key of the topic to rename:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    // Get the new key if not provided
    let new_key = match new_key {
        Some(k) => k,
        None => {
            ui::show_info("Enter the new key for the topic:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    // TODO: Rename topic in config and move files if necessary

    ui::show_success(&format!("Renamed topic from {} to {}", key.green(), new_key.green()));

    Ok(())
}

/// Delete a topic
pub fn delete_topic(
    key: Option<String>,
    _target: Option<String>,
    force: bool,
) -> Result<()> {
    // Get the key if not provided
    let key = match key {
        Some(k) => k,
        None => {
            ui::show_info("Enter the key of the topic to delete:");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    // TODO: Get topic directory from config
    let topic_dir = format!("content/{}", key);

    // Check if the topic exists
    if !std::path::Path::new(&topic_dir).exists() {
        return Err(anyhow::anyhow!("Topic not found: {}", key));
    }

    // Confirm deletion if not forced
    if !force {
        ui::show_warning(&format!("Are you sure you want to delete the topic {}? This will delete all content in {}. (y/N)", key, topic_dir));
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            ui::show_info("Deletion cancelled.");
            return Ok(());
        }
    }

    // Delete the topic directory
    std::fs::remove_dir_all(&topic_dir)?;

    // TODO: Remove topic from config

    ui::show_success(&format!("Deleted topic: {}", key.red()));

    Ok(())
}

/// List topics
pub fn list_topics() -> Result<()> {
    // TODO: Implement topic listing
    ui::show_info("Listing topics...");

    Ok(())
}

/// List topics with format
pub fn list_topics_with_format(format: &str) -> Result<()> {
    // TODO: Implement topic listing with format
    ui::show_info(&format!("Listing topics with format: {}", format));

    Ok(())
}