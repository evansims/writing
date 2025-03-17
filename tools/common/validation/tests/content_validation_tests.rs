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

// Strategy for generating valid content
fn valid_content() -> impl Strategy<Value = String> {
    (valid_frontmatter(), ".{1,500}")
        .prop_map(|(frontmatter, body)| {
            format!("{}\n\n{}", frontmatter, body)
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