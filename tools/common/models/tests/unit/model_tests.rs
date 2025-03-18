//! Unit tests for the models module
//!
//! This file contains unit tests for the models in the common models library.

use crate::*;

#[test]
fn test_article_with_word_count_and_reading_time() {
    let article = Article {
        frontmatter: Frontmatter::default(),
        content: "This is a test article with some content.".to_string(),
        slug: "test-article".to_string(),
        topic: "blog".to_string(),
        path: "content/blog/test-article.md".to_string(),
        word_count: Some(100),
        reading_time: Some(1),
    };

    assert_eq!(article.word_count, Some(100));
    assert_eq!(article.reading_time, Some(1));
}

#[test]
fn test_valid_topic_config() {
    let topic = TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    };

    assert_eq!(topic.name, "Blog");
    assert_eq!(topic.description, "Blog posts");
    assert_eq!(topic.directory, "blog");
}

#[test]
fn test_frontmatter_default() {
    let frontmatter = Frontmatter::default();

    assert_eq!(frontmatter.title, "Untitled");
    assert_eq!(frontmatter.is_draft, Some(true));
    assert!(frontmatter.slug.is_none());
}

#[test]
fn test_config_structure() {
    let config = Config {
        title: "Test Site".to_string(),
        email: "test@example.com".to_string(),
        url: "https://test.example.com".to_string(),
        image: "https://test.example.com/image.jpg".to_string(),
        default_topic: Some("blog".to_string()),
        content: ContentConfig {
            base_dir: "/content".to_string(),
            topics: std::collections::HashMap::new(),
            tags: None,
        },
        images: ImageConfig {
            formats: vec!["jpg".to_string()],
            format_descriptions: None,
            sizes: std::collections::HashMap::new(),
            naming: None,
            quality: None,
        },
        publication: PublicationConfig {
            author: "Test Author".to_string(),
            copyright: "Test Copyright".to_string(),
            site_url: None,
        },
    };

    assert_eq!(config.title, "Test Site");
    assert_eq!(config.email, "test@example.com");
    assert_eq!(config.url, "https://test.example.com");
    assert_eq!(config.image, "https://test.example.com/image.jpg");
    assert_eq!(config.default_topic, Some("blog".to_string()));
    assert_eq!(config.content.base_dir, "/content");
    assert_eq!(config.images.formats, vec!["jpg"]);
    assert_eq!(config.publication.author, "Test Author");
}