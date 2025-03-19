//! # Content Management Module
//!
//! This module provides functionality for managing content, including creating,
//! editing, moving, deleting, and listing content.

use anyhow::Result;
use colored::*;
use crate::ui;
use crate::ui::feedback;
use common_traits::tools::{
    ContentCreator, ContentEditor, ContentMover, ContentDeleter, ContentValidator, ContentSearcher,
    ContentOptions, EditOptions, MoveOptions, ValidationOptions, SearchOptions, ToolFactory
};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::sync::Arc;
use common_models;
use crate::tools::factory::WriteToolFactory;

/// Content tools for the Write CLI
///
/// This struct provides content management operations for the Write CLI.
pub struct ContentTools {
    creator: Box<dyn ContentCreator + Send + Sync>,
    editor: Box<dyn ContentEditor + Send + Sync>,
    mover: Box<dyn ContentMover + Send + Sync>,
    deleter: Box<dyn ContentDeleter + Send + Sync>,
    validator: Box<dyn ContentValidator + Send + Sync>,
    searcher: Box<dyn ContentSearcher + Send + Sync>,
}

impl ContentTools {
    /// Create a new ContentTools instance with the given dependencies
    pub fn new(
        creator: Box<dyn ContentCreator + Send + Sync>,
        editor: Box<dyn ContentEditor + Send + Sync>,
        mover: Box<dyn ContentMover + Send + Sync>,
        deleter: Box<dyn ContentDeleter + Send + Sync>,
        validator: Box<dyn ContentValidator + Send + Sync>,
        searcher: Box<dyn ContentSearcher + Send + Sync>,
    ) -> Self {
        Self {
            creator,
            editor,
            mover,
            deleter,
            validator,
            searcher,
        }
    }

    /// Create new content
    pub fn create_content(
        &self,
        title: Option<String>,
        topic: Option<String>,
        description: Option<String>,
        tags: Option<String>,
        _content_type: Option<String>,
        draft: bool,
        template: Option<String>,
        _introduction: Option<String>,
    ) -> Result<()> {
        // Parse tags
        let tags = tags.map(|t| {
            t.split(',')
                .map(|tag| tag.trim().to_string())
                .collect::<Vec<String>>()
        });

        // Create options
        let options = ContentOptions {
            slug: None,  // Will be generated from title
            title,
            topic,
            description: description,
            template,
            tags,
            draft: Some(draft),
        };

        // Call the content creator
        let path = self.creator.create_content(&options)?;

        ui::show_success(&format!("Created content: {}", path.to_string_lossy().green()));

        Ok(())
    }

    /// Edit existing content
    pub fn edit_content(
        &self,
        slug: Option<String>,
        topic: Option<String>,
        _frontmatter: bool,
        editor: bool,
    ) -> Result<()> {
        // Create options
        let options = EditOptions {
            slug,
            topic,
            field: None,  // Editing the whole file
            value: None,  // No specific value to update
            editor,
        };

        // Call the content editor
        let path = self.editor.edit_content(&options)?;

        let action = if editor { "Opened" } else { "Edited" };
        ui::show_success(&format!("{} content: {}", action, path.to_string_lossy().green()));

        Ok(())
    }

    /// Move content from one location to another
    pub fn move_content(
        &self,
        slug: Option<String>,
        new_slug: Option<String>,
        from_topic: Option<String>,
        to_topic: Option<String>,
    ) -> Result<()> {
        // Create options
        let options = MoveOptions {
            slug,
            new_slug,
            from_topic,
            to_topic,
        };

        // First validate the move
        self.mover.validate_move(&options)?;

        // Then perform the move
        let new_path = self.mover.move_content(&options)?;

        ui::show_success(&format!("Moved content to: {}", new_path.to_string_lossy().green()));

        Ok(())
    }

    /// Delete content
    pub fn delete_content(
        &self,
        slug: Option<String>,
        topic: Option<String>,
        force: bool,
    ) -> Result<()> {
        // Get the slug if not provided
        let slug = match slug {
            Some(s) => s,
            None => {
                ui::show_info("Enter the slug of the content to delete:");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                input.trim().to_string()
            }
        };

        // Confirm deletion if not forced
        if !force {
            ui::show_warning(&format!("Are you sure you want to delete content '{}'?", slug));
            ui::show_warning("This action cannot be undone.");

            let confirm = dialoguer::Confirm::new()
                .with_prompt("Delete this content?")
                .default(false)
                .interact()?;

            if !confirm {
                ui::show_info("Deletion cancelled.");
                return Ok(());
            }
        }

        // Check if content can be safely deleted
        if !self.deleter.can_delete(&slug, topic.as_deref())? && !force {
            feedback::show_error("Content cannot be safely deleted. Use --force to override.");
            return Ok(());
        }

        // Delete the content
        self.deleter.delete_content(&slug, topic.as_deref(), force)?;

        ui::show_success(&format!("Deleted content: {}", slug.green()));

        Ok(())
    }

    /// Validate content
    pub fn validate_content(
        &self,
        slug: Option<String>,
        topic: Option<String>,
        validation_types: Option<Vec<String>>,
        check_links: bool,
        timeout: Option<u64>,
        dictionary: Option<String>,
        include_drafts: bool,
        verbose: bool,
    ) -> Result<()> {
        // Create options
        let options = ValidationOptions {
            slug,
            topic,
            validation_types,
            check_external_links: check_links,
            external_link_timeout: timeout,
            dictionary,
            include_drafts,
            verbose,
            fix: !verbose,  // If not verbose, try to fix issues
        };

        // Call the content validator
        let issues = self.validator.validate_content(&options)?;

        // Display results
        if issues.is_empty() {
            ui::show_success("No validation issues found.");
        } else {
            ui::show_warning(&format!("Found {} validation issues:", issues.len()));
            for issue in issues {
                feedback::show_error(&issue);
            }

            // If fix mode is enabled
            if options.fix {
                ui::show_info("Attempting to fix issues...");
                let fixed = self.validator.fix_validation_issues(&options)?;

                if fixed.is_empty() {
                    ui::show_warning("No issues could be automatically fixed.");
                } else {
                    ui::show_success(&format!("Fixed {} issues:", fixed.len()));
                    for fix in fixed {
                        ui::show_info(&fix);
                    }
                }
            }
        }

        Ok(())
    }

    /// List content
    pub fn list_content_with_options(
        &self,
        topic: Option<String>,
        include_drafts: bool,
        format: &str,
    ) -> Result<()> {
        // Create a search with empty query to list all content
        let options = SearchOptions {
            query: "".to_string(),
            topic,
            content_type: None,
            tags: None,
            limit: None,
            include_drafts,
            title_only: false,
            index_path: None,
            rebuild: false,
        };

        // Call the content searcher
        let results = self.searcher.search_content(&options)?;

        // Display results
        match format {
            "table" => {
                ui::show_info(&format!("Found {} content items:", results.len()));
                // TODO: Implement table output
                for (i, path) in results.iter().enumerate() {
                    ui::show_info(&format!("{}. {}", i + 1, path.to_string_lossy()));
                }
            },
            "json" => {
                let json = serde_json::to_string_pretty(&results.iter().map(|p| p.to_string_lossy().to_string()).collect::<Vec<_>>())?;
                println!("{}", json);
            },
            _ => {
                for path in results {
                    println!("{}", path.to_string_lossy());
                }
            }
        }

        Ok(())
    }

    /// Search content
    pub fn search_content(
        &self,
        query: String,
        topic: Option<String>,
        content_type: Option<String>,
        tags: Option<String>,
        limit: Option<usize>,
        include_drafts: bool,
        title_only: bool,
        index_path: Option<String>,
        rebuild: bool,
    ) -> Result<()> {
        // Create options
        let options = SearchOptions {
            query,
            topic,
            content_type,
            tags,
            limit,
            include_drafts,
            title_only,
            index_path,
            rebuild,
        };

        // Rebuild the search index if requested
        if rebuild {
            ui::show_info("Rebuilding search index...");
            self.searcher.build_search_index(include_drafts)?;
            ui::show_success("Search index rebuilt.");
        }

        // Call the content searcher
        let results = self.searcher.search_content(&options)?;

        // Display results
        if results.is_empty() {
            ui::show_info("No results found.");
        } else {
            ui::show_success(&format!("Found {} results:", results.len()));
            for (i, path) in results.iter().enumerate() {
                ui::show_info(&format!("{}. {}", i + 1, path.to_string_lossy()));
            }
        }

        Ok(())
    }

    /// Update a specific frontmatter field
    pub fn update_frontmatter_field(
        &self,
        slug: &str,
        topic: Option<&str>,
        field: &str,
        value: &str,
    ) -> Result<()> {
        // Call the content editor
        self.editor.update_frontmatter_field(slug, topic, field, value)?;

        ui::show_success(&format!("Updated field '{}' to '{}'.", field, value));

        Ok(())
    }
}

/// List templates
pub fn list_templates() -> Result<()> {
    // TODO: Implement template listing
    ui::show_info("Listing templates...");

    Ok(())
}

/// Create template
pub fn create_template(
    _name: Option<String>,
    _content_type: Option<String>,
) -> Result<()> {
    // For now, just show a placeholder message
    ui::show_info("Template creation is not yet implemented");
    Ok(())
}

/// Generate a slug from a title
#[allow(dead_code)]
fn slugify(title: &str) -> String {
    // A simple slugification implementation
    // In a real application, you would use a proper slugification library
    title
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect()
}

// Add module-level functions that delegate to a ContentTools instance

// Create a singleton ContentTools instance for module-level functions
static CONTENT_TOOLS: Lazy<Mutex<Option<ContentTools>>> = Lazy::new(|| Mutex::new(None));

/// Initialize the ContentTools singleton with the provided instance
pub fn init_content_tools(tools: ContentTools) {
    let mut singleton = CONTENT_TOOLS.lock().unwrap();
    *singleton = Some(tools);
}

/// Get or create a ContentTools instance
pub fn lazy_content_tools() -> Result<()> {
    // If the tools are already initialized, just return Ok
    if let Ok(tools) = get_content_tools() {
        if tools.is_some() {
            return Ok(());
        }
    }

    // Create a placeholder config for now - in a real implementation,
    // this would load the actual config
    let config = Arc::new(common_models::Config::default());

    // Create the factory
    let factory = WriteToolFactory::new(config);

    // Create the ContentTools instance with dependencies from the factory
    let tools = ContentTools::new(
        factory.create_content_creator(),
        factory.create_content_editor(),
        factory.create_content_mover(),
        factory.create_content_deleter(),
        factory.create_content_validator(),
        factory.create_content_searcher(),
    );

    // Initialize the singleton
    init_content_tools(tools);

    Ok(())
}

// Helper to get the ContentTools singleton
fn get_content_tools() -> Result<std::sync::MutexGuard<'static, Option<ContentTools>>> {
    let tools = CONTENT_TOOLS.lock().unwrap();
    if tools.is_none() {
        return Err(anyhow::anyhow!("ContentTools not initialized"));
    }
    Ok(tools)
}

// Module-level functions that delegate to the ContentTools instance

/// Create new content
pub fn create_content(
    title: Option<String>,
    topic: Option<String>,
    description: Option<String>,
    tags: Option<String>,
    content_type: Option<String>,
    draft: bool,
    template: Option<String>,
    introduction: Option<String>,
) -> Result<()> {
    let tools = get_content_tools()?;
    tools.as_ref().unwrap().create_content(title, topic, description, tags, content_type, draft, template, introduction)
}

/// Edit content
pub fn edit_content(
    slug: Option<String>,
    topic: Option<String>,
    _frontmatter: bool,
    editor: bool,
) -> Result<()> {
    let tools = get_content_tools()?;
    tools.as_ref().unwrap().edit_content(slug, topic, _frontmatter, editor)
}

/// Move content
pub fn move_content(
    slug: Option<String>,
    new_slug: Option<String>,
    from_topic: Option<String>,
    to_topic: Option<String>,
) -> Result<()> {
    let tools = get_content_tools()?;
    tools.as_ref().unwrap().move_content(slug, new_slug, from_topic, to_topic)
}

/// Delete content
pub fn delete_content(
    slug: Option<String>,
    topic: Option<String>,
    force: bool,
) -> Result<()> {
    let tools = get_content_tools()?;
    tools.as_ref().unwrap().delete_content(slug, topic, force)
}

/// Validate content
pub fn validate_content(
    slug: Option<String>,
    topic: Option<String>,
    validation_types: Option<Vec<String>>,
    check_links: bool,
    timeout: Option<u64>,
    dictionary: Option<String>,
    include_drafts: bool,
    verbose: bool,
) -> Result<()> {
    let tools = get_content_tools()?;
    tools.as_ref().unwrap().validate_content(slug, topic, validation_types, check_links, timeout, dictionary, include_drafts, verbose)
}

/// List content
pub fn list_content_with_options(
    topic: Option<String>,
    include_drafts: bool,
    format: &str,
) -> Result<()> {
    let tools = get_content_tools()?;
    tools.as_ref().unwrap().list_content_with_options(topic, include_drafts, format)
}

/// Search content
pub fn search_content(
    query: String,
    topic: Option<String>,
    content_type: Option<String>,
    tags: Option<String>,
    limit: Option<usize>,
    include_drafts: bool,
    title_only: bool,
    index_path: Option<String>,
    rebuild: bool,
) -> Result<()> {
    let tools = get_content_tools()?;
    tools.as_ref().unwrap().search_content(query, topic, content_type, tags, limit, include_drafts, title_only, index_path, rebuild)
}

/// Update frontmatter field
pub fn update_frontmatter_field(
    slug: &str,
    topic: Option<&str>,
    field: &str,
    value: &str,
) -> Result<()> {
    let tools = get_content_tools()?;
    tools.as_ref().unwrap().update_frontmatter_field(slug, topic, field, value)
}
