use anyhow::Result;
use common_models::TopicConfig;
use common_test_utils::{TestFixture, with_test_fixture};
use content_new::{NewOptions, create_content};
use std::path::PathBuf;

#[test]
fn test_create_content_with_valid_options() -> Result<()> {
    // Arrange: Create test fixture and register test configuration
    let fixture = TestFixture::new()?;
    fixture.register_test_config();

    // Set up test environment variable to use test mode
    std::env::set_var("TEST_MODE", "1");

    // Create valid options with required fields
    let options = NewOptions {
        slug: Some("test-slug".to_string()),
        title: Some("Test Title".to_string()),
        topic: Some("blog".to_string()),
        description: Some("Test description".to_string()),
        template: None,
        tags: Some(vec!["test".to_string(), "example".to_string()]),
        draft: Some(false),
    };

    // Act: Create content with the options
    let result = create_content(&options);

    // Assert: Content was created successfully
    assert!(result.is_ok());
    let content_path = result.unwrap();

    // Verify the file was created at the expected location
    let expected_path = PathBuf::from(fixture.path())
        .join("content")
        .join("blog")
        .join("test-slug")
        .join("index.mdx");
    assert_eq!(content_path, expected_path);

    // Verify the file exists and has the expected content
    let content = std::fs::read_to_string(&content_path)?;
    assert!(content.contains("title: \"Test Title\""));
    assert!(content.contains("tagline: \"Test description\""));
    assert!(content.contains("# Test Title"));
    assert!(content.contains("\"test\""));
    assert!(content.contains("\"example\""));

    // Clean up test environment variable
    std::env::remove_var("TEST_MODE");

    Ok(())
}

#[test]
fn test_create_content_draft_mode() -> Result<()> {
    // Arrange
    let fixture = TestFixture::new()?;
    fixture.register_test_config();
    std::env::set_var("TEST_MODE", "1");

    // Create options with draft mode enabled
    let options = NewOptions {
        slug: Some("draft-post".to_string()),
        title: Some("Draft Post".to_string()),
        topic: Some("blog".to_string()),
        description: Some("This is a draft post".to_string()),
        template: None,
        tags: None,
        draft: Some(true),
    };

    // Act
    let result = create_content(&options);

    // Assert
    assert!(result.is_ok());
    let content_path = result.unwrap();

    // Verify draft status in content
    let content = std::fs::read_to_string(&content_path)?;
    assert!(content.contains("date: DRAFT"));
    assert!(content.contains("draft: true"));

    // Clean up
    std::env::remove_var("TEST_MODE");

    Ok(())
}

#[test]
fn test_create_content_missing_required_fields() {
    // Arrange: Test various combinations of missing required fields
    let test_cases = vec![
        // Missing slug
        NewOptions {
            slug: None,
            title: Some("Test Title".to_string()),
            topic: Some("blog".to_string()),
            description: None,
            template: None,
            tags: None,
            draft: None,
        },
        // Missing title
        NewOptions {
            slug: Some("test-slug".to_string()),
            title: None,
            topic: Some("blog".to_string()),
            description: None,
            template: None,
            tags: None,
            draft: None,
        },
        // Missing topic
        NewOptions {
            slug: Some("test-slug".to_string()),
            title: Some("Test Title".to_string()),
            topic: None,
            description: None,
            template: None,
            tags: None,
            draft: None,
        },
    ];

    // Register test config
    with_test_fixture!(fixture => {
        fixture.register_test_config();
        std::env::set_var("TEST_MODE", "1");

        // Test each case
        for options in test_cases {
            // Act
            let result = create_content(&options);

            // Assert: Should fail with appropriate error
            assert!(result.is_err());
            let err = result.unwrap_err().to_string();

            // Check error message matches the missing field
            if options.slug.is_none() {
                assert!(err.contains("slug is required"));
            } else if options.title.is_none() {
                assert!(err.contains("Title is required"));
            } else if options.topic.is_none() {
                assert!(err.contains("Topic is required"));
            }
        }

        // Clean up
        std::env::remove_var("TEST_MODE");
    });
}

#[test]
fn test_create_content_nonexistent_topic() -> Result<()> {
    // Arrange
    let fixture = TestFixture::new()?;
    fixture.register_test_config();
    std::env::set_var("TEST_MODE", "1");

    // Create options with a topic that doesn't exist
    let options = NewOptions {
        slug: Some("test-slug".to_string()),
        title: Some("Test Title".to_string()),
        topic: Some("nonexistent-topic".to_string()),
        description: None,
        template: None,
        tags: None,
        draft: None,
    };

    // Act
    let result = create_content(&options);

    // Assert: Should fail with topic not found error
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Topic not found"));

    // Clean up
    std::env::remove_var("TEST_MODE");

    Ok(())
}

#[test]
fn test_create_content_with_template() -> Result<()> {
    // This test requires mocking the template loader, which would be better handled
    // in a more isolated test with explicit dependency injection.
    // For now, we can test the TEST_MODE path which doesn't use templates

    // Arrange
    let fixture = TestFixture::new()?;
    fixture.register_test_config();
    std::env::set_var("TEST_MODE", "1");

    // Create options with a template
    let options = NewOptions {
        slug: Some("template-post".to_string()),
        title: Some("Template Post".to_string()),
        topic: Some("blog".to_string()),
        description: Some("This post uses a template".to_string()),
        template: Some("article".to_string()),
        tags: None,
        draft: None,
    };

    // Act
    let result = create_content(&options);

    // Assert
    assert!(result.is_ok());

    // Clean up
    std::env::remove_var("TEST_MODE");

    Ok(())
}

#[test]
fn test_create_content_existing_content() -> Result<()> {
    // Arrange
    let fixture = TestFixture::new()?;
    fixture.register_test_config();
    std::env::set_var("TEST_MODE", "1");

    // Create options
    let options = NewOptions {
        slug: Some("existing-post".to_string()),
        title: Some("Existing Post".to_string()),
        topic: Some("blog".to_string()),
        description: None,
        template: None,
        tags: None,
        draft: None,
    };

    // Create the directory structure to simulate existing content
    let content_dir = PathBuf::from(fixture.path())
        .join("content")
        .join("blog")
        .join("existing-post");
    std::fs::create_dir_all(&content_dir)?;

    // Act: Attempt to create content with same slug
    let result = create_content(&options);

    // Assert: Should fail with content already exists error
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Content already exists"));

    // Clean up
    std::env::remove_var("TEST_MODE");

    Ok(())
}