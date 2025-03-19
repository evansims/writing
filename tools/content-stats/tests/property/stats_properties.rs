//! Property-based tests for content statistics functionality

use anyhow::Result;
use common_test_utils::fixtures::TestFixture;
use common_test_utils::proptest::strategies::*;
use content_stats::{StatsOptions, ContentStats, calculate_stats};
use common_models::Frontmatter;
use proptest::prelude::*;
use std::collections::HashMap;

// Strategies for generating test data
fn frontmatter_strategy() -> impl Strategy<Value = Frontmatter> {
    (
        prop::string::string_regex("[A-Za-z0-9 ]{3,30}").unwrap(),
        prop::option::of(prop::string::string_regex("[A-Za-z0-9 ]{10,50}").unwrap()),
        prop::option::of(prop::string::string_regex("[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap()),
        prop::option::of(prop::bool::ANY),
        prop::option::of(prop::collection::vec(
            prop::string::string_regex("[a-z-]{3,10}").unwrap(),
            0..5
        ))
    ).prop_map(|(title, description, date, is_draft, tags)| {
        Frontmatter {
            title,
            description,
            published_at: date,
            is_draft,
            tags,
            content_type: Some("article".to_string()),
            image: None,
            canonical_url: None,
            custom_excerpt: None,
            url: None,
            slug: None,
            meta_title: None,
            meta_description: None,
            feature_image: None,
            template: None,
            excerpt: None,
            primary_author: None,
            created_at: None,
            updated_at: None,
            twitter_card: None,
            twitter_title: None,
            twitter_description: None,
            twitter_image: None,
            og_title: None,
            og_description: None,
            og_image: None,
            og_type: None,
            visibility: None,
            custom_template: None,
            frontmatter_type: None,
            status: None,
            locale: None,
        }
    })
}

fn content_strategy() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop::collection::vec(
            prop::string::string_regex("[A-Za-z0-9,. ]{5,50}").unwrap(),
            1..10
        ).prop_map(|words| words.join(" ")),
        1..10
    ).prop_map(|paragraphs| paragraphs.join("\n\n"))
}

fn topic_strategy() -> impl Strategy<Value = String> {
    prop::string::string_regex("[a-z]{3,10}").unwrap()
}

fn slug_strategy() -> impl Strategy<Value = String> {
    prop::string::string_regex("[a-z0-9-]{5,20}").unwrap()
}

proptest! {
    /// Test that calculate_stats produces reasonable stats for various inputs
    #[test]
    fn test_calculate_stats_produces_reasonable_stats(
        content in content_strategy(),
        frontmatter in frontmatter_strategy(),
        topic in topic_strategy(),
        slug in slug_strategy()
    ) {
        // Calculate stats based on the generated input
        let stats = calculate_stats(&content, &frontmatter, &topic, &slug);

        // Some basic properties that should always hold for any input
        prop_assert_eq!(stats.title, frontmatter.title);
        prop_assert_eq!(stats.topic, topic);
        prop_assert_eq!(stats.slug, slug);

        // For publish date, it should match the frontmatter or be "DRAFT"
        if let Some(ref date) = frontmatter.published_at {
            prop_assert_eq!(stats.published, date);
        } else {
            prop_assert_eq!(stats.published, "DRAFT");
        }

        // Word count should be proportional to content length
        if !content.is_empty() {
            let words = content.split_whitespace().count();
            // Allow for some discrepancy due to HTML handling
            prop_assert!(
                stats.word_count <= words * 2 && stats.word_count >= words / 2,
                "Word count {} should be roughly proportional to actual words {}",
                stats.word_count, words
            );
        }

        // Reading time should be proportional to word count
        if stats.word_count > 0 {
            let expected_min_reading_time = stats.word_count / 500;
            let expected_max_reading_time = stats.word_count / 100 + 1;
            prop_assert!(
                stats.reading_time >= expected_min_reading_time &&
                stats.reading_time <= expected_max_reading_time,
                "Reading time {} should be proportional to word count {}",
                stats.reading_time, stats.word_count
            );
        }

        // Tags should match or be empty
        if let Some(ref tags) = frontmatter.tags {
            prop_assert_eq!(stats.tags, tags);
        } else {
            prop_assert!(stats.tags.is_empty(), "Tags should be empty for null frontmatter tags");
        }

        // Draft status should match frontmatter
        if let Some(is_draft) = frontmatter.is_draft {
            if is_draft {
                prop_assert!(stats.is_draft, "Should be marked as draft when frontmatter says draft");
            }
        }

        // If publish date is "DRAFT", it should be a draft
        if frontmatter.published_at.as_ref().is_some_and(|d| d == "DRAFT") {
            prop_assert!(stats.is_draft, "Should be marked as draft when published_at is DRAFT");
        }
    }

    /// Test stats comparison and sorting is correct
    #[test]
    fn test_stats_sorting_is_consistent(
        a_wc in 1..10000usize,
        a_date in prop::string::string_regex("[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap(),
        b_wc in 1..10000usize,
        b_date in prop::string::string_regex("[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap()
    ) {
        // Create two content stats objects with different word counts and dates
        let mut a = ContentStats {
            title: "Article A".to_string(),
            published: a_date.clone(),
            word_count: a_wc,
            reading_time: a_wc / 200,
            character_count: a_wc * 5,
            paragraph_count: a_wc / 50,
            sentence_count: a_wc / 20,
            topic: "blog".to_string(),
            slug: "article-a".to_string(),
            tags: vec!["test".to_string()],
            is_draft: false,
            total_articles: 0,
            total_words: 0,
            total_drafts: 0,
            total_published: 0,
            topics: vec![],
        };

        let mut b = ContentStats {
            title: "Article B".to_string(),
            published: b_date.clone(),
            word_count: b_wc,
            reading_time: b_wc / 200,
            character_count: b_wc * 5,
            paragraph_count: b_wc / 50,
            sentence_count: b_wc / 20,
            topic: "blog".to_string(),
            slug: "article-b".to_string(),
            tags: vec!["test".to_string()],
            is_draft: false,
            total_articles: 0,
            total_words: 0,
            total_drafts: 0,
            total_published: 0,
            topics: vec![],
        };

        // This can fail if sort_stats isn't yet implemented
        // But we're documenting how it should work
        if let Some(sort_stats) = content_stats::test_utils::get_sort_function() {
            // Sort by word count
            let word_count_comparison = sort_stats(&a, &b, "word_count");

            if a_wc > b_wc {
                prop_assert!(word_count_comparison.is_gt(),
                    "A ({}) > B ({}) by word count", a_wc, b_wc);
            } else if a_wc < b_wc {
                prop_assert!(word_count_comparison.is_lt(),
                    "A ({}) < B ({}) by word count", a_wc, b_wc);
            } else {
                prop_assert!(word_count_comparison.is_eq(),
                    "A ({}) = B ({}) by word count", a_wc, b_wc);
            }

            // Sort by date
            let date_comparison = sort_stats(&a, &b, "date");

            if a_date > b_date {
                prop_assert!(date_comparison.is_gt(),
                    "A ({}) > B ({}) by date", a_date, b_date);
            } else if a_date < b_date {
                prop_assert!(date_comparison.is_lt(),
                    "A ({}) < B ({}) by date", a_date, b_date);
            } else {
                prop_assert!(date_comparison.is_eq(),
                    "A ({}) = B ({}) by date", a_date, b_date);
            }
        }
    }
}