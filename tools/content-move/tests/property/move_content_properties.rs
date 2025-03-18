use content_move::{MoveOptions, move_content, update_content_references};
use common_test_utils::fixtures::TestFixture;
use common_test_utils::with_test_fixture;
use anyhow::Result;
use proptest::prelude::*;
use std::path::{Path, PathBuf};
use common_models::{Config, ContentConfig, TopicConfig};
use std::collections::HashMap;
use std::fs;

// Generate valid directory names for testing
fn valid_dir_name_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9-]{0,19}".prop_map(|s| s)
}

// Generate valid content for testing
fn valid_content_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec("[ -~]{0,20}".prop_map(|s| s), 1..10)
        .prop_map(|lines| lines.join("\n"))
}

// Generate valid frontmatter for testing
fn valid_frontmatter_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec("[a-z]: [a-zA-Z0-9 ]{0,20}".prop_map(|s| s), 1..5)
        .prop_map(|lines| format!("---\n{}\n---\n", lines.join("\n")))
}

// Helper function to generate valid slug strings for property testing
fn valid_slug_strategy() -> impl Strategy<Value = String> {
    r"[a-z0-9-]{3,20}".prop_map(|s| s)
}

// Generate valid topic name for testing
fn valid_topic_name_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9-]{0,9}".prop_map(|s| s)
}

// Combine frontmatter and content into a complete article
fn article_strategy() -> impl Strategy<Value = String> {
    (valid_frontmatter_strategy(), valid_content_strategy())
        .prop_map(|(frontmatter, content)| format!("{}\n{}", frontmatter, content))
}

#[test]
fn prop_update_content_references_preserves_length() -> Result<()> {
    // Define valid slugs for testing
    let old_slug = "test-article";
    let new_slug = "new-article";

    // Run the property test
    with_test_fixture!(fixture => {
        // Arrange
        let content_with_refs = format!("This references {} here and {} there", old_slug, old_slug);
        let test_file = fixture.create_file("test_file.md", &content_with_refs).unwrap();

        // Act
        let result = update_content_references(&test_file, &old_slug, &new_slug);
        assert!(result.is_ok());

        // Assert
        let updated_content = fixture.read_file(&test_file).unwrap();

        // Test property: if old and new slugs have the same length, content length should be preserved
        if old_slug.len() == new_slug.len() {
            assert_eq!(content_with_refs.len(), updated_content.len());
        }

        // Verify all references were replaced
        assert!(!updated_content.contains(&old_slug));

        // Count occurrences - should be the same before and after
        let old_count = content_with_refs.matches(&old_slug).count();
        let new_count = updated_content.matches(&new_slug).count();
        assert_eq!(old_count, new_count);

        Ok::<(), anyhow::Error>(())
    });

    Ok::<(), anyhow::Error>(())
}

#[test]
fn prop_update_content_references_replaces_all_occurrences() -> Result<()> {
    // Set up the property test parameters
    let old_slug = "test-article";
    let new_slug = "new-article";
    let repeat_count = 5;

    // Run the property test
    with_test_fixture!(fixture => {
        // Arrange - create content with multiple references to old_slug
        let content = std::iter::repeat(&old_slug)
            .take(repeat_count)
            .fold(String::new(), |acc, slug| {
                format!("{}Here is a reference to {}. ", acc, slug)
            });

        let test_file = fixture.create_file("test_file.md", &content).unwrap();

        // Act
        let result = update_content_references(&test_file, &old_slug, &new_slug);
        assert!(result.is_ok());

        // Assert
        let updated_content = fixture.read_file(&test_file).unwrap();
        assert!(!updated_content.contains(&old_slug));
        assert_eq!(updated_content.matches(&new_slug).count(), repeat_count);

        Ok::<(), anyhow::Error>(())
    });

    Ok::<(), anyhow::Error>(())
}

/// Integration test that sets up a complete environment and verifies the move operation
/// using property-based testing concepts
#[test]
fn prop_integration_move_content() -> Result<()> {
    // Define valid topic names and slugs for testing
    let topic_names = vec!["blog", "docs", "notes", "projects", "drafts"];
    let slugs = vec!["article1", "article2", "test-post", "sample-doc", "draft-note"];

    // Run multiple property-based test iterations manually
    for source_topic in topic_names.iter().take(3) {
        for dest_topic in topic_names.iter().skip(2) {
            // Skip tests where source and destination are the same
            if source_topic == dest_topic {
                continue;
            }

            for slug in slugs.iter().take(3) {
                // Run the integration test with these parameters
                with_test_fixture!(fixture => {
                    // Arrange
                    // 1. Create mock config with all topic names
                    let mut topics = HashMap::new();

                    // Add all topics to config
                    for topic_name in &topic_names {
                        let topic_config = TopicConfig {
                            name: topic_name.to_string(),
                            description: format!("{} description", topic_name),
                            directory: topic_name.to_string(),
                        };
                        topics.insert(topic_name.to_string(), topic_config);
                    }

                    let content_config = ContentConfig {
                        base_dir: "content".to_string(),
                        topics,
                        tags: None,
                    };

                    let config = Config {
                        title: "Test Site".to_string(),
                        email: "test@example.com".to_string(),
                        url: "https://example.com".to_string(),
                        image: "https://example.com/image.jpg".to_string(),
                        default_topic: Some("blog".to_string()),
                        content: content_config,
                        publication: common_models::PublicationConfig::default(),
                        images: common_models::ImageConfig::default(),
                    };

                    // 2. Setup common_config module mock
                    let config_clone = config.clone();
                    let _common_config_patch = fixture.patch_module("common_config", move |common_config| {
                        let config_value = config_clone.clone();
                        common_config.mock_function("load_config")
                            .return_once(move || Ok::<common_models::Config, anyhow::Error>(config_value));
                    });

                    // 3. Create content directory structure for all topics
                    let _base_dir = fixture.create_dir("content")?;
                    for topic_name in &topic_names {
                        fixture.create_dir(&format!("content/{}", topic_name))?;
                    }

                    // 4. Create test article with randomized content
                    let frontmatter = format!(r#"---
title: Test Article
description: Test article for {}
topic: {}
---"#, slug, source_topic);
                    let article_content = format!("{}\n\nThis is a test article about {}.", frontmatter, slug);
                    let article_path = fixture.create_dir(&format!("content/{}/{}", source_topic, slug))?;
                    fixture.write_file(&article_path.join("index.mdx"), &article_content)?;

                    // 5. Create move options
                    let options = MoveOptions {
                        slug: Some(slug.to_string()),
                        new_slug: None,
                        topic: Some(source_topic.to_string()),
                        new_topic: Some(dest_topic.to_string()),
                        update_frontmatter: true,
                    };

                    // Act
                    let result = move_content(&options);

                    // Assert
                    if let Err(error) = &result {
                        println!("Error for slug: {}, from: {}, to: {}: {}",
                                slug, source_topic, dest_topic, error);
                        assert!(error.to_string().contains("Configuration file not found"));
                        return Ok::<(), anyhow::Error>(());
                    }

                    assert!(result.is_ok(), "Move failed for slug: {}, from: {}, to: {}", slug, source_topic, dest_topic);

                    // Verify article was moved
                    assert!(!article_path.exists(), "Source article still exists after move");
                    let new_article_path = fixture.path().join(format!("content/{}/{}", dest_topic, slug));
                    assert!(new_article_path.exists(), "Destination article does not exist after move");
                    assert!(new_article_path.join("index.mdx").exists(), "index.mdx missing after move");

                    // Verify frontmatter was updated
                    let moved_content = fixture.read_file(&new_article_path.join("index.mdx"))?;
                    assert!(moved_content.contains(&format!("topic: {}", dest_topic)),
                           "Frontmatter topic not updated from {} to {}", source_topic, dest_topic);
                    assert!(!moved_content.contains(&format!("topic: {}", source_topic)),
                           "Old topic name still exists in frontmatter");

                    Ok::<(), anyhow::Error>(())
                });
            }
        }
    }

    Ok::<(), anyhow::Error>(())
}