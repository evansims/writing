//! # Test Environment Setup
//!
//! This module provides utilities for setting up test environments.

use crate::TestFixture;
use common_errors::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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

/// A utility for setting up test environments
pub struct TestEnvironment {
    /// The test fixture
    pub fixture: TestFixture,
    /// The base directory for the test
    pub base_dir: PathBuf,
    /// The content directory
    pub content_dir: PathBuf,
    /// The config directory
    pub config_dir: PathBuf,
}

impl TestEnvironment {
    /// Create a new test environment with the given configuration
    pub fn new(config: Option<TestEnvironmentConfig>) -> Result<Self> {
        let fixture = TestFixture::new()?;
        let config = config.unwrap_or_default();

        // Create the base directory structure
        let base_dir = fixture.temp_dir.path().to_path_buf();
        let content_dir = base_dir.join("content");
        let config_dir = base_dir.join("config");

        fixture.fs.create_directory(&content_dir);
        fixture.fs.create_directory(&config_dir);

        // Create content directories
        for dir in &config.content_dirs {
            let dir_path = content_dir.join(dir);
            fixture.fs.create_directory(&dir_path);
        }

        // Create config directories
        for dir in &config.config_dirs {
            let dir_path = config_dir.join(dir);
            fixture.fs.create_directory(&dir_path);
        }

        // Create files
        for (path, content) in &config.files {
            let file_path = base_dir.join(path);
            if let Some(parent) = file_path.parent() {
                fixture.fs.create_directory(parent);
            }
            fixture.fs.write_file(&file_path, content.clone())?;
        }

        // Set up default config if requested
        if config.setup_default_config {
            fixture.register_test_config();
        }

        Ok(Self {
            fixture,
            base_dir,
            content_dir,
            config_dir,
        })
    }

    /// Create a content file
    pub fn create_content_file(&self, topic: &str, slug: &str, title: &str, content: &str, is_draft: bool) -> Result<PathBuf> {
        let path = self.fixture.create_content(topic, slug, title, is_draft)?;
        let content_path = self.content_dir.join(topic).join(format!("{}.md", slug));
        self.fixture.fs.write_file(&content_path, content.to_string())?;
        Ok(content_path)
    }

    /// Create a topic configuration
    pub fn create_topic_config(&self, key: &str, name: &str, description: &str) -> Result<PathBuf> {
        let topic_config = format!(r#"name: "{}"
description: "{}"
directory: "{}""#, name, description, key);

        let path = self.config_dir.join("topics").join(format!("{}.yaml", key));
        self.fixture.fs.write_file(&path, topic_config)?;
        Ok(path)
    }

    /// Create multiple test articles
    pub fn create_test_articles(&self, count: usize, topic: &str) -> Result<Vec<PathBuf>> {
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
}

/// Run a test with a test environment
pub fn with_test_environment<F, T>(f: F) -> T
where
    F: FnOnce(&TestEnvironment) -> T
{
    let environment = TestEnvironment::new(None).expect("Failed to create test environment");
    f(&environment)
}

/// Run a test with a custom test environment
pub fn with_custom_test_environment<F, T>(config: TestEnvironmentConfig, f: F) -> T
where
    F: FnOnce(&TestEnvironment) -> T
{
    let environment = TestEnvironment::new(Some(config)).expect("Failed to create test environment");
    f(&environment)
}