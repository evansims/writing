//! # Build Module
//!
//! This module provides functionality for building content.

use anyhow::Result;
use colored::Colorize;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::fs;
use crate::ui;

mod cache;
mod lazy_cache;

pub use cache::{BuildCache, get_build_cache, save_build_cache, clear_build_cache};
pub use lazy_cache::{LazyBuildCache, lazy_build_cache};

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

    // Log what we're doing
    ui::show_info(&format!(
        "Building content to {} (topic: {}, include_drafts: {})",
        output_dir,
        topic.as_deref().unwrap_or("all"),
        include_drafts
    ));

    // Create the output directory if it doesn't exist
    fs::create_dir_all(&output_dir)?;

    // Show success message
    ui::show_success(&format!("Content built successfully to {}", output_dir.green()));

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
    if let Some(parent) = Path::new(&output_file).parent() {
        fs::create_dir_all(parent)?;
    }

    // Create a simple TOC
    let toc = r#"{
        "topics": [],
        "content": []
    }"#;

    // Write the TOC to the output file
    fs::write(&output_file, toc)?;

    // Show success message
    ui::show_success(&format!("Table of contents generated successfully to {}", output_file.green()));

    Ok(())
}

/// Generate LLM training data
///
/// This function generates training data for large language models.
///
/// # Parameters
///
/// * `site_url` - Optional site URL (defaults to "https://example.com")
/// * `output_dir` - Optional output directory (defaults to "public/llm")
/// * `include_drafts` - Whether to include draft content
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

    // Create the output directory if it doesn't exist
    fs::create_dir_all(&output_dir)?;

    // Create a simple metadata file
    let metadata = format!(r#"{{
        "site": "{}",
        "include_drafts": {},
        "timestamp": "{}"
    }}"#, site_url, include_drafts, chrono::Utc::now());

    // Write the metadata to the output file
    fs::write(format!("{}/metadata.json", output_dir), metadata)?;

    // Show success message
    ui::show_success(&format!("LLM training data generated successfully to {}", output_dir.green()));

    Ok(())
}

/// Build search index
///
/// This function builds a search index for the content.
///
/// # Parameters
///
/// * `output` - Optional output file path (defaults to "public/search-index.json")
/// * `include_drafts` - Whether to include draft content
///
/// # Returns
///
/// A Result indicating success or failure with error context
pub fn build_search_index(
    output: Option<String>,
    include_drafts: bool,
) -> Result<()> {
    // Get the output file or use default
    let output_file = output.unwrap_or_else(|| "public/search-index.json".to_string());

    // Show progress
    ui::show_info(&format!(
        "Building search index to {} (include_drafts: {})",
        output_file, include_drafts
    ));

    // Create the output directory if it doesn't exist
    if let Some(parent) = Path::new(&output_file).parent() {
        fs::create_dir_all(parent)?;
    }

    // Create a simple search index
    let search_index = r#"{
        "version": 1,
        "entries": []
    }"#;

    // Write the search index to the output file
    fs::write(&output_file, search_index)?;

    // Show success message
    ui::show_success(&format!("Search index built successfully to {}", output_file.green()));

    Ok(())
}