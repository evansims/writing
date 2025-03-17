use anyhow::{Context, Result};
use common_config::load_config;
use common_markdown::extract_frontmatter_and_content;
use common_models::Config;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Options for table of contents generation
#[derive(Debug)]
pub struct TocOptions {
    /// Output file path
    pub output: PathBuf,
    /// Custom title (optional)
    pub title: Option<String>,
    /// Custom description (optional)
    pub description: Option<String>,
}

impl Default for TocOptions {
    fn default() -> Self {
        Self {
            output: PathBuf::from("index.md"),
            title: None,
            description: None,
        }
    }
}

/// Article information used for TOC generation
pub struct ArticleInfo {
    /// Article title
    pub title: String,
    /// Article tagline/description
    pub tagline: String,
    /// Path to the article
    pub path: PathBuf,
}

/// Helper function to convert string to title case
pub fn to_title_case(s: &str) -> String {
    let mut c = 0;
    s.chars().map(|x| {
        if c == 0 || s.chars().nth(c - 1).unwrap_or(' ') == ' ' {
            c += 1;
            x.to_uppercase().next().unwrap_or(x)
        } else {
            c += 1;
            x
        }
    }).collect()
}

/// Collect all articles from the content directory, organized by topic
pub fn collect_articles(config: &Config) -> Result<HashMap<String, Vec<ArticleInfo>>> {
    // Initialize articles map with all topics from config using a more functional approach
    let mut articles: HashMap<String, Vec<ArticleInfo>> = config.content.topics.keys()
        .map(|topic_key| (topic_key.clone(), Vec::new()))
        .collect();
    
    // Process content directory and collect articles
    let walkdir_iter = WalkDir::new(&config.content.base_dir)
        .min_depth(3)
        .max_depth(3)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_name() == "index.mdx");
    
    for entry in walkdir_iter {
        let path = entry.path();
        
        // Use functional error handling with and_then
        if let Err(e) = fs::read_to_string(path)
            .context(format!("Failed to read file: {:?}", path))
            .and_then(|content| {
                // Extract frontmatter and add to appropriate topics
                extract_frontmatter_and_content(&content)
                    .map_err(|e| {
                        anyhow::anyhow!("Failed to parse frontmatter in {:?}: {}", path, e)
                    })
                    .and_then(|(frontmatter, _)| {
                        let article_path = path.parent().unwrap();
                        let relative_path = article_path.strip_prefix(Path::new("."))?;
                        
                        // Process topic references if present
                        if let Some(topics) = &frontmatter.topics {
                            for topic_key in topics {
                                if let Some(articles_for_topic) = articles.get_mut(topic_key) {
                                    articles_for_topic.push(ArticleInfo {
                                        title: frontmatter.title.clone(),
                                        tagline: frontmatter.tagline.clone().unwrap_or_default(),
                                        path: relative_path.to_path_buf(),
                                    });
                                }
                            }
                        }
                        
                        Ok(())
                    })
            })
        {
            eprintln!("Warning: {}", e);
        }
    }
    
    Ok(articles)
}

/// Generate table of contents markdown
pub fn generate_toc_content(
    config: &Config,
    articles: &HashMap<String, Vec<ArticleInfo>>,
    options: &TocOptions,
) -> String {
    let mut toc = String::new();
    
    // Add title
    let title = options.title.as_deref().unwrap_or("Writing Collection");
    toc.push_str(&format!("# {}\n\n", title));
    
    // Add description
    let description = options.description.as_deref().unwrap_or(
        "A curated collection of personal writings exploring creativity, engineering, focus, mindset, strategy, and tools."
    );
    toc.push_str(&format!("{}\n\n", description));
    
    // Add table of contents with topic descriptions
    for (topic_key, topic_config) in &config.content.topics {
        toc.push_str(&format!("## {}\n\n", topic_config.name));
        toc.push_str(&format!("{}\n\n", topic_config.description));
        
        if let Some(articles_for_topic) = articles.get(topic_key) {
            if articles_for_topic.is_empty() {
                toc.push_str("*No articles yet*\n\n");
            } else {
                for article in articles_for_topic {
                    toc.push_str(&format!("- [{}]({}) - {}\n", 
                        article.title, 
                        article.path.display(), 
                        article.tagline
                    ));
                }
                toc.push_str("\n");
            }
        }
    }
    
    // Add footer
    toc.push_str("---\n\n");
    toc.push_str("This collection is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/).\n");
    
    toc
}

/// Generate table of contents and write to file
pub fn generate_toc(options: &TocOptions) -> Result<PathBuf> {
    // Load configuration
    let config = load_config()?;
    
    // Collect articles by topic
    let articles = collect_articles(&config)?;
    
    // Generate table of contents content
    let toc_content = generate_toc_content(&config, &articles, options);
    
    // Write to output file
    fs::write(&options.output, toc_content)
        .context(format!("Failed to write to file: {:?}", options.output))?;
    
    Ok(options.output.clone())
} 