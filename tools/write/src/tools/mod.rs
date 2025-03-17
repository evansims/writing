//! # Writing Tools Module
//! 
//! This module provides the core functionality for the writing tools application.
//! It is organized into sub-modules by domain area.

use anyhow::Result;
use crate::cli::{ContentCommands, TopicCommands, ImageCommands, BuildCommands};

// Import domain-specific modules
mod content;
mod topic;
mod image;
mod build;
mod stats;
mod utils;

// Re-export domain-specific functionality
pub use content::*;
pub use topic::*;
pub use image::*;
pub use build::*;
pub use stats::*;
pub use utils::*;

/// Execute a content command
pub fn execute_content_command(command: ContentCommands) -> Result<()> {
    match command {
        ContentCommands::New { title, topic, tagline, tags, draft, template, edit } => {
            let content = title.clone();
            let topic_clone = topic.clone();
            
            // Create the content
            create_content(
                title,
                topic,
                tagline,
                tags,
                None,
                draft,
                template,
                None,
            )?;
            
            // Open the editor if requested
            if edit {
                edit_content(content, topic_clone, true, true)?;
            }
            
            Ok(())
        },
        ContentCommands::Edit { slug, topic, field, value, editor } => {
            if let (Some(field), Some(value)) = (field, value) {
                // Update a specific field
                update_frontmatter_field(&slug.unwrap_or_default(), topic.as_deref(), &field, &value)
            } else {
                // Edit the whole content
                edit_content(slug, topic, true, editor)
            }
        },
        ContentCommands::Move { slug, from, to } => {
            move_content(slug, None, from, to)
        },
        ContentCommands::Delete { slug, topic, force } => {
            delete_content(slug, topic, force)
        },
        ContentCommands::Validate { slug, topic, all, fix } => {
            let validation_types = None;
            validate_content(
                slug,
                topic,
                validation_types,
                false, // Don't check external links by default
                None,
                None,
                false, // Don't include drafts by default
                !fix,  // Verbose if not fixing
            )
        },
        ContentCommands::List { topic, drafts, format } => {
            list_content_with_options(topic, drafts, &format)
        },
        ContentCommands::Search { query, topic, drafts, format } => {
            search_content(
                query.unwrap_or_default(),
                topic,
                None,
                None,
                None,
                drafts,
                false,
                None,
                false,
            )
        },
        ContentCommands::Template { template, list, output } => {
            if list {
                list_templates()
            } else if let Some(template_name) = template {
                create_template(Some(template_name), None)
            } else {
                create_template(None, None)
            }
        },
    }
}

/// Execute a topic command
pub fn execute_topic_command(command: TopicCommands) -> Result<()> {
    match command {
        TopicCommands::Add { key, name, description, directory } => {
            add_topic(key, name, description, directory)
        },
        TopicCommands::Edit { key, name, description, directory } => {
            edit_topic_with_directory(key, name, description, directory)
        },
        TopicCommands::Rename { from, to } => {
            rename_topic(from, to, None, None)
        },
        TopicCommands::Delete { key, force } => {
            delete_topic(key, None, force)
        },
        TopicCommands::List { format } => {
            list_topics_with_format(&format)
        },
    }
}

/// Execute an image command
pub fn execute_image_command(command: ImageCommands) -> Result<()> {
    match command {
        ImageCommands::Build { topic, rebuild } => {
            if rebuild {
                // Force rebuilding all images
                build_images(None, topic, None)
            } else {
                // Only build new images
                build_images(None, topic, None)
            }
        },
        ImageCommands::Optimize { topic, reoptimize } => {
            optimize_images(topic, reoptimize)
        },
    }
}

/// Execute a build command
pub fn execute_build_command(command: BuildCommands) -> Result<()> {
    match command {
        BuildCommands::Content { topic, rebuild } => {
            build_content(
                None,
                None,
                topic,
                false, // Don't include drafts by default
                false, // Don't skip HTML
                false, // Don't skip JSON
                false, // Don't skip RSS
                false, // Don't skip sitemap
                true,  // Verbose output
            )
        },
        BuildCommands::Toc { topic } => {
            // Topic is ignored for now, as the TOC is generated for all content
            generate_toc(None)
        },
    }
}

/// Execute the stats command
pub fn execute_stats_command(
    slug: Option<String>,
    topic: Option<String>,
    include_drafts: bool,
    sort_by: &str,
    detailed: bool,
) -> Result<()> {
    generate_content_stats(slug, topic, include_drafts, sort_by.to_string(), detailed)
}

// Helper functions for command execution
fn list_content_with_options(topic: Option<String>, include_drafts: bool, format: &str) -> Result<()> {
    // Implementation will be moved to content.rs
    content::list_content_with_options(topic, include_drafts, format)
}

fn list_topics_with_format(format: &str) -> Result<()> {
    // Implementation will be moved to topic.rs
    topic::list_topics_with_format(format)
}

fn edit_topic_with_directory(
    key: Option<String>,
    name: Option<String>,
    description: Option<String>,
    directory: Option<String>,
) -> Result<()> {
    // Implementation will be moved to topic.rs
    topic::edit_topic_with_directory(key, name, description, directory)
}

fn optimize_images(topic: Option<String>, reoptimize: bool) -> Result<()> {
    // Implementation will be moved to image.rs
    image::optimize_images(topic, reoptimize)
}

fn update_frontmatter_field(
    slug: &str,
    topic: Option<&str>,
    field: &str,
    value: &str,
) -> Result<()> {
    // Implementation will be moved to content.rs
    content::update_frontmatter_field(slug, topic, field, value)
} 