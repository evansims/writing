//! Integration tests for the content-search CLI

use anyhow::Result;
use common_test_utils::fixtures::TestFixture;
use common_test_utils::with_test_fixture;
use common_test_utils::integration::TestCommand;
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod cli_tests {
    use super::*;

    /// Set up a test article in the fixture
    fn setup_test_article(fixture: &TestFixture) -> Result<()> {
        // Create content directory structure
        let content_dir = fixture.create_dir("content")?;
        let blog_dir = fixture.create_dir("content/blog")?;
        let article_dir = fixture.create_dir("content/blog/test-article")?;

        // Create a test article
        let article_content = r#"---
title: Test Article
date: 2023-01-01
draft: false
tags: ["rust", "test"]
---

# Test Article

This is a test article that contains some searchable content.
We will use this to test the search functionality.

## Section with Keywords

This section contains some keywords like "searchable" and "test" and "rust".
"#;

        fixture.create_file(
            &format!("{}/index.mdx", article_dir.display()),
            article_content
        )?;

        // Create a test config
        fixture.create_file(
            ".write.yaml",
            r#"
content:
  root: "./content"
topics:
  - name: blog
    title: Blog
    description: Blog posts
"#
        )?;

        Ok(())
    }

    #[test]
    fn test_search_cli_basic() -> Result<()> {
        with_test_fixture!(fixture => {
            // Set up test environment
            setup_test_article(&fixture)?;

            // Create a test command
            let command = TestCommand::cargo_binary("content-search")?;

            // Test basic search
            let output = command.run_with_args(&["--query", "searchable"])?;

            assert!(output.status.success(), "Command failed: {}", output.stderr);
            assert!(output.stdout.contains("searchable"),
                   "Output doesn't contain the search term: {}", output.stdout);

            Ok(())
        })
    }

    #[test]
    fn test_search_cli_with_topic() -> Result<()> {
        with_test_fixture!(fixture => {
            // Set up test environment
            setup_test_article(&fixture)?;

            // Create a test command
            let command = TestCommand::cargo_binary("content-search")?;

            // Test search with topic
            let output = command.run_with_args(&["--query", "test", "--topic", "blog"])?;

            assert!(output.status.success(), "Command failed: {}", output.stderr);
            assert!(output.stdout.contains("test"),
                   "Output doesn't contain the search term: {}", output.stdout);

            Ok(())
        })
    }

    #[test]
    fn test_search_cli_with_tags() -> Result<()> {
        with_test_fixture!(fixture => {
            // Set up test environment
            setup_test_article(&fixture)?;

            // Create a test command
            let command = TestCommand::cargo_binary("content-search")?;

            // Test search with tags
            let output = command.run_with_args(&["--query", "test", "--tags", "rust"])?;

            assert!(output.status.success(), "Command failed: {}", output.stderr);
            assert!(output.stdout.contains("test"),
                   "Output doesn't contain the search term: {}", output.stdout);

            Ok(())
        })
    }

    #[test]
    fn test_search_cli_no_results() -> Result<()> {
        with_test_fixture!(fixture => {
            // Set up test environment
            setup_test_article(&fixture)?;

            // Create a test command
            let command = TestCommand::cargo_binary("content-search")?;

            // Test search with no results
            let output = command.run_with_args(&["--query", "nonexistent"])?;

            // The command should still succeed even with no results
            assert!(output.status.success(), "Command failed: {}", output.stderr);

            // Output should indicate no results
            assert!(output.stdout.contains("No results") || output.stdout.contains("0 results"),
                   "Output doesn't indicate no results: {}", output.stdout);

            Ok(())
        })
    }

    #[test]
    fn test_search_cli_invalid_query() -> Result<()> {
        with_test_fixture!(fixture => {
            // Set up test environment
            setup_test_article(&fixture)?;

            // Create a test command
            let command = TestCommand::cargo_binary("content-search")?;

            // Test search with empty query
            let output = command.run_with_args(&["--query", ""])?;

            // The command should fail with an invalid query
            assert!(!output.status.success(), "Command should have failed with empty query");

            // Error should indicate invalid query
            assert!(output.stderr.contains("Invalid") || output.stderr.contains("invalid"),
                   "Error doesn't indicate invalid query: {}", output.stderr);

            Ok(())
        })
    }
}