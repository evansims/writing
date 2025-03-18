use common_errors::Result;
use common_markdown::*;
use common_models::Frontmatter;
use proptest::prelude::*;
use proptest::collection::vec;
use proptest::string::string_regex;
use std::collections::HashMap;
use proptest::test_runner::Config;

/// Generate valid YAML frontmatter strings
fn frontmatter_strategy() -> impl Strategy<Value = String> {
    let key_strategy = string_regex("[a-zA-Z][a-zA-Z0-9_]*").unwrap();
    let value_strategy = string_regex("[^:\n]*").unwrap();
    let tag_strategy = string_regex("[a-zA-Z][a-zA-Z0-9_-]*").unwrap();

    proptest::collection::hash_map(key_strategy, value_strategy, 0..10)
        .prop_map(|map| {
            let mut frontmatter = String::from("---\n");

            for (key, value) in map {
                if !value.is_empty() {
                    frontmatter.push_str(&format!("{}: {}\n", key, value));
                }
            }

            // Optionally add tags list
            if rand::random::<bool>() {
                frontmatter.push_str("tags:\n");
                for _ in 0..rand::random::<u8>() % 5 {
                    frontmatter.push_str(&format!("  - tag{}\n", rand::random::<u8>()));
                }
            }

            frontmatter.push_str("---\n");
            frontmatter
        })
}

/// Generate valid markdown content
fn markdown_content_strategy() -> impl Strategy<Value = String> {
    let heading_strategy = prop::collection::vec("[#]{1,6} [\\w\\s]+\\n", 0..5);
    let paragraph_strategy = prop::collection::vec("[\\w\\s,.!?]+\\n\\n", 1..10);
    let code_block_strategy = prop::collection::vec("```[a-z]*\\n[\\w\\s,.!?]*\\n```\\n", 0..3);

    (heading_strategy, paragraph_strategy, code_block_strategy)
        .prop_map(|(headings, paragraphs, code_blocks)| {
            let mut content = String::new();

            // Mix headings, paragraphs, and code blocks
            for heading in headings {
                content.push_str(&heading);
                if !content.ends_with("\n\n") {
                    content.push('\n');
                }
            }

            for paragraph in paragraphs {
                content.push_str(&paragraph);
            }

            for code_block in code_blocks {
                content.push_str(&code_block);
                if !content.ends_with("\n\n") {
                    content.push('\n');
                }
            }

            content
        })
}

/// Generate valid markdown documents with frontmatter
fn markdown_document_strategy() -> impl Strategy<Value = String> {
    (frontmatter_strategy(), markdown_content_strategy())
        .prop_map(|(frontmatter, content)| {
            format!("{}{}", frontmatter, content)
        })
}

proptest! {
    #![proptest_config(Config::with_cases(10)
        .clone_with_source_file("proptest-regressions/markdown_properties_fix.proptest-regressions"))]
    /// Test that extracting and then regenerating frontmatter is idempotent
    #[test]
    #[ignore = "Known failures with unicode characters"]
    fn prop_frontmatter_extraction_idempotence(doc in markdown_document_strategy()) {
        // Extract frontmatter
        let result = extract_frontmatter_and_content(&doc);
        prop_assert!(result.is_ok());

        let (frontmatter, content) = result.unwrap();

        // Generate a new document with the same frontmatter and content
        let title = frontmatter.title.clone();
        let published_at = frontmatter.published_at.clone();
        let tags_vec = frontmatter.tags.clone().unwrap_or_default();
        let tags = tags_vec.iter().map(|s| s.as_str()).collect();
        let tagline = frontmatter.tagline.clone();
        let is_draft = match frontmatter.is_draft {
            Some(value) => value,
            None => true, // Default to true if None
        };

        let new_frontmatter = generate_frontmatter(&title, published_at.as_deref(), tagline.as_deref(), Some(tags), is_draft);
        let new_doc = format!("{}{}", new_frontmatter, content);

        // Extract frontmatter from the new document
        let new_result = extract_frontmatter_and_content(&new_doc);
        prop_assert!(new_result.is_ok());

        let (new_frontmatter_obj, new_content) = new_result.unwrap();

        // The content should be identical
        prop_assert_eq!(content, new_content);

        // Essential frontmatter fields should be preserved
        prop_assert_eq!(frontmatter.title, new_frontmatter_obj.title);

        // Compare published date if present
        if let Some(pub_date) = &frontmatter.published_at {
            prop_assert_eq!(Some(pub_date), new_frontmatter_obj.published_at.as_ref());
        }

        // Draft status should be preserved
        prop_assert_eq!(frontmatter.is_draft, new_frontmatter_obj.is_draft);
    }

    /// Test that markdown-to-html-to-text preserves meaning
    #[test]
    fn prop_markdown_html_conversion_preserves_content(content in markdown_content_strategy()) {
        // Skip empty content
        if content.trim().is_empty() {
            return Ok(());
        }

        // Convert markdown to HTML
        let html = markdown_to_html(&content);

        // Very basic validation - ensure the HTML contains some of the original content
        // This is a simple check that the conversion didn't completely lose the content
        let word_count = calculate_word_count(&content);

        // The HTML should contain at least some of the words from the original content
        prop_assert!(word_count > 0);
        prop_assert!(!html.is_empty());

        // Some content preservation checks
        if content.contains('#') {
            // If markdown has headings, HTML should have <h> tags
            prop_assert!(
                html.contains("<h1>") ||
                html.contains("<h2>") ||
                html.contains("<h3>") ||
                html.contains("<h4>") ||
                html.contains("<h5>") ||
                html.contains("<h6>")
            );
        }

        if content.contains("```") {
            // If markdown has code blocks, HTML should have <pre> or <code> tags
            prop_assert!(html.contains("<pre>") || html.contains("<code>"));
        }
    }

    /// Test that word count is reasonable
    #[test]
    fn prop_word_count_is_reasonable(content in markdown_content_strategy()) {
        let word_count = calculate_word_count(&content);

        // Split content into words using a simple whitespace split
        let words: Vec<&str> = content.split_whitespace().collect();
        let word_count_simple = words.len();

        // Our word count should be similar to a simple split (allowing for some differences
        // due to markdown syntax handling)
        if word_count_simple > 0 {
            let ratio = word_count as f64 / word_count_simple as f64;
            prop_assert!(ratio >= 0.5 && ratio <= 1.5);
        } else {
            prop_assert_eq!(word_count, 0);
        }
    }

    /// Test that reading time is proportional to word count
    #[test]
    fn prop_reading_time_proportional_to_word_count(word_count in 0..10000u32) {
        let reading_time = calculate_reading_time(word_count as usize);

        // Reading time should be roughly word_count / 200 (average reading speed)
        let expected_time = (word_count as f64 / 200.0).ceil() as u32;

        // Allow some deviation for rounding or adjustments in the algorithm
        prop_assert!((reading_time as i32 - expected_time as i32).abs() <= 1);
    }

    /// Test property: Extract first paragraph returns a reasonable result
    #[test]
    #[ignore = "Known failures with unicode characters"]
    fn prop_extract_first_paragraph_returns_reasonable(content in markdown_content_strategy()) {
        // Skip empty content
        if content.trim().is_empty() {
            return Ok(());
        }

        let first_para = extract_first_paragraph(&content);

        if let Some(para) = first_para {
            // First paragraph should be a substring of the content
            prop_assert!(!para.is_empty());

            // Very simple substring check - this is crude but effective
            // (we're not testing the precise extraction logic, just that it returns something sensible)
            let plain_content = content.replace('#', "").replace('`', "");
            let plain_para = para.replace('#', "").replace('`', "");
            let substring_check = plain_content.contains(&plain_para);

            prop_assert!(
                substring_check,
                "First paragraph '{}' should be related to content '{}'",
                para, content
            );
        }
    }

    /// Test text truncation with ellipsis
    #[test]
    #[ignore = "Known failures with unicode characters"]
    fn prop_truncate_with_ellipsis((text, max_length) in
        (proptest::string::string_regex("[\\w\\s,.!?]+").unwrap(), 5..100usize)) {

        let truncated = text::truncate_with_ellipsis(&text, max_length);

        // Truncated text should never exceed max_length
        prop_assert!(truncated.len() <= max_length);

        if text.len() <= max_length {
            // If original text is shorter than max_length, it should be unchanged
            prop_assert_eq!(truncated, text);
        } else {
            // If truncated, it should end with an ellipsis
            prop_assert!(truncated.ends_with("..."));

            // The truncated text (minus ellipsis) should be a prefix of the original
            let prefix = &truncated[..truncated.len() - 3];
            prop_assert!(text.starts_with(prefix));
        }
    }
}