//! # Common Error Handling
//! 
//! This module provides common error handling utilities for the writing tools.
//! 
//! ## Features
//! 
//! - Custom error types for different error scenarios
//! - Error context for better error messages
//! - Utility traits for working with Result and Option types
//! - Conversion from standard error types
//! 
//! ## Example
//! 
//! ```rust
//! use common_errors::{Result, WritingError, ResultExt};
//! use std::path::Path;
//! 
//! fn read_config(path: &Path) -> Result<String> {
//!     std::fs::read_to_string(path)
//!         .with_context(|| format!("Failed to read config file: {}", path.display()))
//! }
//! 
//! fn validate_config(config: &str) -> Result<()> {
//!     if config.is_empty() {
//!         return Err(WritingError::validation_error("Config cannot be empty"));
//!     }
//!     Ok(())
//! }
//! ```

use thiserror::Error;
use std::path::{Path, PathBuf};
use serde_yaml;

/// Common error types for the writing tools
#[derive(Error, Debug)]
pub enum WritingError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Content not found: {0}")]
    ContentNotFound(String),

    #[error("Topic error: {0}")]
    TopicError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Yaml parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Invalid format: {0}")]
    FormatError(String),

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    
    #[error("Directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unknown error: {0}")]
    Other(String),
}

/// Result type alias using our custom error type
pub type Result<T> = std::result::Result<T, WritingError>;

/// Helper functions for error creation and transformation
impl WritingError {
    /// Create a ConfigError with a formatted message
    pub fn config_error<S: AsRef<str>>(msg: S) -> Self {
        WritingError::ConfigError(msg.as_ref().to_string())
    }

    /// Create a ContentNotFound error with a formatted message
    pub fn content_not_found<S: AsRef<str>>(msg: S) -> Self {
        WritingError::ContentNotFound(msg.as_ref().to_string())
    }

    /// Create a TopicError with a formatted message
    pub fn topic_error<S: AsRef<str>>(msg: S) -> Self {
        WritingError::TopicError(msg.as_ref().to_string())
    }

    /// Create a FileNotFound error from a path
    pub fn file_not_found<P: AsRef<Path>>(path: P) -> Self {
        WritingError::FileNotFound(path.as_ref().to_path_buf())
    }

    /// Create a DirectoryNotFound error from a path
    pub fn directory_not_found<P: AsRef<Path>>(path: P) -> Self {
        WritingError::DirectoryNotFound(path.as_ref().to_path_buf())
    }

    /// Create a ValidationError with a formatted message
    pub fn validation_error<S: AsRef<str>>(msg: S) -> Self {
        WritingError::ValidationError(msg.as_ref().to_string())
    }

    /// Create a FormatError with a formatted message
    pub fn format_error<S: AsRef<str>>(msg: S) -> Self {
        WritingError::FormatError(msg.as_ref().to_string())
    }

    /// Convert from any error type that implements std::error::Error
    pub fn from_error<E: std::error::Error>(err: E) -> Self {
        WritingError::Other(err.to_string())
    }
}

/// Extension trait for Result to add methods for better error handling
pub trait ResultExt<T, E> {
    /// Add context to an error
    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: AsRef<str>;

    /// Convert file not found IO errors to our FileNotFound error
    fn file_not_found_if_not_exists<P: AsRef<Path>>(self, path: P) -> Result<T>;
}

impl<T, E: std::error::Error> ResultExt<T, E> for std::result::Result<T, E> {
    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: AsRef<str>,
    {
        self.map_err(|e| {
            let context = f();
            WritingError::Other(format!("{}: {}", context.as_ref(), e))
        })
    }

    fn file_not_found_if_not_exists<P: AsRef<Path>>(self, path: P) -> Result<T> {
        self.map_err(|e| {
            // Check if the error message contains something indicating a file not found
            let err_str = e.to_string().to_lowercase();
            if err_str.contains("no such file") || err_str.contains("not found") || 
               err_str.contains("does not exist") {
                return WritingError::file_not_found(path);
            }
            WritingError::Other(e.to_string())
        })
    }
}

/// Extension trait for Option to add methods for better error handling
pub trait OptionExt<T> {
    /// Convert None to a ContentNotFound error
    fn content_not_found<S: AsRef<str>>(self, msg: S) -> Result<T>;
    
    /// Convert None to a ValidationError
    fn or_validation_error<S: AsRef<str>>(self, msg: S) -> Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn content_not_found<S: AsRef<str>>(self, msg: S) -> Result<T> {
        self.ok_or_else(|| WritingError::content_not_found(msg))
    }
    
    fn or_validation_error<S: AsRef<str>>(self, msg: S) -> Result<T> {
        self.ok_or_else(|| WritingError::validation_error(msg))
    }
} 