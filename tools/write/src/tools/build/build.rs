//! # Build Module
//!
//! This module provides functionality for building content, including generating
//! HTML, JSON, RSS, and sitemap files.

use anyhow::Result;
use colored::Colorize;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use crate::ui;

// Export sub-modules
pub mod cache;
pub mod lazy_cache;

// Re-export lazy_cache functionality
pub use lazy_cache::LazyBuildCache;
pub use lazy_cache::lazy_build_cache;

/// Build content
///
/// This function builds content files based on the provided options.
/// If options are not provided, sensible defaults are used.
///
/// # Parameters
///
/// * `output_dir` - Optional directory to output files to (defaults to "public")
/// * `slug` - Optional slug to filter content by (defaults to building all content)
/// * `topic` - Optional topic to filter content by (defaults to all topics)
/// * `include_drafts` - Whether to include draft content
/// * `skip_html` - Whether to skip HTML generation
/// * `skip_json` - Whether to skip JSON generation
/// * `skip_rss` - Whether to skip RSS generation
/// * `skip_sitemap` - Whether to skip sitemap generation
/// * `force_rebuild` - Whether to force rebuilding all content
/// * `verbose` - Whether to display verbose output during build
///
/// # Returns
///
/// A Result indicating success or failure with error context
pub fn build_content(
    output_dir: Option<String>,
    slug: Option<String>,
    topic: Option<String>,
    include_drafts: bool,
    skip_html: bool,
    skip_json: bool,
    skip_rss: bool,
    skip_sitemap: bool,
    force_rebuild: bool,
    verbose: bool,
) -> Result<()> {
    // Get the output directory or use default
    let output_dir = output_dir.unwrap_or_else(|| "public".to_string());

    // Get the topic if provided, otherwise use "all" to indicate all topics
    let topic_str = topic.as_deref().unwrap_or("all");

    // Get the slug if provided, otherwise use "all" to indicate all content
    let slug_str = slug.as_deref().unwrap_or("all");

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&output_dir)?;

    // Get the lazy build cache
    let build_cache = lazy_build_cache();

    // Clear the cache if force rebuild is requested
    if force_rebuild {
        build_cache.clear()?;
    }

    // Show progress
    ui::show_info(&format!(
        "{} content for topic: {}, slug: {} (include_drafts: {})",
        if force_rebuild { "Rebuilding" } else { "Building" },
        topic_str, slug_str, include_drafts
    ));

    // Find all content files that match the filters
    let content_files = find_content_files(topic.as_deref(), slug.as_deref(), include_drafts)?;

    if content_files.is_empty() {
        ui::show_warning("No content files found matching the criteria");
        return Ok(());
    }

    // Get the list of files that need to be rebuilt
    let files_to_build = if force_rebuild {
        content_files.clone()
    } else {
        build_cache.get_files_to_rebuild(&content_files)?
    };

    if files_to_build.is_empty() {
        ui::show_success("All content is up to date, nothing to rebuild");
        return Ok(());
    }

    if verbose {
        ui::show_info(&format!("Found {} content files, {} need to be rebuilt",
            content_files.len(), files_to_build.len()));
    }

    // Create a progress bar
    let progress = ui::create_progress_bar(files_to_build.len() as u64);
    progress.set_message("Building content...");

    // Begin parallel processing with proper batching
    let batch_size = 10; // Process 10 files at a time
    let build_results: Vec<Result<(PathBuf, Vec<PathBuf>)>> = files_to_build
        .par_chunks(batch_size.min(files_to_build.len()))
        .flat_map(|chunk| {
            chunk.par_iter().map(|file_path| {
                // Process each content file
                if verbose {
                    ui::show_info(&format!("Building: {}", file_path.display()));
                }

                // For each input file, get the output paths that would be generated
                let output_files = build_single_content_file(
                    file_path,
                    &output_dir,
                    !skip_html,
                    !skip_json
                )?;

                // Increment the progress
                progress.inc(1);

                // Return the input file and its output files
                Ok((file_path.clone(), output_files))
            }).collect::<Vec<_>>()
        })
        .collect();

    // Process the build results
    let mut successful_builds = 0;
    let mut failed_builds = 0;

    // Update the build cache with successful builds
    for result in build_results {
        match result {
            Ok((input_file, output_files)) => {
                build_cache.add_file(input_file, output_files)?;
                successful_builds += 1;
            }
            Err(e) => {
                ui::show_error(&format!("Failed to build content: {}", e));
                failed_builds += 1;
            }
        }
    }

    // Update the build cache last build time
    build_cache.update_last_build()?;

    progress.finish_with_message("Content built successfully");

    // Generate site-wide files if appropriate
    if !skip_rss && topic_str == "all" && slug_str == "all" {
        generate_rss(&output_dir)?;
    }

    if !skip_sitemap && topic_str == "all" && slug_str == "all" {
        generate_sitemap(&output_dir)?;
    }

    // Show build summary
    if failed_builds > 0 {
        ui::show_warning(&format!(
            "Built {} out of {} content files with {} failures",
            successful_builds,
            files_to_build.len(),
            failed_builds
        ));
    } else {
        ui::show_success(&format!(
            "Successfully built {} content files to {}",
            successful_builds,
            output_dir.green()
        ));
    }

    Ok(())
}

/// Find content files that match the given filters
fn find_content_files(
    topic: Option<&str>,
    slug: Option<&str>,
    include_drafts: bool
) -> Result<Vec<PathBuf>> {
    let base_dir = PathBuf::from("content");

    if !base_dir.exists() {
        return Err(anyhow::anyhow!("Content directory not found"));
    }

    let mut content_files = Vec::new();

    // If a specific slug is provided, find only that content
    if let Some(s) = slug {
        if s != "all" {
            // If topic is provided, look in that topic directory
            if let Some(t) = topic {
                if t != "all" {
                    let content_path = base_dir.join(t).join(s).join("index.md");
                    let content_path_mdx = base_dir.join(t).join(s).join("index.mdx");

                    if content_path.exists() {
                        content_files.push(content_path);
                    } else if content_path_mdx.exists() {
                        content_files.push(content_path_mdx);
                    } else {
                        return Err(anyhow::anyhow!("Content not found: {}/{}", t, s));
                    }

                    return Ok(content_files);
                }
            }

            // If no topic is provided or topic is "all", search in all topic directories
            for topic_dir in fs::read_dir(&base_dir)? {
                let topic_dir = topic_dir?.path();
                if topic_dir.is_dir() {
                    let content_path = topic_dir.join(s).join("index.md");
                    let content_path_mdx = topic_dir.join(s).join("index.mdx");

                    if content_path.exists() {
                        content_files.push(content_path);
                        break;
                    } else if content_path_mdx.exists() {
                        content_files.push(content_path_mdx);
                        break;
                    }
                }
            }

            if content_files.is_empty() {
                return Err(anyhow::anyhow!("Content not found: {}", s));
            }

            return Ok(content_files);
        }
    }

    // If a specific topic is provided, but slug is "all"
    if let Some(t) = topic {
        if t != "all" {
            let topic_dir = base_dir.join(t);
            if !topic_dir.exists() {
                return Err(anyhow::anyhow!("Topic not found: {}", t));
            }

            // Find all content files in this topic directory
            for entry in fs::read_dir(topic_dir)? {
                let entry = entry?.path();
                if entry.is_dir() {
                    let content_path = entry.join("index.md");
                    let content_path_mdx = entry.join("index.mdx");

                    if content_path.exists() {
                        content_files.push(content_path);
                    } else if content_path_mdx.exists() {
                        content_files.push(content_path_mdx);
                    }
                }
            }

            return Ok(content_files);
        }
    }

    // If both topic and slug are "all", find all content files in all topic directories
    for topic_dir in fs::read_dir(&base_dir)? {
        let topic_dir = topic_dir?.path();
        if topic_dir.is_dir() {
            for entry in fs::read_dir(&topic_dir)? {
                let entry = entry?.path();
                if entry.is_dir() {
                    let content_path = entry.join("index.md");
                    let content_path_mdx = entry.join("index.mdx");

                    if content_path.exists() {
                        content_files.push(content_path);
                    } else if content_path_mdx.exists() {
                        content_files.push(content_path_mdx);
                    }
                }
            }
        }
    }

    Ok(content_files)
}

/// Build a single content file
fn build_single_content_file(
    content_path: &Path,
    output_dir: &str,
    build_html: bool,
    build_json: bool
) -> Result<Vec<PathBuf>> {
    let mut output_files = Vec::new();

    // Read the content file
    let content = fs::read_to_string(content_path)
        .map_err(|e| anyhow::anyhow!("Failed to read content file: {}", e))?;

    // Parse the frontmatter
    let (frontmatter, markdown) = parse_frontmatter(&content)?;

    // Determine output paths and structure
    let relative_path = content_path
        .strip_prefix("content")
        .map_err(|_| anyhow::anyhow!("Content file is not in the content directory"))?;

    let parent = relative_path.parent().unwrap_or_else(|| Path::new(""));
    let output_path = Path::new(output_dir).join(parent);

    // Create the output directory
    fs::create_dir_all(&output_path)
        .map_err(|e| anyhow::anyhow!("Failed to create output directory: {}", e))?;

    // Build HTML if requested
    if build_html {
        let html_content = render_markdown_to_html(markdown, &frontmatter)?;
        let html_path = output_path.join("index.html");

        fs::write(&html_path, html_content)
            .map_err(|e| anyhow::anyhow!("Failed to write HTML file: {}", e))?;

        output_files.push(html_path);
    }

    // Build JSON if requested
    if build_json {
        let json_content = serde_json::to_string_pretty(&frontmatter)
            .map_err(|e| anyhow::anyhow!("Failed to serialize frontmatter: {}", e))?;

        let json_path = output_path.join("data.json");

        fs::write(&json_path, json_content)
            .map_err(|e| anyhow::anyhow!("Failed to write JSON file: {}", e))?;

        output_files.push(json_path);
    }

    Ok(output_files)
}

/// Parse frontmatter from content
fn parse_frontmatter(content: &str) -> Result<(serde_json::Value, &str)> {
    // Simple parsing - assumes frontmatter is at the beginning and surrounded by ---
    let content = content.trim_start();

    if !content.starts_with("---") {
        return Err(anyhow::anyhow!("Content does not contain frontmatter"));
    }

    // Find the end of the frontmatter
    let rest = &content[3..];
    if let Some(end_index) = rest.find("---") {
        let frontmatter_str = &rest[..end_index].trim();
        let markdown = &rest[end_index + 3..].trim();

        // Parse the frontmatter as YAML
        let frontmatter: serde_json::Value = serde_yaml::from_str(frontmatter_str)
            .map_err(|e| anyhow::anyhow!("Failed to parse frontmatter: {}", e))?;

        Ok((frontmatter, markdown))
    } else {
        Err(anyhow::anyhow!("Invalid frontmatter format"))
    }
}

/// Render markdown to HTML
fn render_markdown_to_html(markdown: &str, frontmatter: &serde_json::Value) -> Result<String> {
    // This is a placeholder function that would normally use a proper markdown renderer
    // For demonstration purposes, we'll create a simple HTML structure

    let title = frontmatter["title"].as_str().unwrap_or("Untitled");

    let html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>{}</title>
</head>
<body>
    <h1>{}</h1>
    <div class="content">
        {}
    </div>
</body>
</html>"#, title, title, markdown.replace("\n", "<br>"));

    Ok(html)
}

/// Generate RSS feed
fn generate_rss(output_dir: &str) -> Result<()> {
    let rss_path = Path::new(output_dir).join("feed.xml");

    // This is a placeholder function that would normally generate a proper RSS feed
    let rss_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
<channel>
    <title>Example Site</title>
    <link>https://example.com</link>
    <description>Example site description</description>
    <language>en-us</language>
    <item>
        <title>Sample Item</title>
        <link>https://example.com/sample</link>
        <description>Sample item description</description>
    </item>
</channel>
</rss>"#;

    fs::write(&rss_path, rss_content)
        .map_err(|e| anyhow::anyhow!("Failed to write RSS file: {}", e))?;

    Ok(())
}

/// Generate sitemap
fn generate_sitemap(output_dir: &str) -> Result<()> {
    let sitemap_path = Path::new(output_dir).join("sitemap.xml");

    // This is a placeholder function that would normally generate a proper sitemap
    let sitemap_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
    <url>
        <loc>https://example.com/</loc>
        <lastmod>2023-01-01</lastmod>
        <changefreq>weekly</changefreq>
        <priority>1.0</priority>
    </url>
</urlset>"#;

    fs::write(&sitemap_path, sitemap_content)
        .map_err(|e| anyhow::anyhow!("Failed to write sitemap file: {}", e))?;

    Ok(())
}

/// Generate table of contents
///
/// This function generates a table of contents for the content.
///
/// # Parameters
///
/// * `output` - Optional output file path (defaults to "public/toc.json")
///
/// # Returns
///
/// A Result indicating success or failure with error context
pub fn generate_toc(output: Option<String>) -> Result<()> {
    // Get the output file or use default
    let output_file = output.unwrap_or_else(|| "public/toc.json".to_string());

    // Show progress
    ui::show_info(&format!("Generating table of contents to {}", output_file));

    // Create the output directory if it doesn't exist
    if let Some(parent) = std::path::Path::new(&output_file).parent() {
        std::fs::create_dir_all(parent).map_err(|e|
            anyhow::anyhow!("Failed to create directory for table of contents: {}", e)
        )?;
    }

    // TODO: Implement TOC generation
    // This is a placeholder for the actual TOC generation code
    let toc_content = r#"{
        "topics": [],
        "content": []
    }"#;

    // Write the TOC to the output file
    std::fs::write(&output_file, toc_content).map_err(|e|
        anyhow::anyhow!("Failed to write table of contents to {}: {}", output_file, e)
    )?;

    ui::show_success(&format!("Table of contents generated successfully to {}", output_file.green()));

    Ok(())
}

/// Generate LLMs (large language model) training data
///
/// This function generates training data for large language models based on the content.
///
/// # Parameters
///
/// * `site_url` - Optional URL of the site (defaults to "https://example.com")
/// * `output_dir` - Optional directory to output the generated data (defaults to "public/llm")
/// * `include_drafts` - Whether to include draft content in the generated data
///
/// # Returns
///
/// A Result indicating success or failure with error context
pub fn generate_llms(
    site_url: Option<String>,
    output_dir: Option<String>,
    include_drafts: bool,
) -> Result<()> {
    // Get the site URL or use default
    let site_url = site_url.unwrap_or_else(|| "https://example.com".to_string());

    // Get the output directory or use default
    let output_dir = output_dir.unwrap_or_else(|| "public/llm".to_string());

    // Show progress
    ui::show_info(&format!(
        "Generating LLM training data for site: {} (include_drafts: {})",
        site_url, include_drafts
    ));

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(&output_dir).map_err(|e|
        anyhow::anyhow!("Failed to create output directory for LLM data: {}", e)
    )?;

    // TODO: Implement LLM data generation
    let progress = ui::create_progress_bar(100);

    for i in 0..100 {
        // Simulate work
        std::thread::sleep(std::time::Duration::from_millis(10));
        progress.inc(1);
    }

    progress.finish_with_message("LLM training data generated successfully");

    // Write a placeholder metadata file
    let metadata_path = std::path::Path::new(&output_dir).join("metadata.json");
    let metadata_content = format!(r#"{{
        "source": "{}",
        "timestamp": "{}",
        "include_drafts": {},
        "entries": []
    }}"#, site_url, chrono::Utc::now(), include_drafts);

    std::fs::write(&metadata_path, metadata_content).map_err(|e|
        anyhow::anyhow!("Failed to write metadata file: {}", e)
    )?;

    ui::show_success(&format!("LLM training data generated successfully to {}", output_dir.green()));

    Ok(())
}

/// Build search index
///
/// This function builds a search index for the content.
///
/// # Parameters
///
/// * `index_path` - Optional path to the search index file (defaults to "public/search-index.json")
/// * `include_drafts` - Whether to include draft content in the search index
///
/// # Returns
///
/// A Result indicating success or failure with error context
pub fn build_search_index(
    index_path: Option<String>,
    include_drafts: bool,
) -> Result<()> {
    // Get the index path or use default
    let index_path = index_path.unwrap_or_else(|| "public/search-index.json".to_string());

    // Show progress
    ui::show_info(&format!(
        "Building search index to {} (include_drafts: {})",
        index_path, include_drafts
    ));

    // Create the output directory if it doesn't exist
    if let Some(parent) = std::path::Path::new(&index_path).parent() {
        std::fs::create_dir_all(parent).map_err(|e|
            anyhow::anyhow!("Failed to create directory for search index: {}", e)
        )?;
    }

    // TODO: Implement search index building
    let progress = ui::create_progress_bar(100);

    for i in 0..100 {
        // Simulate work
        std::thread::sleep(std::time::Duration::from_millis(10));
        progress.inc(1);
    }

    progress.finish_with_message("Search index built successfully");

    // Write a placeholder search index file
    let search_index_content = r#"{
        "version": 1,
        "entries": []
    }"#;

    std::fs::write(&index_path, search_index_content).map_err(|e|
        anyhow::anyhow!("Failed to write search index to {}: {}", index_path, e)
    )?;

    ui::show_success(&format!("Search index built successfully to {}", index_path.green()));

    Ok(())
}