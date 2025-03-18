//! # Property-Based Testing Examples
//!
//! This example shows how to use property-based testing with the test utilities.

use common_test_utils::proptest::*;
use common_test_utils::assertions::*;
use common_test_utils::with_test_environment;
use common_models::Frontmatter;
use proptest::prelude::*;
use proptest::collection::vec;

/// Example of using property-based testing to validate slug formatting
fn test_slug_formatting() {
    proptest!(|(slug in valid_slug_strategy())| {
        // Valid slugs should not contain uppercase letters
        assert!(!slug.chars().any(|c| c.is_uppercase()));

        // Valid slugs should not contain spaces
        assert!(!slug.contains(' '));

        // Valid slugs should not contain consecutive hyphens
        assert!(!slug.contains("--"));

        // Valid slugs should not start or end with a hyphen
        assert!(!slug.starts_with('-'));
        assert!(!slug.ends_with('-'));
    });
}

/// Example of using property-based testing to validate frontmatter
fn test_frontmatter_consistency() {
    proptest!(|(frontmatter in valid_frontmatter_strategy())| {
        // If a frontmatter has a published_at date, it should also have a slug
        if frontmatter.published_at.is_some() {
            assert!(frontmatter.slug.is_some(), "Published content should have a slug");
        }

        // Draft content should be marked as such
        if frontmatter.is_draft.unwrap_or(false) {
            assert_eq!(frontmatter.is_draft, Some(true), "Draft content should be marked as draft");
        }
    });
}

/// Example of using property-based testing with content collections
fn test_content_collection_operations() {
    proptest!(|(content_collection in content_collection_strategy(5, 20))| {
        // All content should have a unique slug
        let slugs: Vec<_> = content_collection.iter().map(|c| &c.slug).collect();
        let unique_slugs: std::collections::HashSet<_> = slugs.iter().collect();

        assert_eq!(slugs.len(), unique_slugs.len(), "All slugs should be unique");

        // Published content should have publication dates
        for content in &content_collection {
            if !content.is_draft {
                assert!(!content.published_at.is_empty(), "Published content should have a publication date");
            }
        }
    });
}

/// Example of using property-based testing with the test scenario
fn test_scenario_consistency() {
    proptest!(|(scenario in test_scenario_strategy())| {
        // Verify that all articles reference valid topics
        for article in &scenario.articles {
            // Every article's topic should exist in the topics map
            if !article.topic.is_empty() {
                assert!(
                    scenario.topics.contains_key(&article.topic),
                    format!("Article references unknown topic: {}", article.topic)
                );
            }
        }

        // Verify that draft articles are correctly classified
        let draft_articles = scenario.draft_articles();
        for article in &draft_articles {
            assert!(
                article.frontmatter.is_draft.unwrap_or(false),
                "Article incorrectly classified as draft"
            );
        }

        // Verify that published articles are correctly classified
        let published_articles = scenario.published_articles();
        for article in &published_articles {
            assert!(
                !article.frontmatter.is_draft.unwrap_or(false),
                "Article incorrectly classified as published"
            );
        }
    });
}

/// Example of combining property-based testing with the test environment
fn test_with_environment_and_properties() {
    // Use the with_test_environment helper to get a test environment
    with_test_environment(|env| {
        // Then use property-based testing inside the environment
        proptest!(|(slug in valid_slug_strategy(), title in valid_title_strategy())| {
            // Create content in the environment with the property-generated data
            let result = env.create_content_file("blog", &slug, &title, "Test content", false);

            // Verify the content was created successfully
            assert!(result.is_ok(), "Failed to create content: {:?}", result.err());
            let path = result.unwrap();
            assert_is_file(&path);

            // Read back the file to verify its contents
            let content = std::fs::read_to_string(&path).unwrap();
            assert_contains(&content, &title);
        });
    });
}

/// Demonstrates high-level examples of property-based testing using the test utilities.
fn main() {
    println!("Running property testing examples...");

    // Run each property test example
    test_slug_formatting();
    test_frontmatter_consistency();
    test_content_collection_operations();
    test_scenario_consistency();
    test_with_environment_and_properties();

    println!("All property testing examples passed!");
}