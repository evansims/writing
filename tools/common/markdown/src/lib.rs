//! # Common Markdown Operations
//!
//! This module provides common markdown operations for the writing tools.
//!
//! ## Features
//!
//! - Frontmatter extraction and generation (requires 'frontmatter' feature)
//! - Markdown to HTML conversion (requires 'html' feature)
//! - Word count and reading time calculation
//! - Paragraph extraction (requires 'html' feature)
//!
//! ## Feature Flags
//!
//! - `html`: Enables HTML conversion functionality (enabled by default)
//! - `frontmatter`: Enables frontmatter handling (enabled by default)
//! - `syntax-highlight`: Enables syntax highlighting (disabled by default)
//!
//! ## Example
//!
//! ```rust
//! use common_markdown::{calculate_word_count};
//!
//! #[cfg(feature = "frontmatter")]
//! use common_markdown::extract_frontmatter_and_content;
//!
//! #[cfg(feature = "html")]
//! use common_markdown::markdown_to_html;
//!
//! fn process_markdown(content: &str) -> common_errors::Result<String> {
//!     #[cfg(feature = "frontmatter")]
//!     let (frontmatter, markdown) = extract_frontmatter_and_content(content)?;
//!
//!     #[cfg(not(feature = "frontmatter"))]
//!     let markdown = content;
//!
//!     let word_count = calculate_word_count(&markdown);
//!     println!("Word count: {}", word_count);
//!
//!     #[cfg(feature = "html")]
//!     let html = markdown_to_html(&markdown);
//!
//!     #[cfg(feature = "html")]
//!     return Ok(html);
//!
//!     #[cfg(not(feature = "html"))]
//!     return Ok(markdown.to_string());
//! }
//! ```

use common_errors::{Result, WritingError, ResultExt};
use common_models::Frontmatter;

#[cfg(feature = "html")]
use pulldown_cmark::{html, Event, Options, Parser, Tag};

#[cfg(feature = "frontmatter")]
use regex::Regex;

/// Extract frontmatter and content from a markdown file
///
/// Requires the `frontmatter` feature
#[cfg(feature = "frontmatter")]
pub fn extract_frontmatter_and_content(content: &str) -> Result<(Frontmatter, String)> {
    // Look for frontmatter between --- markers
    let re = Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)$")
        .map_err(|e| WritingError::format_error(format!("Failed to compile regex: {}", e)))?;

    if let Some(captures) = re.captures(content) {
        let frontmatter_yaml = captures.get(1).unwrap().as_str();
        let markdown_content = captures.get(2).unwrap().as_str();

        let frontmatter: Frontmatter = serde_yaml::from_str(frontmatter_yaml)
            .with_context(|| "Failed to parse frontmatter")?;

        Ok((frontmatter, markdown_content.to_string()))
    } else {
        Err(WritingError::format_error("No frontmatter found in content"))
    }
}

/// Extract frontmatter from a string
///
/// Requires the `frontmatter` feature
#[cfg(feature = "frontmatter")]
pub fn extract_frontmatter(content: &str) -> Result<(serde_yaml::Value, String)> {
    // Look for frontmatter between --- markers
    let re = Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)$")
        .map_err(|e| WritingError::format_error(format!("Failed to compile regex: {}", e)))?;

    if let Some(captures) = re.captures(content) {
        let frontmatter_yaml = captures.get(1).unwrap().as_str();
        let markdown_content = captures.get(2).unwrap().as_str();

        let frontmatter: serde_yaml::Value = serde_yaml::from_str(frontmatter_yaml)
            .with_context(|| "Failed to parse frontmatter")?;

        Ok((frontmatter, markdown_content.to_string()))
    } else {
        Err(WritingError::format_error("No frontmatter found in content"))
    }
}

/// Calculate word count from markdown content
pub fn calculate_word_count(content: &str) -> usize {
    content.split_whitespace().count()
}

/// Calculate reading time in minutes from word count
pub fn calculate_reading_time(word_count: usize) -> u32 {
    let words_per_minute = 200;
    let reading_time = (word_count as f64 / words_per_minute as f64).ceil() as u32;
    std::cmp::max(1, reading_time) // Minimum reading time of 1 minute
}

/// Extract the first paragraph from markdown content
///
/// Requires the `html` feature
#[cfg(feature = "html")]
pub fn extract_first_paragraph(content: &str) -> Option<String> {
    let mut first_paragraph = String::new();
    let mut in_paragraph = false;

    let parser = Parser::new(content);

    for event in parser {
        match event {
            Event::Start(Tag::Paragraph) => {
                in_paragraph = true;
            },
            Event::End(Tag::Paragraph) => {
                if in_paragraph {
                    return Some(first_paragraph);
                }
            },
            Event::Text(text) => {
                if in_paragraph {
                    first_paragraph.push_str(&text);
                }
            },
            _ => {}
        }
    }

    if first_paragraph.is_empty() {
        None
    } else {
        Some(first_paragraph)
    }
}

/// Convert markdown to HTML
///
/// Requires the `html` feature
#[cfg(feature = "html")]
pub fn markdown_to_html(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

/// Generate frontmatter with required fields
///
/// Requires the `frontmatter` feature
#[cfg(feature = "frontmatter")]
pub fn generate_frontmatter(
    title: &str,
    published: Option<&str>,
    tagline: Option<&str>,
    tags: Option<Vec<&str>>,
    draft: bool,
) -> String {
    let mut frontmatter = String::from("---\n");

    frontmatter.push_str(&format!("title: \"{}\"\n", title));

    if let Some(published_date) = published {
        frontmatter.push_str(&format!("published: {}\n", published_date));
    }

    if let Some(tagline_text) = tagline {
        frontmatter.push_str(&format!("tagline: \"{}\"\n", tagline_text));
    }

    if let Some(tag_list) = tags {
        frontmatter.push_str("tags:\n");
        for tag in tag_list {
            frontmatter.push_str(&format!("  - {}\n", tag));
        }
    }

    if draft {
        frontmatter.push_str("draft: true\n");
    }

    frontmatter.push_str("---\n\n");

    frontmatter
}

/// Utility module for string manipulation
pub mod text {
    /// Truncates a string to a specified length,
    /// adding an ellipsis if truncation occurs
    pub fn truncate_with_ellipsis(text: &str, max_length: usize) -> String {
        if text.len() <= max_length {
            text.to_string()
        } else {
            let mut truncated = text.chars().take(max_length).collect::<String>();
            truncated.push_str("...");
            truncated
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "frontmatter")]
    #[test]
    fn test_extract_frontmatter_and_content() {
        let content = r#"---
title: "Test Title"
published: 2023-01-01
tagline: "Test Tagline"
tags:
  - test
  - markdown
draft: true
---

# Test Content

This is a test paragraph."#;

        let result = extract_frontmatter_and_content(content);
        assert!(result.is_ok());

        let (frontmatter, markdown) = result.unwrap();
        assert_eq!(frontmatter.title, "Test Title");
        assert_eq!(frontmatter.published_at, Some("2023-01-01".to_string()));
        assert_eq!(frontmatter.tagline, Some("Test Tagline".to_string()));
        assert_eq!(frontmatter.tags, Some(vec!["test".to_string(), "markdown".to_string()]));
        assert_eq!(frontmatter.is_draft, Some(true));

        assert!(markdown.contains("# Test Content"));
        assert!(markdown.contains("This is a test paragraph."));
    }

    #[cfg(feature = "frontmatter")]
    #[test]
    fn test_extract_frontmatter_and_content_missing_frontmatter() {
        let content = "# Test Content\n\nThis is a test paragraph.";

        let result = extract_frontmatter_and_content(content);
        assert!(result.is_err());

        let err = result.unwrap_err();
        let err_msg = format!("{}", err);
        assert!(err_msg.contains("No frontmatter found"));
    }

    #[test]
    fn test_extract_frontmatter_and_content_invalid_yaml() {
        let content = r#"---
title: "Test Title
invalid yaml
---

# Test Content"#;

        let result = extract_frontmatter_and_content(content);
        assert!(result.is_err());

        let err = result.unwrap_err();
        let err_msg = format!("{}", err);

        // The actual error message mentions "unexpected end of stream" and "quoted scalar"
        // which is the YAML parsing error
        assert!(
            err_msg.contains("unexpected end of stream") ||
            err_msg.contains("quoted scalar") ||
            err_msg.contains("Failed to parse frontmatter") ||
            err_msg.contains("YAML")
        );
    }

    #[test]
    fn test_calculate_word_count() {
        let content = "This is a test paragraph with 8 words.";
        assert_eq!(calculate_word_count(content), 8);

        let content = "";
        assert_eq!(calculate_word_count(content), 0);

        let content = "One";
        assert_eq!(calculate_word_count(content), 1);
    }

    #[test]
    fn test_calculate_reading_time() {
        // 200 words per minute, so 200 words should be 1 minute
        assert_eq!(calculate_reading_time(200), 1);

        // 300 words should be 2 minutes
        assert_eq!(calculate_reading_time(300), 2);

        // 50 words should be 1 minute (minimum)
        assert_eq!(calculate_reading_time(50), 1);

        // 0 words should be 1 minute (minimum)
        assert_eq!(calculate_reading_time(0), 1);
    }

    #[cfg(feature = "html")]
    #[test]
    fn test_extract_first_paragraph() {
        let content = "# Heading\n\nThis is the first paragraph.\n\nThis is the second paragraph.";
        let result = extract_first_paragraph(content);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "This is the first paragraph.");

        // Test with no paragraphs
        let content = "# Heading\n\n";
        let result = extract_first_paragraph(content);
        assert!(result.is_none());
    }

    #[test]
    fn test_truncate_with_ellipsis() {
        assert_eq!(text::truncate_with_ellipsis("Hello", 10), "Hello");
        assert_eq!(text::truncate_with_ellipsis("Hello world", 5), "Hello...");
        assert_eq!(text::truncate_with_ellipsis("Hello", 5), "Hello");
        assert_eq!(text::truncate_with_ellipsis("", 5), "");
    }
}