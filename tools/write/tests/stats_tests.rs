//! Tests for the stats module
//!
//! This file contains tests for the statistics functionality.

use anyhow::Result;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

// Import the module under test
use write::tools::*;

/// Set up a test environment with a temporary directory
fn setup() -> Result<tempfile::TempDir> {
    let temp_dir = tempdir()?;
    
    // Create content directory structure
    let content_dir = temp_dir.path().join("content");
    let blog_dir = content_dir.join("blog");
    let notes_dir = content_dir.join("notes");
    fs::create_dir_all(&blog_dir)?;
    fs::create_dir_all(&notes_dir)?;
    
    // Create a few articles for testing
    let article1_dir = blog_dir.join("article1");
    let article2_dir = blog_dir.join("article2");
    let note1_dir = notes_dir.join("note1");
    fs::create_dir_all(&article1_dir)?;
    fs::create_dir_all(&article2_dir)?;
    fs::create_dir_all(&note1_dir)?;
    
    // Create content files
    let article1_content = r#"---
title: Article 1
date: 2023-01-01
tags: test,article
---

# Article 1

This is the first test article with some content.
It has multiple paragraphs and should be considered
as a regular article.
"#;
    
    let article2_content = r#"---
title: Article 2
date: 2023-02-01
tags: test,article,featured
draft: true
---

# Article 2

This is the second test article.
It is marked as a draft and has some different tags.
"#;
    
    let note1_content = r#"---
title: Note 1
date: 2023-03-01
tags: note,important
---

# Note 1

This is a test note that belongs to a different topic.
"#;
    
    fs::write(article1_dir.join("index.mdx"), article1_content)?;
    fs::write(article2_dir.join("index.mdx"), article2_content)?;
    fs::write(note1_dir.join("index.mdx"), note1_content)?;
    
    Ok(temp_dir)
}

#[test]
fn test_generate_stats_all_content() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Generate stats for all content
    generate_content_stats(
        None,
        None,
        false, // Don't include drafts
        "date".to_string(),
        false, // Not detailed
    )?;
    
    // Since the implementation is a stub, we just test that the function doesn't crash
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_generate_stats_with_drafts() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Generate stats including drafts
    generate_content_stats(
        None,
        None,
        true,  // Include drafts
        "date".to_string(),
        false, // Not detailed
    )?;
    
    // Since the implementation is a stub, we just test that the function doesn't crash
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_generate_stats_specific_topic() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Generate stats for a specific topic
    generate_content_stats(
        None,
        Some("blog".to_string()),
        false, // Don't include drafts
        "date".to_string(),
        false, // Not detailed
    )?;
    
    // Since the implementation is a stub, we just test that the function doesn't crash
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
}

#[test]
fn test_generate_stats_detailed() -> Result<()> {
    let temp_dir = setup()?;
    let current_dir = std::env::current_dir()?;
    
    // Change to the temp directory for the test
    std::env::set_current_dir(temp_dir.path())?;
    
    // Generate detailed stats
    generate_content_stats(
        None,
        None,
        false, // Don't include drafts
        "words".to_string(),
        true,  // Detailed
    )?;
    
    // Since the implementation is a stub, we just test that the function doesn't crash
    
    // Restore the current directory
    std::env::set_current_dir(current_dir)?;
    
    Ok(())
} 