use content_build::process_content;
use common_test_utils::fixtures::TestFixture;
use common_test_utils::mocks::MockFileSystem;
use mockall::predicate;
use proptest::prelude::*;
use std::path::PathBuf;
use std::collections::HashMap;

// Strategy for valid content slugs
fn valid_slug_strategy() -> impl Strategy<Value = String> {
    r"[a-z0-9][a-z0-9\-]{1,50}".prop_map(String::from)
}

// Strategy for valid frontmatter fields
fn valid_title_strategy() -> impl Strategy<Value = String> {
    r#"[A-Za-z0-9\s\.\,\-\:\;\"\'\!\?]{1,100}"#.prop_map(String::from)
}

fn valid_description_strategy() -> impl Strategy<Value = String> {
    r#"[A-Za-z0-9\s\.\,\-\:\;\"\'\!\?]{1,200}"#.prop_map(String::from)
}

fn valid_date_strategy() -> impl Strategy<Value = String> {
    r"\d{4}\-\d{2}\-\d{2}".prop_map(String::from)
}

// Strategy for valid markdown content
fn valid_markdown_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec("[A-Za-z0-9\\s\\.\\,\\-\\:\\;\\\"\\\'\\!\\?\\#\\*\\(\\)\\[\\]]{1,50}".prop_map(String::from), 1..10)
        .prop_map(|lines| lines.join("\n"))
}

// Generate valid MDX content with frontmatter
fn generate_valid_mdx(
    title: String,
    description: Option<String>,
    published_at: Option<String>,
    is_draft: Option<bool>,
    content: String,
) -> String {
    let mut frontmatter = String::from("---\n");
    frontmatter.push_str(&format!("title: \"{}\"\n", title));

    if let Some(desc) = description {
        frontmatter.push_str(&format!("description: \"{}\"\n", desc));
    }

    if let Some(date) = published_at {
        frontmatter.push_str(&format!("published_at: \"{}\"\n", date));
    }

    if let Some(draft) = is_draft {
        frontmatter.push_str(&format!("is_draft: {}\n", draft));
    }

    frontmatter.push_str("---\n");
    frontmatter.push_str(&content);

    frontmatter
}

proptest! {
    #[test]
    fn test_process_content_with_valid_input(
        slug in valid_slug_strategy(),
        title in valid_title_strategy(),
        description in valid_description_strategy(),
        published_at in valid_date_strategy(),
        content in valid_markdown_strategy(),
    ) {
        // Setup test fixture
        let mut fixture = TestFixture::new().unwrap();
        let mut mock_fs = MockFileSystem::new();

        // Define test paths
        let content_dir = fixture.path().join(format!("content/blog/{}", slug));
        let index_file = content_dir.join("index.mdx");

        // Generate valid MDX content
        let mdx_content1 = generate_valid_mdx(
            title.clone(),
            Some(description.clone()),
            Some(published_at.clone()),
            None,
            content.clone(),
        );

        // Mock file system checks
        mock_fs.expect_dir_exists()
            .with(predicate::eq(index_file.clone()))
            .returning(|_| Ok(true));

        mock_fs.expect_read_file()
            .with(predicate::eq(index_file.clone()))
            .returning(move |_| Ok("test content".to_string()));

        // Register mock file system
        fixture.fs = mock_fs;

        // Process the content
        let result = process_content(&content_dir, false);

        // Verify result
        prop_assert!(result.is_ok(), "Processing content should succeed with valid input");

        let article = result.unwrap();
        prop_assert_eq!(article.frontmatter.title, title, "Title should match input");
        prop_assert_eq!(article.frontmatter.description.as_ref().unwrap(), &description, "Description should match input");
        prop_assert_eq!(article.frontmatter.published_at.as_ref().unwrap(), &published_at, "Published date should match input");
        prop_assert_eq!(article.slug, slug, "Slug should match directory name");
        prop_assert!(article.content.contains(&content), "Content should match input");
        prop_assert!(article.word_count.is_some(), "Word count should be calculated");
        prop_assert!(article.reading_time.is_some(), "Reading time should be calculated");
    }

    #[test]
    fn test_process_content_with_draft_content(
        slug in valid_slug_strategy(),
        title in valid_title_strategy(),
        content in valid_markdown_strategy(),
    ) {
        // Setup test fixture
        let mut fixture = TestFixture::new().unwrap();
        let mut mock_fs = MockFileSystem::new();

        // Define test paths
        let content_dir = fixture.path().join(format!("content/blog/{}", slug));
        let index_file = content_dir.join("index.mdx");

        // Generate MDX content with draft flag
        let mdx_content1 = generate_valid_mdx(
            title.clone(),
            None,
            None,
            Some(true),
            content.clone(),
        );

        // Mock file system checks
        mock_fs.expect_dir_exists()
            .with(predicate::eq(index_file.clone()))
            .returning(|_| Ok(true));

        mock_fs.expect_read_file()
            .with(predicate::eq(index_file.clone()))
            .returning(move |_| Ok("test content".to_string()));

        // Register mock file system
        fixture.fs = mock_fs;

        // Process the content with drafts excluded
        let result_excluded = process_content(&content_dir, false);

        // Should fail when drafts are excluded
        prop_assert!(result_excluded.is_err(), "Processing draft content should fail when drafts are excluded");
        prop_assert!(result_excluded.unwrap_err().to_string().contains("draft"), "Error message should mention draft");

        // Re-setup for the second test
        let mut fixture2 = TestFixture::new().unwrap();
        let mut mock_fs2 = MockFileSystem::new();

        // Mock file system checks again
        mock_fs2.expect_dir_exists()
            .with(predicate::eq(index_file.clone()))
            .returning(|_| Ok(true));

        mock_fs2.expect_read_file()
            .with(predicate::eq(index_file.clone()))
            .returning(move |_| Ok("test content".to_string()));

        // Register mock file system
        fixture2.fs = mock_fs2;

        // Process the content with drafts included
        let result_included = process_content(&content_dir, true);

        // Should succeed when drafts are included
        prop_assert!(result_included.is_ok(), "Processing draft content should succeed when drafts are included");

        let article = result_included.unwrap();
        prop_assert_eq!(article.frontmatter.title, title, "Title should match input");
        prop_assert!(article.frontmatter.is_draft.unwrap(), "Draft flag should be set to true");
    }

    #[test]
    fn test_reading_time_calculation(content in prop::collection::vec("[a-zA-Z0-9]{1,10}", 1..2000)) {
        // Setup test fixture
        let mut fixture = TestFixture::new().unwrap();
        let mut mock_fs = MockFileSystem::new();

        // Define test content with varying word counts
        let content_text = content.join(" "); // Join words with spaces
        let word_count = content.len();

        // Define test paths
        let content_file = fixture.path().join("content/blog/test-article/index.mdx");

        // Generate MDX content with the specified word count
        let mdx_content = format!(r#"---
title: "Test Article"
---
{}"#, content_text);

        // Mock file system checks
        mock_fs.expect_dir_exists()
            .with(predicate::eq(content_file.clone()))
            .returning(|_| Ok(true));

        mock_fs.expect_read_file()
            .with(predicate::eq(content_file.clone()))
            .returning(move |_| Ok("test content".to_string()));

        // Register mock file system
        fixture.fs = mock_fs;

        // Process the content
        let result = process_content(&content_file, false);

        // Verify result
        prop_assert!(result.is_ok(), "Processing content should succeed");

        let article = result.unwrap();

        // Calculate expected reading time (200 words per minute, rounded up)
        let expected_reading_time = (word_count as f64 / 200.0).ceil() as u32;

        // Check word count and reading time
        prop_assert_eq!(article.word_count.unwrap(), word_count, "Word count should match input");
        prop_assert_eq!(article.reading_time.unwrap(), expected_reading_time, "Reading time should be calculated properly");
    }
}