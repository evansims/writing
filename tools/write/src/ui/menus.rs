//! # Interactive Menu Components
//!
//! This module provides interactive menus for the CLI experience.

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

use crate::cli::{Commands, ContentCommands, TopicCommands, ImageCommands, BuildCommands};

/// Interactive menu for the main commands
///
/// This function displays a menu with the main command categories
/// and returns the selected command.
///
/// # Returns
///
/// The selected command, or None if the user chose to exit
///
/// # Errors
///
/// Returns an error if there is an issue with displaying the menu
/// or getting user input
///
/// # Examples
///
/// ```no_run
/// use write::ui::menus::show_main_menu;
///
/// match show_main_menu() {
///     Ok(Some(command)) => println!("Selected command: {:?}", command),
///     Ok(None) => println!("Exiting..."),
///     Err(err) => eprintln!("Error: {}", err),
/// }
/// ```
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
///
/// This function displays a menu with content management options
/// and returns the selected command.
///
/// # Returns
///
/// The selected content command, or None if the user chose to go back
///
/// # Errors
///
/// Returns an error if there is an issue with displaying the menu
/// or getting user input
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

    // Match based on selection and gather appropriate options
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

            let description_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Description (optional)")
                .allow_empty(true)
                .interact()?;

            let description = if description_input.is_empty() { None } else { Some(description_input) };

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
                description,
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

            let frontmatter_only = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Edit frontmatter only?")
                .default(false)
                .interact()?;

            let content_only = if frontmatter_only {
                false
            } else {
                Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt("Edit content only?")
                    .default(false)
                    .interact()?
            };

            // Convert UI options to ContentCommands::Edit options
            let field = if frontmatter_only {
                Some("frontmatter".to_string())
            } else if content_only {
                Some("content".to_string())
            } else {
                Some("all".to_string())
            };

            let value = None; // This will be edited in the editor
            let editor = true; // Always use editor for interactive mode

            Ok(Some(ContentCommands::Edit {
                slug,
                topic,
                field,
                value,
                editor,
            }))
        },
        // ... rest of content menu options
        // Implementation continues with remaining options
        // Note: Full implementation would include all menu options
        8 => Ok(None), // Back
        _ => unreachable!(),
    }
}

/// Interactive menu for topic commands
///
/// This function displays a menu with topic management options
/// and returns the selected command.
///
/// # Returns
///
/// The selected topic command, or None if the user chose to go back
///
/// # Errors
///
/// Returns an error if there is an issue with displaying the menu
/// or getting user input
pub fn show_topic_menu() -> Result<Option<TopicCommands>> {
    let items = vec![
        "Create New Topic",
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

    // Implementation would include gathering options for each menu item
    // Simplified implementation for brevity
    match selection {
        0 => {
            // New topic options
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
                .with_prompt("Directory")
                .interact()?;

            Ok(Some(TopicCommands::Add {
                key: key_input,
                name: Some(name_input),
                description: Some(description_input),
                directory: Some(directory_input),
            }))
        },
        // ... rest of topic menu options
        5 => Ok(None), // Back
        _ => unreachable!(),
    }
}

/// Interactive menu for image commands
///
/// This function displays a menu with image management options
/// and returns the selected command.
///
/// # Returns
///
/// The selected image command, or None if the user chose to go back
///
/// # Errors
///
/// Returns an error if there is an issue with displaying the menu
/// or getting user input
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

    // Implementation would include gathering options for each menu item
    // Simplified implementation for brevity
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
        // ... rest of image menu options
        2 => Ok(None), // Back
        _ => unreachable!(),
    }
}

/// Interactive menu for build commands
///
/// This function displays a menu with build operation options
/// and returns the selected command.
///
/// # Returns
///
/// The selected build command, or None if the user chose to go back
///
/// # Errors
///
/// Returns an error if there is an issue with displaying the menu
/// or getting user input
pub fn show_build_menu() -> Result<Option<BuildCommands>> {
    let items = vec![
        "Build Site",
        "Build Table of Contents",
        "Back",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a build operation")
        .default(0)
        .items(&items)
        .interact()?;

    // Implementation would include gathering options for each menu item
    // Simplified implementation for brevity
    match selection {
        0 => {
            // Build site options
            let topic_input = Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("Topic (optional)")
                .allow_empty(true)
                .interact()?;

            let topic = if topic_input.is_empty() { None } else { Some(topic_input) };

            let rebuild = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Force rebuild?")
                .default(false)
                .interact()?;

            Ok(Some(BuildCommands::Content { topic, rebuild }))
        },
        // ... rest of build menu options
        2 => Ok(None), // Back
        _ => unreachable!(),
    }
}