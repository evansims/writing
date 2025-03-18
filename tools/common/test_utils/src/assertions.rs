//! # Test Assertions
//!
//! This module provides standard assertion helpers for common test patterns.

use common_errors::{Result, WritingError};
use common_models::{Frontmatter, Article};
use std::fmt::Debug;
use std::path::{Path, PathBuf};

/// Assert that a result is ok and return the unwrapped value
pub fn assert_ok<T: Debug>(result: Result<T>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => {
            panic!("Expected Ok, got Err: {:?}", err);
        }
    }
}

/// Assert that a result is an error and optionally matches a specific error message
pub fn assert_err(result: Result<()>, expected_message: Option<&str>) {
    match result {
        Ok(_) => {
            panic!("Expected Err, got Ok");
        }
        Err(err) => {
            if let Some(expected) = expected_message {
                let err_string = format!("{}", err);
                assert!(
                    err_string.contains(expected),
                    "Error message '{}' did not contain expected substring '{}'",
                    err_string,
                    expected
                );
            }
        }
    }
}

/// Assert that a string contains a substring
pub fn assert_contains(haystack: &str, needle: &str) {
    assert!(
        haystack.contains(needle),
        "String '{}' did not contain expected substring '{}'",
        haystack,
        needle
    );
}

/// Assert that a string does not contain a substring
pub fn assert_not_contains(haystack: &str, needle: &str) {
    assert!(
        !haystack.contains(needle),
        "String '{}' contained unexpected substring '{}'",
        haystack,
        needle
    );
}

/// Assert that a path exists
pub fn assert_path_exists(path: &Path) {
    assert!(
        path.exists(),
        "Path does not exist: {}",
        path.display()
    );
}

/// Assert that a path does not exist
pub fn assert_path_not_exists(path: &Path) {
    assert!(
        !path.exists(),
        "Path exists but should not: {}",
        path.display()
    );
}

/// Assert that a path is a file
pub fn assert_is_file(path: &Path) {
    assert!(
        path.exists() && path.is_file(),
        "Path is not a file: {}",
        path.display()
    );
}

/// Assert that a path is a directory
pub fn assert_is_dir(path: &Path) {
    assert!(
        path.exists() && path.is_dir(),
        "Path is not a directory: {}",
        path.display()
    );
}

/// Assert that a frontmatter has expected values
pub fn assert_frontmatter(frontmatter: &Frontmatter, expected: &[(&str, &str)]) {
    for (key, value) in expected {
        match *key {
            "title" => assert_eq!(frontmatter.title, *value, "Frontmatter title did not match"),
            "tagline" => assert_eq!(frontmatter.tagline.as_deref().unwrap_or(""), *value, "Frontmatter tagline did not match"),
            "published_at" => assert_eq!(frontmatter.published_at.as_deref().unwrap_or(""), *value, "Frontmatter published_at did not match"),
            "updated_at" => assert_eq!(frontmatter.updated_at.as_deref().unwrap_or(""), *value, "Frontmatter updated_at did not match"),
            "slug" => assert_eq!(frontmatter.slug.as_deref().unwrap_or(""), *value, "Frontmatter slug did not match"),
            _ => panic!("Unsupported frontmatter field: {}", key),
        }
    }
}

/// Assert that an article has expected values in its frontmatter and content
pub fn assert_article(article: &Article, frontmatter_values: &[(&str, &str)], content_contains: Option<&str>) {
    // Check frontmatter values
    assert_frontmatter(&article.frontmatter, frontmatter_values);

    // Check content if provided
    if let Some(content) = content_contains {
        assert_contains(&article.content, content);
    }
}

/// Assert that a vector contains a certain number of items
pub fn assert_count<T>(items: &[T], expected: usize) {
    assert_eq!(
        items.len(),
        expected,
        "Expected {} items, got {}",
        expected,
        items.len()
    );
}

/// Assert that all items in a vector satisfy a predicate
pub fn assert_all<T, F>(items: &[T], mut predicate: F, message: &str)
where
    F: FnMut(&T) -> bool,
{
    for (i, item) in items.iter().enumerate() {
        assert!(
            predicate(item),
            "{} (failed on item {} of {})",
            message,
            i,
            items.len()
        );
    }
}

/// Assert that any item in a vector satisfies a predicate
pub fn assert_any<T, F>(items: &[T], mut predicate: F, message: &str)
where
    F: FnMut(&T) -> bool,
{
    assert!(
        items.iter().any(|item| predicate(item)),
        "{} (no items matched the predicate out of {} items)",
        message,
        items.len()
    );
}

/// Assert that no items in a vector satisfy a predicate
pub fn assert_none<T, F>(items: &[T], mut predicate: F, message: &str)
where
    F: FnMut(&T) -> bool,
{
    assert!(
        !items.iter().any(|item| predicate(item)),
        "{} (found items that matched the predicate out of {} items)",
        message,
        items.len()
    );
}

/// Assert that all files in a directory satisfy a predicate
pub fn assert_all_files<F>(directory: &Path, mut predicate: F, message: &str)
where
    F: FnMut(&Path) -> bool,
{
    let entries = std::fs::read_dir(directory).expect("Failed to read directory");

    let mut any_files = false;
    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.is_file() {
            any_files = true;
            assert!(
                predicate(&path),
                "{} (failed on file {})",
                message,
                path.display()
            );
        }
    }

    assert!(any_files, "No files found in directory to assert against");
}