//! Integration tests for JSON serialization/deserialization
//!
//! This file contains integration tests for the JSON serialization and
//! deserialization of models, ensuring they work correctly with external formats.

use serde_json;
use std::collections::HashMap;

use crate::*;

#[test]
fn test_full_config_json_roundtrip() {
    // Create a complex, complete configuration
    let mut topics = HashMap::new();
    topics.insert(
        "blog".to_string(),
        TopicConfig {
            name: "Blog".to_string(),
            description: "Blog posts".to_string(),
            directory: "blog".to_string(),
        },
    );
    topics.insert(
        "tutorials".to_string(),
        TopicConfig {
            name: "Tutorials".to_string(),
            description: "Tutorial content".to_string(),
            directory: "tutorials".to_string(),
        },
    );

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

    let mut tags = HashMap::new();
    tags.insert(
        "categories".to_string(),
        vec!["tech".to_string(), "personal".to_string()],
    );

    let original_config = Config {
        title: "Test Site".to_string(),
        email: "test@example.com".to_string(),
        url: "https://test-site.com".to_string(),
        image: "https://test-site.com/image.jpg".to_string(),
        default_topic: Some("blog".to_string()),
        content: ContentConfig {
            base_dir: "/content".to_string(),
            topics,
            tags: Some(tags),
        },
        images: ImageConfig {
            formats: vec!["jpg".to_string(), "webp".to_string()],
            format_descriptions: Some({
                let mut desc = HashMap::new();
                desc.insert("jpg".to_string(), "JPEG image".to_string());
                desc.insert("webp".to_string(), "WebP image".to_string());
                desc
            }),
            sizes,
            naming: Some(ImageNaming {
                pattern: "{slug}-{size}.{format}".to_string(),
                examples: vec!["post-small.jpg".to_string()],
            }),
            quality: Some({
                let mut quality = HashMap::new();
                let mut jpg_quality = HashMap::new();
                jpg_quality.insert("small".to_string(), 80);
                jpg_quality.insert("medium".to_string(), 85);
                quality.insert("jpg".to_string(), jpg_quality);
                quality
            }),
        },
        publication: PublicationConfig {
            author: "Test Author".to_string(),
            copyright: "Test Copyright".to_string(),
            site_url: Some("https://example.com".to_string()),
        },
    };

    // Convert to JSON
    let json = serde_json::to_string_pretty(&original_config).expect("Failed to serialize to JSON");

    // Convert back from JSON
    let deserialized_config: Config = serde_json::from_str(&json).expect("Failed to deserialize from JSON");

    // Verify structure is preserved
    assert_eq!(deserialized_config.title, original_config.title);
    assert_eq!(deserialized_config.email, original_config.email);
    assert_eq!(deserialized_config.url, original_config.url);
    assert_eq!(deserialized_config.image, original_config.image);
    assert_eq!(deserialized_config.default_topic, original_config.default_topic);
    assert_eq!(deserialized_config.content.base_dir, original_config.content.base_dir);
    assert_eq!(deserialized_config.content.topics.len(), original_config.content.topics.len());
    assert_eq!(deserialized_config.images.formats, original_config.images.formats);
    assert_eq!(deserialized_config.publication.author, original_config.publication.author);
    assert_eq!(deserialized_config.publication.copyright, original_config.publication.copyright);
    assert_eq!(deserialized_config.publication.site_url, original_config.publication.site_url);

    // Check specific nested elements
    let blog_topic = deserialized_config.content.topics.get("blog").expect("Blog topic missing");
    assert_eq!(blog_topic.name, "Blog");

    let small_size = deserialized_config.images.sizes.get("small").expect("Small size missing");
    assert_eq!(small_size.width, 480);
    assert_eq!(small_size.height, 320);
}

#[test]
fn test_frontmatter_yaml_compatibility() {
    // Create a frontmatter with all fields populated
    let frontmatter = Frontmatter {
        title: "Test Article".to_string(),
        published_at: Some("2023-01-01".to_string()),
        updated_at: Some("2023-01-15".to_string()),
        slug: Some("test-article".to_string()),
        description: Some("This is a test article".to_string()),
        tags: Some(vec!["test".to_string(), "article".to_string()]),
        topics: Some(vec!["blog".to_string()]),
        is_draft: Some(false),
        featured_image_path: Some("images/featured.jpg".to_string()),
    };

    // Convert to JSON (to simulate YAML serialization/deserialization)
    let json = serde_json::to_string_pretty(&frontmatter).expect("Failed to serialize to JSON");

    // Convert back from JSON
    let deserialized: Frontmatter = serde_json::from_str(&json).expect("Failed to deserialize from JSON");

    // Verify all fields are preserved
    assert_eq!(deserialized.title, frontmatter.title);
    assert_eq!(deserialized.published_at, frontmatter.published_at);
    assert_eq!(deserialized.updated_at, frontmatter.updated_at);
    assert_eq!(deserialized.slug, frontmatter.slug);
    assert_eq!(deserialized.description, frontmatter.description);
    assert_eq!(deserialized.tags, frontmatter.tags);
    assert_eq!(deserialized.topics, frontmatter.topics);
    assert_eq!(deserialized.is_draft, frontmatter.is_draft);
    assert_eq!(deserialized.featured_image_path, frontmatter.featured_image_path);
}

#[test]
fn test_article_serialization() {
    // Create a complete article
    let article = Article {
        frontmatter: Frontmatter {
            title: "Test Article".to_string(),
            published_at: Some("2023-01-01".to_string()),
            updated_at: None,
            slug: Some("test-article".to_string()),
            description: None,
            tags: Some(vec!["test".to_string()]),
            topics: Some(vec!["blog".to_string()]),
            is_draft: Some(false),
            featured_image_path: None,
        },
        content: "# Test Article\n\nThis is a test article.".to_string(),
        slug: "test-article".to_string(),
        topic: "blog".to_string(),
        path: "/content/blog/test-article.md".to_string(),
        word_count: Some(6),
        reading_time: Some(1),
    };

    // Convert to JSON
    let json = serde_json::to_string_pretty(&article).expect("Failed to serialize to JSON");

    // Convert back from JSON
    let deserialized: Article = serde_json::from_str(&json).expect("Failed to deserialize from JSON");

    // Verify all fields are preserved
    assert_eq!(deserialized.frontmatter.title, article.frontmatter.title);
    assert_eq!(deserialized.content, article.content);
    assert_eq!(deserialized.slug, article.slug);
    assert_eq!(deserialized.topic, article.topic);
    assert_eq!(deserialized.path, article.path);
    assert_eq!(deserialized.word_count, article.word_count);
    assert_eq!(deserialized.reading_time, article.reading_time);
}

#[test]
fn test_serialize_article_to_json() {
    let article = Article {
        frontmatter: Frontmatter {
            title: "Test Article".to_string(),
            published_at: Some("2023-01-01".to_string()),
            updated_at: None,
            slug: Some("test-article".to_string()),
            description: Some("A test article".to_string()),
            tags: Some(vec!["test".to_string(), "sample".to_string()]),
            topics: Some(vec!["blog".to_string()]),
            is_draft: Some(false),
            featured_image_path: None,
        },
        content: "This is a test article with some content.".to_string(),
        slug: "test-article".to_string(),
        topic: "blog".to_string(),
        path: "content/blog/test-article.md".to_string(),
        word_count: Some(100),
        reading_time: Some(1),
    };

    let json = serde_json::to_string(&article).unwrap();
    assert!(json.contains("\"title\":\"Test Article\""));
    assert!(json.contains("\"published\":\"2023-01-01\""));
    assert!(json.contains("\"content\":\"This is a test article with some content.\""));
    assert!(json.contains("\"slug\":\"test-article\""));
    assert!(json.contains("\"topic\":\"blog\""));
    assert!(json.contains("\"path\":\"content/blog/test-article.md\""));
    assert!(json.contains("\"word_count\":100"));
    assert!(json.contains("\"reading_time\":1"));
}

#[test]
fn test_deserialize_json_to_article() {
    let json = r#"{
        "frontmatter": {
            "title": "Test Article",
            "published": "2023-01-01",
            "slug": "test-article",
            "description": "A test article",
            "tags": ["test", "sample"],
            "topics": ["blog"],
            "draft": false
        },
        "content": "This is a test article with some content.",
        "slug": "test-article",
        "topic": "blog",
        "path": "content/blog/test-article.md",
        "word_count": 100,
        "reading_time": 1
    }"#;

    let article: Article = serde_json::from_str(json).unwrap();
    assert_eq!(article.frontmatter.title, "Test Article");
    assert_eq!(article.frontmatter.published_at, Some("2023-01-01".to_string()));
    assert_eq!(article.content, "This is a test article with some content.");
    assert_eq!(article.slug, "test-article");
    assert_eq!(article.topic, "blog");
    assert_eq!(article.path, "content/blog/test-article.md");
    assert_eq!(article.word_count, Some(100));
    assert_eq!(article.reading_time, Some(1));
}

#[test]
fn test_serialize_and_deserialize_config() {
    let original_config = Config {
        title: "Test Site".to_string(),
        email: "test@example.com".to_string(),
        url: "https://test-site.com".to_string(),
        image: "https://test-site.com/image.jpg".to_string(),
        default_topic: Some("blog".to_string()),
        content: ContentConfig {
            base_dir: "/content".to_string(),
            topics: HashMap::new(),
            tags: None,
        },
        images: ImageConfig {
            formats: vec!["jpg".to_string(), "png".to_string()],
            format_descriptions: None,
            sizes: HashMap::new(),
            naming: None,
            quality: None,
        },
        publication: PublicationConfig {
            author: "Test Author".to_string(),
            copyright: "Test Copyright".to_string(),
            site_url: Some("https://example.com".to_string()),
        },
    };

    let json = serde_json::to_string(&original_config).unwrap();
    let deserialized_config: Config = serde_json::from_str(&json).unwrap();

    assert_eq!(original_config, deserialized_config);
}