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
//! - Standardized option validation patterns
//! 
//! ## Example
//! 
//! ```rust
//! use common_errors::{Result, WritingError, ResultExt, OptionValidationExt};
//! use std::path::Path;
//! 
//! fn read_config(path: &Path) -> Result<String> {
//!     std::fs::read_to_string(path)
//!         .map_err(WritingError::from)
//!         .with_context(|| format!("Failed to read config file: {}", path.display()))
//! }
//! 
//! fn get_required_value(value: Option<String>) -> Result<String> {
//!     value.validate_required("Value is required")
//! }
//! ```

// Add the validation module definition
mod validation;
// Add the context module definition
mod context;
// Add the category module definition
mod category;
// Add the reporting module definition
mod reporting;

// Add comprehensive test modules
#[cfg(test)]
mod tests;

// Re-export the validation traits
pub use validation::OptionValidationExt;
// Re-export the context types
pub use context::{ErrorContext, IoResultExt};
// Re-export the category types
pub use category::{ErrorCategory};
// Re-export the reporting types and functions
pub use reporting::{ErrorReporter, ErrorDisplayStyle, get_default_reporter, 
                    print_error_simple, print_error_detailed, print_error_debug};

use std::path::{Path, PathBuf};
use thiserror::Error;

/// Custom error type for the writing tools
///
/// This enum represents all possible errors that can occur in the writing tools.
/// It provides specific error variants for different error scenarios and
/// implements conversion from standard error types.
///
/// # Example
///
/// ```rust
/// use common_errors::WritingError;
/// use std::path::Path;
///
/// // Create a specific error
/// let error = WritingError::file_not_found(Path::new("config.yaml"));
///
/// // Convert from a standard error
/// let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
/// let error = WritingError::from(io_error);
/// ```
#[derive(Error, Debug, PartialEq)]
pub enum WritingError {
    /// Error related to configuration
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Error when content is not found
    #[error("Content not found: {0}")]
    ContentNotFound(String),

    /// Error related to topics
    #[error("Topic error: {0}")]
    TopicError(String),

    /// I/O error from the standard library
    #[error("I/O error: {0}")]
    IoError(String),

    /// YAML parsing error
    #[error("Yaml parsing error: {0}")]
    YamlError(String),

    /// Error for invalid formats
    #[error("Invalid format: {0}")]
    FormatError(String),

    /// Error when a file is not found
    #[error("File not found: {0}")]
    FileNotFound(PathBuf),
    
    /// Error when a directory is not found
    #[error("Directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    /// Error for validation failures
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    /// Error for permission denied
    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),
    
    /// Error when content already exists
    #[error("Content already exists: {0}")]
    ContentAlreadyExists(String),
    
    /// Error for invalid arguments
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    /// Error for command execution failures
    #[error("Command error: {0}")]
    CommandError(String),
    
    /// Error for template processing failures
    #[error("Template error: {0}")]
    TemplateError(String),
    
    /// Error for content parsing failures
    #[error("Content parsing error: {0}")]
    ContentParsingError(String),

    /// Generic error for other cases
    #[error("Unknown error: {0}")]
    Other(String),
}

/// Result type alias for the writing tools
///
/// This type alias simplifies error handling by using the custom WritingError type.
///
/// # Example
///
/// ```rust
/// use common_errors::Result;
///
/// fn do_something() -> Result<()> {
///     // Implementation that might return an error
///     Ok(())
/// }
/// ```
pub type Result<T> = std::result::Result<T, WritingError>;

impl WritingError {
    /// Create a new configuration error
    ///
    /// # Parameters
    ///
    /// * `msg` - Error message
    ///
    /// # Returns
    ///
    /// A new WritingError::ConfigError
    pub fn config_error<S: AsRef<str>>(msg: S) -> Self {
        WritingError::ConfigError(msg.as_ref().to_string())
    }

    /// Create a new content not found error
    ///
    /// # Parameters
    ///
    /// * `msg` - Error message
    ///
    /// # Returns
    ///
    /// A new WritingError::ContentNotFound
    pub fn content_not_found<S: AsRef<str>>(msg: S) -> Self {
        WritingError::ContentNotFound(msg.as_ref().to_string())
    }

    /// Create a new topic error
    ///
    /// # Parameters
    ///
    /// * `msg` - Error message
    ///
    /// # Returns
    ///
    /// A new WritingError::TopicError
    pub fn topic_error<S: AsRef<str>>(msg: S) -> Self {
        WritingError::TopicError(msg.as_ref().to_string())
    }

    /// Create a new file not found error
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the file that was not found
    ///
    /// # Returns
    ///
    /// A new WritingError::FileNotFound
    pub fn file_not_found<P: AsRef<Path>>(path: P) -> Self {
        WritingError::FileNotFound(path.as_ref().to_path_buf())
    }

    /// Create a new directory not found error
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the directory that was not found
    ///
    /// # Returns
    ///
    /// A new WritingError::DirectoryNotFound
    pub fn directory_not_found<P: AsRef<Path>>(path: P) -> Self {
        WritingError::DirectoryNotFound(path.as_ref().to_path_buf())
    }

    /// Create a new validation error
    ///
    /// # Parameters
    ///
    /// * `msg` - Error message
    ///
    /// # Returns
    ///
    /// A new WritingError::ValidationError
    pub fn validation_error<S: AsRef<str>>(msg: S) -> Self {
        WritingError::ValidationError(msg.as_ref().to_string())
    }

    /// Create a new format error
    ///
    /// # Parameters
    ///
    /// * `msg` - Error message
    ///
    /// # Returns
    ///
    /// A new WritingError::FormatError
    pub fn format_error<S: AsRef<str>>(msg: S) -> Self {
        WritingError::FormatError(msg.as_ref().to_string())
    }

    /// Create a new permission denied error
    ///
    /// # Parameters
    ///
    /// * `path` - Path to the file or directory that had permission denied
    ///
    /// # Returns
    ///
    /// A new WritingError::PermissionDenied
    pub fn permission_denied<P: AsRef<Path>>(path: P) -> Self {
        WritingError::PermissionDenied(path.as_ref().to_path_buf())
    }

    /// Create a new content already exists error
    ///
    /// # Parameters
    ///
    /// * `msg` - Error message
    ///
    /// # Returns
    ///
    /// A new WritingError::ContentAlreadyExists
    pub fn content_already_exists<S: AsRef<str>>(msg: S) -> Self {
        WritingError::ContentAlreadyExists(msg.as_ref().to_string())
    }

    /// Create a new invalid argument error
    ///
    /// # Parameters
    ///
    /// * `msg` - Error message
    ///
    /// # Returns
    ///
    /// A new WritingError::InvalidArgument
    pub fn invalid_argument<S: AsRef<str>>(msg: S) -> Self {
        WritingError::InvalidArgument(msg.as_ref().to_string())
    }

    /// Create a new command error
    ///
    /// # Parameters
    ///
    /// * `msg` - Error message
    ///
    /// # Returns
    ///
    /// A new WritingError::CommandError
    pub fn command_error<S: AsRef<str>>(msg: S) -> Self {
        WritingError::CommandError(msg.as_ref().to_string())
    }

    /// Create a new template error
    ///
    /// # Parameters
    ///
    /// * `msg` - Error message
    ///
    /// # Returns
    ///
    /// A new WritingError::TemplateError
    pub fn template_error<S: AsRef<str>>(msg: S) -> Self {
        WritingError::TemplateError(msg.as_ref().to_string())
    }

    /// Create a new content parsing error
    ///
    /// # Parameters
    ///
    /// * `msg` - Error message
    ///
    /// # Returns
    ///
    /// A new WritingError::ContentParsingError
    pub fn content_parsing_error<S: AsRef<str>>(msg: S) -> Self {
        WritingError::ContentParsingError(msg.as_ref().to_string())
    }

    /// Create a new path error
    ///
    /// # Parameters
    ///
    /// * `msg` - Error message
    ///
    /// # Returns
    ///
    /// A new WritingError::Other
    pub fn path_error<S: AsRef<str>>(msg: S) -> Self {
        WritingError::Other(format!("Path error: {}", msg.as_ref()))
    }

    /// Create a new generic error
    ///
    /// # Parameters
    ///
    /// * `msg` - Error message
    ///
    /// # Returns
    ///
    /// A new WritingError::Other
    pub fn other<S: AsRef<str>>(msg: S) -> Self {
        WritingError::Other(msg.as_ref().to_string())
    }

    /// Create a new error from a standard error
    ///
    /// # Parameters
    ///
    /// * `err` - Standard error
    ///
    /// # Returns
    ///
    /// A new WritingError::Other
    pub fn from_error<E: std::error::Error>(err: E) -> Self {
        WritingError::Other(err.to_string())
    }
}

impl From<std::io::Error> for WritingError {
    fn from(err: std::io::Error) -> Self {
        WritingError::IoError(err.to_string())
    }
}

impl From<serde_yaml::Error> for WritingError {
    fn from(err: serde_yaml::Error) -> Self {
        WritingError::YamlError(err.to_string())
    }
}

/// Extension trait for Result types
///
/// This trait provides additional methods for Result types to simplify error handling.
///
/// # Example
///
/// ```rust
/// use common_errors::{Result, ResultExt};
/// use std::path::Path;
///
/// fn read_file(path: &Path) -> Result<String> {
///     std::fs::read_to_string(path)
///         .with_context(|| format!("Failed to read file: {}", path.display()))
/// }
/// ```
pub trait ResultExt<T, E>: Sized {
    /// Add context to an error
    ///
    /// This method adds context to an error, making it more informative.
    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: AsRef<str>;

    /// Convert a Result to a FileNotFound error if the error is a NotFound IO error
    ///
    /// This method converts a Result to a FileNotFound error if the error is a NotFound IO error.
    /// Otherwise, it returns the original error.
    fn file_not_found_if_not_exists<P: AsRef<Path>>(self, path: P) -> Result<T>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> ResultExt<T, E> for std::result::Result<T, E> {
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
            // Check if the error is an IO error with NotFound kind
            let is_not_found = std::any::type_name::<E>().contains("std::io::Error") && 
                format!("{:?}", e).contains("NotFound");
            
            if is_not_found {
                return WritingError::file_not_found(path);
            }
            
            WritingError::Other(format!("{}", e))
        })
    }
}

/// Extension trait for Option types
///
/// This trait provides additional methods for Option types to simplify error handling.
///
/// # Example
///
/// ```rust
/// use common_errors::{Result, OptionExt};
///
/// fn get_content(content: Option<String>) -> Result<String> {
///     content.content_not_found("Content not found")
/// }
/// ```
pub trait OptionExt<T> {
    /// Convert None to a ContentNotFound error
    ///
    /// # Parameters
    ///
    /// * `msg` - Error message
    ///
    /// # Returns
    ///
    /// A Result with the value or a ContentNotFound error
    fn content_not_found<S: AsRef<str>>(self, msg: S) -> Result<T>;
    
    /// Convert None to a ValidationError
    ///
    /// # Parameters
    ///
    /// * `msg` - Error message
    ///
    /// # Returns
    ///
    /// A Result with the value or a ValidationError
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