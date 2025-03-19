//! # Content Templates Management
//!
//! This module provides template management functionality for content creation.
//!
//! ## Features
//!
//! - Template loading and validation
//! - Template variable substitution
//! - Template discovery and listing
//! - Template file management
//!
//! ## Example
//!
//! ```rust
//! use common_templates::{Template, load_template, list_templates};
//!
//! fn create_from_template(template_name: &str, variables: &[(&str, &str)]) -> common_errors::Result<String> {
//!     // Load template by name
//!     let template = load_template(template_name)?;
//!
//!     // Replace variables in template
//!     let content = template.render(variables)?;
//!
//!     Ok(content)
//! }
//! ```

use common_errors::{Result, WritingError};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use common_fs::normalize::{normalize_path, join_paths};

/// Template structure representing a content template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    /// The name of the template
    pub name: String,

    /// Description of the template
    pub description: String,

    /// Content type this template is for (article, note, etc.)
    pub content_type: String,

    /// Path to the template file
    pub path: PathBuf,

    /// Content of the template
    #[serde(skip)]
    content: Option<String>,
}

impl Template {
    /// Create a new template
    pub fn new(name: &str, description: &str, content_type: &str, path: &Path) -> Self {
        Template {
            name: name.to_string(),
            description: description.to_string(),
            content_type: content_type.to_string(),
            path: path.to_path_buf(),
            content: None,
        }
    }

    /// Load template content
    pub fn load(&mut self) -> Result<()> {
        self.content = Some(common_fs::read_file(&self.path)?);
        Ok(())
    }

    /// Check if template content is loaded
    pub fn is_loaded(&self) -> bool {
        self.content.is_some()
    }

    /// Get template content (loads if not already loaded)
    pub fn get_content(&mut self) -> Result<&str> {
        if !self.is_loaded() {
            self.load()?;
        }

        Ok(self.content.as_ref().unwrap())
    }

    /// Render template with variable substitutions
    pub fn render(&mut self, variables: &[(&str, &str)]) -> Result<String> {
        let content = self.get_content()?;

        // Create a variable map for easier lookups
        let var_map: HashMap<&str, &str> = variables.iter().cloned().collect();

        // Replace variables in the format {{ variable_name }}
        let re = Regex::new(r"\{\{\s*([a-zA-Z0-9_]+)\s*\}\}")
            .map_err(|e| WritingError::format_error(format!("Failed to compile regex: {}", e)))?;

        // Functional approach using fold instead of imperative loop with mutable state
        let result = re.captures_iter(content)
            .fold((String::new(), 0), |(mut output, last_end), cap| {
                let whole_match = cap.get(0).unwrap();
                let var_name = cap.get(1).unwrap().as_str();

                // Add everything up to this match
                output.push_str(&content[last_end..whole_match.start()]);

                // Add the variable replacement or the original if not found
                let replacement = var_map.get(var_name).copied()
                    .unwrap_or(whole_match.as_str());
                output.push_str(replacement);

                (output, whole_match.end())
            });

        // Add any remaining content after the last match
        let (mut output, last_end) = result;
        output.push_str(&content[last_end..]);

        Ok(output)
    }
}

/// Get the template directory path
pub fn get_templates_dir() -> Result<PathBuf> {
    // Try to load from config if available
    match common_config::load_config() {
        Ok(config) => {
            // First check if a templates directory is specified in the config
            // For now, we'll just use a default location relative to content
            let base_dir = PathBuf::from(&config.content.base_dir);
            let templates_dir = normalize_path(join_paths(&base_dir, "../templates"));
            Ok(templates_dir)
        },
        Err(_) => {
            // Fallback to searching for templates directory
            // Look in the current directory and parent directories
            let mut current_dir = std::env::current_dir()?;
            let templates_dir = current_dir.join("templates");

            if templates_dir.exists() && templates_dir.is_dir() {
                return Ok(templates_dir);
            }

            // Try parent directories
            while current_dir.pop() {
                let templates_dir = current_dir.join("templates");
                if templates_dir.exists() && templates_dir.is_dir() {
                    return Ok(templates_dir);
                }
            }

            // If we get here, we couldn't find the templates directory
            Err(WritingError::directory_not_found(PathBuf::from("templates")))
        }
    }
}

/// List all available templates
pub fn list_templates() -> Result<Vec<Template>> {
    let templates_dir = get_templates_dir()?;
    let mut templates = Vec::new();

    // Read the templates directory
    if !templates_dir.exists() {
        return Err(WritingError::directory_not_found(&templates_dir));
    }

    // List all .mdx files in the templates directory
    for entry in fs::read_dir(&templates_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() &&
           path.extension().is_some_and(|ext| ext == "mdx" || ext == "md") &&
           path.file_name().is_some_and(|name| name.to_string_lossy().contains("template"))
        {
            // Extract template information from filename
            let file_stem = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            let parts: Vec<&str> = file_stem.split('-').collect();
            if parts.len() >= 2 && parts.last().unwrap() == &"template" {
                let content_type = parts[0];
                let template_name = file_stem.to_string();
                let description = format!("Template for {} content", content_type);

                templates.push(Template::new(
                    &template_name,
                    &description,
                    content_type,
                    &path
                ));
            }
        }
    }

    // If no templates found, add default template info
    if templates.is_empty() {
        let default_path = templates_dir.join("article-template.mdx");
        templates.push(Template::new(
            "article-template",
            "Default template for article content",
            "article",
            &default_path
        ));
    }

    Ok(templates)
}

/// Load a specific template by name
pub fn load_template(name: &str) -> Result<Template> {
    let templates = list_templates()?;

    // Find template by name
    let template = templates.into_iter()
        .find(|t| t.name == name || t.name.contains(name))
        .ok_or_else(|| WritingError::format_error(format!("Template not found: {}", name)))?;

    // Load template content
    let mut template = template;
    template.load()?;

    Ok(template)
}

/// Load a template for a specific content type
pub fn load_template_for_content_type(content_type: &str) -> Result<Template> {
    let templates = list_templates()?;

    // First try to find an exact match
    if let Some(template) = templates.iter().find(|t| t.content_type == content_type) {
        let mut template = template.clone();
        template.load()?;
        return Ok(template);
    }

    // If no exact match, look for a partial match
    if let Some(template) = templates.iter().find(|t| t.content_type.contains(content_type) || content_type.contains(&t.content_type)) {
        let mut template = template.clone();
        template.load()?;
        return Ok(template);
    }

    // If still no match, use the first template as default
    if let Some(template) = templates.first() {
        let mut template = template.clone();
        template.load()?;
        return Ok(template);
    }

    // No templates available
    Err(WritingError::format_error(format!("No template available for content type: {}", content_type)))
}

/// Create a new template
pub fn create_template(name: &str, content_type: &str, content: &str) -> Result<Template> {
    let templates_dir = get_templates_dir()?;

    // Ensure templates directory exists
    common_fs::create_dir_all(&templates_dir)?;

    // Sanitize name to be filesystem-friendly
    let sanitized_name = slug::slugify(name);
    let filename = format!("{}-template.mdx", sanitized_name);
    let path = templates_dir.join(&filename);

    // Write template content
    common_fs::write_file(&path, content)?;

    // Create and return template object
    let description = format!("Template for {} content", content_type);
    let template = Template::new(
        &sanitized_name,
        &description,
        content_type,
        &path
    );

    Ok(template)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn setup_test_template() -> (tempfile::TempDir, Template) {
        let temp_dir = tempdir().unwrap();
        let template_path = temp_dir.path().join("article-template.mdx");

        let content = r#"---
title: "{{ title }}"
description: "{{ description }}"
slug: "{{ slug }}"
topics: ["{{ topic }}"]
tags:
  [
    {{ tags }}
  ]
published: "{{ date }}"
---

## Introduction

{{ introduction }}

## Main Content

This is the main content.
"#;

        fs::write(&template_path, content).unwrap();

        let template = Template::new(
            "article-template",
            "Test template",
            "article",
            &template_path
        );

        (temp_dir, template)
    }

    #[test]
    fn test_template_loading() {
        let (_temp_dir, mut template) = setup_test_template();

        // Test loading
        assert!(!template.is_loaded());
        let result = template.load();
        assert!(result.is_ok());
        assert!(template.is_loaded());

        // Test content retrieval
        let content = template.get_content().unwrap();
        assert!(content.contains("{{ title }}"));
        assert!(content.contains("{{ description }}"));
    }

    #[test]
    fn test_template_rendering() {
        let (_temp_dir, mut template) = setup_test_template();

        // Load template
        template.load().unwrap();

        // Define variables
        let variables = [
            ("title", "Test Article"),
            ("description", "This is a test article"),
            ("slug", "test-article"),
            ("topic", "test"),
            ("tags", "\"test\", \"article\""),
            ("date", "2023-01-01"),
            ("introduction", "This is the introduction paragraph."),
        ];

        // Render template
        let rendered = template.render(&variables).unwrap();

        // Check variable substitutions
        assert!(rendered.contains("title: \"Test Article\""));
        assert!(rendered.contains("description: \"This is a test article\""));
        assert!(rendered.contains("slug: \"test-article\""));
        assert!(rendered.contains("topics: [\"test\"]"));
        assert!(rendered.contains("\"test\", \"article\""));
        assert!(rendered.contains("published: \"2023-01-01\""));
        assert!(rendered.contains("This is the introduction paragraph."));
    }

    #[test]
    fn test_template_render_missing_variables() {
        let (_temp_dir, mut template) = setup_test_template();

        // Load template
        template.load().unwrap();

        // Define partial variables
        let variables = [
            ("title", "Test Article"),
            ("slug", "test-article"),
        ];

        // Render template
        let rendered = template.render(&variables).unwrap();

        // Check variable substitutions (replaced ones)
        assert!(rendered.contains("title: \"Test Article\""));
        assert!(rendered.contains("slug: \"test-article\""));

        // Check variable substitutions (not replaced ones)
        assert!(rendered.contains("description: \"{{ description }}\""));
        assert!(rendered.contains("topics: [\"{{ topic }}\"]"));
    }
}