use common_test_utils::TestFixture;
use content_new::{NewOptions, create_content};
use proptest::prelude::*;
use std::path::PathBuf;

/// Generate valid slug strings
fn valid_slug_strategy() -> impl Strategy<Value = String> {
    prop::string::string_regex("[a-z0-9-]{1,50}").unwrap()
}

/// Generate valid title strings
fn valid_title_strategy() -> impl Strategy<Value = String> {
    prop::string::string_regex("[A-Za-z0-9 ]{3,100}").unwrap()
}

/// Generate valid topic strings
fn valid_topic_strategy() -> impl Strategy<Value = String> {
    prop::sample::select(vec!["blog".to_string(), "creativity".to_string(), "strategy".to_string()])
}

/// Generate valid description strings
fn valid_description_strategy() -> impl Strategy<Value = String> {
    prop::string::string_regex("[A-Za-z0-9 ,.!?-]{0,200}").unwrap()
}

/// Generate valid tag lists
fn valid_tags_strategy() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec(
        prop::string::string_regex("[a-z0-9-]{1,20}").unwrap(),
        0..5
    )
}

proptest! {
    /// Property: Create content with valid options should always succeed and
    /// produce content file at the expected location
    #[test]
    fn prop_create_content_with_valid_options(
        slug in valid_slug_strategy(),
        title in valid_title_strategy(),
        topic in valid_topic_strategy(),
        description in valid_description_strategy(),
        tags in valid_tags_strategy(),
        draft in proptest::bool::ANY,
    ) {
        // We need to use a TestFixture for each test run
        let fixture = TestFixture::new().unwrap();
        fixture.register_test_config();

        // Set test mode for each test
        std::env::set_var("TEST_MODE", "1");

        // Create new options with the property generated values
        let options = NewOptions {
            slug: Some(slug.clone()),
            title: Some(title.clone()),
            topic: Some(topic.clone()),
            description: Some(description.clone()),
            template: None,
            tags: Some(tags.clone()),
            draft: Some(draft),
        };

        // Create content
        let result = create_content(&options);

        // Clean up test environment
        std::env::remove_var("TEST_MODE");

        // Assert
        prop_assert!(result.is_ok());
        let path = result.unwrap();

        // Should have created a file at the expected path
        let expected_path = PathBuf::from(fixture.path())
            .join("content")
            .join(&topic)
            .join(&slug)
            .join("index.mdx");

        prop_assert_eq!(path, expected_path);

        // File should exist
        prop_assert!(path.exists());

        // Content should contain expected elements
        let content = std::fs::read_to_string(&path).unwrap();

        prop_assert!(content.contains(&format!("title: \"{title}\"")));

        if !description.is_empty() {
            prop_assert!(content.contains(&format!("tagline: \"{description}\"")));
        }

        // If draft is true, should have draft marker
        if draft {
            prop_assert!(content.contains("date: DRAFT") || content.contains("draft: true"));
        }

        // Should contain tag references if there are tags
        if !tags.is_empty() {
            for tag in tags {
                prop_assert!(content.contains(&tag));
            }
        }
    }

    /// Property: Similar slugs should always be preserved in the final content
    #[test]
    fn prop_slug_preserved_in_content(
        slug in valid_slug_strategy(),
    ) {
        // Arrange
        let fixture = TestFixture::new().unwrap();
        fixture.register_test_config();
        std::env::set_var("TEST_MODE", "1");

        let options = NewOptions {
            slug: Some(slug.clone()),
            title: Some("Test Title".to_string()),
            topic: Some("blog".to_string()),
            description: None,
            template: None,
            tags: None,
            draft: None,
        };

        // Act
        let result = create_content(&options);
        std::env::remove_var("TEST_MODE");

        // Assert
        prop_assert!(result.is_ok());
        let path = result.unwrap();

        // Verify the file location contains the exact slug
        let parent = path.parent().unwrap();
        let dir_name = parent.file_name().unwrap().to_string_lossy();
        prop_assert_eq!(dir_name, slug);
    }

    /// Property: Draft status properly reflected in content
    #[test]
    fn prop_draft_status_preserved(
        slug in valid_slug_strategy(),
        draft in proptest::bool::ANY,
    ) {
        // Arrange
        let fixture = TestFixture::new().unwrap();
        fixture.register_test_config();
        std::env::set_var("TEST_MODE", "1");

        let options = NewOptions {
            slug: Some(slug.clone()),
            title: Some("Test Title".to_string()),
            topic: Some("blog".to_string()),
            description: None,
            template: None,
            tags: None,
            draft: Some(draft),
        };

        // Act
        let result = create_content(&options);
        std::env::remove_var("TEST_MODE");

        // Assert
        prop_assert!(result.is_ok());
        let path = result.unwrap();
        let content = std::fs::read_to_string(&path).unwrap();

        if draft {
            prop_assert!(content.contains("date: DRAFT") || content.contains("draft: true"));
        } else {
            prop_assert!(!content.contains("date: DRAFT"));
            // Should have a real date instead
            prop_assert!(content.contains("date: \"20"));
        }
    }
}