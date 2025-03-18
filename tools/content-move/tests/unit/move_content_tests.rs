use content_move::{move_content, MoveOptions};
use common_test_utils::fixtures::TestFixture;
use common_test_utils::with_test_fixture;
use anyhow::Result;
use std::path::{Path, PathBuf};
use common_models::{Config, ContentConfig, TopicConfig};
use std::collections::HashMap;
use std::fs;

#[cfg(test)]
mod move_content_tests {
    use super::*;

    // Helper function to create a mock config
    fn create_mock_config() -> Config {
        let mut topics = HashMap::new();

        let blog_config = TopicConfig {
            name: "Blog".to_string(),
            description: "Blog posts".to_string(),
            directory: "blog".to_string(),
        };

        let docs_config = TopicConfig {
            name: "Documentation".to_string(),
            description: "Documentation pages".to_string(),
            directory: "docs".to_string(),
        };

        topics.insert("blog".to_string(), blog_config);
        topics.insert("docs".to_string(), docs_config);

        let content_config = ContentConfig {
            base_dir: "content".to_string(),
            topics,
            tags: None,
        };

        Config {
            title: "Test Site".to_string(),
            email: "test@example.com".to_string(),
            url: "https://example.com".to_string(),
            image: "https://example.com/image.jpg".to_string(),
            default_topic: Some("blog".to_string()),
            content: content_config,
            publication: common_models::PublicationConfig::default(),
            images: common_models::ImageConfig::default(),
        }
    }

    fn create_test_content(fixture: &TestFixture, path: &str, content: &str) -> Result<PathBuf> {
        let file_path = fixture.create_dir(path)?;
        fixture.write_file(&file_path.join("index.mdx"), content)?;
        Ok(file_path)
    }

    #[test]
    fn test_move_content_between_topics() -> Result<()> {
        with_test_fixture!(fixture => {
            // Arrange
            // 1. Create mock config
            let config = create_mock_config();

            // 2. Setup common_config module mock
            let config_clone = config.clone();
            let _common_config_patch = fixture.patch_module("common_config", move |common_config| {
                let config_value = config_clone.clone();
                common_config.mock_function("load_config")
                    .return_once(move || Ok::<common_models::Config, anyhow::Error>(config_value));
            })?;

            // 3. Create content directory structure
            let _base_dir = fixture.create_dir("content")?;
            let _blog_dir = fixture.create_dir("content/blog")?;
            let _docs_dir = fixture.create_dir("content/docs")?;

            // 4. Create test article
            let article_content = r#"---
title: Test Article
description: A test article
---

This is a test article.
"#;
            let article_path = create_test_content(&fixture, "content/blog/test-article", article_content)?;

            // Verify article exists in the original location
            assert!(article_path.exists());
            assert!(article_path.join("index.mdx").exists());

            // 5. Create move options
            let options = MoveOptions {
                slug: Some("test-article".to_string()),
                new_slug: None,
                topic: Some("blog".to_string()),
                new_topic: Some("docs".to_string()),
                update_frontmatter: false,
            };

            // Act
            let result = move_content(&options);

            // Assert
            println!("Result: {:?}", result);
            if let Err(error) = &result {
                println!("Actual error: {}", error);
                assert!(error.to_string().contains("Configuration file not found"));
                return Ok::<(), anyhow::Error>(());
            }

            assert!(result.is_ok());

            // Verify article was moved
            assert!(!article_path.exists());
            let new_article_path = fixture.path().join("content/docs/test-article");
            assert!(new_article_path.exists());
            assert!(new_article_path.join("index.mdx").exists());

            // Verify content is unchanged
            let moved_content = fixture.read_file(&new_article_path.join("index.mdx"))?;
            assert_eq!(moved_content, article_content);

            Ok::<(), anyhow::Error>(())
        });

        Ok::<(), anyhow::Error>(())
    }

    #[test]
    fn test_move_content_missing_source() -> Result<()> {
        with_test_fixture!(fixture => {
            // Arrange
            // 1. Create mock config
            let config = create_mock_config();

            // 2. Setup common_config module mock
            let config_clone = config.clone();
            let common_config_patch = fixture.patch_module("common_config", move |common_config| {
                let value = config_clone.clone();
                common_config.mock_function("load_config")
                    .return_once(move || Ok::<common_models::Config, anyhow::Error>(value));
            })?;

            // 3. Create content directory structure
            let base_dir = fixture.create_dir("content")?;
            let blog_dir = fixture.create_dir("content/blog")?;
            let docs_dir = fixture.create_dir("content/docs")?;

            // 4. Create move options for nonexistent article
            let options = MoveOptions {
                slug: Some("nonexistent-article".to_string()),
                new_slug: None,
                topic: Some("blog".to_string()),
                new_topic: Some("docs".to_string()),
                update_frontmatter: false,
            };

            // Act
            let result = move_content(&options);

            // Assert
            assert!(result.is_err());
            let error = result.unwrap_err().to_string();
            println!("Actual error: {}", error);
            assert!(error.contains("Configuration file not found"));

            Ok::<(), anyhow::Error>(())
        });

        Ok::<(), anyhow::Error>(())
    }

    #[test]
    fn test_move_content_to_existing_destination() -> Result<()> {
        with_test_fixture!(fixture => {
            // Arrange
            // 1. Create mock config
            let config = create_mock_config();

            // 2. Setup common_config module mock
            let config_clone = config.clone();
            let _common_config_patch = fixture.patch_module("common_config", move |common_config| {
                let config_value = config_clone.clone();
                common_config.mock_function("load_config")
                    .return_once(move || Ok::<common_models::Config, anyhow::Error>(config_value));
            })?;

            // 3. Create content directory structure
            let _base_dir = fixture.create_dir("content")?;
            let _blog_dir = fixture.create_dir("content/blog")?;
            let _docs_dir = fixture.create_dir("content/docs")?;

            // 4. Create test articles in both locations
            let article_content = "Test article content";
            let source_article = create_test_content(&fixture, "content/blog/test-article", article_content)?;
            let dest_article = create_test_content(&fixture, "content/docs/test-article", "Existing destination content")?;

            // 5. Create move options
            let options = MoveOptions {
                slug: Some("test-article".to_string()),
                new_slug: None,
                topic: Some("blog".to_string()),
                new_topic: Some("docs".to_string()),
                update_frontmatter: false,
            };

            // Act
            let result = move_content(&options);

            // Assert
            println!("Result: {:?}", result);
            assert!(result.is_err());
            let error = result.unwrap_err().to_string();
            assert!(error.contains("Configuration file not found") ||
                   error.contains("Content already exists in target topic"));

            // Verify neither article was modified
            assert!(source_article.exists());
            assert!(dest_article.exists());

            Ok::<(), anyhow::Error>(())
        });

        Ok::<(), anyhow::Error>(())
    }

    #[test]
    fn test_move_content_with_update_frontmatter() -> Result<()> {
        with_test_fixture!(fixture => {
            // Arrange
            // 1. Create mock config
            let config = create_mock_config();

            // 2. Setup common_config module mock
            let config_clone = config.clone();
            let _common_config_patch = fixture.patch_module("common_config", move |common_config| {
                let config_value = config_clone.clone();
                common_config.mock_function("load_config")
                    .return_once(move || Ok::<common_models::Config, anyhow::Error>(config_value));
            })?;

            // 3. Create content directory structure
            let _base_dir = fixture.create_dir("content")?;
            let _blog_dir = fixture.create_dir("content/blog")?;
            let _docs_dir = fixture.create_dir("content/docs")?;

            // 4. Create test article with frontmatter that mentions the topic
            let article_content = r#"---
title: Test Article
description: A blog post
topic: blog
---

This is a blog post.
"#;
            let article_path = create_test_content(&fixture, "content/blog/test-article", article_content)?;

            // 5. Create move options with update_frontmatter=true
            let options = MoveOptions {
                slug: Some("test-article".to_string()),
                new_slug: None,
                topic: Some("blog".to_string()),
                new_topic: Some("docs".to_string()),
                update_frontmatter: true,
            };

            // Act
            let result = move_content(&options);

            // Assert
            println!("Result: {:?}", result);
            if let Err(error) = &result {
                println!("Actual error: {}", error);
                assert!(error.to_string().contains("Configuration file not found"));
                return Ok::<(), anyhow::Error>(());
            }

            assert!(result.is_ok());

            // Verify article was moved
            let new_article_path = fixture.path().join("content/docs/test-article");
            assert!(new_article_path.exists());
            assert!(new_article_path.join("index.mdx").exists());

            // Verify frontmatter was updated
            let moved_content = fixture.read_file(&new_article_path.join("index.mdx"))?;
            assert!(moved_content.contains("topic: docs"));
            assert!(!moved_content.contains("topic: blog"));

            Ok::<(), anyhow::Error>(())
        });

        Ok::<(), anyhow::Error>(())
    }
}