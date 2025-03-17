use common_errors::{WritingError, Result};
use common_test_utils::{TestFixture, MockFs};
use common_validation::{validate_content_path, content_exists};
use proptest::prelude::*;
use std::path::{Path, PathBuf};

// Helper to create a test config fixture with mock topics
fn setup_mock_config_fixture() -> TestFixture {
    let fixture = TestFixture::new().unwrap();
    fixture.with_config(r#"
    content:
      base_dir: "/mock/content"
      topics:
        blog:
          directory: "blog"
          title: "Blog Posts"
          description: "Technical articles and tutorials"
        creativity:
          directory: "creativity"
          title: "Creative Writing"
          description: "Stories, poems, and other creative works"
        newsletter:
          directory: "newsletter"
          title: "Newsletter"
          description: "Regular updates and newsletters"
    "#).unwrap();
    fixture
}

// Helper to create a mock filesystem with content
fn setup_mock_fs() -> MockFs {
    let mock_fs = MockFs::new();
    
    // Create mock content directories and files
    mock_fs.create_dir_all("/mock/content/blog/test-post").unwrap();
    mock_fs.create_file("/mock/content/blog/test-post/index.md", "---\ntitle: Test\n---").unwrap();
    
    mock_fs.create_dir_all("/mock/content/creativity/story").unwrap();
    mock_fs.create_file("/mock/content/creativity/story/index.mdx", "---\ntitle: Story\n---").unwrap();
    
    mock_fs.create_dir_all("/mock/content/newsletter/issue-1").unwrap();
    mock_fs.create_file("/mock/content/newsletter/issue-1/index.md", "---\ntitle: Issue 1\n---").unwrap();
    
    mock_fs
}

#[test]
fn test_validate_content_path_with_valid_topic() {
    let _fixture = setup_mock_config_fixture();
    
    // Test with a valid topic
    let path = validate_content_path("test-post", Some("blog")).unwrap();
    assert!(path.to_string_lossy().contains("blog/test-post/index.md"));
    
    // Test with another valid topic
    let path = validate_content_path("story", Some("creativity")).unwrap();
    assert!(path.to_string_lossy().contains("creativity/story/index.md"));
}

#[test]
fn test_validate_content_path_with_default_topic() {
    let _fixture = setup_mock_config_fixture();
    
    // Test with no topic specified (should use the first topic - blog)
    let path = validate_content_path("test-post", None).unwrap();
    assert!(path.to_string_lossy().contains("blog/test-post/index.md"));
}

#[test]
fn test_validate_content_path_with_invalid_topic() {
    let _fixture = setup_mock_config_fixture();
    
    // Test with an invalid topic
    let result = validate_content_path("test-post", Some("nonexistent"));
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Invalid topic: nonexistent"));
}

#[test]
fn test_find_content_path_with_existing_content() {
    let _fixture = setup_mock_config_fixture();
    let _mock_fs = setup_mock_fs();
    
    // Test finding existing content
    let path = common_validation::find_content_path("test-post", Some("blog")).unwrap();
    assert!(path.to_string_lossy().contains("blog/test-post/index.md"));
    
    // Test finding existing mdx content
    let path = common_validation::find_content_path("story", Some("creativity")).unwrap();
    assert!(path.to_string_lossy().contains("creativity/story/index.mdx"));
}

#[test]
fn test_find_content_path_with_nonexistent_content() {
    let _fixture = setup_mock_config_fixture();
    let _mock_fs = setup_mock_fs();
    
    // Test finding non-existent content
    let result = common_validation::find_content_path("nonexistent", Some("blog"));
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Content not found"));
}

#[test]
fn test_find_content_path_across_topics() {
    let _fixture = setup_mock_config_fixture();
    let _mock_fs = setup_mock_fs();
    
    // Test finding content without specifying a topic
    let path = common_validation::find_content_path("test-post", None).unwrap();
    assert!(path.to_string_lossy().contains("blog/test-post/index.md"));
    
    // Another content item in a different topic
    let path = common_validation::find_content_path("story", None).unwrap();
    assert!(path.to_string_lossy().contains("creativity/story/index.mdx"));
}

#[test]
fn test_content_exists_with_existing_content() {
    let _fixture = setup_mock_config_fixture();
    let _mock_fs = setup_mock_fs();
    
    // Test checking if existing content exists
    let exists = content_exists("test-post", Some("blog")).unwrap();
    assert!(exists);
    
    // Test checking if existing content exists without specifying topic
    let exists = content_exists("test-post", None).unwrap();
    assert!(exists);
}

#[test]
fn test_content_exists_with_nonexistent_content() {
    let _fixture = setup_mock_config_fixture();
    let _mock_fs = setup_mock_fs();
    
    // Test checking if non-existent content exists
    let exists = content_exists("nonexistent", Some("blog")).unwrap();
    assert!(!exists);
    
    // Test checking if non-existent content exists without specifying topic
    let exists = content_exists("nonexistent", None).unwrap();
    assert!(!exists);
}

proptest! {
    // Test with various slug patterns
    #[test]
    fn test_validate_content_path_with_various_slugs(slug in "[a-z0-9-]{1,50}") {
        let _fixture = setup_mock_config_fixture();
        
        let path = validate_content_path(&slug, Some("blog")).unwrap();
        prop_assert!(path.to_string_lossy().contains(&format!("blog/{}/index.md", slug)));
    }
    
    // Test with various topic and slug combinations
    #[test]
    fn test_validate_content_path_with_various_topics_and_slugs(
        slug in "[a-z0-9-]{1,50}",
        topic_idx in 0usize..3
    ) {
        let _fixture = setup_mock_config_fixture();
        
        let topics = ["blog", "creativity", "newsletter"];
        let topic = topics[topic_idx % topics.len()];
        
        let path = validate_content_path(&slug, Some(topic)).unwrap();
        prop_assert!(path.to_string_lossy().contains(&format!("{}/{}/index.md", topic, slug)));
    }
}

// Edge case tests
#[test]
fn test_path_with_special_characters() {
    let _fixture = setup_mock_config_fixture();
    
    // Test with slug containing special characters (should be validated before reaching this function)
    let path = validate_content_path("test-post-with-hyphens", Some("blog")).unwrap();
    assert!(path.to_string_lossy().contains("blog/test-post-with-hyphens/index.md"));
}

#[test]
fn test_path_with_nested_directories() {
    let _fixture = setup_mock_config_fixture();
    let mut mock_fs = MockFs::new();
    
    // Create mock config with nested topic directories
    _fixture.with_config(r#"
    content:
      base_dir: "/mock/content"
      topics:
        technical:
          directory: "tech/tutorials"
          title: "Tech Tutorials"
        creativity:
          directory: "writing/creative"
          title: "Creative Writing"
    "#).unwrap();
    
    // Create nested directories
    mock_fs.create_dir_all("/mock/content/tech/tutorials").unwrap();
    mock_fs.create_dir_all("/mock/content/writing/creative").unwrap();
    
    // Test with nested directory topics
    let path = validate_content_path("rust-guide", Some("technical")).unwrap();
    assert!(path.to_string_lossy().contains("tech/tutorials/rust-guide/index.md"));
    
    let path = validate_content_path("short-story", Some("creativity")).unwrap();
    assert!(path.to_string_lossy().contains("writing/creative/short-story/index.md"));
}

#[test]
fn test_with_missing_config() {
    // Create an empty fixture without proper config
    let fixture = TestFixture::new().unwrap();
    fixture.with_config(r#"
    # Intentionally empty config
    "#).unwrap();
    
    // Test with missing config (no topics configured)
    let result = validate_content_path("test-post", None);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("No topics configured") || 
            err.to_string().contains("Failed to load configuration"));
}

#[test]
fn test_with_invalid_config_structure() {
    // Create a fixture with invalid config structure
    let fixture = TestFixture::new().unwrap();
    fixture.with_config(r#"
    content:
      base_dir: "/mock/content"
      # Missing topics section
    "#).unwrap();
    
    // Test with invalid config structure
    let result = validate_content_path("test-post", None);
    assert!(result.is_err());
}

// New tests for cross-platform path handling and path normalization edge cases

#[test]
fn test_path_normalization_with_different_separators() {
    let _fixture = setup_mock_config_fixture();
    
    // Create mock config with both unix and windows-style paths
    _fixture.with_config(r#"
    content:
      base_dir: "/mock/content"
      topics:
        windows_style:
          directory: "windows\\style\\path"
          title: "Windows Style Path"
        unix_style:
          directory: "unix/style/path"
          title: "Unix Style Path"
    "#).unwrap();
    
    // Test with windows-style path separators
    let path = validate_content_path("test-doc", Some("windows_style")).unwrap();
    
    // Regardless of the input format, the output should be normalized to the platform's standard
    #[cfg(windows)]
    assert!(path.to_string_lossy().contains("windows\\style\\path\\test-doc\\index.md"));
    
    #[cfg(not(windows))]
    assert!(path.to_string_lossy().contains("windows/style/path/test-doc/index.md"));
    
    // Test with unix-style path separators
    let path = validate_content_path("test-doc", Some("unix_style")).unwrap();
    
    #[cfg(windows)]
    assert!(path.to_string_lossy().contains("unix\\style\\path\\test-doc\\index.md"));
    
    #[cfg(not(windows))]
    assert!(path.to_string_lossy().contains("unix/style/path/test-doc/index.md"));
}

#[test]
fn test_path_with_absolute_base_dir() {
    let fixture = TestFixture::new().unwrap();
    
    // Create config with absolute base path (different on Windows vs Unix)
    #[cfg(windows)]
    fixture.with_config(r#"
    content:
      base_dir: "C:\\absolute\\path"
      topics:
        absolute:
          directory: "docs"
          title: "Documentation"
    "#).unwrap();
    
    #[cfg(not(windows))]
    fixture.with_config(r#"
    content:
      base_dir: "/absolute/path"
      topics:
        absolute:
          directory: "docs"
          title: "Documentation"
    "#).unwrap();
    
    // Test with absolute base path
    let path = validate_content_path("test-doc", Some("absolute")).unwrap();
    
    // Verify the path is correctly joined with the absolute base path
    #[cfg(windows)]
    assert!(path.to_string_lossy().contains("C:\\absolute\\path\\docs\\test-doc\\index.md"));
    
    #[cfg(not(windows))]
    assert!(path.to_string_lossy().contains("/absolute/path/docs/test-doc/index.md"));
}

#[test]
fn test_path_with_parent_directory_references() {
    let fixture = TestFixture::new().unwrap();
    
    // Create config with parent directory references
    fixture.with_config(r#"
    content:
      base_dir: "/mock/content"
      topics:
        parent_refs:
          directory: "parent/../actual/path"
          title: "Parent References"
    "#).unwrap();
    
    // Test with path containing parent references
    let path = validate_content_path("test-doc", Some("parent_refs")).unwrap();
    
    // The path should be normalized to remove the parent reference
    assert!(path.to_string_lossy().contains("actual/path/test-doc/index.md"));
    assert!(!path.to_string_lossy().contains("parent/../"));
}

#[test]
fn test_path_with_current_directory_references() {
    let fixture = TestFixture::new().unwrap();
    
    // Create config with current directory references
    fixture.with_config(r#"
    content:
      base_dir: "/mock/content"
      topics:
        current_refs:
          directory: "./current/./path"
          title: "Current References"
    "#).unwrap();
    
    // Test with path containing current directory references
    let path = validate_content_path("test-doc", Some("current_refs")).unwrap();
    
    // The path should be normalized to remove the current directory references
    assert!(path.to_string_lossy().contains("current/path/test-doc/index.md"));
    assert!(!path.to_string_lossy().contains("./"));
}

#[test]
fn test_path_with_redundant_separators() {
    let fixture = TestFixture::new().unwrap();
    
    // Create config with redundant separators
    fixture.with_config(r#"
    content:
      base_dir: "/mock//content"
      topics:
        redundant:
          directory: "redundant///separators"
          title: "Redundant Separators"
    "#).unwrap();
    
    // Test with path containing redundant separators
    let path = validate_content_path("test-doc", Some("redundant")).unwrap();
    
    // The path should be normalized to remove redundant separators
    assert!(path.to_string_lossy().contains("redundant/separators/test-doc/index.md"));
    assert!(!path.to_string_lossy().contains("///"));
}

#[test]
fn test_path_with_mixed_case_on_case_insensitive_systems() {
    let fixture = TestFixture::new().unwrap();
    
    // Create config with mixed case directories
    fixture.with_config(r#"
    content:
      base_dir: "/mock/Content"
      topics:
        mixed_case:
          directory: "MixedCase/path"
          title: "Mixed Case"
    "#).unwrap();
    
    // Setup mock filesystem with case variations
    let mock_fs = MockFs::new();
    
    #[cfg(windows)]
    {
        // On Windows, create with one case but access with another
        mock_fs.create_dir_all("/mock/content/mixedcase/path").unwrap();
        
        // Test with mixed case path
        let path = validate_content_path("test-doc", Some("mixed_case")).unwrap();
        
        // Windows is case-insensitive, so this should work
        let normalized_path = common_fs::normalize::normalize_path(path);
        assert!(normalized_path.exists());
    }
    
    #[cfg(not(windows))]
    {
        // On Unix, these are different paths
        mock_fs.create_dir_all("/mock/Content/MixedCase/path").unwrap();
        
        // Test with the exact case
        let path = validate_content_path("test-doc", Some("mixed_case")).unwrap();
        assert!(path.to_string_lossy().contains("MixedCase/path"));
    }
}

#[test]
fn test_path_with_unicode_characters() {
    let fixture = TestFixture::new().unwrap();
    
    // Create config with Unicode characters in directory names
    fixture.with_config(r#"
    content:
      base_dir: "/mock/content"
      topics:
        unicode:
          directory: "unicøde/文件夹/путь"
          title: "Unicode Path"
    "#).unwrap();
    
    // Test with path containing Unicode characters
    let path = validate_content_path("test-doc", Some("unicode")).unwrap();
    
    // The path should contain the Unicode characters
    assert!(path.to_string_lossy().contains("unicøde/文件夹/путь/test-doc/index.md"));
}

// Test finding content with the fs module mocked to simulate real file existence
#[test]
fn test_find_content_with_non_standard_extensions() {
    let _fixture = setup_mock_config_fixture();
    let mock_fs = MockFs::new();
    
    // Create content with non-standard extensions
    mock_fs.create_dir_all("/mock/content/blog/markdown-doc").unwrap();
    mock_fs.create_file("/mock/content/blog/markdown-doc/index.markdown", "---\ntitle: Markdown\n---").unwrap();
    
    mock_fs.create_dir_all("/mock/content/blog/mdoc-doc").unwrap();
    mock_fs.create_file("/mock/content/blog/mdoc-doc/index.mdoc", "---\ntitle: MDoc\n---").unwrap();
    
    // These should fail since we only support .md and .mdx
    let result = common_validation::find_content_path("markdown-doc", Some("blog"));
    assert!(result.is_err());
    
    let result = common_validation::find_content_path("mdoc-doc", Some("blog"));
    assert!(result.is_err());
} 