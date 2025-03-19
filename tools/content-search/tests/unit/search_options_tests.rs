//! Unit tests for SearchOptions struct and its Default implementation

use content_search::SearchOptions;
use anyhow::Result;

#[cfg(test)]
mod search_options_tests {
    use super::*;

    #[test]
    fn test_search_options_default() {
        // Create a default SearchOptions instance
        let options = SearchOptions::default();

        // Verify default values
        assert_eq!(options.query, "");
        assert_eq!(options.topic, None);
        assert_eq!(options.content_type, None);
        assert_eq!(options.tags, None);
        assert_eq!(options.limit, 20);
        assert_eq!(options.include_drafts, false);
        assert_eq!(options.title_only, false);
        assert_eq!(options.raw_query, false);
        assert_eq!(options.case_sensitive, false);
        assert_eq!(options.include_metadata, true);
    }

    #[test]
    fn test_search_options_with_custom_values() {
        // Create a SearchOptions instance with custom values
        let options = SearchOptions {
            query: "test query".to_string(),
            topic: Some("blog".to_string()),
            content_type: Some("article".to_string()),
            tags: Some(vec!["rust".to_string(), "programming".to_string()]),
            limit: 50,
            include_drafts: true,
            title_only: true,
            raw_query: true,
            case_sensitive: true,
            include_metadata: false,
        };

        // Verify custom values
        assert_eq!(options.query, "test query");
        assert_eq!(options.topic, Some("blog".to_string()));
        assert_eq!(options.content_type, Some("article".to_string()));
        assert_eq!(
            options.tags,
            Some(vec!["rust".to_string(), "programming".to_string()])
        );
        assert_eq!(options.limit, 50);
        assert_eq!(options.include_drafts, true);
        assert_eq!(options.title_only, true);
        assert_eq!(options.raw_query, true);
        assert_eq!(options.case_sensitive, true);
        assert_eq!(options.include_metadata, false);
    }
}