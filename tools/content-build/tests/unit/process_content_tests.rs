use content_build::process_content;
use common_test_utils::fixtures::TestFixture;
use common_test_utils::mocks::MockFileSystem;
use mockall::predicate;
use std::path::{Path, PathBuf};
use common_models::{ContentConfig, TopicConfig};

#[test]
fn test_process_content_with_directory_path() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    let base_dir = fixture.path().join("content");
    let blog_dir = base_dir.join("blog");
    let article_dir = blog_dir.join("test-article");
    let article_file = article_dir.join("index.mdx");

    // Mock directory existence check
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_dir.clone()))
        .returning(|_| Ok(true));

    // Mock directory check (is_dir)
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_dir.clone()))
        .returning(|_| Ok(true));

    // Mock file existence check
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_file.clone()))
        .returning(|_| Ok(true));

    // Mock file content
    mock_fs.expect_read_file()
        .with(predicate::eq(article_file))
        .returning(|_| Ok(r#"---
title: "Test Article"
description: "This is a test article"
published_at: "2023-01-01"
---
# Test Article

This is the content of the test article."#.to_string()));

    fixture.fs = mock_fs;

    // Create content config for the test
    let mut topics = std::collections::HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });


    // Act
    let result = process_content(Path::new("blog/test-article"), false);

    // Assert
    assert!(result.is_ok());
    let article = result.unwrap();
    assert_eq!(article.slug, "test-article");
    assert_eq!(article.topic, "blog");
    assert_eq!(article.frontmatter.title, "Test Article");
    assert_eq!(article.frontmatter.description.unwrap(), "This is a test article");
    assert_eq!(article.frontmatter.published_at.unwrap(), "2023-01-01");
    assert_eq!(article.frontmatter.is_draft, None);
    assert!(article.content.contains("# Test Article"));
}

#[test]
fn test_process_content_with_file_path() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    let base_dir = fixture.path().join("content");
    let blog_dir = base_dir.join("blog");
    let article_file = blog_dir.join("test-article.mdx");

    // Mock file existence check
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_file.clone()))
        .returning(|_| Ok(true));

    // Mock directory check (is_dir)
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_file.clone()))
        .returning(|_| Ok(false));

    // Mock file content
    mock_fs.expect_read_file()
        .with(predicate::eq(article_file))
        .returning(|_| Ok(r#"---
title: "Test Article"
description: "This is a test article"
published_at: "2023-01-01"
---
# Test Article

This is the content of the test article."#.to_string()));

    fixture.fs = mock_fs;

    // Create content config for the test
    let mut topics = std::collections::HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });


    // Act
    let result = process_content(Path::new("blog/test-article.mdx"), false);

    // Assert
    assert!(result.is_ok());
    let article = result.unwrap();
    assert_eq!(article.slug, "test-article");
    assert_eq!(article.topic, "blog");
    assert_eq!(article.frontmatter.title, "Test Article");
    assert_eq!(article.frontmatter.description.unwrap(), "This is a test article");
    assert_eq!(article.frontmatter.published_at.unwrap(), "2023-01-01");
    assert_eq!(article.frontmatter.is_draft, None);
    assert!(article.content.contains("# Test Article"));
}

#[test]
fn test_process_content_with_draft_include_drafts_true() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    let base_dir = fixture.path().join("content");
    let blog_dir = base_dir.join("blog");
    let article_dir = blog_dir.join("draft-article");
    let article_file = article_dir.join("index.mdx");

    // Mock directory existence check
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_dir.clone()))
        .returning(|_| Ok(true));

    // Mock directory check (is_dir)
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_dir.clone()))
        .returning(|_| Ok(true));

    // Mock file existence check
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_file.clone()))
        .returning(|_| Ok(true));

    // Mock file content
    mock_fs.expect_read_file()
        .with(predicate::eq(article_file))
        .returning(|_| Ok(r#"---
title: "Draft Article"
description: "This is a draft article"
published_at: "2023-01-01"
draft: true
---
# Draft Article

This is the content of a draft article."#.to_string()));

    fixture.fs = mock_fs;

    // Create content config for the test
    let mut topics = std::collections::HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });


    // Act
    let result = process_content(Path::new("blog/draft-article"), true);

    // Assert
    assert!(result.is_ok());
    let article = result.unwrap();
    assert_eq!(article.slug, "draft-article");
    assert_eq!(article.topic, "blog");
    assert_eq!(article.frontmatter.title, "Draft Article");
    assert_eq!(article.frontmatter.description.unwrap(), "This is a draft article");
    assert_eq!(article.frontmatter.published_at.unwrap(), "2023-01-01");
    assert_eq!(article.frontmatter.is_draft, Some(true));
    assert!(article.content.contains("# Draft Article"));
}

#[test]
fn test_process_content_with_draft_include_drafts_false() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    let base_dir = fixture.path().join("content");
    let blog_dir = base_dir.join("blog");
    let article_dir = blog_dir.join("draft-article");
    let article_file = article_dir.join("index.mdx");

    // Mock directory existence check
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_dir.clone()))
        .returning(|_| Ok(true));

    // Mock directory check (is_dir)
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_dir.clone()))
        .returning(|_| Ok(true));

    // Mock file existence check
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_file.clone()))
        .returning(|_| Ok(true));

    // Mock file content
    mock_fs.expect_read_file()
        .with(predicate::eq(article_file))
        .returning(|_| Ok(r#"---
title: "Draft Article"
description: "This is a draft article"
published_at: "2023-01-01"
draft: true
---
# Draft Article

This is the content of a draft article."#.to_string()));

    fixture.fs = mock_fs;

    // Create content config for the test
    let mut topics = std::collections::HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });


    // Act
    let result = process_content(Path::new("blog/draft-article"), false);

    // Assert
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Draft content"));
}

#[test]
fn test_process_content_file_not_found() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    let base_dir = fixture.path().join("content");
    let blog_dir = base_dir.join("blog");
    let article_dir = blog_dir.join("missing-article");
    let article_file = article_dir.join("index.mdx");

    // Mock directory existence check
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_dir.clone()))
        .returning(|_| Ok(true));

    // Mock directory check (is_dir)
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_dir.clone()))
        .returning(|_| Ok(true));

    // Mock file existence check - file doesn't exist
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_file.clone()))
        .returning(|_| Ok(false));

    fixture.fs = mock_fs;

    // Create content config for the test
    let mut topics = std::collections::HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });


    // Act
    let result = process_content(Path::new("blog/missing-article"), false);

    // Assert
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Content file not found"));
}

#[test]
fn test_process_content_invalid_frontmatter() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    let base_dir = fixture.path().join("content");
    let blog_dir = base_dir.join("blog");
    let article_dir = blog_dir.join("invalid-article");
    let article_file = article_dir.join("index.mdx");

    // Mock directory existence check
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_dir.clone()))
        .returning(|_| Ok(true));

    // Mock directory check (is_dir)
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_dir.clone()))
        .returning(|_| Ok(true));

    // Mock file existence check
    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_file.clone()))
        .returning(|_| Ok(true));

    // Mock file content with invalid frontmatter
    mock_fs.expect_read_file()
        .with(predicate::eq(article_file))
        .returning(|_| Ok(r#"---
invalid frontmatter
---
# Invalid Article

This article has invalid frontmatter."#.to_string()));

    fixture.fs = mock_fs;

    // Create content config for the test
    let mut topics = std::collections::HashMap::new();
    topics.insert("blog".to_string(), TopicConfig {
        name: "Blog".to_string(),
        description: "Blog posts".to_string(),
        directory: "blog".to_string(),
    });


    // Act
    let result = process_content(Path::new("blog/invalid-article"), false);

    // Assert
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Failed to parse frontmatter"));
}