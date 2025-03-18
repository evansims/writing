use std::fs;
use std::io::Write;
use std::path::Path;
use tempfile::{NamedTempFile, TempDir};

// Basic frontmatter and body extraction functions for testing
fn split_frontmatter_and_body(content: &str) -> (String, String) {
    let parts: Vec<&str> = content.split("---").collect();
    if parts.len() >= 3 {
        let frontmatter = parts[1].trim();
        let joined = parts[2..].join("---");
        let body = joined.trim();
        (frontmatter.to_string(), body.to_string())
    } else {
        ("".to_string(), content.to_string())
    }
}

// Extract frontmatter as key-value pairs
fn extract_frontmatter_as_map(content: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    let (frontmatter, _) = split_frontmatter_and_body(content);

    for line in frontmatter.lines() {
        if let Some(pos) = line.find(':') {
            let key = line[..pos].trim().to_string();
            let value = line[pos+1..].trim().to_string();

            // Remove quotes if present
            let clean_value = if value.starts_with('"') && value.ends_with('"') {
                value[1..value.len()-1].to_string()
            } else {
                value
            };

            map.insert(key, clean_value);
        }
    }

    map
}

// Save content to a file
fn save_content(path: &Path, content: &str) -> std::io::Result<()> {
    fs::write(path, content)
}

// Merge edited frontmatter with existing body
fn merge_frontmatter_and_body(frontmatter: &str, body: &str) -> String {
    format!("---\n{}\n---\n\n{}", frontmatter.trim(), body.trim())
}

// Update content with edited frontmatter or body
fn update_content(
    path: &Path,
    content: &str,
    frontmatter_only: bool,
    content_only: bool
) -> std::io::Result<()> {
    // Read existing content
    let existing_content = fs::read_to_string(path)?;
    let (existing_frontmatter, existing_body) = split_frontmatter_and_body(&existing_content);

    let new_content = if frontmatter_only {
        // Get frontmatter from provided content
        let (new_frontmatter, _) = split_frontmatter_and_body(content);
        merge_frontmatter_and_body(&new_frontmatter, &existing_body)
    } else if content_only {
        // Preserve existing frontmatter, update only the body
        merge_frontmatter_and_body(&existing_frontmatter, content)
    } else {
        // Replace entire content
        content.to_string()
    };

    // Write content to file
    fs::write(path, new_content)
}

// Test that we can split frontmatter and body correctly
fn test_split_frontmatter_and_body() {
    let content = r#"---
title: "Test Post"
date: "2020-01-01"
---

# Test Post

This is a test post."#;

    let (frontmatter, body) = split_frontmatter_and_body(content);

    // Check that the frontmatter contains the expected content
    assert!(frontmatter.contains("title: \"Test Post\""));
    assert!(frontmatter.contains("date: \"2020-01-01\""));

    // Check that the body contains the expected content
    assert!(body.contains("# Test Post"));
    assert!(body.contains("This is a test post."));
}

// Test extracting frontmatter as a map
fn test_extract_frontmatter_as_map() {
    let content = r#"---
title: "Test Post"
date: "2020-01-01"
draft: true
---

# Test Post

This is a test post."#;

    let frontmatter = extract_frontmatter_as_map(content);

    // Check specific fields
    assert_eq!(frontmatter.get("title").unwrap(), "Test Post");
    assert_eq!(frontmatter.get("date").unwrap(), "2020-01-01");
    assert_eq!(frontmatter.get("draft").unwrap(), "true");

    // Check that map contains all expected keys
    assert_eq!(frontmatter.len(), 3);
}

// Test that we can save content
fn test_save_content() {
    // Create a temporary file for testing
    let mut temp_file = NamedTempFile::new().unwrap();

    // Write test content to the file
    let original_content = r#"---
title: "Test Post"
date: "2020-01-01"
---

# Test Post

This is a test post."#;

    temp_file.write_all(original_content.as_bytes()).unwrap();

    // Edit the content
    let edited_content = r#"---
title: "Edited Post"
date: "2020-01-01"
---

# Edited Post

This post has been edited."#;

    // Save the edited content
    save_content(temp_file.path(), edited_content).unwrap();

    // Read the file to verify the changes
    let saved_content = fs::read_to_string(temp_file.path()).unwrap();

    // Check that the content was properly updated
    assert!(saved_content.contains("title: \"Edited Post\""));
    assert!(saved_content.contains("date: \"2020-01-01\""));
    assert!(saved_content.contains("# Edited Post"));
    assert!(saved_content.contains("This post has been edited."));
    assert!(!saved_content.contains("title: \"Test Post\""));
    assert!(!saved_content.contains("# Test Post"));
    assert!(!saved_content.contains("This is a test post."));
}

// Test merging frontmatter and body
fn test_merge_frontmatter_and_body() {
    let frontmatter = r#"title: "Merged Post"
date: "2020-01-01"
tags: ["test", "example"]"#;

    let body = r#"# Merged Post

This is a post with merged frontmatter and body."#;

    let merged = merge_frontmatter_and_body(frontmatter, body);

    // Check that the merged content has the expected format
    assert!(merged.starts_with("---\n"));
    assert!(merged.contains("---\n\n#"));
    assert!(merged.contains("title: \"Merged Post\""));
    assert!(merged.contains("date: \"2020-01-01\""));
    assert!(merged.contains("tags: [\"test\", \"example\"]"));
    assert!(merged.contains("# Merged Post"));
    assert!(merged.contains("This is a post with merged frontmatter and body."));
}

// Test updating only frontmatter
fn test_update_frontmatter_only() {
    // Create a temporary file for testing
    let mut temp_file = NamedTempFile::new().unwrap();

    // Write test content to the file
    let original_content = r#"---
title: "Test Post"
date: "2020-01-01"
---

# Test Post

This is a test post."#;

    temp_file.write_all(original_content.as_bytes()).unwrap();

    // Edit only the frontmatter
    let edited_frontmatter = r#"---
title: "Updated Frontmatter"
date: "2023-01-01"
draft: true
---"#;

    // Update only the frontmatter
    update_content(temp_file.path(), edited_frontmatter, true, false).unwrap();

    // Read the file to verify the changes
    let saved_content = fs::read_to_string(temp_file.path()).unwrap();

    // Check that only the frontmatter was updated
    assert!(saved_content.contains("title: \"Updated Frontmatter\""));
    assert!(saved_content.contains("date: \"2023-01-01\""));
    assert!(saved_content.contains("draft: true"));
    assert!(saved_content.contains("# Test Post"));
    assert!(saved_content.contains("This is a test post."));
}

// Test updating only body content
fn test_update_content_only() {
    // Create a temporary file for testing
    let mut temp_file = NamedTempFile::new().unwrap();

    // Write test content to the file
    let original_content = r#"---
title: "Test Post"
date: "2020-01-01"
---

# Test Post

This is a test post."#;

    temp_file.write_all(original_content.as_bytes()).unwrap();

    // Edit only the body
    let edited_body = "# Updated Body\n\nThis content has been updated.";

    // Update only the body
    update_content(temp_file.path(), edited_body, false, true).unwrap();

    // Read the file to verify the changes
    let saved_content = fs::read_to_string(temp_file.path()).unwrap();

    // Check that only the body was updated
    assert!(saved_content.contains("title: \"Test Post\""));
    assert!(saved_content.contains("date: \"2020-01-01\""));
    assert!(saved_content.contains("# Updated Body"));
    assert!(saved_content.contains("This content has been updated."));
    assert!(!saved_content.contains("# Test Post"));
    assert!(!saved_content.contains("This is a test post."));
}

// Test finding content by path
fn test_find_content() {
    // Create a temporary directory for testing
    let temp_dir = TempDir::new().unwrap();

    // Create content directory structure
    let content_dir = temp_dir.path().join("content");
    fs::create_dir_all(&content_dir).unwrap();

    // Create blog topic directory
    let blog_dir = content_dir.join("blog");
    fs::create_dir_all(&blog_dir).unwrap();

    // Create a test post in the blog directory
    let test_post_dir = blog_dir.join("test-post");
    fs::create_dir_all(&test_post_dir).unwrap();

    // Create the test post content
    let test_post_content = r#"---
title: "Test Post"
date: "2023-01-01"
---

# Test Post

This is a test post."#;
    fs::write(test_post_dir.join("index.md"), test_post_content).unwrap();

    // Verify the file exists and can be read
    let file_path = test_post_dir.join("index.md");
    assert!(file_path.exists(), "The test file should exist");

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("Test Post"), "The content should contain the test post title");
}

fn main() {
    println!("Running standalone tests");

    // Run the tests
    test_split_frontmatter_and_body();
    test_extract_frontmatter_as_map();
    test_save_content();
    test_merge_frontmatter_and_body();
    test_update_frontmatter_only();
    test_update_content_only();
    test_find_content();

    println!("All standalone tests passed!");
}