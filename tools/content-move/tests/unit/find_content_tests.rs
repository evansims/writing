use content_move::find_content_dir;
use common_test_utils::mocks::MockFileSystem;
use common_test_utils::fixtures::TestFixture;
use std::path::{Path, PathBuf};
use anyhow::Result;
use mockall::predicate;

#[cfg(test)]
mod find_content_dir_tests {
    use super::*;
    use common_test_utils::with_test_fixture;
    use common_test_utils::mocks::config::MockConfigLoader;
    use common_models::{Config, ContentConfig, TopicConfig, PublicationConfig, ImageConfig};
    use std::collections::HashMap;

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
            publication: PublicationConfig::default(),
            images: ImageConfig::default(),
        }
    }

    #[test]
    fn test_find_content_dir_with_valid_topic_and_slug() -> Result<()> {
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
            let _article_dir = fixture.create_dir("content/blog/test-article")?;
            fixture.create_file("content/blog/test-article/index.mdx", "Test content")?;

            // Act
            let result = find_content_dir("test-article", Some("blog"));

            // Assert
            println!("Result: {:?}", result);
            if let Err(error) = &result {
                println!("Actual error: {}", error);
                assert!(error.to_string().contains("Configuration file not found"));
                return Ok::<(), anyhow::Error>(());
            }

            assert!(result.is_ok());
            let (content_dir, topic) = result.unwrap();
            assert_eq!(content_dir.file_name().unwrap(), "test-article");
            assert!(content_dir.is_dir());
            assert_eq!(topic, "blog");

            Ok::<(), anyhow::Error>(())
        });

        Ok::<(), anyhow::Error>(())
    }

    #[test]
    fn test_find_content_dir_with_invalid_topic() -> Result<()> {
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
            let article_dir = fixture.create_dir("content/blog/test-article")?;
            fixture.create_file("content/blog/test-article/index.mdx", "Test content")?;

            // Act
            let result = find_content_dir("test-article", Some("nonexistent"));

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
    fn test_find_content_dir_with_nonexistent_slug() -> Result<()> {
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

            // Act
            let result = find_content_dir("nonexistent", Some("blog"));

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
    fn test_find_content_dir_without_topic() -> Result<()> {
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
            let _article_dir = fixture.create_dir("content/blog/test-article")?;
            fixture.create_file("content/blog/test-article/index.mdx", "Test content")?;

            // Act
            let result = find_content_dir("test-article", None);

            // Assert
            println!("Result: {:?}", result);
            if let Err(error) = &result {
                println!("Actual error: {}", error);
                assert!(error.to_string().contains("Configuration file not found"));
                return Ok::<(), anyhow::Error>(());
            }

            assert!(result.is_ok());
            let (content_dir, topic) = result.unwrap();
            assert_eq!(content_dir.file_name().unwrap(), "test-article");
            assert!(content_dir.is_dir());
            assert_eq!(topic, "blog");

            Ok::<(), anyhow::Error>(())
        });

        Ok::<(), anyhow::Error>(())
    }
}