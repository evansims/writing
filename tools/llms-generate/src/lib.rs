use anyhow::{Context, Result};
use chrono::Utc;
use comrak::{markdown_to_html, ComrakOptions};
use common_config::load_config;
use common_markdown::extract_frontmatter_and_content;
use common_models::{Config, Frontmatter};
use common_fs::write_file;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Options for generating LLMS files
#[derive(Debug)]
pub struct LlmsOptions {
    /// Output directory for generated files
    pub output_dir: PathBuf,
    /// Site URL for generating absolute URLs
    pub site_url: Option<String>,
    /// Whether to include drafts in the output
    pub include_drafts: bool,
}

impl Default for LlmsOptions {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("."),
            site_url: None,
            include_drafts: false,
        }
    }
}

/// Article information used for LLMS generation
#[derive(Debug, Clone)]
pub struct Article {
    /// Article title
    pub title: String,
    /// Article tagline/description
    pub tagline: String,
    /// Article slug
    pub slug: String,
    /// Topics the article belongs to
    pub topics: Vec<String>,
    /// Tags associated with the article
    pub tags: Vec<String>,
    /// Publication date
    pub published: String,
    /// Path to the article
    pub path: PathBuf,
    /// Article content (markdown)
    pub content: String,
    /// Whether the article is a draft
    pub draft: bool,
}

/// Strip HTML tags from a string
pub fn strip_html_tags(html: &str) -> String {
    let re = Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(html, "").into_owned()
}

/// Collect all articles from the content directory
pub fn collect_articles(config: &Config, include_drafts: bool) -> Result<Vec<Article>> {
    let mut articles = Vec::new();
    
    // Walk through content directory
    for entry in WalkDir::new(&config.content.base_dir).min_depth(3).max_depth(3) {
        let entry = entry?;
        let path = entry.path();
        
        if path.file_name().unwrap_or_default() == "index.mdx" {
            let content = fs::read_to_string(path)
                .context(format!("Failed to read file: {:?}", path))?;
            
            // Extract frontmatter and content
            let (frontmatter, markdown_content) = match extract_frontmatter_and_content(&content) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Warning: Failed to parse frontmatter in {:?}: {}", path, e);
                    continue;
                }
            };
            
            // Skip drafts if not included
            if frontmatter.draft.unwrap_or(false) && !include_drafts {
                continue;
            }
            
            // Skip articles without required fields
            if frontmatter.title.is_empty() || frontmatter.published.is_none() {
                continue;
            }
            
            let article_path = path.parent().unwrap();
            let relative_path = article_path.strip_prefix(Path::new("."))?;
            
            articles.push(Article {
                title: frontmatter.title.clone(),
                tagline: frontmatter.tagline.clone().unwrap_or_default(),
                slug: frontmatter.slug.clone().unwrap_or_default(),
                topics: frontmatter.topics.clone().unwrap_or_default(),
                tags: frontmatter.tags.clone().unwrap_or_default(),
                published: frontmatter.published.clone().unwrap_or_default(),
                path: relative_path.to_path_buf(),
                content: markdown_content.clone(),
                draft: frontmatter.draft.unwrap_or(false),
            });
        }
    }
    
    // Sort by publication date (newest first)
    articles.sort_by(|a, b| b.published.cmp(&a.published));
    
    Ok(articles)
}

/// Generate llms.txt content according to the llmstxt.org standard
pub fn generate_llms_txt(articles: &[Article], site_url: &str) -> String {
    let mut content = String::new();
    
    // Add header
    content.push_str("# LLMS\n");
    content.push_str("# Link List Metadata Standard\n");
    content.push_str(&format!("# Generated: {}\n\n", Utc::now().to_rfc3339()));
    
    // Add non-draft articles
    for article in articles.iter().filter(|a| !a.draft) {
        let url = if site_url.ends_with('/') {
            format!("{}{}", site_url, article.path.display())
        } else {
            format!("{}/{}", site_url, article.path.display())
        };
        
        content.push_str(&format!("# {}\n", article.title));
        content.push_str(&format!("{}\n", url));
        content.push_str(&format!("{}\n\n", article.tagline));
    }
    
    content
}

/// Generate llms-full.txt content with additional information
pub fn generate_llms_full_txt(articles: &[Article], site_url: &str, include_drafts: bool) -> String {
    let mut content = String::new();
    
    // Add header
    content.push_str("# LLMS-FULL\n");
    content.push_str("# Link List Metadata Standard - Full Version\n");
    content.push_str(&format!("# Generated: {}\n\n", Utc::now().to_rfc3339()));
    
    // Filter articles based on inclusion of drafts
    let filtered_articles: Vec<&Article> = if include_drafts {
        articles.iter().collect()
    } else {
        articles.iter().filter(|a| !a.draft).collect()
    };
    
    // Add articles with full content
    for article in filtered_articles {
        let url = if site_url.ends_with('/') {
            format!("{}{}", site_url, article.path.display())
        } else {
            format!("{}/{}", site_url, article.path.display())
        };
        
        // Convert markdown to HTML, then strip tags for plain text
        let mut comrak_options = ComrakOptions::default();
        comrak_options.extension.strikethrough = true;
        let html = markdown_to_html(&article.content, &comrak_options);
        let text_content = strip_html_tags(&html);
        
        // Add article metadata
        content.push_str(&format!("# {}\n", article.title));
        content.push_str(&format!("{}\n", url));
        content.push_str(&format!("Published: {}\n", article.published));
        content.push_str(&format!("Tags: {}\n", article.tags.join(", ")));
        content.push_str(&format!("Topics: {}\n", article.topics.join(", ")));
        content.push_str(&format!("Description: {}\n\n", article.tagline));
        
        // Add content with 80 character wrapping
        let mut current_line = String::new();
        for word in text_content.split_whitespace() {
            if current_line.len() + word.len() + 1 > 80 {
                content.push_str(&format!("{}\n", current_line));
                current_line = word.to_string();
            } else {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            }
        }
        
        if !current_line.is_empty() {
            content.push_str(&format!("{}\n", current_line));
        }
        
        content.push_str("\n---\n\n");
    }
    
    content
}

/// Generate LLMS files from the provided options
pub fn generate_llms(options: &LlmsOptions) -> Result<(PathBuf, PathBuf)> {
    // Get site URL
    let site_url = match &options.site_url {
        Some(url) => url.clone(),
        None => {
            // Try to get from config
            let config = load_config()?;
            match config.publication.site {
                Some(url) => url,
                None => return Err(anyhow::anyhow!("Site URL is required. Please provide it with --site-url or set it in config.yaml")),
            }
        }
    };
    
    if site_url.is_empty() {
        return Err(anyhow::anyhow!("Site URL is required. Please provide it with --site-url or set it in config.yaml"));
    }
    
    // Read configuration
    let config = load_config()?;
    
    // Collect articles
    let articles = collect_articles(&config, options.include_drafts)?;
    
    if articles.is_empty() {
        return Err(anyhow::anyhow!("No articles found"));
    }
    
    // Create output directory if it doesn't exist
    if !options.output_dir.exists() {
        fs::create_dir_all(&options.output_dir)
            .context(format!("Failed to create output directory: {:?}", options.output_dir))?;
    }
    
    // Generate llms.txt
    let llms_txt = generate_llms_txt(&articles, &site_url);
    let llms_txt_path = options.output_dir.join("llms.txt");
    write_file(&llms_txt_path, &llms_txt)
        .context(format!("Failed to write file: {:?}", llms_txt_path))?;
    
    // Generate llms-full.txt
    let llms_full_txt = generate_llms_full_txt(&articles, &site_url, options.include_drafts);
    let llms_full_txt_path = options.output_dir.join("llms-full.txt");
    write_file(&llms_full_txt_path, &llms_full_txt)
        .context(format!("Failed to write file: {:?}", llms_full_txt_path))?;
    
    Ok((llms_txt_path, llms_full_txt_path))
} 