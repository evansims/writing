//! # Property-Based Testing Utilities
//!
//! This module provides utilities for property-based testing using proptest.
//!
//! ## Features
//!
//! - Generators for common domain types
//! - Strategies for testing with realistic data
//! - Helpers for generating test scenarios
//!
//! ## Example
//!
//! ```rust
//! use common_test_utils::proptest::*;
//! use proptest::prelude::*;
//!
//! proptest! {
//!     #[test]
//!     fn test_slug_validation(slug in valid_slug_strategy()) {
//!         // Valid slugs should pass validation
//!         assert!(validate_slug(&slug).is_ok());
//!     }
//! }
//! ```

use common_models::{Frontmatter, Article, TopicConfig, Config};
use proptest::prelude::*;
use proptest::strategy::Just;
use std::path::PathBuf;
use std::collections::HashMap;

/// Generate a valid slug
pub fn valid_slug_strategy() -> impl Strategy<Value = String> {
    // Slugs typically consist of lowercase letters, numbers, and hyphens
    // They must start with a letter or number and end with a letter or number
    // They cannot contain consecutive hyphens

    // Generate segments of lowercase letters and numbers
    let segment_strategy = prop::string::string_regex("[a-z0-9]{1,10}").unwrap();

    // Generate 1-5 segments separated by single hyphens
    prop::collection::vec(segment_strategy, 1..5)
        .prop_map(|segments| {
            segments.join("-")
        })
        .prop_filter("Slug must not be empty", |s| !s.is_empty())
        .prop_filter("Slug must not start with hyphen", |s| !s.starts_with('-'))
        .prop_filter("Slug must not end with hyphen", |s| !s.ends_with('-'))
        .prop_filter("Slug must not contain consecutive hyphens", |s| !s.contains("--"))
        .prop_filter("Slug must not be too long", |s| s.len() <= 100)
}

/// Generate an invalid slug
pub fn invalid_slug_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        // Empty slug
        Just("".to_string()),
        // Slug with spaces
        prop::string::string_regex("[a-z0-9]+ [a-z0-9]+").unwrap(),
        // Slug with uppercase letters
        prop::string::string_regex("[A-Z][a-z0-9]+").unwrap(),
        // Slug with special characters
        prop::string::string_regex("[a-z0-9]*[!@#$%^&*()][a-z0-9]*").unwrap(),
        // Slug with consecutive hyphens
        Just("test--slug".to_string()),
        // Slug starting with hyphen
        Just("-test-slug".to_string()),
        // Slug ending with hyphen
        Just("test-slug-".to_string()),
        // Very long slug (exactly 101 characters)
        Just("a".repeat(101)),
        // Very long slug (exactly 105 characters)
        Just("a".repeat(105))
    ]
}

/// Generate a valid title
pub fn valid_title_strategy() -> impl Strategy<Value = String> {
    // Titles can have a wide variety of characters, but should have reasonable length
    prop::string::string_regex("[A-Za-z0-9 ,.!?:;-]{5,100}").unwrap()
}

/// Generate an invalid title
pub fn invalid_title_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        // Empty title
        Just("".to_string()),
        // Title with only whitespace
        prop::string::string_regex("\\s+").unwrap(),
        // Very long title (beyond typical limits)
        prop::collection::vec(prop::char::range('a', 'z'), 1000..2000).prop_map(|v| v.into_iter().collect())
    ]
}

/// Generate a valid date string (YYYY-MM-DD)
pub fn valid_date_strategy() -> impl Strategy<Value = String> {
    // Year between 2000 and current year, month 1-12, day 1-28 (to avoid edge cases)
    (2000..2024u32, 1..13u32, 1..29u32).prop_map(|(y, m, d)| format!("{:04}-{:02}-{:02}", y, m, d))
}

/// Generate an invalid date string
pub fn invalid_date_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        // Empty date
        Just("".to_string()),
        // Invalid format
        Just("2020/01/01".to_string()),
        // Invalid values
        Just("2020-13-01".to_string()),
        Just("2020-01-32".to_string()),
        // Malformed
        Just("20-1-1".to_string()),
        // Non-numeric
        Just("abcd-ef-gh".to_string())
    ]
}

/// Generate a valid tag string
pub fn valid_tags_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(valid_tag_strategy(), 0..10)
        .prop_map(|tags| tags.join(", "))
}

/// Generate a valid single tag
pub fn valid_tag_strategy() -> impl Strategy<Value = String> {
    prop::string::string_regex("[a-z][a-z0-9-]{2,30}").unwrap()
}

/// Generate a valid topic key
pub fn valid_topic_key_strategy() -> impl Strategy<Value = String> {
    prop::string::string_regex("[a-z][a-z0-9-]{2,20}").unwrap()
}

/// Generate a valid path for testing
pub fn valid_path_strategy() -> impl Strategy<Value = PathBuf> {
    prop::collection::vec(valid_slug_strategy(), 1..5)
        .prop_map(|parts| parts.into_iter().collect::<Vec<_>>().join("/"))
        .prop_map(PathBuf::from)
}

/// Generate a valid frontmatter strategy
pub fn valid_frontmatter_strategy() -> impl Strategy<Value = Frontmatter> {
    (
        valid_title_strategy(),
        valid_title_strategy().prop_map(Some), // tagline is optional
        valid_date_strategy(),
        valid_slug_strategy(),
        valid_tags_strategy().prop_map(|t| Some(vec![t])), // tags are optional
        valid_topic_key_strategy().prop_map(|t| Some(vec![t])), // topic is optional
        prop::bool::ANY,
    ).prop_map(|(title, tagline, published, slug, tags, topics, draft)| {
        Frontmatter {
            title,
            tagline,
            published_at: Some(published.clone()),
            updated_at: Some(published),
            slug: Some(slug),
            tags,
            topics,
            featured_image_path: None,
            is_draft: Some(draft),
        }
    })
}

/// Generate a valid article strategy
pub fn valid_article_strategy() -> impl Strategy<Value = Article> {
    (
        valid_frontmatter_strategy(),
        prop::string::string_regex("[\\s\\S]{10,1000}").unwrap(),
        valid_path_strategy(),
    ).prop_map(|(frontmatter, content, path)| {
        let slug = frontmatter.slug.clone().unwrap_or_default();
        let topics = frontmatter.topics.clone();
        let topic = topics.as_ref().and_then(|t| t.first().cloned()).unwrap_or_default();

        Article {
            frontmatter,
            content,
            path: path.to_string_lossy().to_string(),
            slug,
            reading_time: Some(2),
            word_count: Some(100),
            topic,
        }
    })
}

/// Generate a valid TopicConfig
pub fn valid_topic_config_strategy() -> impl Strategy<Value = TopicConfig> {
    (
        valid_title_strategy(),
        valid_title_strategy(),
        valid_slug_strategy()
    ).prop_map(|(name, description, directory)| {
        TopicConfig {
            name,
            description,
            directory,
        }
    })
}

/// Generate a complex valid article with nested data structures
pub fn complex_article_strategy() -> impl Strategy<Value = Article> {
    let metadata_strategy = proptest::collection::hash_map(
        any::<String>(),
        any::<String>(),
        0..5
    );

    (
        valid_frontmatter_strategy(),
        prop::string::string_regex("[\\s\\S]{10,1000}").unwrap(),
        valid_path_strategy(),
        proptest::option::of(prop::num::u32::ANY),  // word_count
        proptest::option::of(prop::num::u32::ANY),  // reading_time
        metadata_strategy,
    ).prop_map(|(frontmatter, content, path, word_count, reading_time, metadata)| {
        let slug = frontmatter.slug.clone().unwrap_or_default();
        let topics = frontmatter.topics.clone();
        let topic = topics.as_ref().and_then(|t| t.first().cloned()).unwrap_or_default();

        let mut article = Article {
            frontmatter,
            content,
            path: path.to_string_lossy().to_string(),
            slug,
            reading_time: reading_time.map(|rt| rt % 60), // Keep within reasonable bounds
            word_count: word_count.map(|wc| wc as usize), // Convert from u32 to usize
            topic,
        };

        // Add any additional metadata
        if !metadata.is_empty() {
            // We would store metadata here if the model supported it
        }

        article
    })
}

/// Generate a valid Configuration strategy
pub fn valid_config_strategy() -> impl Strategy<Value = Config> {
    let default_topic = valid_topic_key_strategy();
    let topics = topic_config_map_strategy(1, 5);

    (default_topic, topics).prop_map(|(default_topic, topics)| {
        let mut config = Config::default();
        config.title = "Test Site".to_string();
        config.email = "test@example.com".to_string();
        config.url = "https://example.com".to_string();
        config.image = "logo.png".to_string();
        config.default_topic = Some(default_topic);
        config.content.topics = topics;
        config.content.base_dir = "content".to_string();
        config
    })
}

/// Generate a map of topic configs
pub fn topic_config_map_strategy(min: usize, max: usize) -> impl Strategy<Value = HashMap<String, TopicConfig>> {
    proptest::collection::vec((valid_topic_key_strategy(), valid_topic_config_strategy()), min..max)
        .prop_map(|vec| {
            vec.into_iter().collect()
        })
}

/// Generate a valid file path with a specified extension
pub fn valid_file_path_strategy(extension: &'static str) -> impl Strategy<Value = PathBuf> {
    valid_path_strategy()
        .prop_map(move |mut path| {
            path.set_extension(extension);
            path
        })
}

/// Generate a valid directory path for testing
pub fn valid_directory_path_strategy() -> impl Strategy<Value = PathBuf> {
    prop::collection::vec(valid_slug_strategy(), 1..5)
        .prop_map(|parts| parts.into_iter().collect::<Vec<_>>().join("/"))
        .prop_map(PathBuf::from)
}

/// Generate a valid content directory structure strategy
pub fn valid_content_directory_strategy() -> impl Strategy<Value = (PathBuf, Vec<PathBuf>)> {
    (
        valid_directory_path_strategy(),
        proptest::collection::vec(valid_slug_strategy(), 1..10),
    ).prop_map(|(base, slugs)| {
        let files = slugs.into_iter()
            .map(|slug| base.join(format!("{}.md", slug)))
            .collect();
        (base, files)
    })
}

/// Generate a structured set of test data for integration testing
pub fn test_scenario_strategy() -> impl Strategy<Value = TestScenario> {
    (
        valid_config_strategy(),
        topic_config_map_strategy(1, 5),
        proptest::collection::vec(complex_article_strategy(), 1..10),
    ).prop_map(|(config, topics, articles)| {
        TestScenario {
            config,
            topics,
            articles,
        }
    })
}

/// Test scenario with a complete test environment
///
/// This struct contains a complete test environment with a configuration,
/// topics, and articles.
#[derive(Debug)]
pub struct TestScenario {
    pub config: Config,
    pub topics: HashMap<String, TopicConfig>,
    pub articles: Vec<Article>,
}

impl TestScenario {
    /// Get articles that belong to a specific topic
    pub fn articles_for_topic(&self, topic: &str) -> Vec<&Article> {
        self.articles.iter()
            .filter(|a| a.topic == topic)
            .collect()
    }

    /// Get published articles (not drafts)
    pub fn published_articles(&self) -> Vec<&Article> {
        self.articles.iter()
            .filter(|a| !a.frontmatter.is_draft.unwrap_or(false))
            .collect()
    }

    /// Get draft articles
    pub fn draft_articles(&self) -> Vec<&Article> {
        self.articles.iter()
            .filter(|a| a.frontmatter.is_draft.unwrap_or(false))
            .collect()
    }
}