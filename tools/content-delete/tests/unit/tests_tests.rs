//! Unit tests extracted from lib.rs

use content_delete::*;
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_extract_title_from_content() {
        // Create a temporary file with test content
        let mut file = NamedTempFile::new().unwrap();
        let content = r#"---
title: "Test Article"
published: "2023-01-01"
tags:
  - test
---

# Test Content

This is a test article."#;

        file.write_all(content.as_bytes()).unwrap();

        // Test extracting the title
        let title = extract_title_from_content(file.path()).unwrap();
        assert_eq!(title, "Test Article");
    }
}
