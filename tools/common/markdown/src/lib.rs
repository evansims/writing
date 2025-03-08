use anyhow::{Context, Result};
use common_models::Frontmatter;
use pulldown_cmark::{html, Event, Options, Parser, Tag};
use regex::Regex;
use std::path::Path;

/// Extract frontmatter and content from a markdown file
pub fn extract_frontmatter_and_content(content: &str) -> Result<(Frontmatter, String)> {
    // Look for frontmatter between --- markers
    let re = Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)$")?;
    
    if let Some(captures) = re.captures(content) {
        let frontmatter_yaml = captures.get(1).unwrap().as_str();
        let markdown_content = captures.get(2).unwrap().as_str();
        
        let frontmatter: Frontmatter = serde_yaml::from_str(frontmatter_yaml)
            .with_context(|| "Failed to parse frontmatter")?;
        
        Ok((frontmatter, markdown_content.to_string()))
    } else {
        Err(anyhow::anyhow!("No frontmatter found in content"))
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