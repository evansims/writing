//! # Common Test Utilities
//! 
//! This module provides common test utilities for the writing tools.
//! 
//! ## Features
//! 
//! - Test fixture creation with temporary directories
//! - Configuration generation for tests
//! - Content file creation for tests
//! 
//! ## Example
//! 
//! ```rust
//! use common_test_utils::TestFixture;
//! 
//! #[test]
//! fn test_something() {
//!     let fixture = TestFixture::new().unwrap();
//!     
//!     // Create test content
//!     let content_file = fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
//!     
//!     // Test something with the content
//!     assert!(content_file.exists());
//!     
//!     // The fixture will be cleaned up automatically when it goes out of scope
//! }

use common_errors::Result;
use common_models::{Config, ContentConfig, PublicationConfig, TopicConfig, ImageConfig, ImageSize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};

/// A test fixture with a temporary directory and configuration
pub struct TestFixture {
    pub temp_dir: TempDir,
    pub config_path: PathBuf,
    pub content_dir: PathBuf,
}

impl TestFixture {
    /// Create a new test fixture with default configuration
    pub fn new() -> Result<Self> {
        let temp_dir = tempdir()?;
        let config_path = temp_dir.path().join("config.yaml");
        let content_dir = temp_dir.path().join("content");
        let fixture = Self {
            temp_dir,
            config_path,
            content_dir,
        };
        
        fixture.create_default_config()?;
        fs::create_dir_all(&fixture.content_dir)?;
        
        Ok(fixture)
    }
    
    /// Create default configuration
    pub fn create_default_config(&self) -> Result<()> {
        // Create topics
        let mut topics = HashMap::new();
        topics.insert(
            "blog".to_string(),
            TopicConfig {
                name: "Blog".to_string(),
                description: "Blog posts".to_string(),
                path: "blog".to_string(),
            },
        );
        topics.insert(
            "notes".to_string(),
            TopicConfig {
                name: "Notes".to_string(),
                description: "Short notes".to_string(),
                path: "notes".to_string(),
            },
        );
        
        // Create image sizes
        let mut sizes = HashMap::new();
        sizes.insert(
            "small".to_string(),
            ImageSize {
                width: 480,
                height: 320,
                description: "Small image".to_string(),
            },
        );
        sizes.insert(
            "medium".to_string(),
            ImageSize {
                width: 800,
                height: 600,
                description: "Medium image".to_string(),
            },
        );
        
        // Create config
        let config = Config {
            content: ContentConfig {
                base_dir: self.content_dir.to_string_lossy().to_string(),
                topics,
                tags: None,
            },
            images: ImageConfig {
                formats: vec!["webp".to_string(), "jpg".to_string()],
                format_descriptions: None,
                sizes,
                naming: None,
                quality: None,
            },
            publication: PublicationConfig {
                author: "Test Author".to_string(),
                copyright: "Test Copyright".to_string(),
                site: Some("https://example.com".to_string()),
            },
        };
        
        let config_yaml = serde_yaml::to_string(&config)?;
        fs::write(&self.config_path, config_yaml)?;
        
        Ok(())
    }
    
    /// Create a test content file
    pub fn create_content(&self, topic: &str, slug: &str, title: &str, is_draft: bool) -> Result<PathBuf> {
        let topic_dir = self.content_dir.join(topic);
        let content_dir = topic_dir.join(slug);
        let content_file = content_dir.join("index.mdx");
        
        fs::create_dir_all(&content_dir)?;
        
        let frontmatter = format!(
            r#"---
title: "{}"
date: "2023-01-01"
draft: {}
---
"#,
            title, is_draft
        );
        
        let content = format!("{}\n\n# {}\n\nThis is test content.", frontmatter, title);
        fs::write(&content_file, content)?;
        
        Ok(content_file)
    }
    
    /// Get the path to this test fixture
    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }
    
    /// Create a subdirectory in the test fixture
    pub fn create_dir(&self, path: &str) -> Result<PathBuf> {
        let dir_path = self.temp_dir.path().join(path);
        fs::create_dir_all(&dir_path)?;
        Ok(dir_path)
    }
    
    /// Create a file in the test fixture
    pub fn create_file(&self, path: &str, content: &str) -> Result<PathBuf> {
        let file_path = self.temp_dir.path().join(path);
        
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(&file_path, content)?;
        Ok(file_path)
    }
} 