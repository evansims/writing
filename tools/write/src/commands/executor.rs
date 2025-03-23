//! # Command Executor
//!
//! This module provides functions for executing CLI commands.

use crate::cli::{
    BuildCommands, Commands, ContentCommands, ImageCommands, MigrateCommands, TopicCommands,
};
use crate::tools::{build, content, image, topic};
use anyhow::Result;

/// Execute a command
pub fn execute_command(command: Commands) -> Result<()> {
    match command {
        Commands::Content(cmd) => execute_content_command(cmd),
        Commands::Topic(cmd) => execute_topic_command(cmd),
        Commands::Image(cmd) => execute_image_command(cmd),
        Commands::Build(cmd) => execute_build_command(cmd),
        Commands::Migrate(cmd) => execute_migrate_command(cmd),
        Commands::Stats {
            slug,
            topic,
            include_drafts,
            sort_by,
            detailed,
        } => {
            // Implementing a simple stats command
            println!("Content Statistics:");
            println!("- Slug filter: {}", slug.as_deref().unwrap_or("None"));
            println!("- Topic filter: {}", topic.as_deref().unwrap_or("None"));
            println!("- Include drafts: {}", include_drafts);
            println!("- Sort by: {}", sort_by);
            println!("- Detailed: {}", detailed);

            // In a real implementation, we would call a function to generate stats
            // content::generate_content_stats(slug, topic, include_drafts, sort_by.to_string(), detailed)
            Ok(())
        }
    }
}

/// Execute a content command
pub fn execute_content_command(command: ContentCommands) -> Result<()> {
    // Initialize the ContentTools singleton
    crate::tools::content::lazy_content_tools()?;

    match command {
        ContentCommands::New {
            title,
            topic,
            description,
            tags,
            draft,
            template,
            edit,
        } => {
            let content = title.clone();
            let topic_clone = topic.clone();

            // Create the content
            content::create_content(
                Some(title),
                Some(topic),
                description,
                tags,
                None,
                draft,
                template,
                None,
            )?;

            // Open the editor if requested
            if edit {
                content::edit_content(Some(content), Some(topic_clone), true, true)?;
            }

            Ok(())
        }
        ContentCommands::Edit {
            slug,
            topic,
            field,
            value,
            editor,
        } => {
            if let (Some(field), Some(value)) = (field.as_deref(), value.as_deref()) {
                // Update a specific field
                content::update_frontmatter_field(
                    &slug.unwrap_or_default(),
                    topic.as_deref(),
                    field,
                    value,
                )
            } else {
                // Edit the whole content
                content::edit_content(slug, topic, true, editor)
            }
        }
        ContentCommands::Move { slug, from, to } => {
            content::move_content(Some(slug), None, from, to)
        }
        ContentCommands::Delete { slug, topic, force } => {
            content::delete_content(Some(slug), topic, force)
        }
        ContentCommands::Validate {
            slug,
            topic,
            all: _,
            fix,
        } => {
            let validation_types = None;
            content::validate_content(
                slug,
                topic,
                validation_types,
                false, // Don't check external links by default
                None,
                None,
                false, // Don't include drafts by default
                !fix,  // Verbose if not fixing
            )
        }
        ContentCommands::List {
            topic,
            drafts,
            format,
        } => content::list_content_with_options(topic, drafts, &format),
        ContentCommands::Search {
            query,
            topic,
            drafts,
            format: _,
        } => content::search_content(
            query.unwrap_or_default(),
            topic,
            None,
            None,
            None,
            drafts,
            false,
            None,
            false,
        ),
        ContentCommands::Template {
            template,
            list,
            output,
        } => {
            if list {
                content::list_templates()
            } else if let Some(template_name) = template {
                content::create_template(Some(template_name), output)
            } else {
                content::create_template(None, output)
            }
        }
    }
}

/// Execute a topic command
pub fn execute_topic_command(command: TopicCommands) -> Result<()> {
    match command {
        TopicCommands::Add {
            key,
            name,
            description,
            directory,
        } => topic::add_topic(Some(key), name, description, directory),
        TopicCommands::Edit {
            key,
            name,
            description,
            directory,
        } => topic::edit_topic_with_directory(Some(key), name, description, directory),
        TopicCommands::Rename { from, to } => topic::rename_topic(Some(from), Some(to), None, None),
        TopicCommands::Delete { key, force } => topic::delete_topic(Some(key), None, force),
        TopicCommands::List { format } => topic::list_topics_with_format(&format),
    }
}

/// Execute an image command
pub fn execute_image_command(command: ImageCommands) -> Result<()> {
    match command {
        ImageCommands::Build { topic, rebuild } => {
            if rebuild {
                // Force rebuilding all images
                image::build_images(None, topic, None)
            } else {
                // Only build new images
                image::build_images(None, topic, None)
            }
        }
        ImageCommands::Optimize { topic, reoptimize } => image::optimize_images(topic, reoptimize),
    }
}

/// Execute a build command
pub fn execute_build_command(command: BuildCommands) -> Result<()> {
    // Initialize the lazy build cache for better performance
    let _cache = build::lazy_build_cache();

    match command {
        BuildCommands::Content { topic, rebuild } => {
            build::build_content(
                None, None, topic, false,   // Don't include drafts by default
                false,   // Don't skip HTML
                false,   // Don't skip JSON
                false,   // Don't skip RSS
                false,   // Don't skip sitemap
                rebuild, // Pass the rebuild flag as force_rebuild
                true,    // Verbose output
            )
        }
        BuildCommands::Toc { topic: _ } => {
            // Topic is ignored for now, as the TOC is generated for all content
            build::generate_toc(None)
        }
        BuildCommands::Benchmark { .. } => {
            // TODO: Implement benchmark functionality
            Ok(())
        }
    }
}

/// Execute a migrate command
pub fn execute_migrate_command(command: MigrateCommands) -> Result<()> {
    match command {
        MigrateCommands::Content {
            topic,
            dry_run,
            delete_old,
            verbose,
        } => {
            // Use the content-migrate binary to perform the migration
            let mut cmd = std::process::Command::new("cargo");
            cmd.args(["run", "--bin", "content-migrate", "--"]);

            // Add arguments from the user's command
            if let Some(topic) = topic {
                cmd.args(["--topic", &topic]);
            }

            if dry_run {
                cmd.arg("--dry-run");
            }

            if delete_old {
                cmd.arg("--delete-old");
            }

            if verbose {
                cmd.arg("--verbose");
            }

            // Execute the command
            let output = cmd.output()?;

            // Print the output
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }

            if !output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }

            if output.status.success() {
                Ok(())
            } else {
                Err(anyhow::anyhow!(
                    "Migration failed with exit code: {}",
                    output.status
                ))
            }
        }
    }
}
