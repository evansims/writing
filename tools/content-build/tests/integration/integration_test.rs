use content_build::{build_content, BuildOptions};
use common_test_utils::fixtures::TestFixture;
use common_test_utils::mocks::{MockFileSystem, MockConfigLoader};
use mockall::predicate;
use std::path::PathBuf;
use std::collections::HashMap;
use common_models::{Config, ContentConfig, TopicConfig, PublicationConfig};

#[test]
fn test_build_content_integration() {
    // Arrange
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths and directory structure
    let base_dir = fixture.path().join("content");
    let output_dir = fixture.path().join("public");

    // Blog structure
    let blog_dir = base_dir.join("blog");
    let article1_dir = blog_dir.join("article1");
    let article1_file = article1_dir.join("index.mdx");
    let article2_dir = blog_dir.join("article2");
    let article2_file = article2_dir.join("index.mdx");

    // Podcast structure
    let podcast_dir = base_dir.join("podcast");
    let episode1_dir = podcast_dir.join("episode1");
    let episode1_file = episode1_dir.join("index.mdx");

    // Output structure
    let data_dir = output_dir.join("data");

    // Mock directory checks
    mock_fs.expect_exists()
        .with(predicate::eq(blog_dir.clone()))
        .returning(|_| true);

    mock_fs.expect_exists()
        .with(predicate::eq(podcast_dir.clone()))
        .returning(|_| true);

    // Mock file existence checks
    mock_fs.expect_exists()
        .with(predicate::eq(article1_file.clone()))
        .returning(|_| true);

    mock_fs.expect_exists()
        .with(predicate::eq(article2_file.clone()))
        .returning(|_| true);

    mock_fs.expect_exists()
        .with(predicate::eq(episode1_file.clone()))
        .returning(|_| true);

    // Mock directory listing
    mock_fs.expect_read_dir()
        .with(predicate::eq(blog_dir.clone()))
        .returning(move |_| Ok(vec![article1_dir.clone(), article2_dir.clone()]));

    mock_fs.expect_read_dir()
        .with(predicate::eq(podcast_dir.clone()))
        .returning(move |_| Ok(vec![episode1_dir.clone()]));

    // Mock file content
    mock_fs.expect_read_to_string()
        .with(predicate::eq(article1_file.clone()))
        .returning(|_| Ok(r#"---
title: "Article 1"
description: "This is article 1"
published_at: "2023-01-01"
---
# Article 1

This is the content of article 1."#.to_string()));

    mock_fs.expect_read_to_string()
        .with(predicate::eq(article2_file.clone()))
        .returning(|_| Ok(r#"---
title: "Article 2"
description: "This is article 2"
published_at: "2023-01-02"
---
# Article 2

This is the content of article 2."#.to_string()));

    mock_fs.expect_read_to_string()
        .with(predicate::eq(episode1_file.clone()))
        .returning(|_| Ok(r#"---
title: "Episode 1"
description: "This is episode 1"
published_at: "2023-01-03"
---
# Episode 1

This is the content of episode 1."#.to_string()));

    // Mock directory and file creation
    mock_fs.expect_create_dir_all()
        .with(predicate::eq(output_dir.clone()))
        .returning(|_| Ok(()));

    mock_fs.expect_create_dir_all()
        .with(predicate::eq(data_dir.clone()))
        .returning(|_| Ok(()));

    // Mock file writing (for each expected output file)
    // Individual JSON files
    mock_fs.expect_write_file()
        .with(predicate::eq(data_dir.join("article1.json")), predicate::always())
        .returning(|_, _| Ok(()));

    mock_fs.expect_write_file()
        .with(predicate::eq(data_dir.join("article2.json")), predicate::always())
        .returning(|_, _| Ok(()));

    mock_fs.expect_write_file()
        .with(predicate::eq(data_dir.join("episode1.json")), predicate::always())
        .returning(|_, _| Ok(()));

    // All.json file
    mock_fs.expect_write_file()
        .with(predicate::eq(data_dir.join("all.json")), predicate::always())
        .returning(|_, _| Ok(()));

    // For sitemap generation
    mock_fs.expect_write_file()
        .with(predicate::eq(output_dir.join("sitemap.xml")), predicate::always())
        .returning(|_, _| Ok(()));

    // For RSS feed generation
    mock_fs.expect_write_file()
        .with(predicate::eq(output_dir.join("rss.xml")), predicate::always())
        .returning(|_, _| Ok(()));

    // Check for templates directory (doesn't exist)
    let templates_dir = PathBuf::from("templates");

    mock_fs.expect_exists()
        .with(predicate::eq(templates_dir.clone()))
        .returning(|_| false);

    // Create a mock config
    let mut topics = HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });
    topics.insert("podcast".to_string(), TopicConfig {
        name: "Podcast".to_string(),
        description: "Podcast episodes".to_string(),
        directory: "podcast".to_string(),
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

    // Create build options for integrated test
    let options = BuildOptions {
        output_dir: Some(output_dir.to_string_lossy().to_string()),
        slug: None,
        topic: None, // Build all content
        include_drafts: false,
        skip_html: false,
        skip_json: false,
        skip_rss: false,
        skip_sitemap: false,
        verbose: true,
    };

    // Act - build all content
    let result = build_content(&options);

    // Assert
    assert!(result.is_ok(), "Building content should succeed: {:?}", result.err());
}

#[test]
fn test_build_specific_content_integration() {
    // Arrange
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let base_dir = fixture.path().join("content");
    let output_dir = fixture.path().join("public");
    let blog_dir = base_dir.join("blog");
    let article_dir = blog_dir.join("test-article");
    let article_file = article_dir.join("index.mdx");
    let data_dir = output_dir.join("data");

    // Mock directory checks
    mock_fs.expect_exists()
        .with(predicate::eq(blog_dir.clone()))
        .returning(|_| true);

    // Mock file existence checks
    mock_fs.expect_exists()
        .with(predicate::eq(article_file.clone()))
        .returning(|_| true);

    // Mock directory listing
    mock_fs.expect_read_dir()
        .with(predicate::eq(blog_dir.clone()))
        .returning(move |_| Ok(vec![article_dir.clone()]));

    // Mock file content
    mock_fs.expect_read_to_string()
        .with(predicate::eq(article_file.clone()))
        .returning(|_| Ok(r#"---
title: "Test Article"
description: "This is a test article"
published_at: "2023-01-01"
---
# Test Article

This is a test article with some content."#.to_string()));

    // Mock directory and file creation
    mock_fs.expect_create_dir_all()
        .with(predicate::eq(output_dir.clone()))
        .returning(|_| Ok(()));

    mock_fs.expect_create_dir_all()
        .with(predicate::eq(data_dir.clone()))
        .returning(|_| Ok(()));

    // Mock file writing
    mock_fs.expect_write_file()
        .with(predicate::eq(data_dir.join("test-article.json")), predicate::always())
        .returning(|_, _| Ok(()));

    mock_fs.expect_write_file()
        .with(predicate::eq(data_dir.join("all.json")), predicate::always())
        .returning(|_, _| Ok(()));

    // For sitemap generation
    mock_fs.expect_write_file()
        .with(predicate::eq(output_dir.join("sitemap.xml")), predicate::always())
        .returning(|_, _| Ok(()));

    // For RSS feed generation
    mock_fs.expect_write_file()
        .with(predicate::eq(output_dir.join("rss.xml")), predicate::always())
        .returning(|_, _| Ok(()));

    // Check for templates directory (doesn't exist)
    let templates_dir = PathBuf::from("templates");

    mock_fs.expect_exists()
        .with(predicate::eq(templates_dir.clone()))
        .returning(|_| false);

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

    // Create build options for specific slug
    let options = BuildOptions {
        output_dir: Some(output_dir.to_string_lossy().to_string()),
        slug: Some("test-article".to_string()),
        topic: Some("blog".to_string()),
        include_drafts: false,
        skip_html: false,
        skip_json: false,
        skip_rss: false,
        skip_sitemap: false,
        verbose: true,
    };

    // Act - build specific content
    let result = build_content(&options);

    // Assert
    assert!(result.is_ok(), "Building specific content should succeed: {:?}", result.err());
}

#[test]
fn test_build_content_with_optional_features() {
    // Arrange
    let fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let base_dir = fixture.path().join("content");
    let output_dir = fixture.path().join("public");
    let blog_dir = base_dir.join("blog");
    let article_dir = blog_dir.join("test-article");
    let article_file = article_dir.join("index.mdx");
    let data_dir = output_dir.join("data");
    let html_dir = output_dir.join("html");

    // Mock directory checks
    mock_fs.expect_exists()
        .with(predicate::eq(blog_dir.clone()))
        .returning(|_| true);

    // Mock file existence checks
    mock_fs.expect_exists()
        .with(predicate::eq(article_file.clone()))
        .returning(|_| true);

    // Mock directory listing
    mock_fs.expect_read_dir()
        .with(predicate::eq(blog_dir.clone()))
        .returning(move |_| Ok(vec![article_dir.clone()]));

    // Mock file content
    mock_fs.expect_read_to_string()
        .with(predicate::eq(article_file.clone()))
        .returning(|_| Ok(r#"---
title: "Test Article"
description: "This is a test article"
published_at: "2023-01-01"
---
# Test Article

This is a test article with some content."#.to_string()));

    // Mock directory and file creation
    mock_fs.expect_create_dir_all()
        .with(predicate::eq(output_dir.clone()))
        .returning(|_| Ok(()));

    mock_fs.expect_create_dir_all()
        .with(predicate::eq(data_dir.clone()))
        .returning(|_| Ok(()));

    mock_fs.expect_create_dir_all()
        .with(predicate::eq(html_dir.clone()))
        .returning(|_| Ok(()));

    // Mock file writing
    mock_fs.expect_write_file()
        .with(predicate::eq(data_dir.join("test-article.json")), predicate::always())
        .returning(|_, _| Ok(()));

    mock_fs.expect_write_file()
        .with(predicate::eq(data_dir.join("all.json")), predicate::always())
        .returning(|_, _| Ok(()));

    // For HTML generation
    let templates_dir = PathBuf::from("templates");
    let template_file = templates_dir.join("article.hbs");

    mock_fs.expect_exists()
        .with(predicate::eq(templates_dir.clone()))
        .returning(|_| true);

    mock_fs.expect_exists()
        .with(predicate::eq(template_file.clone()))
        .returning(|_| true);

    mock_fs.expect_read_to_string()
        .with(predicate::eq(template_file.clone()))
        .returning(|_| Ok("{{title}} - {{{content}}}".to_string()));

    mock_fs.expect_write_file()
        .with(predicate::eq(html_dir.join("test-article.html")), predicate::always())
        .returning(|_, _| Ok(()));

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

    // Create build options with all features enabled
    let options = BuildOptions {
        output_dir: Some(output_dir.to_string_lossy().to_string()),
        slug: Some("test-article".to_string()),
        topic: Some("blog".to_string()),
        include_drafts: false,
        skip_html: false, // Enable HTML generation
        skip_json: false, // Enable JSON generation
        skip_rss: false,  // Enable RSS feed generation
        skip_sitemap: false, // Enable sitemap generation
        verbose: true,
    };

    // Act - build with all features
    let result = build_content(&options);

    // Assert
    assert!(result.is_ok(), "Building content with all features should succeed: {:?}", result.err());
}