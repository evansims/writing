//! # UI Components
//!
//! This module provides UI components for the interactive CLI experience.

use anyhow::Result;
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use common_errors::{WritingError, print_error_detailed};

use crate::cli::{Commands, ContentCommands, TopicCommands, ImageCommands, BuildCommands};

/// Interactive menu for the main commands
pub fn show_main_menu() -> Result<Option<Commands>> {
    let items = vec![
        "Content Management",
        "Topic Management",
        "Image Management",
        "Build Operations",
        "Statistics",
        "Exit",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an operation")
        .default(0)
        .items(&items)
        .interact()?;

    match selection {
        0 => show_content_menu().map(|cmd| cmd.map(Commands::Content)),
        1 => show_topic_menu().map(|cmd| cmd.map(Commands::Topic)),
        2 => show_image_menu().map(|cmd| cmd.map(Commands::Image)),
        3 => show_build_menu().map(|cmd| cmd.map(Commands::Build)),
        4 => {
            // Statistics options
            let slug_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Content slug (optional)")
                .allow_empty(true)
                .interact()?;

            let slug = if slug_input.is_empty() { None } else { Some(slug_input) };

            let topic_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic (optional)")
                .allow_empty(true)
                .interact()?;

            let topic = if topic_input.is_empty() { None } else { Some(topic_input) };

            let include_drafts = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Include drafts?")
                .default(false)
                .interact()?;

            let sort_options = vec!["date", "words", "time"];
            let sort_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Sort by")
                .default(0)
                .items(&sort_options)
                .interact()?;

            let detailed = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Show detailed statistics?")
                .default(false)
                .interact()?;

            Ok(Some(Commands::Stats {
                slug,
                topic,
                include_drafts,
                sort_by: sort_options[sort_selection].to_string(),
                detailed,
            }))
        },
        5 => Ok(None), // Exit
        _ => unreachable!(),
    }
}

/// Interactive menu for content commands
pub fn show_content_menu() -> Result<Option<ContentCommands>> {
    let items = vec![
        "Create New Content",
        "Edit Content",
        "Move Content",
        "Delete Content",
        "Validate Content",
        "List Content",
        "Search Content",
        "Use Template",
        "Back",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a content operation")
        .default(0)
        .items(&items)
        .interact()?;

    match selection {
        0 => {
            // New content options
            let title_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Title")
                .interact()?;

            let title = Some(title_input);

            let topic_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic")
                .allow_empty(true)
                .interact()?;

            let topic = if topic_input.is_empty() { None } else { Some(topic_input) };

            let tagline_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Tagline (optional)")
                .allow_empty(true)
                .interact()?;

            let tagline = if tagline_input.is_empty() { None } else { Some(tagline_input) };

            let tags_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Tags (comma-separated, optional)")
                .allow_empty(true)
                .interact()?;

            let tags = if tags_input.is_empty() { None } else { Some(tags_input) };

            let draft = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Is this a draft?")
                .default(true)
                .interact()?;

            let template_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Template (optional)")
                .allow_empty(true)
                .interact()?;

            let template = if template_input.is_empty() { None } else { Some(template_input) };

            let edit = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Edit after creation?")
                .default(true)
                .interact()?;

            Ok(Some(ContentCommands::New {
                title: title.unwrap_or_default(),
                topic: topic.unwrap_or_default(),
                tagline,
                tags,
                draft,
                template,
                edit,
            }))
        },
        1 => {
            // Edit content options
            let slug_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Slug")
                .interact()?;

            let slug = Some(slug_input);

            let topic_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic (optional)")
                .allow_empty(true)
                .interact()?;

            let topic = if topic_input.is_empty() { None } else { Some(topic_input) };

            let field_options = vec!["title", "tagline", "tags", "content", "all"];
            let field_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Field to edit")
                .default(3) // content
                .items(&field_options)
                .interact()?;

            let field = Some(field_options[field_selection].to_string());

            let value = if field_options[field_selection] != "content" {
                let input = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("New value")
                    .allow_empty(true)
                    .interact()?;
                if input.is_empty() { None } else { Some(input) }
            } else {
                None
            };

            let editor = field_options[field_selection] == "content" || field_options[field_selection] == "all";

            Ok(Some(ContentCommands::Edit {
                slug,
                topic,
                field,
                value,
                editor,
            }))
        },
        2 => {
            // Move content options
            let slug_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Slug")
                .interact()?;

            let slug = Some(slug_input);

            let from_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Source topic")
                .interact()?;

            let from = Some(from_input);

            let to_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Destination topic")
                .interact()?;

            let to = Some(to_input);

            Ok(Some(ContentCommands::Move {
                slug: slug.unwrap_or_default(),
                from,
                to
            }))
        },
        3 => {
            // Delete content options
            let slug_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Slug")
                .interact()?;

            let slug = Some(slug_input);

            let topic_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic (optional)")
                .allow_empty(true)
                .interact()?;

            let topic = if topic_input.is_empty() { None } else { Some(topic_input) };

            let force = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Force deletion without confirmation?")
                .default(false)
                .interact()?;

            Ok(Some(ContentCommands::Delete {
                slug: slug.unwrap_or_default(),
                topic,
                force
            }))
        },
        4 => {
            // Validate content options
            let slug_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Slug (optional)")
                .allow_empty(true)
                .interact()?;

            let slug = if slug_input.is_empty() { None } else { Some(slug_input) };

            let topic_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic (optional)")
                .allow_empty(true)
                .interact()?;

            let topic = if topic_input.is_empty() { None } else { Some(topic_input) };

            let all = slug.is_none() && Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Validate all content?")
                .default(true)
                .interact()?;

            let fix = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Fix validation issues automatically?")
                .default(false)
                .interact()?;

            Ok(Some(ContentCommands::Validate {
                slug,
                topic,
                all,
                fix,
            }))
        },
        5 => {
            // List content options
            let topic_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic (optional)")
                .allow_empty(true)
                .interact()?;

            let topic = if topic_input.is_empty() { None } else { Some(topic_input) };

            let drafts = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Show drafts?")
                .default(false)
                .interact()?;

            let format_options = vec!["table", "json", "yaml"];
            let format_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Output format")
                .default(0)
                .items(&format_options)
                .interact()?;

            Ok(Some(ContentCommands::List {
                topic,
                drafts,
                format: format_options[format_selection].to_string(),
            }))
        },
        6 => {
            // Search content options
            let query_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Search query")
                .interact()?;

            let topic_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic (optional)")
                .allow_empty(true)
                .interact()?;

            let topic = if topic_input.is_empty() { None } else { Some(topic_input) };

            let drafts = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Include drafts?")
                .default(false)
                .interact()?;

            let format = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Output format")
                .default(0)
                .items(&["table", "json", "yaml"])
                .interact()?;

            let format_str = match format {
                0 => "table",
                1 => "json",
                2 => "yaml",
                _ => "table",
            }.to_string();

            Ok(Some(ContentCommands::Search {
                query: Some(query_input),
                topic,
                drafts,
                format: format_str
            }))
        },
        7 => {
            // Template options
            let list = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("List available templates?")
                .default(true)
                .interact()?;

            let template = if !list {
                let template_input = Input::<String>::with_theme(&ColorfulTheme::default())
                    .with_prompt("Template")
                    .interact()?;
                Some(template_input)
            } else {
                None
            };

            let output_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Output path (optional)")
                .allow_empty(true)
                .interact()?;

            let output = if output_input.is_empty() { None } else { Some(output_input) };

            Ok(Some(ContentCommands::Template { template, list, output }))
        },
        8 => Ok(None), // Back
        _ => unreachable!(),
    }
}

/// Interactive menu for topic commands
pub fn show_topic_menu() -> Result<Option<TopicCommands>> {
    let items = vec![
        "Add Topic",
        "Edit Topic",
        "Rename Topic",
        "Delete Topic",
        "List Topics",
        "Back",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a topic operation")
        .default(0)
        .items(&items)
        .interact()?;

    match selection {
        0 => {
            // Add topic options
            let key_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Key")
                .interact()?;

            let name_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Name")
                .interact()?;

            let description_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Description")
                .interact()?;

            let directory_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Directory (optional)")
                .allow_empty(true)
                .interact()?;

            let directory = if directory_input.is_empty() { None } else { Some(directory_input) };

            Ok(Some(TopicCommands::Add {
                key: key_input,
                name: Some(name_input),
                description: Some(description_input),
                directory,
            }))
        },
        1 => {
            // Edit topic options
            let key_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Key")
                .interact()?;

            let name_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("New name (optional)")
                .allow_empty(true)
                .interact()?;

            let name = if name_input.is_empty() { None } else { Some(name_input) };

            let description_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("New description (optional)")
                .allow_empty(true)
                .interact()?;

            let description = if description_input.is_empty() { None } else { Some(description_input) };

            let directory_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("New directory (optional)")
                .allow_empty(true)
                .interact()?;

            let directory = if directory_input.is_empty() { None } else { Some(directory_input) };

            Ok(Some(TopicCommands::Edit {
                key: key_input,
                name,
                description,
                directory,
            }))
        },
        2 => {
            // Rename topic options
            let from_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Current key")
                .interact()?;

            let to_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("New key")
                .interact()?;

            Ok(Some(TopicCommands::Rename {
                from: from_input,
                to: to_input,
            }))
        },
        3 => {
            // Delete topic options
            let key_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Key")
                .interact()?;

            let force = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Force deletion without confirmation?")
                .default(false)
                .interact()?;

            Ok(Some(TopicCommands::Delete {
                key: key_input,
                force,
            }))
        },
        4 => {
            // List topics options
            let format_options = vec!["table", "json", "yaml"];
            let format_selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Output format")
                .default(0)
                .items(&format_options)
                .interact()?;

            Ok(Some(TopicCommands::List {
                format: format_options[format_selection].to_string(),
            }))
        },
        5 => Ok(None), // Back
        _ => unreachable!(),
    }
}

/// Interactive menu for image commands
pub fn show_image_menu() -> Result<Option<ImageCommands>> {
    let items = vec![
        "Build Images",
        "Optimize Images",
        "Back",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an image operation")
        .default(0)
        .items(&items)
        .interact()?;

    match selection {
        0 => {
            // Build images options
            let topic_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic (optional)")
                .allow_empty(true)
                .interact()?;

            let topic = if topic_input.is_empty() { None } else { Some(topic_input) };

            let rebuild = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Rebuild all images?")
                .default(false)
                .interact()?;

            Ok(Some(ImageCommands::Build { topic, rebuild }))
        },
        1 => {
            // Optimize images options
            let topic_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic (optional)")
                .allow_empty(true)
                .interact()?;

            let topic = if topic_input.is_empty() { None } else { Some(topic_input) };

            let reoptimize = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Re-optimize all images?")
                .default(false)
                .interact()?;

            Ok(Some(ImageCommands::Optimize { topic, reoptimize }))
        },
        2 => Ok(None), // Back
        _ => unreachable!(),
    }
}

/// Interactive menu for build commands
pub fn show_build_menu() -> Result<Option<BuildCommands>> {
    let items = vec![
        "Build Content",
        "Generate Table of Contents",
        "Back",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a build operation")
        .default(0)
        .items(&items)
        .interact()?;

    match selection {
        0 => {
            // Build content options
            let topic_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic (optional)")
                .allow_empty(true)
                .interact()?;

            let topic = if topic_input.is_empty() { None } else { Some(topic_input) };

            let rebuild = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Rebuild all content?")
                .default(false)
                .interact()?;

            Ok(Some(BuildCommands::Content { topic, rebuild }))
        },
        1 => {
            // Generate TOC options
            let topic_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic (optional)")
                .allow_empty(true)
                .interact()?;

            let topic = if topic_input.is_empty() { None } else { Some(topic_input) };

            Ok(Some(BuildCommands::Toc { topic }))
        },
        2 => Ok(None), // Back
        _ => unreachable!(),
    }
}

/// Create a progress bar with a specific style
pub fn create_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .map_err(|err| {
                WritingError::format_error(format!("Failed to create progress bar template: {}", err))
            })
            .expect("Progress bar template should be valid") // This is a developer error rather than a runtime error
            .progress_chars("#>-"),
    );
    pb
}

/// Display a success message
pub fn show_success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

/// Show an error message
pub fn show_error(message: &str) {
    eprintln!("{} {}", "ERROR:".red().bold(), message);
}

/// Show a detailed error with context and suggestions
pub fn show_detailed_error(error: &WritingError) {
    print_error_detailed(error);
}

/// Display a warning message
pub fn show_warning(message: &str) {
    println!("{} {}", "!".yellow(), message);
}

/// Display an info message
pub fn show_info(message: &str) {
    println!("{} {}", "i".blue(), message);
}