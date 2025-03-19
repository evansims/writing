use content_build::{find_content_files, find_content_by_slug};
use common_test_utils::fixtures::TestFixture;
use common_test_utils::mocks::{MockFileSystem, MockConfigLoader};
use mockall::predicate;
use std::collections::HashMap;
use common_models::{Config, ContentConfig, TopicConfig};

#[test]
fn test_find_content_files_all_topics() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let base_dir = fixture.path().join("content");
    let blog_dir = base_dir.join("blog");
    let podcast_dir = base_dir.join("podcast");

    let article1_dir = blog_dir.join("article1");
    let article2_dir = blog_dir.join("article2");
    let podcast1_dir = podcast_dir.join("episode1");

    // Mock file system directory listing
    mock_fs.expect_dir_exists()
        .with(predicate::eq(blog_dir.clone()))
        .returning(|_| Ok(true));

    mock_fs.expect_dir_exists()
        .with(predicate::eq(podcast_dir.clone()))
        .returning(|_| Ok(true));

    mock_fs.expect_dir_exists()
        .with(predicate::eq(article1_dir.join("index.mdx")))
        .returning(|_| Ok(true));

    mock_fs.expect_dir_exists()
        .with(predicate::eq(article2_dir.join("index.mdx")))
        .returning(|_| Ok(true));

    mock_fs.expect_dir_exists()
        .with(predicate::eq(podcast1_dir.join("index.mdx")))
        .returning(|_| Ok(true));

    mock_fs.expect_list_dirs()
        .with(predicate::eq(blog_dir.clone()))
        .returning(move |_| Ok(vec![article1_dir.clone(), article2_dir.clone()]));

    mock_fs.expect_list_dirs()
        .with(predicate::eq(podcast_dir.clone()))
        .returning(move |_| Ok(vec![podcast1_dir.clone()]));

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
        ..Default::default()
    };

    // Setup mock config loader
    let mut mock_config = MockConfigLoader::new();
    mock_config.expect_load_config()
        .returning(move || Ok(config.clone()));

    // Register mocks with the fixture
    fixture.fs = mock_fs;
    fixture.config = mock_config;

    // Act
    let result = find_content_files(&base_dir, None);

    // Assert
    assert!(result.is_ok(), "Finding content files should succeed");

    let content_files = result.unwrap();
    assert_eq!(content_files.len(), 3);

    // Check that all expected content directories are included
    let paths: Vec<String> = content_files.iter()
        .map(|path| path.file_name().unwrap().to_string_lossy().to_string())
        .collect();

    assert!(paths.contains(&"article1".to_string()));
    assert!(paths.contains(&"article2".to_string()));
    assert!(paths.contains(&"episode1".to_string()));
}

#[test]
fn test_find_content_files_specific_topic() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let base_dir = fixture.path().join("content");
    let blog_dir = base_dir.join("blog");

    let article1_dir = blog_dir.join("article1");
    let article2_dir = blog_dir.join("article2");

    // Mock file system directory listing
    mock_fs.expect_dir_exists()
        .with(predicate::eq(blog_dir.clone()))
        .returning(|_| Ok(true));

    mock_fs.expect_dir_exists()
        .with(predicate::eq(article1_dir.join("index.mdx")))
        .returning(|_| Ok(true));

    mock_fs.expect_dir_exists()
        .with(predicate::eq(article2_dir.join("index.mdx")))
        .returning(|_| Ok(true));

    mock_fs.expect_list_dirs()
        .with(predicate::eq(blog_dir.clone()))
        .returning(move |_| Ok(vec![article1_dir.clone(), article2_dir.clone()]));

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
        ..Default::default()
    };

    // Setup mock config loader
    let mut mock_config = MockConfigLoader::new();
    mock_config.expect_load_config()
        .returning(move || Ok(config.clone()));

    // Register mocks with the fixture
    fixture.fs = mock_fs;
    fixture.config = mock_config;

    // Act
    let result = find_content_files(&base_dir, Some("blog"));

    // Assert
    assert!(result.is_ok(), "Finding content files should succeed");

    let content_files = result.unwrap();
    assert_eq!(content_files.len(), 2);

    // Check that only blog content is included
    let paths: Vec<String> = content_files.iter()
        .map(|path| path.file_name().unwrap().to_string_lossy().to_string())
        .collect();

    assert!(paths.contains(&"article1".to_string()));
    assert!(paths.contains(&"article2".to_string()));
}

#[test]
fn test_find_content_files_invalid_topic() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();

    // Define test paths
    let base_dir = fixture.path().join("content");

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
    fixture.config = mock_config;

    // Act
    let result = find_content_files(&base_dir, Some("nonexistent-topic"));

    // Assert
    assert!(result.is_err(), "Finding content files with invalid topic should fail");
    assert!(result.unwrap_err().to_string().contains("Topic not found"));
}

#[test]
fn test_find_content_by_slug_found() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let base_dir = fixture.path().join("content");
    let blog_dir = base_dir.join("blog");

    let article1_dir = blog_dir.join("test-article");
    let article2_dir = blog_dir.join("another-article");

    // Mock file system directory listing
    mock_fs.expect_dir_exists()
        .with(predicate::eq(blog_dir.clone()))
        .returning(|_| Ok(true));

    mock_fs.expect_dir_exists()
        .with(predicate::eq(article1_dir.join("index.mdx")))
        .returning(|_| Ok(true));

    mock_fs.expect_dir_exists()
        .with(predicate::eq(article2_dir.join("index.mdx")))
        .returning(|_| Ok(true));

    mock_fs.expect_list_dirs()
        .with(predicate::eq(blog_dir.clone()))
        .returning(move |_| Ok(vec![article1_dir.clone(), article2_dir.clone()]));

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
    fixture.fs = mock_fs;
    fixture.config = mock_config;

    // Act
    let result = find_content_by_slug(&base_dir, "test-article", None);

    // Assert
    assert!(result.is_ok(), "Finding content by slug should succeed");

    let content_path = result.unwrap();
    assert_eq!(content_path.file_name().unwrap().to_string_lossy(), "test-article");
}

#[test]
fn test_find_content_by_slug_not_found() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let base_dir = fixture.path().join("content");
    let blog_dir = base_dir.join("blog");

    let article_dir = blog_dir.join("test-article");

    // Mock file system directory listing
    mock_fs.expect_dir_exists()
        .with(predicate::eq(blog_dir.clone()))
        .returning(|_| Ok(true));

    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_dir.join("index.mdx")))
        .returning(|_| Ok(true));

    mock_fs.expect_list_dirs()
        .with(predicate::eq(blog_dir.clone()))
        .returning(move |_| Ok(vec![article_dir.clone()]));

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
    fixture.fs = mock_fs;
    fixture.config = mock_config;

    // Act
    let result = find_content_by_slug(&base_dir, "nonexistent-article", None);

    // Assert
    assert!(result.is_err(), "Finding nonexistent content should fail");
    assert!(result.unwrap_err().to_string().contains("Content not found"));
}

#[test]
fn test_find_content_by_slug_with_topic() {
    // Arrange
    let mut fixture = TestFixture::new().unwrap();
    let mut mock_fs = MockFileSystem::new();

    // Define test paths
    let base_dir = fixture.path().join("content");
    let blog_dir = base_dir.join("blog");
    let podcast_dir = base_dir.join("podcast");

    let article_dir = blog_dir.join("test-article");
    let podcast_dir_same_slug = podcast_dir.join("test-article");

    // Mock file system directory listing
    mock_fs.expect_dir_exists()
        .with(predicate::eq(blog_dir.clone()))
        .returning(|_| Ok(true));

    mock_fs.expect_dir_exists()
        .with(predicate::eq(article_dir.join("index.mdx")))
        .returning(|_| Ok(true));

    mock_fs.expect_list_dirs()
        .with(predicate::eq(blog_dir.clone()))
        .returning(move |_| Ok(vec![article_dir.clone()]));

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
        ..Default::default()
    };

    // Setup mock config loader
    let mut mock_config = MockConfigLoader::new();
    mock_config.expect_load_config()
        .returning(move || Ok(config.clone()));

    // Register mocks with the fixture
    fixture.fs = mock_fs;
    fixture.config = mock_config;

    // Act
    let result = find_content_by_slug(&base_dir, "test-article", Some("blog"));

    // Assert
    assert!(result.is_ok(), "Finding content by slug and topic should succeed");

    let content_path = result.unwrap();
    assert_eq!(content_path.file_name().unwrap().to_string_lossy(), "test-article");
    assert!(content_path.starts_with(&blog_dir));
}