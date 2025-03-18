use common_markdown::*;
use proptest::prelude::*;
use proptest::collection::vec;
use std::collections::HashMap;
use proptest::test_runner::Config;

/// Generate valid YAML frontmatter
fn frontmatter_yaml_strategy() -> impl Strategy<Value = String> {
    let key_strategy = proptest::string::string_regex("[a-zA-Z][a-zA-Z0-9_]*").unwrap();
    let value_strategy = proptest::string::string_regex("[^:\n]*").unwrap();

    proptest::collection::hash_map(key_strategy, value_strategy, 0..10)
        .prop_map(|map| {
            let mut frontmatter = String::from("---\n");

            for (key, value) in map {
                if !value.is_empty() {
                    frontmatter.push_str(&format!("{}: {}\n", key, value));
                }
            }

            frontmatter.push_str("---\n");
            frontmatter
        })
}

/// Generate valid frontmatter with common fields
fn frontmatter_with_common_fields_strategy() -> impl Strategy<Value = String> {
    let title_strategy = proptest::string::string_regex("[A-Za-z0-9 ]{3,50}").unwrap();
    let date_strategy = proptest::string::string_regex("[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
    let tag_strategy = proptest::string::string_regex("[a-z-]{3,15}").unwrap();
    let tags_count = 0..5;

    (title_strategy, date_strategy, proptest::collection::vec(tag_strategy, tags_count))
        .prop_map(|(title, date, tags)| {
            let mut frontmatter = String::from("---\n");
            frontmatter.push_str(&format!("title: \"{}\"\n", title));
            frontmatter.push_str(&format!("published: {}\n", date));

            if !tags.is_empty() {
                frontmatter.push_str("tags:\n");
                for tag in tags {
                    frontmatter.push_str(&format!("  - {}\n", tag));
                }
            }

            // Randomly add draft status
            if rand::random::<bool>() {
                frontmatter.push_str(&format!("draft: {}\n", rand::random::<bool>()));
            }

            frontmatter.push_str("---\n");
            frontmatter
        })
}

/// Generate valid markdown content
fn markdown_content_strategy() -> impl Strategy<Value = String> {
    let heading_strategy = prop::collection::vec("[#]{1,6} [\\w\\s]+\\n", 0..5);
    let paragraph_strategy = prop::collection::vec("[\\w\\s,.!?]+\\n\\n", 1..10);

    (heading_strategy, paragraph_strategy)
        .prop_map(|(headings, paragraphs)| {
            let mut content = String::new();

            for heading in headings {
                content.push_str(&heading);
                if !content.ends_with("\n\n") {
                    content.push('\n');
                }
            }

            for paragraph in paragraphs {
                content.push_str(&paragraph);
            }

            content
        })
}

/// Generate valid markdown documents with frontmatter
fn markdown_document_strategy() -> impl Strategy<Value = String> {
    (frontmatter_with_common_fields_strategy(), markdown_content_strategy())
        .prop_map(|(frontmatter, content)| {
            format!("{}{}", frontmatter, content)
        })
}

proptest! {
    #![proptest_config(Config::with_cases(10)
        .clone_with_source_file("proptest-regressions/frontmatter_properties_fix.proptest-regressions"))]
    /// Test property: Frontmatter extraction correctly separates frontmatter and content
    #[test]
    fn prop_extract_frontmatter_and_content_separates_correctly(doc in markdown_document_strategy()) {
        // Extract frontmatter and content
        let result = extract_frontmatter_and_content(&doc);
        prop_assert!(result.is_ok());

        let (frontmatter, content) = result.unwrap();

        // Check that frontmatter has expected fields
        prop_assert!(!frontmatter.title.is_empty());
        prop_assert!(frontmatter.published_at.is_some());

        // Verify the content doesn't contain the frontmatter delimiter
        prop_assert!(!content.contains("---\n"));
    }

    /// Test property: Malformed frontmatter returns appropriate errors
    #[test]
    fn prop_extract_frontmatter_handles_errors(content in proptest::string::string_regex("[^-]{1,100}").unwrap()) {
        // Try extracting from content without frontmatter
        let result = extract_frontmatter_and_content(&content);

        // Should error due to missing frontmatter
        prop_assert!(result.is_err());
        let err = result.unwrap_err();
        prop_assert!(
            err.to_string().contains("No frontmatter found") ||
            err.to_string().contains("Failed to parse frontmatter")
        );
    }

    /// Test property: Word count is consistent with content length
    #[test]
    #[ignore = "Known failures with unicode characters"]
    fn prop_word_count_consistent_with_content(content in markdown_content_strategy()) {
        let word_count = calculate_word_count(&content);

        // A rough approximation: count spaces + 1
        let space_count = content.chars().filter(|c| *c == ' ').count();

        // Word count should be roughly related to space count
        // This is a very basic heuristic
        if space_count > 0 {
            prop_assert!(word_count >= space_count / 2);
            prop_assert!(word_count <= space_count * 2 + 1);
        }
    }

    /// Test property: Reading time is proportional to word count
    #[test]
    fn prop_reading_time_proportional_to_word_count(words in 0..10000u32) {
        let reading_time = calculate_reading_time(words.try_into().unwrap());

        // Reading time should be words / 200 rounded up, with a minimum of 1
        let expected_time = std::cmp::max(1, (words + 199) / 200);
        prop_assert_eq!(reading_time, expected_time);
    }
}