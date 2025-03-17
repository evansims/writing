//! # Pattern Utilities
//! 
//! This module provides utilities for pattern matching and text processing.

use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SLUG_REGEX: Regex = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
    static ref EMAIL_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    static ref URL_REGEX: Regex = Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap();
    static ref MARKDOWN_LINK_REGEX: Regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
    static ref FRONTMATTER_REGEX: Regex = Regex::new(r"^---\s*\n([\s\S]*?)\n---\s*\n").unwrap();
}

/// Check if a string matches a pattern
pub fn matches_pattern(text: &str, pattern: &str) -> bool {
    match Regex::new(pattern) {
        Ok(regex) => regex.is_match(text),
        Err(_) => false,
    }
}

/// Extract all matches from a string using a regex pattern
pub fn extract_matches<'a>(text: &'a str, pattern: &str) -> Result<Vec<String>> {
    let regex = Regex::new(pattern)
        .map_err(|e| anyhow::anyhow!("Invalid regex pattern: {}", e))?;
    
    let matches = regex.captures_iter(text)
        .map(|cap| {
            cap.get(0)
                .map(|m| m.as_str().to_string())
                .unwrap_or_default()
        })
        .collect();
    
    Ok(matches)
}

/// Extract named captures from a string using a regex pattern
pub fn extract_named_captures<'a>(text: &'a str, pattern: &str) -> Result<Vec<std::collections::HashMap<String, String>>> {
    let regex = Regex::new(pattern)
        .map_err(|e| anyhow::anyhow!("Invalid regex pattern: {}", e))?;
    
    let captures = regex.captures_iter(text)
        .map(|cap| {
            let mut map = std::collections::HashMap::new();
            
            for name in regex.capture_names().flatten() {
                if let Some(value) = cap.name(name) {
                    map.insert(name.to_string(), value.as_str().to_string());
                }
            }
            
            map
        })
        .collect();
    
    Ok(captures)
}

/// Check if a string is a valid slug
pub fn is_valid_slug(slug: &str) -> bool {
    SLUG_REGEX.is_match(slug)
}

/// Check if a string is a valid email
pub fn is_valid_email(email: &str) -> bool {
    EMAIL_REGEX.is_match(email)
}

/// Check if a string is a valid URL
pub fn is_valid_url(url: &str) -> bool {
    URL_REGEX.is_match(url)
}

/// Extract all markdown links from a string
pub fn extract_markdown_links(text: &str) -> Vec<(String, String)> {
    MARKDOWN_LINK_REGEX.captures_iter(text)
        .map(|cap| {
            let text = cap.get(1).map_or("", |m| m.as_str()).to_string();
            let url = cap.get(2).map_or("", |m| m.as_str()).to_string();
            (text, url)
        })
        .collect()
}

/// Extract frontmatter from markdown content
pub fn extract_frontmatter(content: &str) -> Option<String> {
    FRONTMATTER_REGEX.captures(content)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

/// Split content into frontmatter and body
pub fn split_frontmatter_and_body(content: &str) -> (Option<String>, String) {
    if let Some(captures) = FRONTMATTER_REGEX.captures(content) {
        let frontmatter = captures.get(1).map(|m| m.as_str().to_string());
        let body_start = captures.get(0).map_or(0, |m| m.end());
        let body = content[body_start..].to_string();
        (frontmatter, body)
    } else {
        (None, content.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_matches_pattern() {
        assert!(matches_pattern("test123", r"^[a-z]+\d+$"));
        assert!(!matches_pattern("TEST123", r"^[a-z]+\d+$"));
    }
    
    #[test]
    fn test_extract_matches() {
        let text = "apple orange banana";
        let matches = extract_matches(text, r"[a-z]+e").unwrap();
        
        assert_eq!(matches.len(), 2);
        assert!(matches.contains(&"apple".to_string()));
        assert!(matches.contains(&"orange".to_string()));
    }
    
    #[test]
    fn test_is_valid_slug() {
        assert!(is_valid_slug("test-slug"));
        assert!(is_valid_slug("test123"));
        assert!(!is_valid_slug("Test Slug"));
        assert!(!is_valid_slug("test_slug"));
    }
    
    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("user@example.com"));
        assert!(is_valid_email("user.name+tag@example.co.uk"));
        assert!(!is_valid_email("user@example"));
        assert!(!is_valid_email("user@.com"));
    }
    
    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://example.com"));
        assert!(is_valid_url("http://example.com/path?query=value"));
        assert!(!is_valid_url("example.com"));
        assert!(!is_valid_url("https:/example.com"));
    }
    
    #[test]
    fn test_extract_markdown_links() {
        let text = "This is a [link](https://example.com) and another [link2](https://example2.com).";
        let links = extract_markdown_links(text);
        
        assert_eq!(links.len(), 2);
        assert_eq!(links[0], ("link".to_string(), "https://example.com".to_string()));
        assert_eq!(links[1], ("link2".to_string(), "https://example2.com".to_string()));
    }
    
    #[test]
    fn test_extract_frontmatter() {
        let content = "---\ntitle: Test\ndate: 2023-01-01\n---\n\nBody content.";
        let frontmatter = extract_frontmatter(content);
        
        assert!(frontmatter.is_some());
        assert_eq!(frontmatter.unwrap(), "title: Test\ndate: 2023-01-01");
    }
    
    #[test]
    fn test_split_frontmatter_and_body() {
        let content = "---\ntitle: Test\ndate: 2023-01-01\n---\n\nBody content.";
        let (frontmatter, body) = split_frontmatter_and_body(content);
        
        assert!(frontmatter.is_some());
        assert_eq!(frontmatter.unwrap(), "title: Test\ndate: 2023-01-01");
        assert_eq!(body.trim(), "Body content.");
    }
} 