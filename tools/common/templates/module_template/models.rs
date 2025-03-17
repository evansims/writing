//! Data models for the template module.
//!
//! This file contains the data structures used by the template module.

use std::fmt;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Represents a template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateModel {
    /// The name of the template
    pub name: String,
    /// The file system path to the template
    pub path: PathBuf,
    /// The configuration for the template
    pub config: TemplateConfig,
}

impl fmt::Display for TemplateModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.config)
    }
}

/// Configuration for a template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// Optional description of the template
    pub description: Option<String>,
}

impl fmt::Display for TemplateConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.description {
            Some(desc) => write!(f, "{}", desc),
            None => write!(f, "<No description>"),
        }
    }
}

impl TemplateConfig {
    /// Creates a new template configuration.
    pub fn new(description: Option<String>) -> Self {
        Self { description }
    }
    
    /// Creates a template configuration with no description.
    pub fn empty() -> Self {
        Self { description: None }
    }
} 