//! Integration tests for the content-stats CLI

use anyhow::Result;
use common_test_utils::fixtures::TestFixture;
use common_test_utils::with_test_fixture;
use common_test_utils::integration::TestCommand;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod cli_tests {
    use super::*;

    fn setup_test_articles(fixture: &TestFixture) -> Result<()> {
        // Create content directory structure
        let content_dir = fixture.create_dir("content")?;
        let blog_dir = fixture.create_dir("content/blog")?;
        let article1_dir = fixture.create_dir("content/blog/test-article-1")?;
        let article2_dir = fixture.create_dir("content/blog/test-article-2")?;

        // Create the first test article (published)
        let article1_content = r#"---
title: Test Article 1
date: 2023-01-01
draft: false
tags: ["rust", "test"]
---

# Test Article 1

This is a test article with some content for stats calculation.

## Section with Keywords

Some more content here to make it a bit longer for testing stats.
"#;

        // Create the second test article (draft)
        let article2_content = r#"---
title: Test Article 2
date: DRAFT
draft: true
tags: ["rust", "draft"]
---

# Test Article 2

This is a draft article with some content for testing stats.
"#;

        fixture.create_file(
            &format!("{}/index.mdx", article1_dir.display()),
            article1_content
        )?;

        fixture.create_file(
            &format!("{}/index.mdx", article2_dir.display()),
            article2_content
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
    fn test_stats_cli_basic() -> Result<()> {
        with_test_fixture!(fixture => {
            // Set up test environment
            setup_test_articles(&fixture)?;

            // Create a test command
            let command = TestCommand::cargo_binary("content-stats")?;

            // Test basic stats without arguments
            let output = command.run_with_args(&[])?;

            assert!(output.status.success(), "Command failed: {}", output.stderr);
            assert!(output.stdout.contains("Test Article 1"),
                   "Output should contain article title: {}", output.stdout);
            assert!(output.stdout.contains("word count") || output.stdout.contains("words"),
                   "Output should contain word count information: {}", output.stdout);

            Ok(())
        });

        Ok(())
    }

    #[test]
    fn test_stats_cli_with_slug() -> Result<()> {
        with_test_fixture!(fixture => {
            // Set up test environment
            setup_test_articles(&fixture)?;

            // Create a test command
            let command = TestCommand::cargo_binary("content-stats")?;

            // Test stats for a specific slug
            let output = command.run_with_args(&["--slug", "test-article-1"])?;

            assert!(output.status.success(), "Command failed: {}", output.stderr);
            assert!(output.stdout.contains("Test Article 1"),
                   "Output should contain article title: {}", output.stdout);
            assert!(!output.stdout.contains("Test Article 2"),
                   "Output should not contain other article title: {}", output.stdout);

            Ok(())
        });

        Ok(())
    }

    #[test]
    fn test_stats_cli_with_topic() -> Result<()> {
        with_test_fixture!(fixture => {
            // Set up test environment
            setup_test_articles(&fixture)?;

            // Create a test command
            let command = TestCommand::cargo_binary("content-stats")?;

            // Test stats for a specific topic
            let output = command.run_with_args(&["--topic", "blog"])?;

            assert!(output.status.success(), "Command failed: {}", output.stderr);
            assert!(output.stdout.contains("blog"),
                   "Output should mention the topic: {}", output.stdout);

            Ok(())
        });

        Ok(())
    }

    #[test]
    fn test_stats_cli_include_drafts() -> Result<()> {
        with_test_fixture!(fixture => {
            // Set up test environment
            setup_test_articles(&fixture)?;

            // Create a test command
            let command = TestCommand::cargo_binary("content-stats")?;

            // Test stats including drafts
            let output = command.run_with_args(&["--include-drafts"])?;

            assert!(output.status.success(), "Command failed: {}", output.stderr);
            assert!(output.stdout.contains("Test Article 2"),
                   "Output should contain draft article: {}", output.stdout);
            assert!(output.stdout.contains("DRAFT"),
                   "Output should indicate draft status: {}", output.stdout);

            Ok(())
        });

        Ok(())
    }

    #[test]
    fn test_stats_cli_sort_options() -> Result<()> {
        with_test_fixture!(fixture => {
            // Set up test environment
            setup_test_articles(&fixture)?;

            // Create a test command
            let command = TestCommand::cargo_binary("content-stats")?;

            // Test sorting options
            let output = command.run_with_args(&["--sort-by", "word_count"])?;

            assert!(output.status.success(), "Command failed: {}", output.stderr);

            Ok(())
        });

        Ok(())
    }

    #[test]
    fn test_stats_cli_detailed_output() -> Result<()> {
        with_test_fixture!(fixture => {
            // Set up test environment
            setup_test_articles(&fixture)?;

            // Create a test command
            let command = TestCommand::cargo_binary("content-stats")?;

            // Test detailed output
            let output = command.run_with_args(&["--detailed"])?;

            assert!(output.status.success(), "Command failed: {}", output.stderr);
            assert!(output.stdout.contains("characters") || output.stdout.contains("character count"),
                   "Output should include character count: {}", output.stdout);
            assert!(output.stdout.contains("paragraphs") || output.stdout.contains("paragraph count"),
                   "Output should include paragraph count: {}", output.stdout);

            Ok(())
        });

        Ok(())
    }

    #[test]
    fn test_stats_cli_nonexistent_slug() -> Result<()> {
        with_test_fixture!(fixture => {
            // Set up test environment
            setup_test_articles(&fixture)?;

            // Create a test command
            let command = TestCommand::cargo_binary("content-stats")?;

            // Test with nonexistent slug
            let output = command.run_with_args(&["--slug", "nonexistent-article"])?;

            assert!(!output.status.success(), "Command should fail with nonexistent slug");
            assert!(output.stderr.contains("nonexistent") || output.stderr.contains("not found"),
                   "Error should mention the nonexistent slug: {}", output.stderr);

            Ok(())
        });

        Ok(())
    }
}