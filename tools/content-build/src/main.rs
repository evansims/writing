use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use fs_extra::dir::create_all;
use handlebars::Handlebars;
use pulldown_cmark::{html, Options, Parser as MarkdownParser};
use rss::{Channel, ItemBuilder};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yaml;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use chrono::{DateTime, NaiveDate, Utc};
use quick_xml::Writer as XmlWriter;
use quick_xml::events::{Event, BytesStart, BytesEnd, BytesText};
use std::io::Cursor;
use regex;

#[derive(Parser)]
#[command(author, version, about = "Build static JSON and HTML from content")]
struct Args {
    /// Output directory for built content
    #[arg(short, long, default_value = "build/content")]
    output_dir: PathBuf,

    /// Content slug (optional, will build all if not provided)
    #[arg(short, long)]
    slug: Option<String>,
    
    /// Topic (optional)
    #[arg(short, long)]
    topic: Option<String>,
    
    /// Include drafts in build
    #[arg(short, long)]
    include_drafts: bool,
    
    /// Template directory
    #[arg(short, long, default_value = "templates/build")]
    template_dir: PathBuf,
    
    /// Site URL for absolute links
    #[arg(short, long, default_value = "https://example.com")]
    site_url: String,
    
    /// Site title for RSS feed
    #[arg(long, default_value = "My Website")]
    site_title: String,
    
    /// Site description for RSS feed
    #[arg(long, default_value = "My personal website and blog")]
    site_description: String,
    
    /// Site logo URL for RSS feed (relative to site_url)
    #[arg(long, default_value = "logo.png")]
    site_logo: String,
    
    /// Site language for RSS feed
    #[arg(long, default_value = "en-us")]
    site_language: String,
    
    /// Site webmaster email for RSS feed
    #[arg(long)]
    site_webmaster: Option<String>,
    
    /// Site editor email for RSS feed
    #[arg(long)]
    site_editor: Option<String>,
    
    /// Maximum number of items in RSS feed
    #[arg(long, default_value = "20")]
    rss_max_items: usize,
    
    /// Skip generating sitemap.xml
    #[arg(long)]
    skip_sitemap: bool,
    
    /// Skip generating RSS feed
    #[arg(long)]
    skip_rss: bool,
}

#[derive(Deserialize, Debug)]
struct TopicConfig {
    name: String,
    description: String,
    path: String,
}

#[derive(Deserialize, Debug)]
struct ContentConfig {
    base_dir: String,
    topics: HashMap<String, TopicConfig>,
}

#[derive(Deserialize, Debug)]
struct Config {
    content: ContentConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ContentFrontmatter {
    title: String,
    published: String,
    updated: Option<String>,
    description: Option<String>,
    tagline: Option<String>,
    tags: Vec<String>,
    topics: Vec<String>,
    draft: Option<bool>,
    image: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ContentMetadata {
    slug: String,
    topic: String,
    topic_name: String,
    path: String,
    url: String,
    frontmatter: ContentFrontmatter,
    word_count: usize,
    reading_time: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentItem {
    metadata: ContentMetadata,
    content_markdown: String,
    content_html: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContentIndex {
    topics: HashMap<String, TopicInfo>,
    content: Vec<ContentMetadata>,
    tags: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TopicInfo {
    key: String,
    name: String,
    description: String,
    path: String,
    count: usize,
}

fn read_config() -> Result<Config> {
    let config_content = fs::read_to_string("config.yaml")
        .context("Failed to read config.yaml")?;
    
    let config: Config = serde_yaml::from_str(&config_content)
        .context("Failed to parse config.yaml")?;
    
    Ok(config)
}

fn extract_frontmatter_and_content(file_content: &str) -> Result<(ContentFrontmatter, String)> {
    let parts: Vec<&str> = file_content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(anyhow::anyhow!("Invalid frontmatter format"));
    }
    
    let yaml = parts[1];
    let frontmatter: ContentFrontmatter = serde_yaml::from_str(yaml)
        .context("Failed to parse frontmatter")?;
    
    let content = parts[2].trim();
    
    Ok((frontmatter, content.to_string()))
}

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    
    let parser = MarkdownParser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    html_output
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn calculate_reading_time(word_count: usize) -> usize {
    (word_count as f64 / 200.0).ceil() as usize // Assuming 200 words per minute
}

fn process_content(
    content_path: &Path,
    topic_key: &str,
    topic_config: &TopicConfig,
    args: &Args,
) -> Result<Option<ContentItem>> {
    // Check if the path is a directory and look for index.mdx
    let file_path = if content_path.is_dir() {
        content_path.join("index.mdx")
    } else {
        content_path.to_path_buf()
    };
    
    // Check if the file exists
    if !file_path.exists() {
        return Err(anyhow::anyhow!("Content file not found: {:?}", file_path));
    }
    
    let content = fs::read_to_string(&file_path)
        .context(format!("Failed to read content: {:?}", file_path))?;
    
    let (frontmatter, markdown_content) = extract_frontmatter_and_content(&content)?;
    
    // Skip draft content if not including drafts
    if let Some(true) = frontmatter.draft {
        if !args.include_drafts {
            return Ok(None);
        }
    }
    
    // Convert markdown to HTML
    let html_content = markdown_to_html(&markdown_content);
    
    // Count words and calculate reading time
    let word_count = count_words(&markdown_content);
    let reading_time = calculate_reading_time(word_count);
    
    // Get the slug from the directory name
    let slug = if content_path.is_dir() {
        content_path.file_name().unwrap().to_string_lossy().to_string()
    } else {
        content_path.parent().unwrap().file_name().unwrap().to_string_lossy().to_string()
    };
    
    // Use tagline as description if description is not provided
    let description = if frontmatter.description.is_none() && frontmatter.tagline.is_some() {
        frontmatter.tagline.clone()
    } else {
        Some(frontmatter.description.clone().unwrap_or_else(|| String::from("")))
    };
    
    let mut frontmatter_with_description = frontmatter.clone();
    frontmatter_with_description.description = description;
    
    let path = format!("{}/{}", topic_config.path, slug);
    let url = format!("{}/{}/{}", args.site_url, topic_config.path, slug);
    
    let metadata = ContentMetadata {
        slug,
        topic: topic_key.to_string(),
        topic_name: topic_config.name.clone(),
        path,
        url,
        frontmatter: frontmatter_with_description,
        word_count,
        reading_time,
    };
    
    Ok(Some(ContentItem {
        metadata,
        content_markdown: markdown_content,
        content_html: html_content,
    }))
}

fn build_content_index(content_items: &[ContentItem]) -> ContentIndex {
    let mut topics: HashMap<String, TopicInfo> = HashMap::new();
    let mut tags: HashMap<String, Vec<String>> = HashMap::new();
    let mut content: Vec<ContentMetadata> = Vec::new();
    
    for item in content_items {
        // Add to content list
        content.push(item.metadata.clone());
        
        // Update topic info
        let topic_entry = topics.entry(item.metadata.topic.clone()).or_insert_with(|| TopicInfo {
            key: item.metadata.topic.clone(),
            name: item.metadata.topic_name.clone(),
            description: String::new(), // Will be filled later
            path: item.metadata.path.clone().split('/').next().unwrap_or("").to_string(),
            count: 0,
        });
        topic_entry.count += 1;
        
        // Update tag index
        for tag in &item.metadata.frontmatter.tags {
            tags.entry(tag.clone())
                .or_insert_with(Vec::new)
                .push(item.metadata.slug.clone());
        }
    }
    
    // Sort content by date (newest first)
    content.sort_by(|a, b| {
        if a.frontmatter.published == "DRAFT" {
            return std::cmp::Ordering::Greater;
        }
        if b.frontmatter.published == "DRAFT" {
            return std::cmp::Ordering::Less;
        }
        b.frontmatter.published.cmp(&a.frontmatter.published)
    });
    
    ContentIndex {
        topics,
        content,
        tags,
    }
}

fn generate_sitemap(content_items: &[ContentItem], site_url: &str) -> Result<String> {
    let mut buffer = Cursor::new(Vec::new());
    let mut writer = XmlWriter::new_with_indent(&mut buffer, b' ', 2);
    
    // XML declaration
    writer.write_event(Event::Decl(quick_xml::events::BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        None
    )))?;
    
    // Urlset start tag with namespaces
    let mut urlset = BytesStart::new("urlset");
    urlset.push_attribute(("xmlns", "http://www.sitemaps.org/schemas/sitemap/0.9"));
    urlset.push_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"));
    urlset.push_attribute(("xsi:schemaLocation", "http://www.sitemaps.org/schemas/sitemap/0.9 http://www.sitemaps.org/schemas/sitemap/0.9/sitemap.xsd"));
    writer.write_event(Event::Start(urlset))?;
    
    // Add homepage
    writer.write_event(Event::Start(BytesStart::new("url")))?;
    writer.write_event(Event::Start(BytesStart::new("loc")))?;
    writer.write_event(Event::Text(BytesText::new(site_url)))?;
    writer.write_event(Event::End(BytesEnd::new("loc")))?;
    
    // Get the most recent update date from all content items for the homepage lastmod
    let mut latest_update = None;
    for item in content_items {
        if let Some(true) = item.metadata.frontmatter.draft {
            continue;
        }
        
        let item_date = parse_date_for_sitemap(&item.metadata.frontmatter.updated
            .clone()
            .unwrap_or_else(|| item.metadata.frontmatter.published.clone()));
            
        if let Some(date) = item_date {
            if latest_update.is_none() || latest_update.as_ref().unwrap() < &date {
                latest_update = Some(date);
            }
        }
    }
    
    // Add lastmod for homepage if we have content
    if let Some(latest_date) = latest_update {
        writer.write_event(Event::Start(BytesStart::new("lastmod")))?;
        writer.write_event(Event::Text(BytesText::new(&latest_date)))?;
        writer.write_event(Event::End(BytesEnd::new("lastmod")))?;
    }
    
    writer.write_event(Event::Start(BytesStart::new("changefreq")))?;
    writer.write_event(Event::Text(BytesText::new("daily")))?;
    writer.write_event(Event::End(BytesEnd::new("changefreq")))?;
    writer.write_event(Event::Start(BytesStart::new("priority")))?;
    writer.write_event(Event::Text(BytesText::new("1.0")))?;
    writer.write_event(Event::End(BytesEnd::new("priority")))?;
    writer.write_event(Event::End(BytesEnd::new("url")))?;
    
    // Add content items
    for item in content_items {
        // Skip draft content
        if let Some(true) = item.metadata.frontmatter.draft {
            continue;
        }
        
        let url = format!("{}/{}/{}", site_url, item.metadata.topic, item.metadata.slug);
        
        // Format dates according to W3C Datetime format (ISO 8601)
        let published_date = item.metadata.frontmatter.published.clone();
        let updated_date = item.metadata.frontmatter.updated.clone().unwrap_or_else(|| published_date.clone());
        let formatted_date = parse_date_for_sitemap(&updated_date);
        
        // Determine change frequency based on content age
        let change_freq = determine_change_frequency(&published_date, &updated_date);
        
        // Calculate priority based on content type and age
        let priority = calculate_priority(&item.metadata);
        
        writer.write_event(Event::Start(BytesStart::new("url")))?;
        
        // URL location
        writer.write_event(Event::Start(BytesStart::new("loc")))?;
        writer.write_event(Event::Text(BytesText::new(&url)))?;
        writer.write_event(Event::End(BytesEnd::new("loc")))?;
        
        // Last modified date
        if let Some(date) = formatted_date {
            writer.write_event(Event::Start(BytesStart::new("lastmod")))?;
            writer.write_event(Event::Text(BytesText::new(&date)))?;
            writer.write_event(Event::End(BytesEnd::new("lastmod")))?;
        }
        
        // Change frequency
        writer.write_event(Event::Start(BytesStart::new("changefreq")))?;
        writer.write_event(Event::Text(BytesText::new(&change_freq)))?;
        writer.write_event(Event::End(BytesEnd::new("changefreq")))?;
        
        // Priority
        writer.write_event(Event::Start(BytesStart::new("priority")))?;
        writer.write_event(Event::Text(BytesText::new(&priority)))?;
        writer.write_event(Event::End(BytesEnd::new("priority")))?;
        
        writer.write_event(Event::End(BytesEnd::new("url")))?;
    }
    
    // Urlset end tag
    writer.write_event(Event::End(BytesEnd::new("urlset")))?;
    
    let result = String::from_utf8(buffer.into_inner())?;
    Ok(result)
}

// Parse a date string into W3C Datetime format (ISO 8601) for sitemap
fn parse_date_for_sitemap(date_str: &str) -> Option<String> {
    // Try to parse the date string
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        // Convert to DateTime<Utc>
        let datetime = DateTime::<Utc>::from_naive_utc_and_offset(
            date.and_hms_opt(0, 0, 0).unwrap_or_default(),
            Utc,
        );
        // Format as ISO 8601 / W3C Datetime with colon in timezone offset
        let formatted = datetime.format("%Y-%m-%dT%H:%M:%S%z").to_string();
        
        // Insert colon in timezone offset: +0000 -> +00:00
        if formatted.len() > 5 {
            let (timestamp, offset) = formatted.split_at(formatted.len() - 5);
            let formatted_offset = format!("{}:{}", &offset[..3], &offset[3..]);
            Some(format!("{}{}", timestamp, formatted_offset))
        } else {
            Some(formatted)
        }
    } else {
        None
    }
}

// Determine change frequency based on content age
fn determine_change_frequency(published_date: &str, updated_date: &str) -> String {
    // Try to parse the dates
    let published = NaiveDate::parse_from_str(published_date, "%Y-%m-%d").ok();
    let updated = NaiveDate::parse_from_str(updated_date, "%Y-%m-%d").ok();
    
    // Get current date
    let today = Utc::now().date_naive();
    
    // If we can't parse the dates, default to weekly
    if published.is_none() {
        return "weekly".to_string();
    }
    
    // Calculate days since last update
    let last_update = updated.unwrap_or_else(|| published.unwrap());
    let days_since_update = (today - last_update).num_days();
    
    // Determine frequency based on days since update
    if days_since_update < 7 {
        "daily".to_string()
    } else if days_since_update < 30 {
        "weekly".to_string()
    } else if days_since_update < 365 {
        "monthly".to_string()
    } else {
        "yearly".to_string()
    }
}

// Calculate priority based on content type and age
fn calculate_priority(metadata: &ContentMetadata) -> String {
    // Base priority
    let mut priority: f32 = 0.5;
    
    // Adjust based on content type (you can customize this based on your content structure)
    if metadata.frontmatter.tags.contains(&"featured".to_string()) {
        priority += 0.3;
    }
    
    // Adjust based on age
    if let Ok(published) = NaiveDate::parse_from_str(&metadata.frontmatter.published, "%Y-%m-%d") {
        let today = Utc::now().date_naive();
        let days_old = (today - published).num_days();
        
        // Newer content gets higher priority
        if days_old < 30 {
            priority += 0.2;
        } else if days_old < 90 {
            priority += 0.1;
        }
    }
    
    // Ensure priority is between 0.0 and 1.0
    priority = priority.max(0.1).min(0.9);
    
    // Format with one decimal place
    format!("{:.1}", priority)
}

fn generate_rss_feed(
    content_items: &[ContentItem],
    site_url: &str,
    site_title: &str,
    site_description: &str,
    site_logo: &str,
    site_language: &str,
    site_webmaster: Option<&str>,
    site_editor: Option<&str>,
    rss_max_items: usize
) -> Result<String> {
    // Parse the current date
    let now = Utc::now();
    
    // Create a new channel
    let mut channel = Channel::default();
    
    // Set required channel elements
    channel.set_title(site_title);
    channel.set_link(site_url);
    channel.set_description(site_description);
    
    // Set optional but recommended channel elements
    channel.set_language(Some(site_language.to_string()));
    channel.set_last_build_date(Some(now.to_rfc2822()));
    channel.set_pub_date(Some(now.to_rfc2822()));
    channel.set_generator(Some(format!("content-build v{}", env!("CARGO_PKG_VERSION"))));
    channel.set_docs(Some("https://www.rssboard.org/rss-specification".to_string()));
    channel.set_ttl(Some("60".to_string())); // Time to live in minutes
    
    // Set webmaster and managing editor if provided
    if let Some(webmaster) = site_webmaster {
        channel.set_webmaster(Some(webmaster.to_string()));
    }
    
    if let Some(editor) = site_editor {
        channel.set_managing_editor(Some(editor.to_string()));
    }
    
    // Add image for the channel
    let mut channel_image = rss::Image::default();
    channel_image.set_url(format!("{}/{}", site_url, site_logo));
    channel_image.set_title(site_title.to_string());
    channel_image.set_link(site_url.to_string());
    channel_image.set_width(Some("144".to_string()));
    channel_image.set_height(Some("144".to_string()));
    channel_image.set_description(Some(site_description.to_string()));
    channel.set_image(Some(channel_image));
    
    // Add items to the channel
    let mut items = Vec::new();
    
    for item in content_items {
        // Skip draft content
        if let Some(true) = item.metadata.frontmatter.draft {
            continue;
        }
        
        // Parse the published date
        let published_date = match NaiveDate::parse_from_str(&item.metadata.frontmatter.published, "%Y-%m-%d") {
            Ok(date) => {
                let naive_datetime = date.and_hms_opt(0, 0, 0).unwrap_or_default();
                let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);
                Some(datetime.to_rfc2822())
            },
            Err(_) => None,
        };
        
        // Get the description from either the description or tagline field
        let description = item.metadata.frontmatter.description.clone()
            .or_else(|| item.metadata.frontmatter.tagline.clone())
            .unwrap_or_else(|| String::from(""));
        
        // Create the item URL
        let item_url = format!("{}/{}/{}", site_url, item.metadata.topic, item.metadata.slug);
        
        // Build the RSS item
        let mut rss_item_builder = ItemBuilder::default();
        
        // Set required item elements
        rss_item_builder.title(Some(item.metadata.frontmatter.title.clone()));
        rss_item_builder.link(Some(item_url.clone()));
        rss_item_builder.description(Some(description));
        
        // Set optional but recommended item elements
        rss_item_builder.pub_date(published_date);
        
        // Set a unique and permanent GUID
        rss_item_builder.guid(Some(rss::GuidBuilder::default()
            .value(item_url.clone())
            .permalink(true)
            .build()));
        
        // Add content as CDATA if available
        if !item.content_html.is_empty() {
            // Clean up the HTML content to avoid excessive whitespace
            let cleaned_html = item.content_html.clone()
                .replace("\n\n", "\n")  // Remove double line breaks
                .trim()
                .to_string();
            
            rss_item_builder.content(Some(cleaned_html));
        }
        
        // Add categories for tags
        let mut categories = Vec::new();
        for tag in &item.metadata.frontmatter.tags {
            let category = rss::CategoryBuilder::default()
                .name(tag.clone())
                .domain(Some(format!("{}/tags", site_url)))
                .build();
            categories.push(category);
        }
        
        // Add image enclosure if available
        if let Some(image) = &item.metadata.frontmatter.image {
            // Create an enclosure for the image
            let image_url = if image.starts_with("http") {
                image.clone()
            } else {
                format!("{}/{}/{}/{}", site_url, item.metadata.topic, item.metadata.slug, image)
            };
            
            let enclosure = rss::EnclosureBuilder::default()
                .url(image_url)
                .length("0".to_string()) // Length is required but we don't know the size
                .mime_type("image/jpeg".to_string()) // Assume JPEG, adjust as needed
                .build();
                
            rss_item_builder.enclosure(Some(enclosure));
        }
        
        let mut item = rss_item_builder.build();
        item.set_categories(categories);
        
        items.push(item);
    }
    
    // Sort items by publication date (newest first)
    items.sort_by(|a, b| {
        let a_date = a.pub_date().unwrap_or_default();
        let b_date = b.pub_date().unwrap_or_default();
        b_date.cmp(a_date)
    });
    
    // Limit to the specified number of items
    if items.len() > rss_max_items {
        items.truncate(rss_max_items);
    }
    
    // Add the sorted items to the channel
    channel.set_items(items);
    
    // Get the RSS XML as a string
    let rss_xml = channel.to_string();
    
    // Format the XML with indentation
    let formatted_xml = format_rss_xml(&rss_xml);
    
    Ok(formatted_xml)
}

// A specialized formatter for RSS feeds that produces cleaner output
fn format_rss_xml(xml: &str) -> String {
    // First, let's get the XML declaration separately
    let mut result = String::new();
    if let Some(decl_end) = xml.find("?>") {
        result.push_str(&xml[0..decl_end+2]);
        result.push('\n');
    }
    
    // Now let's handle the rest of the XML
    let content = if let Some(decl_end) = xml.find("?>") {
        &xml[decl_end+2..]
    } else {
        xml
    };
    
    // Define tags that should have their content on the same line
    let inline_tags = vec![
        "title", "link", "description", "language", "lastBuildDate", 
        "pubDate", "generator", "docs", "ttl", "webMaster", 
        "managingEditor", "width", "height", "url", "guid", 
        "category", "enclosure"
    ];
    
    // Use a more direct approach with string manipulation
    let mut lines = Vec::new();
    let mut current_indent = 0;
    
    // Split the XML into tags and content
    let mut i = 0;
    let mut in_content_encoded = false;
    
    while i < content.len() {
        // Skip whitespace
        while i < content.len() && content[i..i+1].trim().is_empty() && !in_content_encoded {
            i += 1;
        }
        
        if i >= content.len() {
            break;
        }
        
        // Check for a tag
        if &content[i..i+1] == "<" {
            // Find the end of the tag
            if let Some(tag_end) = content[i..].find('>') {
                let tag_str = &content[i..i+tag_end+1];
                let is_closing = tag_str.starts_with("</");
                
                // Extract tag name
                let name_start = if is_closing { 2 } else { 1 };
                let name_end = tag_str[name_start..].find(|c: char| c.is_whitespace() || c == '>' || c == '/')
                    .map_or(tag_str.len() - 1, |pos| pos + name_start);
                let tag_name = &tag_str[name_start..name_end];
                
                // Check if we're entering or leaving a content:encoded section
                if tag_name == "content:encoded" || tag_name == "content" {
                    if is_closing {
                        in_content_encoded = false;
                    } else {
                        in_content_encoded = true;
                    }
                }
                
                // Handle indentation
                if is_closing {
                    // Decrease indent for closing tags
                    if current_indent > 0 {
                        current_indent -= 1;
                    }
                    
                    // Add the closing tag with proper indentation
                    let indent = "  ".repeat(current_indent);
                    lines.push(format!("{}{}", indent, tag_str));
                } else {
                    // Add the opening tag with proper indentation
                    let indent = "  ".repeat(current_indent);
                    
                    // For inline tags, try to find the content and closing tag
                    if inline_tags.contains(&tag_name) && !tag_str.ends_with("/>") {
                        let content_start = i + tag_end + 1;
                        let closing_tag = format!("</{}>", tag_name);
                        
                        if let Some(closing_pos) = content[content_start..].find(&closing_tag) {
                            let content_end = content_start + closing_pos;
                            let tag_content = content[content_start..content_end].trim();
                            
                            // Add the tag, content, and closing tag all on one line
                            lines.push(format!("{}{}{}{}", indent, tag_str, tag_content, closing_tag));
                            
                            // Move past the closing tag
                            i = content_end + closing_tag.len();
                            continue;
                        } else {
                            // No closing tag found, just add the opening tag
                            lines.push(format!("{}{}", indent, tag_str));
                        }
                    } else {
                        // For non-inline tags, just add the opening tag
                        lines.push(format!("{}{}", indent, tag_str));
                    }
                    
                    // Increase indent for opening tags
                    if !tag_str.ends_with("/>") {
                        current_indent += 1;
                    }
                    
                    // If we're in a content:encoded section, handle CDATA specially
                    if in_content_encoded && !is_closing {
                        // Find the CDATA section
                        if let Some(cdata_start) = content[i+tag_end+1..].find("<![CDATA[") {
                            let cdata_start_pos = i + tag_end + 1 + cdata_start;
                            if let Some(cdata_end) = content[cdata_start_pos..].find("]]>") {
                                let cdata_end_pos = cdata_start_pos + cdata_end + 3;
                                let cdata_content = &content[cdata_start_pos..cdata_end_pos];
                                
                                // Add the CDATA section with proper indentation
                                let indent = "  ".repeat(current_indent);
                                lines.push(format!("{}{}", indent, cdata_content));
                                
                                // Move past the CDATA section
                                i = cdata_end_pos;
                                continue;
                            }
                        }
                    }
                }
                
                // Move past this tag
                i += tag_end + 1;
            } else {
                // No closing '>', just add the character and move on
                i += 1;
            }
        } else {
            // This is content outside of a tag, find the next tag
            let next_tag = content[i..].find('<').unwrap_or(content.len() - i);
            let text = content[i..i+next_tag].trim();
            
            if !text.is_empty() {
                let indent = "  ".repeat(current_indent);
                lines.push(format!("{}{}", indent, text));
            }
            
            i += next_tag;
        }
    }
    
    // Join all lines with newlines
    result.push_str(&lines.join("\n"));
    result.push('\n');
    
    result
}

// A simple XML formatter that adds indentation
fn format_xml(xml: &str) -> String {
    let mut formatted = String::new();
    let mut indent_level: usize = 0;
    let mut in_content = false;
    
    for c in xml.chars() {
        match c {
            '<' => {
                // Check if this is a closing tag
                if xml.chars().skip(xml.find(c).unwrap() + 1).next() == Some('/') {
                    if indent_level > 0 {
                        indent_level -= 1;
                    }
                }
                
                if in_content {
                    formatted.push('\n');
                    formatted.push_str(&"  ".repeat(indent_level));
                }
                
                in_content = false;
                formatted.push(c);
            },
            '>' => {
                formatted.push(c);
                
                // Check if this is a self-closing tag
                let is_self_closing = formatted.ends_with("/>");
                
                // Check if this is an opening tag (not self-closing and not a closing tag)
                if !is_self_closing && !formatted.ends_with("/>") && !formatted.contains("</") {
                    indent_level += 1;
                }
                
                if !is_self_closing {
                    formatted.push('\n');
                    formatted.push_str(&"  ".repeat(indent_level));
                }
                
                in_content = true;
            },
            _ => {
                formatted.push(c);
            }
        }
    }
    
    formatted
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Read configuration
    let config = read_config()?;
    
    // Validate topic if provided
    if let Some(ref topic) = args.topic {
        if !config.content.topics.contains_key(topic) {
            let valid_topics: Vec<String> = config.content.topics.keys()
                .map(|k| k.to_string())
                .collect();
            return Err(anyhow::anyhow!(
                "Invalid topic: {}. Valid topics are: {}", 
                topic, 
                valid_topics.join(", ")
            ));
        }
    }
    
    // Get the build root directory (parent of output_dir)
    let build_root = if let Some(parent) = args.output_dir.parent() {
        parent.to_path_buf()
    } else {
        PathBuf::from("build")
    };
    
    // Create output directory
    create_all(&args.output_dir, false)
        .context(format!("Failed to create output directory: {:?}", args.output_dir))?;
    
    // Create build root directory if it doesn't exist
    create_all(&build_root, false)
        .context(format!("Failed to create build root directory: {:?}", build_root))?;
    
    // Initialize Handlebars
    let mut handlebars = Handlebars::new();
    
    // Register templates
    let content_template_path = args.template_dir.join("content.hbs");
    if content_template_path.exists() {
        handlebars.register_template_file("content", content_template_path.clone())
            .context(format!("Failed to register content template: {:?}", content_template_path))?;
    } else {
        println!("Warning: Content template not found at {:?}", content_template_path);
    }
    
    let index_template_path = args.template_dir.join("index.hbs");
    if index_template_path.exists() {
        handlebars.register_template_file("index", index_template_path.clone())
            .context(format!("Failed to register index template: {:?}", index_template_path))?;
    } else {
        println!("Warning: Index template not found at {:?}", index_template_path);
    }
    
    // Process content
    let mut content_items = Vec::new();
    
    for (topic_key, topic_config) in &config.content.topics {
        let topic_dir = PathBuf::from(format!("{}/{}", config.content.base_dir, topic_config.path));
        
        for entry in WalkDir::new(&topic_dir).min_depth(1).max_depth(1).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            
            if !path.is_dir() {
                continue;
            }
            
            let article_slug = path.file_name().unwrap().to_string_lossy().to_string();
            
            // Filter by slug if provided
            if let Some(ref requested_slug) = args.slug {
                if article_slug != *requested_slug {
                    continue;
                }
            }
            
            if let Some(ref requested_topic) = args.topic {
                if topic_key != requested_topic {
                    continue;
                }
            }
            
            // Process content
            match process_content(path, topic_key, topic_config, &args) {
                Ok(Some(content_item)) => {
                    content_items.push(content_item);
                },
                Ok(None) => {
                    // Skip draft content
                    continue;
                },
                Err(e) => {
                    // Log the error but continue processing other content
                    eprintln!("Warning: Failed to process content at {:?}: {}", path, e);
                    continue;
                }
            }
        }
    }
    
    // Check if we found any content
    if content_items.is_empty() {
        if args.slug.is_some() || args.topic.is_some() {
            return Err(anyhow::anyhow!("No content found matching the specified criteria"));
        } else {
            return Err(anyhow::anyhow!("No content found"));
        }
    }
    
    // Build content index
    let content_index = build_content_index(&content_items);
    
    // Write JSON files
    let json_dir = args.output_dir.join("json");
    create_all(&json_dir, false)
        .context(format!("Failed to create JSON directory: {:?}", json_dir))?;
    
    // Write index JSON
    let index_json = serde_json::to_string_pretty(&content_index)
        .context("Failed to serialize content index")?;
    fs::write(json_dir.join("index.json"), index_json)
        .context("Failed to write index.json")?;
    
    // Write individual content JSON files
    for item in &content_items {
        let content_json = serde_json::to_string_pretty(item)
            .context(format!("Failed to serialize content: {}", item.metadata.slug))?;
        
        let content_dir = json_dir.join(&item.metadata.topic).join(&item.metadata.slug);
        create_all(&content_dir, false)
            .context(format!("Failed to create content directory: {:?}", content_dir))?;
        
        fs::write(content_dir.join("index.json"), content_json)
            .context(format!("Failed to write content JSON: {}", item.metadata.slug))?;
    }
    
    // Write HTML files if templates are available
    if handlebars.has_template("content") && handlebars.has_template("index") {
        let html_dir = args.output_dir.join("html");
        create_all(&html_dir, false)
            .context(format!("Failed to create HTML directory: {:?}", html_dir))?;
        
        // Write index HTML
        let index_html = handlebars.render("index", &content_index)
            .context("Failed to render index HTML")?;
        fs::write(html_dir.join("index.html"), index_html)
            .context("Failed to write index.html")?;
        
        // Write individual content HTML files
        for item in &content_items {
            let content_html = handlebars.render("content", item)
                .context(format!("Failed to render content HTML: {}", item.metadata.slug))?;
            
            let content_dir = html_dir.join(&item.metadata.topic).join(&item.metadata.slug);
            create_all(&content_dir, false)
                .context(format!("Failed to create content HTML directory: {:?}", content_dir))?;
            
            fs::write(content_dir.join("index.html"), content_html)
                .context(format!("Failed to write content HTML: {}", item.metadata.slug))?;
        }
    } else {
        println!("Skipping HTML generation: templates not found");
    }
    
    // Generate and write sitemap.xml
    if !args.skip_sitemap {
        let sitemap_xml = generate_sitemap(&content_items, &args.site_url)
            .context("Failed to generate sitemap.xml")?;
        fs::write(build_root.join("sitemap.xml"), sitemap_xml)
            .context("Failed to write sitemap.xml")?;
        println!("Sitemap: {}", build_root.join("sitemap.xml").display());
    }
    
    // Generate and write RSS feed
    if !args.skip_rss {
        let rss_feed = generate_rss_feed(
            &content_items, 
            &args.site_url, 
            &args.site_title, 
            &args.site_description,
            &args.site_logo,
            &args.site_language,
            args.site_webmaster.as_deref(),
            args.site_editor.as_deref(),
            args.rss_max_items
        )
        .context("Failed to generate RSS feed")?;
        fs::write(build_root.join("feed.xml"), rss_feed)
            .context("Failed to write feed.xml")?;
        println!("RSS feed: {}", build_root.join("feed.xml").display());
    }
    
    println!("{} Built {} content items", "SUCCESS:".green().bold(), content_items.len());
    println!("JSON output: {}", json_dir.display());
    
    if handlebars.has_template("content") && handlebars.has_template("index") {
        println!("HTML output: {}", args.output_dir.join("html").display());
    }
    
    println!("Content build complete!");
    
    Ok(())
} 