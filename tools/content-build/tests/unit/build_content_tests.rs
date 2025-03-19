use content_build::{build_content, BuildOptions};
use common_test_utils::fixtures::TestFixture;
use common_test_utils::mocks::{MockFileSystem, MockConfigLoader};
use mockall::predicate;
use std::collections::HashMap;
use std::path::PathBuf;
use common_models::{Config, ContentConfig, TopicConfig, PublicationConfig};

#[test]
fn test_build_content_with_single_slug() {
    // Arrange
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let base_dir = fixture.path().join("content");
    let output_dir = fixture.path().join("public");
    let blog_dir = base_dir.join("blog");
    let article_dir = blog_dir.join("test-article");
    let index_file = article_dir.join("index.mdx");

    let data_dir = output_dir.join("data");
    let json_file = data_dir.join("test-article.json");
    let all_json_file = data_dir.join("all.json");

    // Mock file system checks and operations
    mock_fs.expect_exists()
        .with(predicate::eq(blog_dir.clone()))
        .returning(|_| true);

    mock_fs.expect_exists()
        .with(predicate::eq(article_dir.join("index.mdx")))
        .returning(|_| true);

    mock_fs.expect_read_dir()
        .with(predicate::eq(blog_dir.clone()))
        .returning(move |_| Ok(vec![article_dir.clone()]));

    mock_fs.expect_read_to_string()
        .with(predicate::eq(index_file.clone()))
        .returning(|_| Ok(r#"---
title: "Test Article"
description: "This is a test article"
published_at: "2023-01-01"
---
# Test Article

This is the content of the test article."#.to_string()));

    // Expect directory and file creation
    mock_fs.expect_create_dir_all()
        .with(predicate::eq(output_dir.clone()))
        .returning(|_| Ok(()));

    mock_fs.expect_create_dir_all()
        .with(predicate::eq(data_dir.clone()))
        .returning(|_| Ok(()));

    mock_fs.expect_write_file()
        .with(predicate::eq(json_file.clone()), predicate::always())
        .returning(|_, _| Ok(()));

    mock_fs.expect_write_file()
        .with(predicate::eq(all_json_file.clone()), predicate::always())
        .returning(|_, _| Ok(()));

    // Setup templates directory mock
    let templates_dir = PathBuf::from("templates");
    let template_file = templates_dir.join("article.hbs");

    mock_fs.expect_exists()
        .with(predicate::eq(templates_dir.clone()))
        .returning(|_| false);

    // For sitemap generation
    mock_fs.expect_write_file()
        .with(predicate::eq(output_dir.join("sitemap.xml")), predicate::always())
        .returning(|_, _| Ok(()));

    // For RSS feed generation
    mock_fs.expect_write_file()
        .with(predicate::eq(output_dir.join("rss.xml")), predicate::always())
        .returning(|_, _| Ok(()));

    // Create a mock config
    let mut topics = HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });

    let config = Config {
        content: ContentConfig {
            base_dir: base_dir.to_string_lossy().to_string(),
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

    // Setup mock config loader
    let mut mock_config = MockConfigLoader::new();
    mock_config.expect_load_config()
        .returning(move || Ok(config.clone()));

    // Register mocks with the fixture
    fixture.register_fs(Box::new(mock_fs));
    fixture.register_config_loader(Box::new(mock_config));

    // Create options with a specific slug
    let options = BuildOptions {
        output_dir: Some(output_dir.to_string_lossy().to_string()),
        slug: Some("test-article".to_string()),
        topic: None,
        include_drafts: false,
        skip_html: false,
        skip_json: false,
        skip_rss: false,
        skip_sitemap: false,
        verbose: true,
    };

    // Act
    let result = build_content(&options);

    // Assert
    assert!(result.is_ok(), "Building content should succeed: {:?}", result);
}

#[test]
fn test_build_content_with_topic() {
    // Arrange
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let base_dir = fixture.path().join("content");
    let output_dir = fixture.path().join("public");
    let blog_dir = base_dir.join("blog");

    let article1_dir = blog_dir.join("article1");
    let article2_dir = blog_dir.join("article2");

    let index1_file = article1_dir.join("index.mdx");
    let index2_file = article2_dir.join("index.mdx");

    let data_dir = output_dir.join("data");

    // Mock file system checks and operations
    mock_fs.expect_exists()
        .with(predicate::eq(blog_dir.clone()))
        .returning(|_| true);

    mock_fs.expect_exists()
        .with(predicate::eq(article1_dir.join("index.mdx")))
        .returning(|_| true);

    mock_fs.expect_exists()
        .with(predicate::eq(article2_dir.join("index.mdx")))
        .returning(|_| true);

    mock_fs.expect_read_dir()
        .with(predicate::eq(blog_dir.clone()))
        .returning(move |_| Ok(vec![article1_dir.clone(), article2_dir.clone()]));

    // Content for article 1
    mock_fs.expect_read_to_string()
        .with(predicate::eq(index1_file.clone()))
        .returning(|_| Ok(r#"---
title: "Article 1"
description: "This is article 1"
published_at: "2023-01-01"
---
# Article 1

This is the content of article 1."#.to_string()));

    // Content for article 2
    mock_fs.expect_read_to_string()
        .with(predicate::eq(index2_file.clone()))
        .returning(|_| Ok(r#"---
title: "Article 2"
description: "This is article 2"
published_at: "2023-01-02"
---
# Article 2

This is the content of article 2."#.to_string()));

    // Expect directory and file creation
    mock_fs.expect_create_dir_all()
        .with(predicate::eq(output_dir.clone()))
        .returning(|_| Ok(()));

    mock_fs.expect_create_dir_all()
        .with(predicate::eq(data_dir.clone()))
        .returning(|_| Ok(()));

    // Article 1 JSON
    mock_fs.expect_write_file()
        .with(predicate::eq(data_dir.join("article1.json")), predicate::always())
        .returning(|_, _| Ok(()));

    // Article 2 JSON
    mock_fs.expect_write_file()
        .with(predicate::eq(data_dir.join("article2.json")), predicate::always())
        .returning(|_, _| Ok(()));

    // All articles JSON
    mock_fs.expect_write_file()
        .with(predicate::eq(data_dir.join("all.json")), predicate::always())
        .returning(|_, _| Ok(()));

    // Setup templates directory mock
    let templates_dir = PathBuf::from("templates");

    mock_fs.expect_exists()
        .with(predicate::eq(templates_dir.clone()))
        .returning(|_| false);

    // For sitemap generation
    mock_fs.expect_write_file()
        .with(predicate::eq(output_dir.join("sitemap.xml")), predicate::always())
        .returning(|_, _| Ok(()));

    // For RSS feed generation
    mock_fs.expect_write_file()
        .with(predicate::eq(output_dir.join("rss.xml")), predicate::always())
        .returning(|_, _| Ok(()));

    // Create a mock config
    let mut topics = HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });
    topics.insert("podcast".to_string(), TopicConfig {
        name: "Podcast".to_string(),
        directory: "podcast".to_string(),
        ..Default::default()
    });

    let config = Config {
        content: ContentConfig {
            base_dir: base_dir.to_string_lossy().to_string(),
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

    // Setup mock config loader
    let mut mock_config = MockConfigLoader::new();
    mock_config.expect_load_config()
        .returning(move || Ok(config.clone()));

    // Register mocks with the fixture
    fixture.register_fs(Box::new(mock_fs));
    fixture.register_config_loader(Box::new(mock_config));

    // Create options with a specific topic
    let options = BuildOptions {
        output_dir: Some(output_dir.to_string_lossy().to_string()),
        slug: None,
        topic: Some("blog".to_string()),
        include_drafts: false,
        skip_html: false,
        skip_json: false,
        skip_rss: false,
        skip_sitemap: false,
        verbose: true,
    };

    // Act
    let result = build_content(&options);

    // Assert
    assert!(result.is_ok(), "Building content for topic should succeed: {:?}", result);
}

#[test]
fn test_build_content_with_no_content_found() {
    // Arrange
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let base_dir = fixture.path().join("content");
    let blog_dir = base_dir.join("blog");

    // Mock file system checks - empty directory
    mock_fs.expect_exists()
        .with(predicate::eq(blog_dir.clone()))
        .returning(|_| true);

    mock_fs.expect_read_dir()
        .with(predicate::eq(blog_dir.clone()))
        .returning(move |_| Ok(vec![]));

    // Create a mock config
    let mut topics = HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });

    let config = Config {
        content: ContentConfig {
            base_dir: base_dir.to_string_lossy().to_string(),
            topics,
            tags: None,
        },
        ..Default::default()
    };

    // Setup mock config loader
    let mut mock_config = MockConfigLoader::new();
    mock_config.expect_load_config()
        .returning(move || Ok(config.clone()));

    // Register mocks with the fixture
    fixture.register_fs(Box::new(mock_fs));
    fixture.register_config_loader(Box::new(mock_config));

    // Create default options
    let options = BuildOptions {
        output_dir: None,
        slug: None,
        topic: Some("blog".to_string()),
        include_drafts: false,
        skip_html: false,
        skip_json: false,
        skip_rss: false,
        skip_sitemap: false,
        verbose: false,
    };

    // Act
    let result = build_content(&options);

    // Assert
    assert!(result.is_err(), "Building content with no content found should fail");
    assert!(result.unwrap_err().to_string().contains("No content found"));
}

#[test]
fn test_build_content_with_skip_options() {
    // Arrange
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let base_dir = fixture.path().join("content");
    let output_dir = fixture.path().join("public");
    let blog_dir = base_dir.join("blog");
    let article_dir = blog_dir.join("test-article");
    let index_file = article_dir.join("index.mdx");

    // Mock file system checks and operations
    mock_fs.expect_exists()
        .with(predicate::eq(blog_dir.clone()))
        .returning(|_| true);

    mock_fs.expect_exists()
        .with(predicate::eq(article_dir.join("index.mdx")))
        .returning(|_| true);

    mock_fs.expect_read_dir()
        .with(predicate::eq(blog_dir.clone()))
        .returning(move |_| Ok(vec![article_dir.clone()]));

    mock_fs.expect_read_to_string()
        .with(predicate::eq(index_file.clone()))
        .returning(|_| Ok(r#"---
title: "Test Article"
description: "This is a test article"
published_at: "2023-01-01"
---
# Test Article

This is the content of the test article."#.to_string()));

    // Expect directory creation but no file writes due to skip options
    mock_fs.expect_create_dir_all()
        .with(predicate::eq(output_dir.clone()))
        .returning(|_| Ok(()));

    // Create a mock config
    let mut topics = HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });

    let config = Config {
        content: ContentConfig {
            base_dir: base_dir.to_string_lossy().to_string(),
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

    // Setup mock config loader
    let mut mock_config = MockConfigLoader::new();
    mock_config.expect_load_config()
        .returning(move || Ok(config.clone()));

    // Register mocks with the fixture
    fixture.register_fs(Box::new(mock_fs));
    fixture.register_config_loader(Box::new(mock_config));

    // Create options with all skip flags enabled
    let options = BuildOptions {
        output_dir: Some(output_dir.to_string_lossy().to_string()),
        slug: None,
        topic: Some("blog".to_string()),
        include_drafts: false,
        skip_html: true,
        skip_json: true,
        skip_rss: true,
        skip_sitemap: true,
        verbose: false,
    };

    // Act
    let result = build_content(&options);

    // Assert
    assert!(result.is_ok(), "Building content with skip options should succeed: {:?}", result);
}

#[test]
fn test_build_content_with_include_drafts() {
    // Arrange
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let base_dir = fixture.path().join("content");
    let output_dir = fixture.path().join("public");
    let blog_dir = base_dir.join("blog");

    let published_dir = blog_dir.join("published-article");
    let draft_dir = blog_dir.join("draft-article");

    let published_file = published_dir.join("index.mdx");
    let draft_file = draft_dir.join("index.mdx");

    let data_dir = output_dir.join("data");

    // Mock file system checks and operations
    mock_fs.expect_exists()
        .with(predicate::eq(blog_dir.clone()))
        .returning(|_| true);

    mock_fs.expect_exists()
        .with(predicate::eq(published_dir.join("index.mdx")))
        .returning(|_| true);

    mock_fs.expect_exists()
        .with(predicate::eq(draft_dir.join("index.mdx")))
        .returning(|_| true);

    mock_fs.expect_read_dir()
        .with(predicate::eq(blog_dir.clone()))
        .returning(move |_| Ok(vec![published_dir.clone(), draft_dir.clone()]));

    // Content for published article
    mock_fs.expect_read_to_string()
        .with(predicate::eq(published_file.clone()))
        .returning(|_| Ok(r#"---
title: "Published Article"
description: "This is a published article"
published_at: "2023-01-01"
---
# Published Article

This is the content of the published article."#.to_string()));

    // Content for draft article
    mock_fs.expect_read_to_string()
        .with(predicate::eq(draft_file.clone()))
        .returning(|_| Ok(r#"---
title: "Draft Article"
description: "This is a draft article"
is_draft: true
---
# Draft Article

This is the content of the draft article."#.to_string()));

    // Expect directory and file creation
    mock_fs.expect_create_dir_all()
        .with(predicate::eq(output_dir.clone()))
        .returning(|_| Ok(()));

    mock_fs.expect_create_dir_all()
        .with(predicate::eq(data_dir.clone()))
        .returning(|_| Ok(()));

    // Published article JSON
    mock_fs.expect_write_file()
        .with(predicate::eq(data_dir.join("published-article.json")), predicate::always())
        .returning(|_, _| Ok(()));

    // Draft article JSON
    mock_fs.expect_write_file()
        .with(predicate::eq(data_dir.join("draft-article.json")), predicate::always())
        .returning(|_, _| Ok(()));

    // All articles JSON
    mock_fs.expect_write_file()
        .with(predicate::eq(data_dir.join("all.json")), predicate::always())
        .returning(|_, _| Ok(()));

    // Setup templates directory mock
    let templates_dir = PathBuf::from("templates");

    mock_fs.expect_exists()
        .with(predicate::eq(templates_dir.clone()))
        .returning(|_| false);

    // For sitemap generation
    mock_fs.expect_write_file()
        .with(predicate::eq(output_dir.join("sitemap.xml")), predicate::always())
        .returning(|_, _| Ok(()));

    // For RSS feed generation
    mock_fs.expect_write_file()
        .with(predicate::eq(output_dir.join("rss.xml")), predicate::always())
        .returning(|_, _| Ok(()));

    // Create a mock config
    let mut topics = HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });

    let config = Config {
        content: ContentConfig {
            base_dir: base_dir.to_string_lossy().to_string(),
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

    // Setup mock config loader
    let mut mock_config = MockConfigLoader::new();
    mock_config.expect_load_config()
        .returning(move || Ok(config.clone()));

    // Register mocks with the fixture
    fixture.register_fs(Box::new(mock_fs));
    fixture.register_config_loader(Box::new(mock_config));

    // Create options with include_drafts enabled
    let options = BuildOptions {
        output_dir: Some(output_dir.to_string_lossy().to_string()),
        slug: None,
        topic: Some("blog".to_string()),
        include_drafts: true,
        skip_html: false,
        skip_json: false,
        skip_rss: false,
        skip_sitemap: false,
        verbose: true,
    };

    // Act
    let result = build_content(&options);

    // Assert
    assert!(result.is_ok(), "Building content with include_drafts should succeed: {:?}", result);
}