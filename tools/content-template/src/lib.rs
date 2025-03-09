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
use content_template::{create_template, TemplateOptions};

let options = TemplateOptions {
    name: Some("blog-post".to_string()),
    content_type: Some("article".to_string()),
};

match create_template(&options) {
    Ok(_) => println!("Template created successfully!"),
    Err(e) => eprintln!("Error creating template: {}", e),
}
```
*/

use anyhow::{Context, Result};
use common_errors::AppError;
use common_templates::{Template, self};
use common_config::Config;
use std::path::{Path, PathBuf};
use std::fs;

/// Options for template creation
#[derive(Debug, Clone)]
pub struct TemplateOptions {
    /// The name of the template
    pub name: Option<String>,
    /// The content type for the template (article, note, etc.)
    pub content_type: Option<String>,
}

/// Create a new template with the given options
///
/// # Arguments
///
/// * `options` - The options for creating the template
///
/// # Returns
///
/// Returns a Result containing the path to the created template file, or an error
pub fn create_template(options: &TemplateOptions) -> Result<PathBuf> {
    // Validate template name
    let name = match &options.name {
        Some(name) if !name.is_empty() => name,
        Some(_) => return Err(AppError::InvalidInput("Template name cannot be empty".to_string()).into()),
        None => return Err(AppError::InvalidInput("Template name is required".to_string()).into()),
    };

    // Validate content type
    let content_type = match &options.content_type {
        Some(ct) => {
            let valid_types = ["article", "note", "tutorial"];
            if !valid_types.contains(&ct.as_str()) {
                return Err(AppError::InvalidInput(format!(
                    "Invalid content type: {}. Must be one of: {}",
                    ct,
                    valid_types.join(", ")
                )).into());
            }
            ct
        },
        None => "article", // Default content type
    };

    // Load config to get templates directory
    let config = Config::load()
        .context("Failed to load configuration while creating template")?;
    
    let templates_dir = config.paths.templates_dir()?;
    
    // Create the template file
    let template_path = templates_dir.join(format!("{}.md", name));
    
    // Check if template already exists
    if template_path.exists() {
        return Err(AppError::AlreadyExists(format!("Template '{}' already exists", name)).into());
    }
    
    // Create template directory if it doesn't exist
    if !templates_dir.exists() {
        fs::create_dir_all(&templates_dir)
            .context(format!("Failed to create templates directory at {:?}", templates_dir))?;
    }
    
    // Create template file with default content
    let template_content = format!(
        "---\nname: {}\ntype: {}\n---\n\n# {{{{ title }}}}\n\n## Introduction\n\n{{{{ introduction }}}}\n\n## Content\n\nYour content goes here.\n",
        name, content_type
    );
    
    fs::write(&template_path, template_content)
        .context(format!("Failed to write template file to {:?}", template_path))?;
    
    Ok(template_path)
}

/// List all available templates
///
/// # Returns
///
/// Returns a Result containing a vector of Template objects, or an error
pub fn list_templates() -> Result<Vec<Template>> {
    common_templates::list_templates()
}

/// Get a template by name
///
/// # Arguments
///
/// * `name` - The name of the template to retrieve
///
/// # Returns
///
/// Returns a Result containing the Template if found, or an error
pub fn get_template(name: &str) -> Result<Template> {
    common_templates::get_template(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_errors::AppError;
    use std::fs;
    use std::path::Path;
    
    #[test]
    fn test_create_template_validation() {
        // Test empty name
        let options = TemplateOptions {
            name: Some("".to_string()),
            content_type: Some("article".to_string()),
        };
        
        let result = create_template(&options);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Template name cannot be empty"));
        
        // Test invalid content type
        let options = TemplateOptions {
            name: Some("test-template".to_string()),
            content_type: Some("invalid-type".to_string()),
        };
        
        let result = create_template(&options);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid content type"));
    }
}
