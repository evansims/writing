use anyhow::Result;
use thiserror::Error;
use common_config::load_config;
use common_fs::read_file;
use common_models::Config;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use pulldown_cmark::{Parser, Event, Options};

/// Error types for content search
#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Search index not found")]
    IndexNotFound,
    
    #[error("Failed to build search index: {0}")]
    IndexBuildError(String),
    
    #[error("Invalid search query: {0}")]
    InvalidQuery(String),
    
    #[error("Article not found: {0}")]
    ArticleNotFound(String),
    
    #[error("Topic not found: {0}")]
    TopicNotFound(String),
    
    #[error("No search results found")]
    NoResults,
}

/// A search result item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Document title
    pub title: String,
    
    /// Document slug
    pub slug: String,
    
    /// Document topic
    pub topic: String,
    
    /// Document path
    pub path: String,
    
    /// Document content (excerpt)
    pub content: String,
    
    /// Document tags
    pub tags: Vec<String>,
    
    /// Document content type
    pub content_type: String,
    
    /// Document date (if available)
    pub date: Option<String>,
    
    /// Search score
    pub score: f32,
}

/// Search options for content
#[derive(Debug, Clone)]
pub struct SearchOptions {
    /// Search query string
    pub query: String,
    
    /// Limit results to this topic
    pub topic: Option<String>,
    
    /// Limit results to this content type
    pub content_type: Option<String>,
    
    /// Limit results to items with these tags
    pub tags: Option<Vec<String>>,
    
    /// Maximum number of results to return
    pub limit: usize,
    
    /// Whether to include draft content
    pub include_drafts: bool,
    
    /// Search only in titles
    pub title_only: bool,
    
    /// Raw query mode (bypasses query parser)
    pub raw_query: bool,
    
    /// Whether the search is case sensitive
    pub case_sensitive: bool,
    
    /// Whether to include metadata in search
    pub include_metadata: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            query: String::new(),
            topic: None,
            content_type: None,
            tags: None,
            limit: 50,
            include_drafts: false,
            title_only: false,
            raw_query: false,
            case_sensitive: false,
            include_metadata: false,
        }
    }
}

/// Extract plain text from markdown content
///
/// This function removes markdown formatting and returns plain text.
///
/// # Parameters
///
/// * `markdown` - The markdown content to convert
///
/// # Returns
///
/// Returns the plain text extracted from markdown
fn extract_text_from_markdown(markdown: &str) -> String {
    let mut text = String::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    
    let parser = Parser::new_ext(markdown, options);
    
    for event in parser {
        match event {
            Event::Text(t) => text.push_str(&t),
            Event::Code(c) => text.push_str(&c),
            Event::SoftBreak => text.push(' '),
            Event::HardBreak => text.push('\n'),
            _ => {}
        }
    }
    
    text
}

/// Extract metadata and content from a markdown file
fn extract_metadata_and_content(content: &str) -> (String, HashMap<String, String>, String) {
    let mut title = String::new();
    let mut metadata = HashMap::new();
    let content_text;
    
    // Simple frontmatter extraction
    let parts: Vec<&str> = content.split("---").collect();
    if parts.len() >= 3 {
        // Extract frontmatter
        let frontmatter = parts[1].trim();
        
        // Parse frontmatter lines
        for line in frontmatter.lines() {
            let line = line.trim();
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim().to_lowercase();
                let value = value.trim().to_string();
                
                // Handle title specially
                if key == "title" {
                    title = value.clone();
                }
                
                metadata.insert(key, value);
            }
        }
        
        // Extract content (everything after the second ---)
        content_text = parts[2..].join("---").trim().to_string();
    } else {
        // No frontmatter, use entire content
        content_text = content.to_string();
    }
    
    // Extract first heading as title if not found in frontmatter
    if title.is_empty() {
        if let Some(line) = content_text.lines().find(|line| line.starts_with('#')) {
            title = line.trim_start_matches('#').trim().to_string();
        }
    }
    
    // Extract plain text for indexing
    let plain_text = extract_text_from_markdown(&content_text);
    
    (title, metadata, plain_text)
}

/// Create an excerpt around the first occurrence of query terms
fn create_excerpt(text: &str, query: &str, max_length: usize) -> String {
    // If query is empty, just return the beginning of the text
    if query.is_empty() {
        return text.chars().take(max_length).collect::<String>() + "...";
    }
    
    // Convert query to lowercase for case-insensitive search
    let query_lower = query.to_lowercase();
    let text_lower = text.to_lowercase();
    
    // Find the first occurrence of any query word
    let mut best_pos = None;
    let mut best_len = 0;
    
    for query_word in query_lower.split_whitespace() {
        if let Some(pos) = text_lower.find(query_word) {
            if best_pos.is_none() || pos < best_pos.unwrap() {
                best_pos = Some(pos);
                best_len = query_word.len();
            }
        }
    }
    
    if let Some(pos) = best_pos {
        // Calculate start and end positions for the excerpt
        let context_size = (max_length - best_len) / 2;
        let start = if pos > context_size {
            pos - context_size
        } else {
            0
        };
        
        let end = if pos + best_len + context_size < text.len() {
            pos + best_len + context_size
        } else {
            text.len()
        };
        
        // Extract the excerpt
        let mut excerpt = String::new();
        if start > 0 {
            excerpt.push_str("...");
        }
        
        excerpt.push_str(&text[start..end]);
        
        if end < text.len() {
            excerpt.push_str("...");
        }
        
        excerpt
    } else {
        // If query not found, return the beginning of the text
        text.chars().take(max_length).collect::<String>() + "..."
    }
}

/// Search for a query in text content
fn search_in_text(text: &str, query: &str, case_sensitive: bool) -> bool {
    if case_sensitive {
        text.contains(query)
    } else {
        text.to_lowercase().contains(&query.to_lowercase())
    }
}

/// Search for a query in metadata
fn search_in_metadata(metadata: &HashMap<String, String>, query: &str, case_sensitive: bool) -> bool {
    for value in metadata.values() {
        if search_in_text(value, query, case_sensitive) {
            return true;
        }
    }
    false
}

/// Find all content files in the workspace
fn find_content_files(config: &Config, include_drafts: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    for (_, topic_config) in &config.content.topics {
        let topic_dir = PathBuf::from(format!("{}/{}", config.content.base_dir, topic_config.directory));
        
        // Find all markdown files in this topic
        let markdown_files = common_fs::find_files_with_extension(&topic_dir, "md")?;
        
        for file in markdown_files {
            // Skip files that aren't index.md (if we're using that convention)
            if file.file_name().unwrap_or_default() != "index.md" {
                continue;
            }
            
            // Skip drafts if not including them
            if !include_drafts {
                let content = read_file(&file)?;
                if content.contains("draft: true") {
                    continue;
                }
            }
            
            files.push(file);
        }
    }
    
    Ok(files)
}

/// Check if the search index exists
pub fn index_exists(index_path: &Path) -> bool {
    index_path.exists() && index_path.is_dir()
}

/// Build a new search index for the content
pub fn build_index(index_path: Option<&Path>, include_drafts: bool) -> Result<()> {
    let config = load_config()?;
    
    // Find all content files
    let content_files = find_content_files(&config, include_drafts)?;
    
    if content_files.is_empty() {
        return Err(anyhow::anyhow!("No content files found to index"));
    }
    
    // In a real implementation, we would build a search index here
    // For now, we'll just print a message
    println!("Found {} content files to index", content_files.len());
    
    // Create the index directory if specified
    if let Some(path) = index_path {
        if !path.exists() {
            std::fs::create_dir_all(path)?;
        }
    }
    
    Ok(())
}

/// Search for content in a specific topic
///
/// This function searches for content in a specific topic directory.
///
/// # Parameters
///
/// * `topic_dir` - Path to the topic directory
/// * `query` - Search query
/// * `options` - Search options
///
/// # Returns
///
/// Returns a list of search results
///
/// # Errors
///
/// Returns an error if the search fails
fn search_topic(topic_dir: &Path, query: &str, options: &SearchOptions) -> Result<Vec<SearchResult>> {
    let mut results = Vec::new();
    
    // Find all markdown files in the topic directory
    let markdown_files = common_fs::find_files_with_extension(topic_dir, "md")?;
    
    for file_path in markdown_files {
        // Only process index.md files
        if file_path.file_name().unwrap_or_default() != "index.md" {
            continue;
        }
        
        // Read the file content
        let content = common_fs::read_file(&file_path)?;
        
        // Extract metadata and content
        let (_title, metadata, content_text) = extract_metadata_and_content(&content);
        
        // Search in content
        let content_matches = search_in_text(&content_text, query, options.case_sensitive);
        
        // Search in metadata if requested
        let metadata_matches = if options.include_metadata {
            search_in_metadata(&metadata, query, options.case_sensitive)
        } else {
            false
        };
        
        // If we found matches, add to results
        if content_matches || metadata_matches {
            results.push(SearchResult {
                title: metadata.get("title").cloned().unwrap_or_default(),
                slug: file_path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                topic: file_path.parent().unwrap().file_name().unwrap().to_string_lossy().to_string(),
                path: file_path.to_string_lossy().to_string(),
                content: create_excerpt(&content_text, query, 160),
                tags: metadata.get("tags")
                    .cloned()
                    .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
                    .unwrap_or_default(),
                content_type: metadata.get("type").cloned().unwrap_or_default(),
                date: metadata.get("date").cloned(),
                score: 0.0,
            });
        }
    }
    
    Ok(results)
}

/// Search for content based on provided options
///
/// This function searches for content based on the provided options.
///
/// # Parameters
///
/// * `options` - Search options
///
/// # Returns
///
/// Returns a list of search results
///
/// # Errors
///
/// Returns an error if the search fails
pub fn search_content(options: &SearchOptions) -> Result<Vec<SearchResult>> {
    let config = load_config()?;
    let mut results = Vec::new();
    
    // If topic is provided, search only in that topic
    if let Some(topic) = &options.topic {
        if let Some(topic_config) = config.content.topics.get(topic) {
            let topic_dir = PathBuf::from(format!("{}/{}", config.content.base_dir, topic_config.directory));
            let topic_results = search_topic(&topic_dir, &options.query, options)?;
            results.extend(topic_results);
        } else {
            return Err(SearchError::TopicNotFound(topic.clone()).into());
        }
    } else {
        // Search in all topics
        for (_topic_key, topic_config) in &config.content.topics {
            let topic_dir = PathBuf::from(format!("{}/{}", config.content.base_dir, topic_config.directory));
            let topic_results = search_topic(&topic_dir, &options.query, options)?;
            results.extend(topic_results);
        }
    }
    
    // Apply limit if provided
    if options.limit > 0 {
        results.truncate(options.limit);
    }
    
    Ok(results)
}

#[cfg(test)]
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