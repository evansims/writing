use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use comrak::{markdown_to_html, ComrakOptions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use chrono::Utc;

#[derive(Parser)]
#[command(author, version, about = "Generate llms.txt and llms-full.txt files according to the llmstxt.org standard")]
struct Args {
    /// Output directory for llms.txt and llms-full.txt (default: current directory)
    #[arg(short, long, default_value = ".")]
    output_dir: PathBuf,

    /// Site URL (required for generating absolute URLs)
    #[arg(short, long)]
    site_url: Option<String>,

    /// Include drafts in the output
    #[arg(short, long)]
    include_drafts: bool,
}

#[derive(Deserialize, Debug, Clone)]
struct Frontmatter {
    title: String,
    tagline: String,
    slug: String,
    topics: Vec<String>,
    tags: Vec<String>,
    published: String,
    #[serde(default)]
    draft: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TopicConfig {
    name: String,
    description: String,
    path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ContentConfig {
    base_dir: String,
    topics: HashMap<String, TopicConfig>,
    tags: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ImagesConfig {
    formats: Vec<HashMap<String, String>>,
    quality: HashMap<String, HashMap<String, u8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PublicationConfig {
    author: String,
    copyright: String,
    site: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Config {
    content: ContentConfig,
    images: ImagesConfig,
    publication: PublicationConfig,
}

struct Article {
    title: String,
    tagline: String,
    slug: String,
    topics: Vec<String>,
    tags: Vec<String>,
    published: String,
    path: PathBuf,
    content: String,
    draft: bool,
}

fn read_config() -> Result<Config> {
    let config_path = "config.yaml";
    let config_content = fs::read_to_string(config_path)
        .context(format!("Failed to read config file: {}", config_path))?;
    
    let config: Config = serde_yaml::from_str(&config_content)
        .context("Failed to parse config.yaml")?;
    
    Ok(config)
}

fn extract_frontmatter_and_content(file_content: &str) -> Result<(Frontmatter, String)> {
    let parts: Vec<&str> = file_content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(anyhow::anyhow!("Invalid frontmatter format"));
    }
    
    let yaml = parts[1];
    let frontmatter: Frontmatter = serde_yaml::from_str(yaml)
        .context("Failed to parse frontmatter")?;
    
    let content = parts[2].trim();
    
    Ok((frontmatter, content.to_string()))
}

fn strip_html_tags(html: &str) -> String {
    // Simple HTML tag removal - for a more robust solution, consider using a proper HTML parser
    let re = regex::Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(html, "").to_string()
}

fn collect_articles(config: &Config, include_drafts: bool) -> Result<Vec<Article>> {
    let mut articles = Vec::new();
    let base_dir = &config.content.base_dir;
    
    // Walk through content directory
    for entry in WalkDir::new(base_dir).min_depth(3).max_depth(3) {
        let entry = entry?;
        let path = entry.path();
        
        if path.file_name().unwrap_or_default() == "index.mdx" {
            let content = fs::read_to_string(path)
                .context(format!("Failed to read file: {:?}", path))?;
            
            let (frontmatter, article_content) = match extract_frontmatter_and_content(&content) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("Warning: Failed to parse frontmatter in {:?}: {}", path, e);
                    continue;
                }
            };
            
            // Skip drafts if not explicitly included
            if frontmatter.draft && !include_drafts {
                continue;
            }
            
            let article = Article {
                title: frontmatter.title,
                tagline: frontmatter.tagline,
                slug: frontmatter.slug,
                topics: frontmatter.topics,
                tags: frontmatter.tags,
                published: frontmatter.published,
                path: path.to_path_buf(),
                content: article_content,
                draft: frontmatter.draft,
            };
            
            articles.push(article);
        }
    }
    
    // Sort articles by publication date (newest first)
    articles.sort_by(|a, b| b.published.cmp(&a.published));
    
    Ok(articles)
}

fn generate_llms_txt(articles: &[Article], site_url: &str) -> String {
    let mut output = String::new();
    
    // Add header
    output.push_str("# llms.txt\n");
    output.push_str("# https://llmstxt.org\n");
    output.push_str("# v=1\n\n");
    
    // Add articles
    for article in articles {
        // Skip drafts for llms.txt
        if article.draft {
            continue;
        }
        
        let article_path = article.path.parent().unwrap();
        let relative_path = article_path.strip_prefix(".").unwrap_or(article_path);
        let url = format!("{}/{}", site_url.trim_end_matches('/'), relative_path.display());
        
        output.push_str(&format!("# {}\n", article.title));
        output.push_str(&format!("url: {}\n", url));
        output.push_str(&format!("date: {}\n", article.published));
        output.push_str(&format!("description: {}\n", article.tagline));
        
        if !article.tags.is_empty() {
            output.push_str(&format!("tags: {}\n", article.tags.join(", ")));
        }
        
        output.push_str("\n");
    }
    
    // Add footer
    output.push_str(&format!("# Generated on {}\n", Utc::now().format("%Y-%m-%d")));
    
    output
}

fn generate_llms_full_txt(articles: &[Article], site_url: &str, include_drafts: bool) -> String {
    let mut output = String::new();
    
    // Add header
    output.push_str("# llms-full.txt\n");
    output.push_str("# https://llmstxt.org\n");
    output.push_str("# v=1\n\n");
    
    // Add articles
    for article in articles {
        // Skip drafts if not explicitly included
        if article.draft && !include_drafts {
            continue;
        }
        
        let article_path = article.path.parent().unwrap();
        let relative_path = article_path.strip_prefix(".").unwrap_or(article_path);
        let url = format!("{}/{}", site_url.trim_end_matches('/'), relative_path.display());
        
        output.push_str(&format!("# {}\n", article.title));
        output.push_str(&format!("url: {}\n", url));
        output.push_str(&format!("date: {}\n", article.published));
        output.push_str(&format!("description: {}\n", article.tagline));
        
        if !article.tags.is_empty() {
            output.push_str(&format!("tags: {}\n", article.tags.join(", ")));
        }
        
        if article.draft {
            output.push_str("draft: true\n");
        }
        
        // Convert markdown to plain text
        let options = ComrakOptions::default();
        let html = markdown_to_html(&article.content, &options);
        let plain_text = strip_html_tags(&html);
        
        output.push_str("\n");
        output.push_str(&plain_text);
        output.push_str("\n\n---\n\n");
    }
    
    // Add footer
    output.push_str(&format!("# Generated on {}\n", Utc::now().format("%Y-%m-%d")));
    
    output
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Get site URL
    let site_url = match args.site_url {
        Some(url) => url,
        None => {
            // Try to get from config
            let config = read_config()?;
            config.publication.site
        }
    };
    
    if site_url.is_empty() {
        return Err(anyhow::anyhow!("Site URL is required. Please provide it with --site-url or set it in config.yaml"));
    }
    
    // Read configuration
    let config = read_config()?;
    
    // Collect articles
    let articles = collect_articles(&config, args.include_drafts)?;
    
    if articles.is_empty() {
        println!("{} No articles found", "WARNING:".yellow().bold());
        return Ok(());
    }
    
    // Create output directory if it doesn't exist
    if !args.output_dir.exists() {
        fs::create_dir_all(&args.output_dir)
            .context(format!("Failed to create output directory: {:?}", args.output_dir))?;
    }
    
    // Generate llms.txt
    let llms_txt = generate_llms_txt(&articles, &site_url);
    let llms_txt_path = args.output_dir.join("llms.txt");
    fs::write(&llms_txt_path, llms_txt)
        .context(format!("Failed to write file: {:?}", llms_txt_path))?;
    
    // Generate llms-full.txt
    let llms_full_txt = generate_llms_full_txt(&articles, &site_url, args.include_drafts);
    let llms_full_txt_path = args.output_dir.join("llms-full.txt");
    fs::write(&llms_full_txt_path, llms_full_txt)
        .context(format!("Failed to write file: {:?}", llms_full_txt_path))?;
    
    println!("{} Generated llms.txt with {} articles", "SUCCESS:".green().bold(), articles.iter().filter(|a| !a.draft).count());
    println!("{} Generated llms-full.txt with {} articles", "SUCCESS:".green().bold(), articles.len());
    
    Ok(())
} 