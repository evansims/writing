use common_errors::WritingError;
use common_models::Frontmatter;
use common_validation::{validate_content, validate_content_body, validate_content_date, validate_content_title, validate_content_type};
use proptest::prelude::*;

// Strategy for generating valid content types
fn valid_content_types() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("article".to_string()),
        Just("note".to_string()),
        Just("tutorial".to_string())
    ]
}

// Strategy for generating invalid content types
fn invalid_content_types() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("".to_string()),
        "\\PC*".prop_map(|s| s + "invalid"),
        Just("post".to_string()),
        Just("page".to_string())
    ].prop_filter("Filtering out valid types", |s| {
        !["article", "note", "tutorial"].contains(&s.as_str())
    })
}

// Strategy for generating valid titles
fn valid_titles() -> impl Strategy<Value = String> {
    ".{1,100}".prop_map(|s| s.trim().to_string())
        .prop_filter("Title must not be empty", |s| !s.is_empty())
}

// Strategy for generating valid dates
fn valid_dates() -> impl Strategy<Value = String> {
    (2000..2050u32, 1..13u32, 1..29u32)
        .prop_map(|(y, m, d)| format!("{:04}-{:02}-{:02}", y, m, d))
}

// Strategy for generating invalid dates
fn invalid_dates() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("".to_string()),
        Just("2020/01/01".to_string()),
        Just("01-01-2020".to_string()),
        Just("2020-13-01".to_string()),
        Just("2020-01-32".to_string()),
        Just("20200101".to_string())
    ]
}

// Strategy for generating valid frontmatter
fn valid_frontmatter() -> impl Strategy<Value = String> {
    (valid_titles(), valid_content_types(), valid_dates())
        .prop_map(|(title, content_type, date)| {
            format!(
                "---\ntitle: \"{}\"\ntype: \"{}\"\ndate: \"{}\"\ntags: []\npublished: false\n---",
                title, content_type, date
            )
        })
}

// Strategy for generating valid frontmatter with custom fields
fn valid_frontmatter_with_custom_fields() -> impl Strategy<Value = String> {
    (valid_titles(), valid_content_types(), valid_dates(), ".{1,50}", ".{1,50}")
        .prop_map(|(title, content_type, date, custom_field1, custom_field2)| {
            format!(
                "---\ntitle: \"{}\"\ntype: \"{}\"\ndate: \"{}\"\ntags: []\npublished: false\ncustom_field1: \"{}\"\ncustom_field2: \"{}\"\n---",
                title, content_type, date, custom_field1, custom_field2
            )
        })
}

// Strategy for generating valid content
fn valid_content() -> impl Strategy<Value = String> {
    (valid_frontmatter(), ".{1,500}")
        .prop_map(|(frontmatter, body)| {
            format!("{}\n\n{}", frontmatter, body)
        })
}

// Strategy for generating valid content with custom frontmatter fields
fn valid_content_with_custom_fields() -> impl Strategy<Value = String> {
    (valid_frontmatter_with_custom_fields(), ".{1,500}")
        .prop_map(|(frontmatter, body)| {
            format!("{}\n\n{}", frontmatter, body)
        })
}

// Strategy for generating array of tags
fn valid_tags() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec("[a-z0-9-]{1,20}", 0..5)
}

// Strategy for generating complex nested frontmatter
fn complex_frontmatter() -> impl Strategy<Value = String> {
    (valid_titles(), valid_content_types(), valid_dates(), valid_tags())
        .prop_map(|(title, content_type, date, tags)| {
            let tags_str = tags.iter()
                .map(|tag| format!("\"{}\"", tag))
                .collect::<Vec<_>>()
                .join(", ");
            
            format!(
                "---\ntitle: \"{}\"\ntype: \"{}\"\ndate: \"{}\"\ntags: [{}]\npublished: false\nseo:\n  description: \"SEO description\"\n  keywords: [\"test\", \"seo\"]\nimages:\n  - src: \"image1.jpg\"\n    alt: \"Image 1\"\n  - src: \"image2.jpg\"\n    alt: \"Image 2\"\n---",
                title, content_type, date, tags_str
            )
        })
}

// Property tests for content type validation
proptest! {
    #[test]
    fn test_valid_content_type_is_accepted(content_type in valid_content_types()) {
        let result = validate_content_type(&content_type);
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap(), content_type);
    }

    #[test]
    fn test_invalid_content_type_is_rejected(content_type in invalid_content_types()) {
        let result = validate_content_type(&content_type);
        prop_assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        prop_assert!(err.contains("Invalid content type") || err.contains("empty"));
    }
}

// Property tests for content title validation
proptest! {
    #[test]
    fn test_valid_title_is_accepted(title in valid_titles()) {
        let result = validate_content_title(&title);
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap(), title.trim());
    }

    #[test]
    fn test_empty_title_is_rejected(title in Just("".to_string())) {
        let result = validate_content_title(&title);
        prop_assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        prop_assert!(err.contains("cannot be empty"));
    }
}

// Property tests for content date validation
proptest! {
    #[test]
    fn test_valid_date_is_accepted(date in valid_dates()) {
        let result = validate_content_date(&date);
        prop_assert!(result.is_ok());
        prop_assert_eq!(result.unwrap(), date);
    }

    #[test]
    fn test_invalid_date_is_rejected(date in invalid_dates()) {
        let result = validate_content_date(&date);
        prop_assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        prop_assert!(err.contains("YYYY-MM-DD") || err.contains("empty"));
    }
}

// Property tests for content body validation
proptest! {
    #[test]
    fn test_nonempty_body_is_accepted(body in ".{1,1000}") {
        let result = validate_content_body(&body);
        prop_assert!(result.is_ok());
    }

    #[test]
    fn test_empty_body_is_rejected() {
        let result = validate_content_body("");
        prop_assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        prop_assert!(err.contains("cannot be empty"));
    }
}

// Property tests for full content validation
proptest! {
    #[test]
    fn test_valid_content_is_accepted(content in valid_content()) {
        let result = validate_content(&content);
        prop_assert!(result.is_ok(), "Content should be valid: {}", content);
    }

    #[test]
    fn test_valid_content_with_custom_fields_is_accepted(content in valid_content_with_custom_fields()) {
        let result = validate_content(&content);
        prop_assert!(result.is_ok(), "Content with custom fields should be valid: {}", content);
    }

    #[test]
    fn test_content_with_complex_frontmatter_is_accepted(
        title in valid_titles(),
        content_type in valid_content_types(),
        date in valid_dates(),
        body in ".{1,1000}"
    ) {
        let content = format!(
            "---\ntitle: \"{}\"\ntype: \"{}\"\ndate: \"{}\"\ntags: [\"tag1\", \"tag2\"]\npublished: false\nseo:\n  description: \"SEO description\"\n  keywords: [\"test\", \"seo\"]\nimages:\n  - src: \"image1.jpg\"\n    alt: \"Image 1\"\n  - src: \"image2.jpg\"\n    alt: \"Image 2\"\n---\n\n{}",
            title, content_type, date, body
        );
        
        let result = validate_content(&content);
        prop_assert!(result.is_ok(), "Content with complex frontmatter should be valid");
    }

    #[test]
    fn test_content_without_frontmatter_is_rejected(body in ".{1,1000}") {
        let content = body;
        let result = validate_content(&content);
        prop_assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        prop_assert!(err.contains("frontmatter"));
    }

    #[test]
    fn test_content_with_incomplete_frontmatter_is_rejected(body in ".{1,1000}") {
        let content = format!("---\nIncomplete frontmatter\n---\n\n{}", body);
        let result = validate_content(&content);
        prop_assert!(result.is_err());
    }
}

// Edge case tests that are difficult to express with property testing
#[test]
fn test_frontmatter_without_closing_delimiter() {
    let content = "---\ntitle: \"Test\"\ntype: \"article\"\ndate: \"2023-01-01\"\n\nBody without closing frontmatter";
    let result = validate_content(&content);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Invalid frontmatter format"));
}

#[test]
fn test_frontmatter_with_missing_required_fields() {
    let content = "---\ntagline: \"Test post\"\n---\n\nBody with missing title";
    let result = validate_content(&content);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("required") || err.contains("Invalid frontmatter"));
}

#[test]
fn test_malformed_yaml_in_frontmatter() {
    let content = "---\ntitle: Test\ntype: : article\ndate: \"2023-01-01\"\n---\n\nBody with malformed YAML";
    let result = validate_content(&content);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Invalid frontmatter"));
}

#[test]
fn test_content_with_only_whitespace_body() {
    let content = "---\ntitle: \"Test\"\ntype: \"article\"\ndate: \"2023-01-01\"\n---\n\n   \t   \n  ";
    let result = validate_content(&content);
    assert!(result.is_ok(), "Content with whitespace body should be valid");
}

#[test]
fn test_content_with_unicode_characters() {
    let content = "---\ntitle: \"测试标题\"\ntype: \"article\"\ndate: \"2023-01-01\"\n---\n\nContent with unicode: こんにちは世界";
    let result = validate_content(&content);
    assert!(result.is_ok(), "Content with unicode characters should be valid");
}

#[test]
fn test_content_with_codeblocks() {
    let content = "---\ntitle: \"Code Test\"\ntype: \"article\"\ndate: \"2023-01-01\"\n---\n\n```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
    let result = validate_content(&content);
    assert!(result.is_ok(), "Content with code blocks should be valid");
}

// New additional tests for improved coverage

#[test]
fn test_frontmatter_with_non_string_values() {
    // Test different YAML types in frontmatter
    let content = "---\ntitle: \"Numeric Test\"\ntype: \"article\"\ndate: \"2023-01-01\"\norder: 42\nisSpecial: true\nrating: 4.5\n---\n\nContent with numeric frontmatter values";
    let result = validate_content(&content);
    assert!(result.is_ok(), "Content with numeric and boolean frontmatter values should be valid");
}

#[test]
fn test_frontmatter_with_array_of_objects() {
    // Test arrays of objects in frontmatter
    let content = "---\ntitle: \"Complex Array Test\"\ntype: \"article\"\ndate: \"2023-01-01\"\nauthors:\n  - name: \"John Doe\"\n    email: \"john@example.com\"\n  - name: \"Jane Smith\"\n    email: \"jane@example.com\"\n---\n\nContent with complex array frontmatter";
    let result = validate_content(&content);
    assert!(result.is_ok(), "Content with array of objects in frontmatter should be valid");
}

#[test]
fn test_frontmatter_with_nested_objects() {
    // Test deeply nested objects in frontmatter
    let content = "---\ntitle: \"Nested Object Test\"\ntype: \"article\"\ndate: \"2023-01-01\"\nmetadata:\n  seo:\n    description: \"Deep nested test\"\n    keywords:\n      - \"test\"\n      - \"nested\"\n    sharing:\n      twitter:\n        card: \"summary\"\n        image: \"image.jpg\"\n---\n\nContent with deeply nested frontmatter";
    let result = validate_content(&content);
    assert!(result.is_ok(), "Content with deeply nested objects in frontmatter should be valid");
}

#[test]
fn test_frontmatter_with_invalid_yaml_mapping() {
    // Test invalid YAML mapping syntax
    let content = "---\ntitle: \"Invalid YAML\"\ntype: \"article\"\ndate: \"2023-01-01\"\n- this is not a valid mapping\n---\n\nContent with invalid YAML mapping";
    let result = validate_content(&content);
    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("Invalid frontmatter") || err.contains("YAML"));
}

#[test]
fn test_frontmatter_with_duplicate_keys() {
    // Test duplicate keys in frontmatter
    let content = "---\ntitle: \"First Title\"\ntitle: \"Second Title\"\ntype: \"article\"\ndate: \"2023-01-01\"\n---\n\nContent with duplicate frontmatter keys";
    let result = validate_content(&content);
    
    // Note: Some YAML parsers might handle this differently, accepting the last value
    // It's important to check how your YAML parser behaves with duplicate keys
    // The test should match the expected behavior of your implementation
    
    // Either the validation should fail for duplicate keys, or it should accept the last value
    if result.is_err() {
        let err = result.unwrap_err().to_string();
        assert!(err.contains("duplicate") || err.contains("Invalid frontmatter"));
    } else {
        // If your YAML parser accepts this, verify it keeps the last value
        let frontmatter = result.unwrap().frontmatter;
        assert_eq!(frontmatter.title, "Second Title");
    }
}

#[test]
fn test_frontmatter_with_invalid_date_format() {
    // Test various invalid date formats
    let invalid_dates = vec![
        "2023/01/01",      // Wrong separator
        "01-01-2023",      // Wrong order
        "2023-13-01",      // Invalid month
        "2023-01-32",      // Invalid day
        "2023-02-30",      // Invalid day for month
        "2023-01",         // Missing day
        "January 1, 2023", // Wrong format
        "2023-01-01T00:00:00Z", // ISO format with time
    ];
    
    for date in invalid_dates {
        let content = format!("---\ntitle: \"Date Test\"\ntype: \"article\"\ndate: \"{}\"\n---\n\nContent with invalid date", date);
        let result = validate_content(&content);
        assert!(result.is_err(), "Date {} should be invalid", date);
        let err = result.unwrap_err().to_string();
        assert!(err.contains("date") || err.contains("Invalid frontmatter"));
    }
}

#[test]
fn test_frontmatter_with_multiline_strings() {
    // Test multiline strings in YAML
    let content = "---\ntitle: \"Multiline Test\"\ntype: \"article\"\ndate: \"2023-01-01\"\ndescription: |\n  This is a multiline description\n  that spans multiple lines\n  and should be properly parsed.\n---\n\nContent with multiline description";
    let result = validate_content(&content);
    assert!(result.is_ok(), "Content with multiline strings in frontmatter should be valid");
    
    let desc = result.unwrap().frontmatter.description.unwrap_or_default();
    assert!(desc.contains("multiline description"));
    assert!(desc.contains("spans multiple lines"));
}

#[test]
fn test_content_with_frontmatter_delimiters_in_body() {
    // Test with frontmatter delimiters in the content body
    let content = "---\ntitle: \"Delimiter Test\"\ntype: \"article\"\ndate: \"2023-01-01\"\n---\n\nContent with --- delimiter in body\n\n---\n\nThis should be treated as horizontal rule, not frontmatter";
    let result = validate_content(&content);
    assert!(result.is_ok(), "Content with frontmatter delimiters in body should be valid");
}

#[test]
fn test_content_with_leading_whitespace_before_frontmatter() {
    // Test with whitespace before frontmatter
    let content = "\n\n  \t  ---\ntitle: \"Whitespace Test\"\ntype: \"article\"\ndate: \"2023-01-01\"\n---\n\nContent with whitespace before frontmatter";
    let result = validate_content(&content);
    assert!(result.is_err(), "Content with whitespace before frontmatter should be invalid");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("frontmatter"));
}

#[test]
fn test_content_with_bom_character() {
    // Test with BOM character at start of file (common issue with some editors)
    let content = "\u{FEFF}---\ntitle: \"BOM Test\"\ntype: \"article\"\ndate: \"2023-01-01\"\n---\n\nContent with BOM character";
    let result = validate_content(&content);
    assert!(result.is_err(), "Content with BOM character should be handled appropriately");
    let err = result.unwrap_err().to_string();
    assert!(err.contains("frontmatter") || err.contains("unexpected character"));
}

#[test]
fn test_content_with_non_breaking_spaces() {
    // Test with non-breaking spaces in content (common issue when copying from web)
    let content = "---\ntitle: \"Non-breaking Space Test\"\ntype: \"article\"\ndate: \"2023-01-01\"\n---\n\nContent with non-breaking\u{00A0}spaces";
    let result = validate_content(&content);
    assert!(result.is_ok(), "Content with non-breaking spaces should be valid");
}

#[test]
fn test_content_with_windows_line_endings() {
    // Test with Windows CRLF line endings
    let content = "---\r\ntitle: \"CRLF Test\"\r\ntype: \"article\"\r\ndate: \"2023-01-01\"\r\n---\r\n\r\nContent with Windows line endings";
    let result = validate_content(&content);
    assert!(result.is_ok(), "Content with Windows line endings should be valid");
}

#[test]
fn test_content_with_very_long_title() {
    // Test with an extremely long title
    let long_title = "a".repeat(500);
    let content = format!("---\ntitle: \"{}\"\ntype: \"article\"\ndate: \"2023-01-01\"\n---\n\nContent with very long title", long_title);
    let result = validate_content(&content);
    
    // Depending on requirements, this might be valid or invalid
    // If there's a title length limit, it should fail
    if result.is_err() {
        let err = result.unwrap_err().to_string();
        assert!(err.contains("title") || err.contains("length"));
    } else {
        // If long titles are allowed, this should pass
        assert_eq!(result.unwrap().frontmatter.title, long_title);
    }
} 