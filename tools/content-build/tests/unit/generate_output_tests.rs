use content_build::{generate_sitemap, generate_rss_feed};
use common_test_utils::fixtures::TestFixture;
use common_test_utils::mocks::MockFileSystem;
use mockall::predicate;
use std::path::PathBuf;
use std::collections::HashMap;
use common_models::{Article, Frontmatter, Config, ContentConfig, TopicConfig, PublicationConfig};

#[test]
fn test_generate_sitemap() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let output_dir = fixture.path().join("public");
    let sitemap_path = output_dir.join("sitemap.xml");

    // Mock file system operations
    mock_fs.expect_write_file()
        .with(predicate::eq(sitemap_path.clone()), predicate::always())
        .returning(|_, _| Ok(()));

    // Register mock file system
    fixture.fs = mock_fs;

    // Create test articles
    let articles = vec![
        Article {
            frontmatter: Frontmatter {
                title: "Article 1".to_string(),
                description: Some("Description 1".to_string()),
                published_at: Some("2023-01-01".to_string()),
                updated_at: None,
                is_draft: None,
                ..Default::default()
            },
            content: "Content 1".to_string(),
            slug: "article-1".to_string(),
            topic: "blog".to_string(),
            path: "content/blog/article-1/index.mdx".to_string(),
            word_count: Some(2),
            reading_time: Some(1),
        },
        Article {
            frontmatter: Frontmatter {
                title: "Article 2".to_string(),
                description: Some("Description 2".to_string()),
                published_at: Some("2023-01-02".to_string()),
                updated_at: Some("2023-01-03".to_string()),
                is_draft: None,
                ..Default::default()
            },
            content: "Content 2".to_string(),
            slug: "article-2".to_string(),
            topic: "blog".to_string(),
            path: "content/blog/article-2/index.mdx".to_string(),
            word_count: Some(2),
            reading_time: Some(1),
        },
        Article {
            frontmatter: Frontmatter {
                title: "Draft Article".to_string(),
                description: Some("Draft Description".to_string()),
                published_at: None,
                updated_at: None,
                is_draft: Some(true),
                ..Default::default()
            },
            content: "Draft Content".to_string(),
            slug: "draft-article".to_string(),
            topic: "blog".to_string(),
            path: "content/blog/draft-article/index.mdx".to_string(),
            word_count: Some(2),
            reading_time: Some(1),
        },
    ];

    // Create config with site URL
    let mut topics = HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });

    let config = Config {
        content: ContentConfig {
            base_dir: "content".to_string(),
            topics,
            tags: None,
        },
        publication: PublicationConfig {
            site_url: Some("https://example.com".to_string()),
            author: "Test Author".to_string(),
            copyright: "Copyright © 2023".to_string(),
        },
        ..Default::default()
    };

    // Act
    let result = generate_sitemap(&output_dir, &articles, &config);

    // Assert
    assert!(result.is_ok(), "Generating sitemap should succeed");
}

#[test]
fn test_generate_sitemap_without_site_url() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let output_dir = fixture.path().join("public");
    let sitemap_path = output_dir.join("sitemap.xml");

    // Mock file system operations
    mock_fs.expect_write_file()
        .with(predicate::eq(sitemap_path.clone()), predicate::always())
        .returning(|_, _| Ok(()));

    // Register mock file system
    fixture.fs = mock_fs;

    // Create test articles
    let articles = vec![
        Article {
            frontmatter: Frontmatter {
                title: "Article 1".to_string(),
                description: Some("Description 1".to_string()),
                published_at: Some("2023-01-01".to_string()),
                updated_at: None,
                is_draft: None,
                ..Default::default()
            },
            content: "Content 1".to_string(),
            slug: "article-1".to_string(),
            topic: "blog".to_string(),
            path: "content/blog/article-1/index.mdx".to_string(),
            word_count: Some(2),
            reading_time: Some(1),
        },
    ];

    // Create config without site URL
    let mut topics = HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });

    let config = Config {
        content: ContentConfig {
            base_dir: "content".to_string(),
            topics,
            tags: None,
        },
        publication: PublicationConfig {
            site_url: None,
            author: "Test Author".to_string(),
            copyright: "Copyright © 2023".to_string(),
        },
        ..Default::default()
    };

    // Act
    let result = generate_sitemap(&output_dir, &articles, &config);

    // Assert
    assert!(result.is_ok(), "Generating sitemap without site URL should succeed (using default URL)");
}

#[test]
fn test_generate_rss_feed() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let output_dir = fixture.path().join("public");
    let rss_path = output_dir.join("rss.xml");

    // Mock file system operations
    mock_fs.expect_write_file()
        .with(predicate::eq(rss_path.clone()), predicate::always())
        .returning(|_, _| Ok(()));

    // Register mock file system
    fixture.fs = mock_fs;

    // Create test articles
    let articles = vec![
        Article {
            frontmatter: Frontmatter {
                title: "Article 1".to_string(),
                description: Some("Description 1".to_string()),
                published_at: Some("2023-01-01".to_string()),
                updated_at: None,
                is_draft: None,
                ..Default::default()
            },
            content: "Content 1".to_string(),
            slug: "article-1".to_string(),
            topic: "blog".to_string(),
            path: "content/blog/article-1/index.mdx".to_string(),
            word_count: Some(2),
            reading_time: Some(1),
        },
        Article {
            frontmatter: Frontmatter {
                title: "Article 2".to_string(),
                description: Some("Description 2".to_string()),
                published_at: Some("2023-01-02".to_string()),
                updated_at: Some("2023-01-03".to_string()),
                is_draft: None,
                ..Default::default()
            },
            content: "Content 2 with <pre><code>some code</code></pre> that should be sanitized".to_string(),
            slug: "article-2".to_string(),
            topic: "blog".to_string(),
            path: "content/blog/article-2/index.mdx".to_string(),
            word_count: Some(10),
            reading_time: Some(1),
        },
        Article {
            frontmatter: Frontmatter {
                title: "Draft Article".to_string(),
                description: Some("Draft Description".to_string()),
                published_at: None,
                updated_at: None,
                is_draft: Some(true),
                ..Default::default()
            },
            content: "Draft Content".to_string(),
            slug: "draft-article".to_string(),
            topic: "blog".to_string(),
            path: "content/blog/draft-article/index.mdx".to_string(),
            word_count: Some(2),
            reading_time: Some(1),
        },
    ];

    // Create config with site URL
    let mut topics = HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });

    let config = Config {
        content: ContentConfig {
            base_dir: "content".to_string(),
            topics,
            tags: None,
        },
        publication: PublicationConfig {
            site_url: Some("https://example.com".to_string()),
            author: "Test Author".to_string(),
            copyright: "Copyright © 2023".to_string(),
        },
        ..Default::default()
    };

    // Act
    let result = generate_rss_feed(&output_dir, &articles, &config);

    // Assert
    assert!(result.is_ok(), "Generating RSS feed should succeed");
}

#[test]
fn test_generate_rss_feed_with_many_articles() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let output_dir = fixture.path().join("public");
    let rss_path = output_dir.join("rss.xml");

    // Mock file system operations
    mock_fs.expect_write_file()
        .with(predicate::eq(rss_path.clone()), predicate::always())
        .returning(|_, _| Ok(()));

    // Register mock file system
    fixture.fs = mock_fs;

    // Create many test articles (more than the limit of 20)
    let mut articles = Vec::new();
    for i in 1..=25 {
        articles.push(Article {
            frontmatter: Frontmatter {
                title: format!("Article {}", i),
                description: Some(format!("Description {}", i)),
                published_at: Some(format!("2023-01-{:02}", i)),
                updated_at: None,
                is_draft: None,
                ..Default::default()
            },
            content: format!("Content {}", i),
            slug: format!("article-{}", i),
            topic: "blog".to_string(),
            path: format!("content/blog/article-{}/index.mdx", i),
            word_count: Some(2),
            reading_time: Some(1),
        });
    }

    // Create config
    let mut topics = HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });

    let config = Config {
        content: ContentConfig {
            base_dir: "content".to_string(),
            topics,
            tags: None,
        },
        publication: PublicationConfig {
            site_url: Some("https://example.com".to_string()),
            author: "Test Author".to_string(),
            copyright: "Copyright © 2023".to_string(),
        },
        ..Default::default()
    };

    // Act
    let result = generate_rss_feed(&output_dir, &articles, &config);

    // Assert
    assert!(result.is_ok(), "Generating RSS feed with many articles should succeed (limiting to 20)");
}

#[test]
fn test_generate_sitemap_file_error() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let output_dir = fixture.path().join("public");
    let sitemap_path = output_dir.join("sitemap.xml");

    // Mock file system operations to fail
    mock_fs.expect_write_file()
        .with(predicate::eq(sitemap_path.clone()), predicate::always())
        .returning(|_, _| Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Permission denied").into()));

    // Register mock file system
    fixture.fs = mock_fs;

    // Create test articles
    let articles = vec![
        Article {
            frontmatter: Frontmatter {
                title: "Article 1".to_string(),
                description: Some("Description 1".to_string()),
                published_at: Some("2023-01-01".to_string()),
                updated_at: None,
                is_draft: None,
                ..Default::default()
            },
            content: "Content 1".to_string(),
            slug: "article-1".to_string(),
            topic: "blog".to_string(),
            path: "content/blog/article-1/index.mdx".to_string(),
            word_count: Some(2),
            reading_time: Some(1),
        },
    ];

    // Create config
    let config = Config::default();

    // Act
    let result = generate_sitemap(&output_dir, &articles, &config);

    // Assert
    assert!(result.is_err(), "Generating sitemap should fail when write_file fails");
    assert!(result.unwrap_err().to_string().contains("Permission denied"));
}

#[test]
fn test_generate_rss_feed_file_error() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let output_dir = fixture.path().join("public");
    let rss_path = output_dir.join("rss.xml");

    // Mock file system operations to fail
    mock_fs.expect_write_file()
        .with(predicate::eq(rss_path.clone()), predicate::always())
        .returning(|_, _| Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Permission denied").into()));

    // Register mock file system
    fixture.fs = mock_fs;

    // Create test articles
    let articles = vec![
        Article {
            frontmatter: Frontmatter {
                title: "Article 1".to_string(),
                description: Some("Description 1".to_string()),
                published_at: Some("2023-01-01".to_string()),
                updated_at: None,
                is_draft: None,
                ..Default::default()
            },
            content: "Content 1".to_string(),
            slug: "article-1".to_string(),
            topic: "blog".to_string(),
            path: "content/blog/article-1/index.mdx".to_string(),
            word_count: Some(2),
            reading_time: Some(1),
        },
    ];

    // Create config
    let config = Config::default();

    // Act
    let result = generate_rss_feed(&output_dir, &articles, &config);

    // Assert
    assert!(result.is_err(), "Generating RSS feed should fail when write_file fails");
    assert!(result.unwrap_err().to_string().contains("Permission denied"));
}