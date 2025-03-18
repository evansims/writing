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
// Add the macros module definition
mod macros;
// Remove the non-existent error module
// mod error;
// Add the error formatter module definition
mod error_formatter;

// Add comprehensive test modules
#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;

// Re-export the validation traits
pub use validation::OptionValidationExt;
// Re-export the context types
pub use context::{ErrorContext, IoResultExt};
// Re-export the category types
pub use category::{ErrorCategory};
// Re-export the reporting types and functions
pub use reporting::{ErrorReporter, ErrorDisplayStyle, get_default_reporter,
                    print_error_simple, print_error_detailed as print_error_detail_report, print_error_debug as print_error_debug_report};
// Re-export macros for convenient usage
// These are already exported via #[macro_export]
// pub use crate::with_context;
// pub use crate::try_with_context;
// pub use crate::error;

// Re-export the error formatter
pub use error_formatter::{
    ErrorFormatter, ErrorFormatterExt, Verbosity,
    print_error,
};

// Re-export the error types and functions without conflicts
// pub use error::{ErrorKind, WritingError};

use std::path::{Path, PathBuf};
use std::error::Error;

/// Error kind for categorizing errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// I/O error
    IoError,
    /// Configuration error
    ConfigError,
    /// Content not found error
    ContentNotFound,
    /// Topic error
    TopicError,
    /// File not found error
    FileNotFound,
    /// Directory not found error
    DirectoryNotFound,
    /// Validation error
    ValidationError,
    /// Format error
    FormatError,
    /// Permission denied error
    PermissionDenied,
    /// Content already exists error
    ContentAlreadyExists,
    /// Invalid argument error
    InvalidArgument,
    /// Command error
    CommandError,
    /// Template error
    TemplateError,
    /// Content parsing error
    ContentParsingError,
    /// Serialization error
    SerializationError,
    /// Deserialization error
    DeserializationError,
    /// Configuration error
    ConfigurationError,
    /// Plugin error
    PluginError,
    /// Execution error
    ExecutionError,
    /// Parsing error
    ParsingError,
    /// Network error
    NetworkError,
    /// Timeout error
    TimeoutError,
    /// Not found error
    NotFoundError,
    /// Invalid input error
    InvalidInputError,
    /// Unauthorized error
    UnauthorizedError,
    /// Lock error
    LockError,
    /// Unsupported operation error
    UnsupportedOperationError,
    /// Unknown error
    UnknownError,
    /// Other error
    Other,
}

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
#[derive(Debug, PartialEq)]
pub enum WritingError {
    /// Error related to configuration
    ConfigError(String),

    /// Error when content is not found
    ContentNotFound(String),

    /// Error related to topics
    TopicError(String),

    /// I/O error from the standard library
    IoError(String),

    /// YAML parsing error
    YamlError(String),

    /// Error for invalid formats
    FormatError(String),

    /// Error when a file is not found
    FileNotFound(PathBuf),

    /// Error when a directory is not found
    DirectoryNotFound(PathBuf),

    /// Error for validation failures
    ValidationError(String),

    /// Error for permission denied
    PermissionDenied(PathBuf),

    /// Error when content already exists
    ContentAlreadyExists(String),

    /// Error for invalid arguments
    InvalidArgument(String),

    /// Error for command execution failures
    CommandError(String),

    /// Error for template processing failures
    TemplateError(String),

    /// Error for content parsing failures
    ContentParsingError(String),

    /// Generic error for other cases
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

    /// Get the error kind
    pub fn kind(&self) -> ErrorKind {
        match self {
            WritingError::ConfigError(_) => ErrorKind::ConfigError,
            WritingError::ContentNotFound(_) => ErrorKind::ContentNotFound,
            WritingError::TopicError(_) => ErrorKind::TopicError,
            WritingError::IoError(_) => ErrorKind::IoError,
            WritingError::YamlError(_) => ErrorKind::Other,
            WritingError::FormatError(_) => ErrorKind::FormatError,
            WritingError::FileNotFound(_) => ErrorKind::FileNotFound,
            WritingError::DirectoryNotFound(_) => ErrorKind::DirectoryNotFound,
            WritingError::ValidationError(_) => ErrorKind::ValidationError,
            WritingError::PermissionDenied(_) => ErrorKind::PermissionDenied,
            WritingError::ContentAlreadyExists(_) => ErrorKind::ContentAlreadyExists,
            WritingError::InvalidArgument(_) => ErrorKind::InvalidArgument,
            WritingError::CommandError(_) => ErrorKind::CommandError,
            WritingError::TemplateError(_) => ErrorKind::TemplateError,
            WritingError::ContentParsingError(_) => ErrorKind::ContentParsingError,
            WritingError::Other(_) => ErrorKind::Other,
        }
    }

    /// Get the error message
    pub fn message(&self) -> String {
        match self {
            WritingError::ConfigError(msg) => msg.clone(),
            WritingError::ContentNotFound(msg) => msg.clone(),
            WritingError::TopicError(msg) => msg.clone(),
            WritingError::IoError(msg) => msg.clone(),
            WritingError::YamlError(msg) => msg.clone(),
            WritingError::FormatError(msg) => msg.clone(),
            WritingError::FileNotFound(path) => format!("File not found: {}", path.display()),
            WritingError::DirectoryNotFound(path) => format!("Directory not found: {}", path.display()),
            WritingError::ValidationError(msg) => msg.clone(),
            WritingError::PermissionDenied(path) => format!("Permission denied: {}", path.display()),
            WritingError::ContentAlreadyExists(msg) => msg.clone(),
            WritingError::InvalidArgument(msg) => msg.clone(),
            WritingError::CommandError(msg) => msg.clone(),
            WritingError::TemplateError(msg) => msg.clone(),
            WritingError::ContentParsingError(msg) => msg.clone(),
            WritingError::Other(msg) => msg.clone(),
        }
    }

    /// Get the error context
    pub fn context(&self) -> Option<String> {
        None
    }

    /// Get the source error
    pub fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    /// Get the backtrace
    pub fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
        None
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

// Add conversion from fs_extra::Error to WritingError
#[cfg(feature = "fs_extra")]
impl From<fs_extra::error::Error> for WritingError {
    fn from(err: fs_extra::error::Error) -> Self {
        WritingError::IoError(err.to_string())
    }
}

// Add conversion from walkdir::Error to WritingError
#[cfg(feature = "walkdir")]
impl From<walkdir::Error> for WritingError {
    fn from(err: walkdir::Error) -> Self {
        WritingError::other(err.to_string())
    }
}

impl From<anyhow::Error> for WritingError {
    fn from(err: anyhow::Error) -> Self {
        WritingError::other(err.to_string())
    }
}

// Manually reexport the ResultExt trait
pub trait ResultExt<T, E>: Sized {
    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: AsRef<str>;

    fn file_not_found_if_not_exists<P: AsRef<Path>>(self, path: P) -> Result<T>;
}

impl<T, E> ResultExt<T, E> for std::result::Result<T, E>
where
    E: Into<WritingError>,
{
    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: AsRef<str>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                let _context = f(); // Mark as used but we don't use it yet
                Err(err.into())
            }
        }
    }

    fn file_not_found_if_not_exists<P: AsRef<Path>>(self, path: P) -> Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(_) => {
                // Just return a file not found error
                Err(WritingError::file_not_found(path))
            }
        }
    }
}

impl Error for WritingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source()
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