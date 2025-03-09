//! # Common Markdown Operations
//! 
//! This module provides common markdown operations for the writing tools.
//! 
//! ## Features
//! 
//! - Frontmatter extraction and generation
//! - Markdown to HTML conversion
//! - Word count and reading time calculation
//! - Paragraph extraction
//! 
//! ## Example
//! 
//! ```rust
//! use common_markdown::{extract_frontmatter_and_content, markdown_to_html, calculate_word_count};
//! 
//! fn process_markdown(content: &str) -> common_errors::Result<String> {
//!     let (frontmatter, markdown) = extract_frontmatter_and_content(content)?;
//!     
//!     let word_count = calculate_word_count(&markdown);
//!     println!("Word count: {}", word_count);
//!     
//!     let html = markdown_to_html(&markdown);
//!     Ok(html)
//! }
//! ```

use common_errors::{Result, WritingError, ResultExt};
use common_models::Frontmatter;
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use regex::Regex;

/// Extract frontmatter and content from a markdown file
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(frontmatter.published, Some("2023-01-01".to_string()));
        assert_eq!(frontmatter.tagline, Some("Test Tagline".to_string()));
        assert_eq!(frontmatter.tags, Some(vec!["test".to_string(), "markdown".to_string()]));
        assert_eq!(frontmatter.draft, Some(true));
        
        assert!(markdown.contains("# Test Content"));
        assert!(markdown.contains("This is a test paragraph."));
    }

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
        assert!(err_msg.contains("Failed to parse frontmatter"));
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
    fn test_markdown_to_html() {
        let content = "# Heading\n\nThis is a paragraph with **bold** text.";
        let html = markdown_to_html(content);
        
        assert!(html.contains("<h1>Heading</h1>"));
        assert!(html.contains("<p>This is a paragraph with <strong>bold</strong> text.</p>"));
        
        // Test with more complex markdown
        let content = "# Heading\n\n- List item 1\n- List item 2\n\n> Blockquote\n\n```\nCode block\n```";
        let html = markdown_to_html(content);
        
        assert!(html.contains("<h1>Heading</h1>"));
        assert!(html.contains("<li>List item 1</li>"));
        assert!(html.contains("<li>List item 2</li>"));
        assert!(html.contains("<blockquote>\n<p>Blockquote</p>\n</blockquote>"));
        assert!(html.contains("<pre><code>Code block\n</code></pre>"));
    }

    #[test]
    fn test_generate_frontmatter() {
        // Test with all fields
        let frontmatter = generate_frontmatter(
            "Test Title",
            Some("2023-01-01"),
            Some("Test Tagline"),
            Some(vec!["test", "markdown"]),
            true
        );
        
        assert!(frontmatter.contains("title: \"Test Title\""));
        assert!(frontmatter.contains("published: 2023-01-01"));
        assert!(frontmatter.contains("tagline: \"Test Tagline\""));
        assert!(frontmatter.contains("tags:"));
        assert!(frontmatter.contains("  - test"));
        assert!(frontmatter.contains("  - markdown"));
        assert!(frontmatter.contains("draft: true"));
        
        // Test with minimal fields
        let frontmatter = generate_frontmatter(
            "Test Title",
            None,
            None,
            None,
            false
        );
        
        assert!(frontmatter.contains("title: \"Test Title\""));
        assert!(!frontmatter.contains("published:"));
        assert!(!frontmatter.contains("tagline:"));
        assert!(!frontmatter.contains("tags:"));
        assert!(!frontmatter.contains("draft:"));
    }
} 