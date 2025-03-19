//! Unit tests for the calculate_stats function

use content_stats::{calculate_stats, ContentStats};
use common_models::Frontmatter;
use anyhow::Result;

#[cfg(test)]
mod calculate_stats_tests {
    use super::*;

    fn create_test_frontmatter(title: &str, published_at: Option<&str>, is_draft: Option<bool>, tags: Option<Vec<&str>>) -> Frontmatter {
        Frontmatter {
            title: title.to_string(),
            description: Some("Test description".to_string()),
            published_at: published_at.map(|s| s.to_string()),
            updated_at: None,
            is_draft,
            tags: tags.map(|t| t.iter().map(|s| s.to_string()).collect()),
            topics: None,
            slug: None,
            featured_image_path: None,
        }
    }

    #[test]
    fn test_calculate_stats_basic() {
        // Create a simple markdown content
        let content = r#"
# Test Article

This is a test article. It contains a few sentences.

## Section 1

Here's another paragraph with some more sentences. This will help test the sentence count.

## Section 2

And a final paragraph to round things out.
"#;

        let frontmatter = create_test_frontmatter(
            "Test Article",
            Some("2023-01-01"),
            Some(false),
            Some(vec!["test", "article"])
        );

        let topic = "blog";
        let slug = "test-article";

        // Calculate stats
        let stats = calculate_stats(content, &frontmatter, topic, slug);

        // Assert results
        assert_eq!(stats.title, "Test Article");
        assert_eq!(stats.published, "2023-01-01");
        assert!(stats.word_count > 0, "Word count should be greater than 0");
        assert!(stats.reading_time > 0, "Reading time should be greater than 0");
        assert!(stats.character_count > 0, "Character count should be greater than 0");
        assert_eq!(stats.paragraph_count, 7); // Title, 3 paragraphs, 2 section headers, 1 blank line
        assert!(stats.sentence_count > 0, "Sentence count should be greater than 0");
        assert_eq!(stats.topic, "blog");
        assert_eq!(stats.slug, "test-article");
        assert_eq!(stats.tags, vec!["test", "article"]);
        assert_eq!(stats.is_draft, false);
    }

    #[test]
    fn test_calculate_stats_draft() {
        // Test with a draft article
        let content = "# Draft Article\n\nThis is a draft.";

        let frontmatter = create_test_frontmatter(
            "Draft Article",
            Some("DRAFT"),
            Some(true),
            None
        );

        let topic = "blog";
        let slug = "draft-article";

        // Calculate stats
        let stats = calculate_stats(content, &frontmatter, topic, slug);

        // Assert results
        assert_eq!(stats.title, "Draft Article");
        assert_eq!(stats.published, "DRAFT");
        assert!(stats.is_draft, "Article should be marked as draft");
        assert_eq!(stats.tags, Vec::<String>::new(), "Tags should be empty");
    }

    #[test]
    fn test_calculate_stats_long_content() {
        // Create a longer markdown content to test the stats calculations
        let mut content = String::from("# Long Test Article\n\n");

        // Add 100 paragraphs with 20 words each
        for i in 1..=100 {
            content.push_str(&format!("## Section {}\n\n", i));
            for _ in 1..=5 {
                content.push_str("This is a long paragraph with twenty words in it. It will help test the word count calculation accuracy.\n\n");
            }
        }

        let frontmatter = create_test_frontmatter(
            "Long Test Article",
            Some("2023-02-15"),
            Some(false),
            Some(vec!["long", "test", "article"])
        );

        let topic = "blog";
        let slug = "long-test-article";

        // Calculate stats
        let stats = calculate_stats(&content, &frontmatter, topic, slug);

        // Assert results - with 100 sections * 5 paragraphs * 20 words = 10,000 expected words
        // Plus headings for a bit more
        assert!(stats.word_count > 10000, "Word count should be over 10,000");
        assert!(stats.reading_time > 30, "Reading time should be over 30 minutes for this long content");
        assert!(stats.paragraph_count > 500, "Paragraph count should be over 500");
        assert!(stats.sentence_count > 1000, "Sentence count should be over 1,000");
    }

    #[test]
    fn test_calculate_stats_html_content() {
        // Test with content containing HTML tags that should be stripped
        let content = r#"
# HTML Test

This content has <strong>HTML</strong> tags that should be <em>ignored</em> in word count.

<div class="custom">
This is inside a div
</div>

<p>Another paragraph with <a href="https://example.com">a link</a> in it.</p>
"#;

        let frontmatter = create_test_frontmatter(
            "HTML Test",
            Some("2023-03-10"),
            Some(false),
            Some(vec!["html", "test"])
        );

        let topic = "blog";
        let slug = "html-test";

        // Calculate stats
        let stats = calculate_stats(content, &frontmatter, topic, slug);

        // Assert that word count ignores HTML tags
        assert_eq!(stats.tags, vec!["html", "test"]);
        assert!(stats.word_count > 0, "Word count should be greater than 0 even with HTML");
    }
}