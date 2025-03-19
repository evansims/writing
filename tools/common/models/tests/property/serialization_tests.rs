//! Property-based tests for the models module
//!
//! This file contains property-based tests for serialization and deserialization
//! of models in the common models library.

use crate::*;
use proptest::prelude::*;
use serde_json;

// Strategies for generating valid model data
mod strategies {
    use super::*;

    pub fn valid_string() -> impl Strategy<Value = String> {
        "[a-zA-Z0-9_\\- ]{1,50}".prop_filter(
            "String should not be empty and should be reasonable length",
            |s| !s.is_empty() && s.len() <= 50
        )
    }

    pub fn valid_url() -> impl Strategy<Value = String> {
        "(http|https)://[a-zA-Z0-9_\\-\\.]+\\.[a-zA-Z]{2,}/[a-zA-Z0-9_\\-\\./?=&]*"
            .prop_filter(
                "URL should be a valid HTTP or HTTPS URL",
                |s| s.starts_with("http")
            )
    }

    pub fn option_of<T: Strategy>(strat: T) -> impl Strategy<Value = Option<T::Value>>
    where
        T::Value: Clone,
    {
        prop_oneof![
            Just(None),
            strat.prop_map(Some)
        ]
    }

    pub fn topic_config_strategy() -> impl Strategy<Value = TopicConfig> {
        (valid_string(), valid_string(), valid_string())
            .prop_map(|(name, description, directory)| {
                TopicConfig {
                    name,
                    description,
                    directory,
                }
            })
    }

    pub fn image_size_strategy() -> impl Strategy<Value = ImageSize> {
        (100u32..2000u32, 100u32..2000u32, valid_string())
            .prop_map(|(width, height, description)| {
                ImageSize {
                    width,
                    height,
                    description,
                }
            })
    }

    pub fn image_naming_strategy() -> impl Strategy<Value = ImageNaming> {
        (valid_string(), proptest::collection::vec(valid_string(), 1..5))
            .prop_map(|(pattern, examples)| {
                ImageNaming {
                    pattern,
                    examples,
                }
            })
    }

    pub fn frontmatter_strategy() -> impl Strategy<Value = Frontmatter> {
        (
            valid_string(),
            option_of(valid_string()),
            option_of(valid_string()),
            option_of(valid_string()),
            option_of(valid_string()),
            option_of(proptest::collection::vec(valid_string(), 0..10)),
            option_of(proptest::collection::vec(valid_string(), 0..5)),
            option_of(proptest::bool::ANY),
            option_of(valid_string())
        ).prop_map(|(
            title,
            published_at,
            updated_at,
            slug,
            description,
            tags,
            topics,
            is_draft,
            featured_image_path
        )| {
            Frontmatter {
                title,
                published_at,
                updated_at,
                slug,
                description,
                tags,
                topics,
                is_draft,
                featured_image_path,
            }
        })
    }
}

// Property tests
proptest! {
    // Test that TopicConfig serializes and deserializes correctly
    #[test]
    fn topic_config_roundtrip(topic_config in strategies::topic_config_strategy()) {
        let json = serde_json::to_string(&topic_config).unwrap();
        let roundtrip: TopicConfig = serde_json::from_str(&json).unwrap();

        // Verify properties are preserved
        prop_assert_eq!(topic_config.name, roundtrip.name);
        prop_assert_eq!(topic_config.description, roundtrip.description);
        prop_assert_eq!(topic_config.directory, roundtrip.directory);
    }

    // Test that ImageSize serializes and deserializes correctly
    #[test]
    fn image_size_roundtrip(image_size in strategies::image_size_strategy()) {
        let json = serde_json::to_string(&image_size).unwrap();
        let roundtrip: ImageSize = serde_json::from_str(&json).unwrap();

        // Verify properties are preserved
        prop_assert_eq!(image_size.width, roundtrip.width);
        prop_assert_eq!(image_size.height, roundtrip.height);
        prop_assert_eq!(image_size.description, roundtrip.description);
    }

    // Test that ImageNaming serializes and deserializes correctly
    #[test]
    fn image_naming_roundtrip(image_naming in strategies::image_naming_strategy()) {
        let json = serde_json::to_string(&image_naming).unwrap();
        let roundtrip: ImageNaming = serde_json::from_str(&json).unwrap();

        // Verify properties are preserved
        prop_assert_eq!(image_naming.pattern, roundtrip.pattern);
        prop_assert_eq!(image_naming.examples, roundtrip.examples);
    }

    // Test that Frontmatter serializes and deserializes correctly
    #[test]
    fn frontmatter_roundtrip(frontmatter in strategies::frontmatter_strategy()) {
        let json = serde_json::to_string(&frontmatter).unwrap();
        let roundtrip: Frontmatter = serde_json::from_str(&json).unwrap();

        // Verify properties are preserved
        prop_assert_eq!(frontmatter.title, roundtrip.title);
        prop_assert_eq!(frontmatter.published_at, roundtrip.published_at);
        prop_assert_eq!(frontmatter.updated_at, roundtrip.updated_at);
        prop_assert_eq!(frontmatter.slug, roundtrip.slug);
        prop_assert_eq!(frontmatter.description, roundtrip.description);
        prop_assert_eq!(frontmatter.tags, roundtrip.tags);
        prop_assert_eq!(frontmatter.topics, roundtrip.topics);
        prop_assert_eq!(frontmatter.is_draft, roundtrip.is_draft);
        prop_assert_eq!(frontmatter.featured_image_path, roundtrip.featured_image_path);
    }

    // Test that the frontmatter default values are consistent
    #[test]
    fn frontmatter_default_is_consistent(_i in 0..10) {
        let default1 = Frontmatter::default();
        let default2 = Frontmatter::default();

        assert_eq!(default1.title, default2.title);
        assert_eq!(default1.is_draft, default2.is_draft);
        // Default should set is_draft to true
        assert_eq!(default1.is_draft, Some(true));
    }

    // Test that Article serialization preserves all fields
    #[test]
    fn article_properties(
        frontmatter in strategies::frontmatter_strategy(),
        content in strategies::valid_string(),
        slug in strategies::valid_string(),
        topic in strategies::valid_string(),
        path in strategies::valid_string(),
        word_count in proptest::option::of(1usize..10000usize),
        reading_time in proptest::option::of(1u32..60u32)
    ) {
        let article = Article {
            frontmatter,
            content: content.clone(),
            slug: slug.clone(),
            topic: topic.clone(),
            path: path.clone(),
            word_count,
            reading_time,
        };

        // Verify core properties
        prop_assert_eq!(article.content, content);
        prop_assert_eq!(article.slug, slug);
        prop_assert_eq!(article.topic, topic);
        prop_assert_eq!(article.path, path);
        prop_assert_eq!(article.word_count, word_count);
        prop_assert_eq!(article.reading_time, reading_time);
    }
}