//! # Test Environment Setup
//!
//! This module provides utilities for setting up test environments.

use crate::TestFixture;
use common_errors::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use mockall::mock;
use crate::mocks::fs::FileSystem;

/// Configuration for a test environment
pub struct TestEnvironmentConfig {
    /// Base content directories to create
    pub content_dirs: Vec<String>,
    /// Config directories to create
    pub config_dirs: Vec<String>,
    /// Files to create with content
    pub files: HashMap<String, String>,
    /// Whether to set up a default config
    pub setup_default_config: bool,
}

impl Default for TestEnvironmentConfig {
    fn default() -> Self {
        Self {
            content_dirs: vec!["blog".to_string(), "podcast".to_string(), "page".to_string()],
            config_dirs: vec!["topics".to_string()],
            files: HashMap::new(),
            setup_default_config: true,
        }
    }
}

/// Test environment for testing
pub struct TestEnvironment {
    /// The test fixture
    pub fixture: TestFixture,
    /// Content directory path
    pub content_dir: PathBuf,
    /// Config directory path
    pub config_dir: PathBuf,
    /// Base directory path
    pub base_dir: PathBuf,
}

impl TestEnvironment {
    /// Create a new test environment
    pub fn new() -> Result<Self> {
        // Create the fixture with mocks
        let mut fixture = TestFixture::new()?;

        // Set up paths
        let base_dir = fixture.temp_dir.path().to_path_buf();
        let content_dir = PathBuf::from("content");
        let config_dir = PathBuf::from(".writing");

        // Create the content and config directories
        fixture.fs.expect_create_dir_all().with(mockall::predicate::eq(content_dir.clone())).returning(|_| Ok(()));
        fixture.fs.expect_create_dir_all().with(mockall::predicate::eq(config_dir.clone())).returning(|_| Ok(()));

        // Set up topic directories
        let topics = vec!["blog", "tutorials", "guides"];
        for topic in topics {
            let dir_path = content_dir.join(topic);
            fixture.fs.expect_create_dir_all().with(mockall::predicate::eq(dir_path.clone())).returning(|_| Ok(()));
        }

        // Set up the templates directory
        let templates_dir = PathBuf::from("templates");
        fixture.fs.expect_create_dir_all().with(mockall::predicate::eq(templates_dir.clone())).returning(|_| Ok(()));

        // If needed, we can also set up some example files
        for (file_path, content) in Self::example_files() {
            // Make sure parent directory exists
            if let Some(parent) = file_path.parent() {
                fixture.fs.expect_create_dir_all().with(mockall::predicate::eq(parent.to_path_buf())).returning(|_| Ok(()));
            }

            // Set up file write expectations
            let file_path_clone = file_path.clone();
            fixture.fs.expect_write_file()
                .with(mockall::predicate::eq(file_path_clone), mockall::predicate::always())
                .returning(|_, _| Ok(()));
        }

        Ok(Self {
            fixture,
            content_dir,
            config_dir,
            base_dir,
        })
    }

    /// Add a content file to the test environment
    pub fn add_content(&mut self, slug: &str, topic: &str, _content: &str) -> Result<()> {
        let content_path = PathBuf::from(format!("content/{}/{}/index.md", topic, slug));

        // Set up parent directory
        let parent = content_path.parent().unwrap().to_path_buf();
        self.fixture.fs.expect_create_dir_all().with(mockall::predicate::eq(parent)).returning(|_| Ok(()));

        // Set up file write expectations
        let content_path_clone = content_path.clone();
        self.fixture.fs.expect_write_file()
            .with(mockall::predicate::eq(content_path_clone), mockall::predicate::always())
            .returning(|_, _| Ok(()));

        Ok(())
    }

    /// Add a topic configuration to the test environment
    pub fn add_topic_config(&mut self, topic_key: &str, _topic_config: &str) -> Result<()> {
        let path = PathBuf::from(format!(".writing/topics/{}.yml", topic_key));

        // Set up parent directory
        let parent = path.parent().unwrap().to_path_buf();
        self.fixture.fs.expect_create_dir_all().with(mockall::predicate::eq(parent)).returning(|_| Ok(()));

        // Set up file write expectations
        let path_clone = path.clone();
        self.fixture.fs.expect_write_file()
            .with(mockall::predicate::eq(path_clone), mockall::predicate::always())
            .returning(|_, _| Ok(()));

        Ok(())
    }

    /// Create a content file
    pub fn create_content_file(&mut self, topic: &str, slug: &str, title: &str, content: &str, is_draft: bool) -> Result<PathBuf> {
        // Create a content file with frontmatter
        let frontmatter = format!(r#"---
title: "{}"
draft: {}
---
"#, title, is_draft);

        let full_content = format!("{}{}", frontmatter, content);
        let content_path = self.content_dir.join(topic).join(format!("{}.md", slug));

        // Set up the expectation for creating the content directory
        let dir_path = content_path.parent().unwrap().to_path_buf();
        self.fixture.fs.expect_create_dir_all()
            .with(mockall::predicate::eq(dir_path))
            .returning(|_| Ok(()));

        // Set up the expectation for writing the file
        self.fixture.fs.expect_write_file()
            .with(mockall::predicate::eq(content_path.clone()), mockall::predicate::eq(full_content))
            .returning(|_, _| Ok(()));

        Ok(content_path)
    }

    /// Create a topic configuration
    pub fn create_topic_config(&mut self, key: &str, name: &str, description: &str) -> Result<PathBuf> {
        let topic_config = format!(r#"name: "{}"
description: "{}"
directory: "{}""#, name, description, key);

        let path = self.config_dir.join("topics").join(format!("{}.yaml", key));

        // Set up the expectation for creating the config directory
        let dir_path = path.parent().unwrap().to_path_buf();
        self.fixture.fs.expect_create_dir_all()
            .with(mockall::predicate::eq(dir_path))
            .returning(|_| Ok(()));

        // Set up the expectation for writing the file
        self.fixture.fs.expect_write_file()
            .with(mockall::predicate::eq(path.clone()), mockall::predicate::eq(topic_config))
            .returning(|_, _| Ok(()));

        Ok(path)
    }

    /// Create multiple test articles
    pub fn create_test_articles(&mut self, count: usize, topic: &str) -> Result<Vec<PathBuf>> {
        let mut paths = Vec::with_capacity(count);
        for i in 0..count {
            let slug = format!("test-article-{}", i);
            let title = format!("Test Article {}", i);
            let content = format!("This is test article content for article {}.", i);
            let is_draft = i % 2 == 0; // Every other article is a draft

            let path = self.create_content_file(topic, &slug, &title, &content, is_draft)?;
            paths.push(path);
        }
        Ok(paths)
    }

    /// Get the absolute path to a relative path in the test environment
    pub fn abs_path(&self, relative_path: &str) -> PathBuf {
        self.base_dir.join(relative_path)
    }

    /// Example files to be set up in the test environment
    fn example_files() -> Vec<(PathBuf, String)> {
        vec![
            // Default configuration
            (PathBuf::from(".writing/config.yml"), r#"
title: Test Writing Environment
email: test@example.com
url: https://example.com
            "#.to_string()),

            // Default blog topic configuration
            (PathBuf::from(".writing/topics/blog.yml"), r#"
name: Blog
directory: blog
description: Blog posts
            "#.to_string()),

            // Default tutorial topic configuration
            (PathBuf::from(".writing/topics/tutorials.yml"), r#"
name: Tutorials
directory: tutorials
description: Tutorial articles
            "#.to_string()),

            // Default template
            (PathBuf::from("templates/blog/index.mdx"), r#"---
title: {{title}}
date: {{date}}
---

# {{title}}

This is a test blog post.
            "#.to_string()),

            // Example blog post
            (PathBuf::from("content/blog/example-post/index.mdx"), r#"---
title: Example Blog Post
date: 2023-01-01
---

# Example Blog Post

This is an example blog post.
            "#.to_string()),
        ]
    }
}

/// Run a function with a test environment
pub fn with_test_environment<F, T>(f: F) -> T
where
    F: FnOnce(&TestEnvironment) -> T
{
    let environment = TestEnvironment::new().expect("Failed to create test environment");
    f(&environment)
}

/// Run a function with a custom test environment
pub fn with_custom_test_environment<F, T>(_config: TestEnvironmentConfig, f: F) -> T
where
    F: FnOnce(&TestEnvironment) -> T
{
    let environment = TestEnvironment::new().expect("Failed to create test environment");
    f(&environment)
}