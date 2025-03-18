/*!
# Content Template Library

This library provides functionality for creating and managing content templates.
It allows users to create new templates, list existing templates, and validate template properties.

## Features
- Create new templates with specified name and content type
- Validate template properties like name and content type
- List available templates

## Example
```rust
use content_template::{create_template, CreateTemplateOptions};

let options = CreateTemplateOptions {
    name: "blog-post".to_string(),
    content_type: "article".to_string(),
    content: None,
};

match create_template(options) {
    Ok(_) => println!("Template created successfully!"),
    Err(e) => eprintln!("Error creating template: {}", e),
}
```
*/

use anyhow::{Context, Result};
use common_config::load_config;
use common_fs::{read_file, write_file, create_dir_all};
use common_models::Config;
use common_templates::Template;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Errors specific to template operations
#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("Template name cannot be empty")]
    EmptyName,

    #[error("Invalid content type: {0}")]
    InvalidContentType(String),

    #[error("Template '{0}' already exists")]
    TemplateExists(String),

    #[error("Template '{0}' not found")]
    TemplateNotFound(String),

    #[error("Failed to read template directory: {0}")]
    ReadDirError(#[from] std::io::Error),
}

/// Options for creating a new template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplateOptions {
    /// The name of the template
    pub name: String,

    /// The content type for the template (article, note, etc.)
    pub content_type: String,

    /// The template content
    pub content: Option<String>,
}

/// Creates a new template with the given options
///
/// # Parameters
///
/// * `options` - Options for creating the template
///
/// # Returns
///
/// Returns the created template
///
/// # Errors
///
/// Returns an error if the template cannot be created
pub fn create_template(options: CreateTemplateOptions) -> Result<Template> {
    // Validate inputs
    if options.name.is_empty() {
        return Err(TemplateError::EmptyName.into());
    }

    let valid_content_types = ["article", "note", "tutorial", "review"];
    if !valid_content_types.contains(&options.content_type.as_str()) {
        return Err(TemplateError::InvalidContentType(options.content_type.clone()).into());
    }

    // Load config
    let config = load_config()?;

    // Ensure templates directory exists
    let templates_dir = get_templates_dir(&config)?;
    if !templates_dir.exists() {
        create_dir_all(&templates_dir)?;
    }

    // Create template file
    let template_path = templates_dir.join(format!("{}.md", options.name));

    // Use provided content or create default content
    let content = options.content.unwrap_or_else(|| {
        format!(
            "---\ntitle: {{{{ title }}}}\ndate: {{{{ date }}}}\n---\n\n# {{{{ title }}}}\n\nYour {} content here.\n",
            options.content_type
        )
    });

    // Write template file
    write_file(&template_path, &content)?;

    // Create and return template object
    Ok(Template::new(&options.name, "Template description", &options.content_type, &template_path))
}

/// Get a template by name
///
/// # Arguments
///
/// * `name` - The name of the template to get
///
/// # Returns
///
/// Returns the template content on success, or an error if something went wrong
pub fn get_template(name: &str) -> Result<String> {
    // Validate inputs
    if name.is_empty() {
        return Err(TemplateError::EmptyName.into());
    }

    // Load config
    let config = load_config()?;

    // Get the templates directory
    let templates_dir = get_templates_dir(&config)?;

    // Create the template file path
    let template_path = templates_dir.join(format!("{}.md", name));

    // Check if the template exists
    if !template_path.exists() {
        return Err(TemplateError::TemplateNotFound(name.to_string()).into());
    }

    // Read the template file
    let content = read_file(&template_path)
        .with_context(|| format!("Failed to read template file: {:?}", template_path))?;

    Ok(content)
}

/// Delete a template
///
/// # Arguments
///
/// * `name` - The name of the template to delete
///
/// # Returns
///
/// Returns Ok(()) on success, or an error if something went wrong
pub fn delete_template(name: &str) -> Result<()> {
    // Validate inputs
    if name.is_empty() {
        return Err(TemplateError::EmptyName.into());
    }

    // Load config
    let config = load_config()?;

    // Get the templates directory
    let templates_dir = get_templates_dir(&config)?;

    // Create the template file path
    let template_path = templates_dir.join(format!("{}.md", name));

    // Check if the template exists
    if !template_path.exists() {
        return Err(TemplateError::TemplateNotFound(name.to_string()).into());
    }

    // Delete the template file
    std::fs::remove_file(&template_path)
        .with_context(|| format!("Failed to delete template file: {:?}", template_path))?;

    Ok(())
}

/// List all available templates
///
/// # Returns
///
/// Returns a list of templates on success, or an error if something went wrong
pub fn list_templates() -> Result<Vec<Template>> {
    // Load config
    let config = load_config()?;

    // Get the templates directory
    let templates_dir = get_templates_dir(&config)?;

    // Check if the templates directory exists
    if !templates_dir.exists() {
        return Ok(Vec::new());
    }

    // Find all template files
    let template_files = std::fs::read_dir(&templates_dir)
        .with_context(|| format!("Failed to read templates directory: {:?}", templates_dir))?;

    let mut templates = Vec::new();

    for entry in template_files {
        let entry = entry?;
        let path = entry.path();

        // Only process markdown files
        if path.extension().is_some_and(|ext| ext == "md") {
            let name = path.file_stem().unwrap().to_string_lossy().to_string();

            // Read the file to determine content type
            let content = read_file(&path)?;

            // Simple content type extraction - a more robust implementation would use proper YAML parsing
            let content_type = if content.contains("type: article") {
                "article"
            } else if content.contains("type: note") {
                "note"
            } else if content.contains("type: tutorial") {
                "tutorial"
            } else {
                "unknown"
            };

            templates.push(Template::new(&name, "Template description", content_type, &path));
        }
    }

    Ok(templates)
}

/// Helper function to get the templates directory from the config
fn get_templates_dir(config: &Config) -> Result<PathBuf> {
    let content_dir = &config.content.base_dir;
    let templates_dir = Path::new(content_dir).join("templates");
    Ok(templates_dir)
}

