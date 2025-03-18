//! # Test Organization Example
//!
//! This file demonstrates the recommended test organization patterns.

use common_errors::Result;
use std::path::PathBuf;
use common_test_utils::{
    // Helper macros
    with_test_fixture, with_mock, assert_contains, assert_ok_matches,
    // Mocks
    mocks::{MockFileSystem, MockConfigLoader}
};

/// Simple system under test
pub struct ContentChecker {
    file_system: Box<dyn common_test_utils::FileSystem>,
    config_loader: Box<dyn common_test_utils::ConfigLoader>,
}

impl ContentChecker {
    /// Create a new ContentChecker
    pub fn new(
        file_system: Box<dyn common_test_utils::FileSystem>,
        config_loader: Box<dyn common_test_utils::ConfigLoader>,
    ) -> Self {
        Self { file_system, config_loader }
    }

    /// Check if content exists and is valid
    pub fn check_content(&self, slug: &str, topic: &str) -> Result<String> {
        // Load config to check if topic is valid
        let config = self.config_loader.load_config()?;
        if !config.topics.contains(&topic.to_string()) {
            return Err(common_errors::WritingError::invalid_input(
                &format!("Topic '{}' is not configured", topic)
            ));
        }

        // Check if content file exists
        let path = PathBuf::from(format!("content/{}/{}.md", topic, slug));
        if !self.file_system.file_exists(&path)? {
            return Err(common_errors::WritingError::not_found(
                &format!("Content not found: {}", path.display())
            ));
        }

        // Read content
        let content = self.file_system.read_file(&path)?;

        // Basic validation
        if !content.contains("---") {
            return Err(common_errors::WritingError::validation_error(
                "Content is missing frontmatter"
            ));
        }

        Ok(content)
    }
}

// Unit tests in the same file
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate;

    // Group tests by function
    mod check_content_tests {
        use super::*;

        // Basic success case test
        #[test]
        fn test_check_content_success() {
            // Use the with_mock macro for cleaner setup
            with_mock!(MockFileSystem, mock_fs => {
                // Setup expectations
                mock_fs.expect_file_exists()
                    .with(predicate::eq(PathBuf::from("content/blog/test-article.md")))
                    .returning(|_| Ok(true));

                mock_fs.expect_read_file()
                    .with(predicate::eq(PathBuf::from("content/blog/test-article.md")))
                    .returning(|_| Ok("---\ntitle: Test\n---\nContent".to_string()));

                // Setup second mock
                with_mock!(MockConfigLoader, mock_config => {
                    // Create test config that includes "blog" topic
                    let mut config = common_config::Config::default();
                    config.topics = vec!["blog".to_string()];

                    mock_config.expect_load_config()
                        .returning(move || Ok(config.clone()));

                    // Create the system under test
                    let checker = ContentChecker::new(
                        Box::new(mock_fs),
                        Box::new(mock_config)
                    );

                    // Act
                    let result = checker.check_content("test-article", "blog");

                    // Assert using helper macro
                    assert_ok_matches!(result, |content| content.contains("title: Test"));
                });
            });
        }

        // Error case - missing topic
        #[test]
        fn test_check_content_invalid_topic() {
            with_mock!(MockConfigLoader, mock_config => {
                // Create test config that doesn't include "blog" topic
                let config = common_config::Config::default();

                mock_config.expect_load_config()
                    .returning(move || Ok(config.clone()));

                // Mock FS shouldn't be called, so we don't need to set expectations
                let mock_fs = MockFileSystem::new();

                // Create the system under test
                let checker = ContentChecker::new(
                    Box::new(mock_fs),
                    Box::new(mock_config)
                );

                // Act
                let result = checker.check_content("test-article", "blog");

                // Assert
                assert!(result.is_err());
                let err = result.unwrap_err();
                assert_contains!(err.to_string(), "Topic 'blog' is not configured");
            });
        }

        // Error case - content not found
        #[test]
        fn test_check_content_not_found() {
            with_mock!(MockFileSystem, mock_fs => {
                // Setup expectations
                mock_fs.expect_file_exists()
                    .with(predicate::eq(PathBuf::from("content/blog/test-article.md")))
                    .returning(|_| Ok(false));

                with_mock!(MockConfigLoader, mock_config => {
                    // Create test config that includes "blog" topic
                    let mut config = common_config::Config::default();
                    config.topics = vec!["blog".to_string()];

                    mock_config.expect_load_config()
                        .returning(move || Ok(config.clone()));

                    // Create the system under test
                    let checker = ContentChecker::new(
                        Box::new(mock_fs),
                        Box::new(mock_config)
                    );

                    // Act
                    let result = checker.check_content("test-article", "blog");

                    // Assert
                    assert!(result.is_err());
                    let err = result.unwrap_err();
                    assert_contains!(err.to_string(), "Content not found");
                });
            });
        }
    }
}

/// Example integration test module - this would normally be in a separate file
#[cfg(test)]
mod integration_tests {
    use super::*;
    use common_test_utils::TestFixture;

    #[test]
    fn test_content_checker_with_real_files() {
        // Use the with_test_fixture macro for cleaner setup
        with_test_fixture!(fixture => {
            // Create test content
            let content_path = fixture.create_content(
                "blog",
                "test-post",
                "Test Post",
                false
            ).unwrap();

            // Create a real file system implementation
            let fs = common_test_utils::mocks::create_test_fs();

            // Create a config with the blog topic
            let mut config_loader = common_test_utils::mocks::create_test_config_loader();

            // Create the system under test with real implementations
            let checker = ContentChecker::new(
                Box::new(fs),
                Box::new(config_loader)
            );

            // Act
            let result = checker.check_content("test-post", "blog");

            // Assert
            assert!(result.is_ok());
            assert_contains!(result.unwrap(), "title: Test Post");
        });
    }
}

/// Example property test module - this would normally be in a separate file
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    use common_test_utils::test_property;

    // Define property test strategies
    fn valid_slugs() -> impl Strategy<Value = String> {
        "[a-z0-9-]{1,50}".prop_filter(
            "Slug must not have consecutive hyphens",
            |s| !s.contains("--")
        )
    }

    fn valid_topics() -> impl Strategy<Value = String> {
        proptest::string::string_regex("[a-z]{3,10}")
            .unwrap()
    }

    proptest! {
        #[test]
        fn test_slug_validation(slug in valid_slugs(), topic in valid_topics()) {
            // Setup mocks with valid responses
            let mut mock_fs = MockFileSystem::new();
            let content_path = format!("content/{}/{}.md", topic, slug);

            mock_fs.expect_file_exists()
                .with(predicate::always())
                .returning(|_| Ok(true));

            mock_fs.expect_read_file()
                .with(predicate::always())
                .returning(|_| Ok("---\ntitle: Test\n---\nContent".to_string()));

            let mut mock_config = MockConfigLoader::new();
            let mut config = common_config::Config::default();

            // Ensure our generated topic is in the config
            config.topics = vec![topic.clone()];

            mock_config.expect_load_config()
                .returning(move || Ok(config.clone()));

            // Create the system under test
            let checker = ContentChecker::new(
                Box::new(mock_fs),
                Box::new(mock_config)
            );

            // Act
            let result = checker.check_content(&slug, &topic);

            // Assert - this property should always hold for valid inputs
            prop_assert!(result.is_ok(), "Expected Ok for valid slug/topic, got: {:?}", result);
        }
    }

    // Example using the test_property macro
    #[test]
    fn test_various_inputs() {
        // Define valid inputs
        let valid_inputs = vec![
            ("test-article", "blog"),
            ("another-post", "tutorials"),
            ("code-example", "examples")
        ];

        // Test property: All valid inputs should produce a valid result
        test_property!(
            inputs = valid_inputs,
            property = |(slug, topic)| {
                // Create mocks
                let mut mock_fs = MockFileSystem::new();
                let content_path = format!("content/{}/{}.md", topic, slug);

                mock_fs.expect_file_exists()
                    .returning(|_| Ok(true));

                mock_fs.expect_read_file()
                    .returning(|_| Ok("---\ntitle: Test\n---\nContent".to_string()));

                let mut mock_config = MockConfigLoader::new();
                let mut config = common_config::Config::default();
                config.topics = vec![topic.to_string()];

                mock_config.expect_load_config()
                    .returning(move || Ok(config.clone()));

                // Create the checker
                let checker = ContentChecker::new(
                    Box::new(mock_fs),
                    Box::new(mock_config)
                );

                // Test the property
                checker.check_content(slug, topic).is_ok()
            },
            description = "All valid inputs should produce valid results"
        );
    }
}

fn main() {
    println!("Test organization example - run with 'cargo test' to see the tests in action");
}