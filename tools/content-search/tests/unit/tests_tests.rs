//! Unit tests extracted from lib.rs

use content_search::*;
mod tests {
    use super::*;

    #[test]
    fn test_extract_metadata_and_content() {
        let content = r#"---
title: Test Article
tags: rust, search, testing
type: article
date: 2023-01-01
---

# Heading 1

This is some test content.

## Heading 2

More content here.
"#;

        let (title, metadata, text) = extract_metadata_and_content(content);

        assert_eq!(title, "Test Article");
        assert_eq!(metadata.get("tags"), Some(&"rust, search, testing".to_string()));
        assert_eq!(metadata.get("type"), Some(&"article".to_string()));
        assert_eq!(metadata.get("date"), Some(&"2023-01-01".to_string()));
        assert!(text.contains("Heading 1"));
        assert!(text.contains("This is some test content."));
    }

    #[test]
    fn test_create_excerpt() {
        let text = "This is a long piece of text that contains the word search somewhere in the middle. And it continues for a while after that.";

        let excerpt = create_excerpt(text, "search", 50);

        assert!(excerpt.contains("search"));
        assert!(excerpt.len() <= 60); // 50 + "..." at beginning and/or end
    }
}
