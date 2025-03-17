//! Error types for the template module.
//!
//! This file defines the error types used by the template module.

use std::fmt;
use std::error::Error;
use std::io;

/// Errors that can occur in the template module.
#[derive(Debug)]
pub enum TemplateError {
    /// Template with the given name was not found
    NotFound(String),
    
    /// Template with the given name already exists
    AlreadyExists(String),
    
    /// Invalid template name
    InvalidName(String),
    
    /// File system error
    FileSystem(io::Error),
    
    /// Configuration error
    Configuration(String),
    
    /// Other error
    Other(String),
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(name) => write!(f, "Template '{}' not found", name),
            Self::AlreadyExists(name) => write!(f, "Template '{}' already exists", name),
            Self::InvalidName(reason) => write!(f, "Invalid template name: {}", reason),
            Self::FileSystem(err) => write!(f, "File system error: {}", err),
            Self::Configuration(reason) => write!(f, "Configuration error: {}", reason),
            Self::Other(reason) => write!(f, "Error: {}", reason),
        }
    }
}

impl Error for TemplateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::FileSystem(err) => Some(err),
            _ => None,
        }
    }
}

// Implement conversions from other error types

impl From<io::Error> for TemplateError {
    fn from(err: io::Error) -> Self {
        Self::FileSystem(err)
    }
}

impl From<&str> for TemplateError {
    fn from(reason: &str) -> Self {
        Self::Other(reason.to_string())
    }
}

impl From<String> for TemplateError {
    fn from(reason: String) -> Self {
        Self::Other(reason)
    }
} 