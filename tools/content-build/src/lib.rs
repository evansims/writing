use anyhow::{Context, Result};
use chrono::Utc;
use common_config::load_config;
use common_fs::{create_dir_all, write_file};
use common_markdown::extract_frontmatter_and_content;
use common_models::Article;
use handlebars::Handlebars;
use pulldown_cmark::{html, Options, Parser};
use quick_xml::se::to_string;
use regex::Regex;
use rss::{ChannelBuilder, ItemBuilder};
use serde::Serialize;
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Options for the build process
pub struct BuildOptions {
    pub output_dir: Option<String>,
    pub slug: Option<String>,
    pub topic: Option<String>,
    pub include_drafts: bool,
    pub skip_html: bool,
    pub skip_json: bool,
    pub skip_rss: bool,
    pub skip_sitemap: bool,
    pub verbose: bool,
}

impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            output_dir: None,
            slug: None,
            topic: None,
            include_drafts: false,
            skip_html: false,
            skip_json: false,
            skip_rss: false,
            skip_sitemap: false,
            verbose: false,
        }
    }
}

/// Process a content file and return an Article
pub fn process_content(
    content_path: &Path,
    include_drafts: bool,
) -> Result<Article> {
    // Check if the path is a directory
    let file_path = if content_path.is_dir() {
        let index_path = content_path.join("index.mdx");
        if index_path.exists() {
            index_path
        } else {
            return Err(anyhow::anyhow!(
                "Directory does not contain an index.mdx file: {:?}",
                content_path
            ));
        }
    } else {
        content_path.to_path_buf()
    };

    // Make sure the file exists
    if !file_path.exists() {
        return Err(anyhow::anyhow!("File not found: {:?}", file_path));
    }

    // Read the file content
    let content = fs::read_to_string(&file_path)
        .with_context(|| format!("Failed to read content file: {}", file_path.display()))?;

    // Extract frontmatter and markdown content
    let (frontmatter, md_content) = extract_frontmatter_and_content(&content)?;

    // Skip draft content unless specifically included
    if frontmatter.is_draft.unwrap_or(false) && !include_drafts {
        return Err(anyhow::anyhow!("Skipping draft content"));
    }

    // Convert markdown to HTML
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(&md_content, options);
    let mut html_content = String::new();
    html::push_html(&mut html_content, parser);

    // Calculate reading time based on words (assuming avg reading speed of 200 wpm)
    let word_count = md_content.split_whitespace().count();
    let reading_time = (word_count as f64 / 200.0).ceil() as u32;

    // Derive slug from directory name or parent directory
    let slug = if content_path.is_dir() {
        content_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string()
    } else {
        content_path
            .parent()
            .and_then(|parent| parent.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string()
    };

    // Find topic from path
    let topic = content_path
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .to_string();

    // Create article
    let article = Article {
        frontmatter,
        content: md_content,
        slug: slug.clone(),
        topic: topic.clone(),
        path: file_path.to_str().unwrap_or("").to_string(),
        word_count: Some(word_count),
        reading_time: Some(reading_time),
    };

    Ok(article)
}

/// Find all content files in a given directory
pub fn find_content_files(
    base_dir: &Path,
    topic_key: Option<&str>,
) -> Result<Vec<PathBuf>> {
    let config = load_config()?;
    let mut content_files = Vec::new();

    // If topic is specified, only check that topic's directory
    if let Some(topic_key) = topic_key {
        // Make sure the topic exists
        if !config.content.topics.contains_key(topic_key) {
            return Err(anyhow::anyhow!("Topic not found: {}", topic_key));
        }

        let topic_dir = base_dir.join(topic_key);
        if !topic_dir.exists() {
            return Err(anyhow::anyhow!("Topic directory not found: {:?}", topic_dir));
        }

        // Walk through the topic directory
        for entry in WalkDir::new(&topic_dir)
            .min_depth(1)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Check for subdirectories with index.mdx or for *.mdx files
            if path.is_dir() {
                let index_path = path.join("index.mdx");
                if index_path.exists() {
                    content_files.push(path.to_path_buf());
                }
            } else if path.extension().map_or(false, |ext| ext == "mdx") {
                if path.file_name().map_or(false, |name| name != "index.mdx") {
                    // Include the parent directory for standalone mdx files
                    if let Some(parent) = path.parent() {
                        content_files.push(parent.to_path_buf());
                    }
                }
            }
        }
    } else {
        // No topic specified, check all topics
        for (topic_key, _topic_config) in &config.content.topics {
            let topic_dir = base_dir.join(topic_key);
            if !topic_dir.exists() {
                continue;
            }

            // Walk through the topic directory
            for entry in WalkDir::new(&topic_dir)
                .min_depth(1)
                .max_depth(2)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();

                // Check for subdirectories with index.mdx or for *.mdx files
                if path.is_dir() {
                    let index_path = path.join("index.mdx");
                    if index_path.exists() {
                        content_files.push(path.to_path_buf());
                    }
                } else if path.extension().map_or(false, |ext| ext == "mdx") {
                    if path.file_name().map_or(false, |name| name != "index.mdx") {
                        // Include the parent directory for standalone mdx files
                        if let Some(parent) = path.parent() {
                            content_files.push(parent.to_path_buf());
                        }
                    }
                }
            }
        }
    }

    Ok(content_files)
}

/// Filter content files to find a specific content item by slug
pub fn find_content_by_slug(
    base_dir: &Path,
    slug: &str,
    topic_key: Option<&str>,
) -> Result<PathBuf> {
    let content_files = find_content_files(base_dir, topic_key)?;

    for path in content_files {
        let path_slug = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();

        if path_slug == slug {
            return Ok(path);
        }
    }

    Err(anyhow::anyhow!("Content not found with slug: {}", slug))
}

/// Process content and generate output files
pub fn build_content(options: &BuildOptions) -> Result<()> {
    // Load config
    let config = load_config()?;

    // Get content base directory
    let content_base_dir = PathBuf::from(&config.content.base_dir);

    // Determine output directory
    let output_dir = match &options.output_dir {
        Some(dir) => PathBuf::from(dir),
        None => PathBuf::from("public"),
    };

    // Create output directory if it doesn't exist
    create_dir_all(&output_dir)?;

    // Find content to process
    let content_files = if let Some(slug) = &options.slug {
        // Process a single content item
        let content_path = find_content_by_slug(
            &content_base_dir,
            slug,
            options.topic.as_deref(),
        )?;
        vec![content_path]
    } else if let Some(topic) = &options.topic {
        // Process all content for a specific topic
        find_content_files(&content_base_dir, Some(topic))?
    } else {
        // Process all content
        find_content_files(&content_base_dir, None)?
    };

    if content_files.is_empty() {
        return Err(anyhow::anyhow!("No content found to process"));
    }

    // Process each content item
    let mut articles = Vec::new();
    for content_path in &content_files {
        match process_content(content_path, options.include_drafts) {
            Ok(article) => {
                articles.push(article);
                if options.verbose {
                    println!("Processed: {}", content_path.display());
                }
            }
            Err(err) => {
                eprintln!("Error processing {}: {}", content_path.display(), err);
            }
        }
    }

    if articles.is_empty() {
        return Err(anyhow::anyhow!("No content items were processed successfully"));
    }

    // Generate JSON files if not skipped
    if !options.skip_json {
        // Create data directory
        let data_dir = output_dir.join("data");
        create_dir_all(&data_dir)?;

        // Write individual JSON files
        for article in &articles {
            let json_path = data_dir.join(format!("{}.json", article.slug));
            let json = serde_json::to_string_pretty(&article)
                .with_context(|| format!("Failed to serialize article to JSON: {}", article.slug))?;
            write_file(&json_path, &json)
                .with_context(|| format!("Failed to write JSON file: {:?}", json_path))?;
        }

        // Write all.json
        let all_json_path = data_dir.join("all.json");
        let json = serde_json::to_string_pretty(&articles)
            .with_context(|| "Failed to serialize all articles to JSON")?;
        write_file(&all_json_path, &json)
            .with_context(|| format!("Failed to write all.json file: {:?}", all_json_path))?;
    }

    // Generate HTML files if not skipped and templates are available
    if !options.skip_html {
        // Check if templates directory exists
        let templates_dir = PathBuf::from("templates");
        if templates_dir.exists() {
            let template_file = templates_dir.join("article.hbs");
            if template_file.exists() {
                // Create html directory
                let html_dir = output_dir.join("html");
                create_dir_all(&html_dir)?;

                // Set up handlebars
                let mut handlebars = Handlebars::new();
                handlebars
                    .register_template_file("article", template_file)
                    .with_context(|| "Failed to register article template")?;

                // Render HTML for each content item
                for article in &articles {
                    let html_path = html_dir.join(format!("{}.html", article.slug));
                    let rendered = handlebars
                        .render("article", &article)
                        .with_context(|| format!("Failed to render HTML for {}", article.slug))?;

                    write_file(&html_path, &rendered)
                        .with_context(|| format!("Failed to write HTML file: {:?}", html_path))?;
                }
            }
        }
    }

    // Generate sitemap if not skipped
    if !options.skip_sitemap {
        generate_sitemap(&output_dir, &articles, &config)?;
    }

    // Generate RSS feed if not skipped
    if !options.skip_rss {
        generate_rss_feed(&output_dir, &articles, &config)?;
    }

    Ok(())
}

/// Definition for XML sitemap
#[derive(Serialize)]
struct Sitemap {
    #[serde(rename = "urlset")]
    urlset: UrlSet,
}

#[derive(Serialize)]
struct UrlSet {
    #[serde(rename = "@xmlns")]
    xmlns: String,
    #[serde(rename = "url")]
    urls: Vec<SitemapUrl>,
}

#[derive(Serialize)]
struct SitemapUrl {
    loc: String,
    lastmod: String,
    changefreq: String,
    priority: String,
}

/// Generate XML sitemap
pub fn generate_sitemap(
    output_dir: &Path,
    articles: &[Article],
    config: &common_models::Config,
) -> Result<()> {
    let mut urls = Vec::new();
    let site_url = config.publication.site_url.clone().unwrap_or_else(|| "https://example.com".to_string());

    // Add homepage
    urls.push(SitemapUrl {
        loc: site_url.clone(),
        lastmod: Utc::now().format("%Y-%m-%d").to_string(),
        changefreq: "daily".to_string(),
        priority: "1.0".to_string(),
    });

    // Add topic pages
    for (topic_key, _topic_config) in &config.content.topics {
        urls.push(SitemapUrl {
            loc: format!("{}/{}", site_url, topic_key),
            lastmod: Utc::now().format("%Y-%m-%d").to_string(),
            changefreq: "weekly".to_string(),
            priority: "0.8".to_string(),
        });
    }

    // Add content pages
    for article in articles {
        if article.frontmatter.is_draft.unwrap_or(false) {
            continue;
        }

        let url = format!("{}/{}/{}", site_url, article.topic, article.slug);
        let last_mod = article.frontmatter.updated_at
            .as_ref()
            .or(article.frontmatter.published_at.as_ref())
            .unwrap_or(&"".to_string())
            .to_string();

        urls.push(SitemapUrl {
            loc: url,
            lastmod: last_mod,
            changefreq: "monthly".to_string(),
            priority: "0.7".to_string(),
        });
    }

    // Create the sitemap
    let sitemap = Sitemap {
        urlset: UrlSet {
            xmlns: "http://www.sitemaps.org/schemas/sitemap/0.9".to_string(),
            urls,
        },
    };

    // Convert to XML
    let xml = to_string(&sitemap).context("Failed to generate sitemap XML")?;

    // Write to file
    let sitemap_path = output_dir.join("sitemap.xml");
    write_file(&sitemap_path, &xml)
        .with_context(|| format!("Failed to write sitemap file: {:?}", sitemap_path))?;

    Ok(())
}

/// Generate RSS feed
pub fn generate_rss_feed(
    output_dir: &Path,
    articles: &[Article],
    config: &common_models::Config,
) -> Result<()> {
    let site_url = config.publication.site_url.clone().unwrap_or_else(|| "https://example.com".to_string());
    let site_title = config.publication.author.clone();
    let site_description = "Articles and content".to_string();
    let empty_string = "".to_string();

    // Sort content items by date (newest first)
    let mut sorted_articles = articles.to_vec();
    sorted_articles.sort_by(|a, b| {
        let a_date = a.frontmatter.published_at.as_ref().unwrap_or(&empty_string);
        let b_date = b.frontmatter.published_at.as_ref().unwrap_or(&empty_string);
        b_date.cmp(a_date)
    });

    // Keep only non-draft items
    sorted_articles.retain(|article| !article.frontmatter.is_draft.unwrap_or(false));

    // Limit to 20 most recent items
    let items_to_include = sorted_articles.iter().take(20);

    // Create RSS items
    let mut rss_items = Vec::new();
    for article in items_to_include {
        // Clean HTML content for RSS (remove code blocks that might not render well in feeds)
        let re = Regex::new(r"<pre><code>(.*?)</code></pre>").unwrap();
        let clean_html = re.replace_all(&article.content, "[Code snippet]");

        // Create RSS item
        let rss_item = ItemBuilder::default()
            .title(article.frontmatter.title.clone())
            .link(format!("{}/{}/{}", site_url, article.topic, article.slug))
            .description(clean_html.to_string())
            .pub_date(article.frontmatter.published_at.clone().unwrap_or_default())
            .build();

        rss_items.push(rss_item);
    }

    // Create RSS channel
    let channel = ChannelBuilder::default()
        .title(site_title)
        .link(site_url)
        .description(site_description)
        .items(rss_items)
        .build();

    // Write to file
    let rss_path = output_dir.join("rss.xml");
    let rss_string = channel.to_string();
    write_file(&rss_path, &rss_string)
        .with_context(|| format!("Failed to write RSS file: {:?}", rss_path))?;

    Ok(())
}