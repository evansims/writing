//! Unit tests for StatsOptions struct

use content_stats::StatsOptions;

#[cfg(test)]
mod stats_options_tests {
    use super::*;

    #[test]
    fn test_stats_options_creation() {
        // Test creating StatsOptions with specific values
        let options = StatsOptions {
            slug: Some("test-article".to_string()),
            topic: Some("blog".to_string()),
            include_drafts: true,
            sort_by: "date".to_string(),
            detailed: true,
        };

        assert_eq!(options.slug, Some("test-article".to_string()));
        assert_eq!(options.topic, Some("blog".to_string()));
        assert_eq!(options.include_drafts, true);
        assert_eq!(options.sort_by, "date".to_string());
        assert_eq!(options.detailed, true);
    }

    #[test]
    fn test_stats_options_with_default_values() {
        // Test creating StatsOptions with some default values
        let options = StatsOptions {
            slug: None,
            topic: None,
            include_drafts: false,
            sort_by: "word_count".to_string(),
            detailed: false,
        };

        assert_eq!(options.slug, None);
        assert_eq!(options.topic, None);
        assert_eq!(options.include_drafts, false);
        assert_eq!(options.sort_by, "word_count".to_string());
        assert_eq!(options.detailed, false);
    }

    #[test]
    fn test_stats_options_with_mixed_values() {
        // Test creating StatsOptions with a mix of default and specific values
        let options = StatsOptions {
            slug: Some("test-article".to_string()),
            topic: None,
            include_drafts: true,
            sort_by: "date".to_string(),
            detailed: false,
        };

        assert_eq!(options.slug, Some("test-article".to_string()));
        assert_eq!(options.topic, None);
        assert_eq!(options.include_drafts, true);
        assert_eq!(options.sort_by, "date".to_string());
        assert_eq!(options.detailed, false);
    }
}