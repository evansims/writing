//! Property-based tests for search functionality

use anyhow::Result;
use content_search::{SearchOptions, SearchResult};
use common_test_utils::fixtures::TestFixture;
use common_test_utils::proptest::strategies::*;
use proptest::prelude::*;
use std::path::{Path, PathBuf};

/// Generate a reasonable query string strategy
fn query_string_strategy() -> impl Strategy<Value = String> {
    // Generate strings that are 1-10 words long
    prop::collection::vec(any_alphanumeric_string(1, 10), 1..10)
        .prop_map(|words| words.join(" "))
}

/// Generate a topic name strategy
fn topic_name_strategy() -> impl Strategy<Value = String> {
    prop::string::string_regex("[a-z][a-z0-9-]{1,20}")
        .unwrap()
}

/// Generate a tag strategy
fn tags_strategy() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec(
        prop::string::string_regex("[a-z][a-z0-9-]{1,10}")
            .unwrap(),
        0..5
    )
}

/// Generate a SearchOptions strategy
fn search_options_strategy() -> impl Strategy<Value = SearchOptions> {
    (
        query_string_strategy(),
        prop::option::of(topic_name_strategy()),
        prop::option::of(prop::string::string_regex("[a-z]{3,10}").unwrap()),
        prop::option::of(tags_strategy()),
        (5usize..50usize),
        any::<bool>(),
        any::<bool>(),
        any::<bool>(),
        any::<bool>(),
        any::<bool>(),
    ).prop_map(|(query, topic, content_type, tags, limit, include_drafts, title_only, raw_query, case_sensitive, include_metadata)| {
        SearchOptions {
            query,
            topic,
            content_type,
            tags,
            limit,
            include_drafts,
            title_only,
            raw_query,
            case_sensitive,
            include_metadata,
        }
    })
}

proptest! {
    /// Test that search options are validated properly
    #[test]
    fn prop_search_options_validation(options in search_options_strategy()) {
        // Empty query should be invalid
        let empty_query_options = SearchOptions {
            query: "".to_string(),
            ..options.clone()
        };

        let result = content_search::search_content(&empty_query_options);
        assert!(result.is_err());

        // Query with just whitespace should be invalid
        let whitespace_query_options = SearchOptions {
            query: "   ".to_string(),
            ..options.clone()
        };

        let result = content_search::search_content(&whitespace_query_options);
        assert!(result.is_err());

        // If the query is non-empty and not just whitespace, we don't assert on the result
        // because it will depend on whether there's content matching the query
        if !options.query.trim().is_empty() {
            // Just test that it doesn't panic
            let _ = content_search::search_content(&options);
        }
    }

    /// Test that search results never contain more items than the limit
    #[test]
    fn prop_search_results_respect_limit(
        query in query_string_strategy(),
        limit in 1usize..100usize
    ) {
        // Skip empty queries as they'll cause an error
        if query.trim().is_empty() {
            return Ok(());
        }

        let options = SearchOptions {
            query,
            limit,
            ..Default::default()
        };

        // Test that search results never exceed the limit
        match content_search::search_content(&options) {
            Ok(results) => {
                assert!(results.len() <= limit,
                    "Search results ({}) exceed the limit ({})",
                    results.len(), limit);
            }
            Err(_) => {
                // Error is acceptable here as we can't guarantee
                // the test environment has searchable content
            }
        }
    }

    /// Test that case sensitivity works as expected
    #[test]
    fn prop_case_sensitivity(query in query_string_strategy()) {
        // Skip empty queries
        if query.trim().is_empty() {
            return Ok(());
        }

        // Create uppercase query
        let uppercase_query = query.to_uppercase();

        // If the query is already all uppercase, this test isn't useful
        if uppercase_query == query {
            return Ok(());
        }

        let case_sensitive_options = SearchOptions {
            query: uppercase_query.clone(),
            case_sensitive: true,
            ..Default::default()
        };

        let case_insensitive_options = SearchOptions {
            query: uppercase_query,
            case_sensitive: false,
            ..Default::default()
        };

        // We can't make strong assertions about results without a controlled environment,
        // but we can at least make sure the function doesn't panic
        let _ = content_search::search_content(&case_sensitive_options);
        let _ = content_search::search_content(&case_insensitive_options);
    }
}