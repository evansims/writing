//! # Error Handling Macros
//!
//! This module provides macros for common error handling patterns.

/// Add context to a result based on a pattern
///
//! # Examples
///
//! ```
//! use common_errors::{Result, with_context};
//! use std::fs;
//! use std::path::Path;
//!
//! fn read_config(path: &Path) -> Result<String> {
//!     with_context!(fs::read_to_string(path), "Failed to read config file: {}", path.display())
//! }
//! ```
#[macro_export]
macro_rules! with_context {
    ($expr:expr, $msg:expr) => {
        $expr.with_context(|| $msg.to_string())
    };
    ($expr:expr, $fmt:expr, $($arg:tt)*) => {
        $expr.with_context(|| format!($fmt, $($arg)*))
    };
}

/// Propagate an error with added context if it occurs
///
//! # Examples
///
//! ```
//! use common_errors::{Result, try_with_context};
//! use std::fs;
//! use std::path::Path;
//!
//! fn read_config(path: &Path) -> Result<String> {
//!     let content = try_with_context!(fs::read_to_string(path), "Failed to read config file: {}", path.display());
//!     Ok(content)
//! }
//! ```
#[macro_export]
macro_rules! try_with_context {
    ($expr:expr, $msg:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.with_context(|| $msg.to_string())),
        }
    };
    ($expr:expr, $fmt:expr, $($arg:tt)*) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.with_context(|| format!($fmt, $($arg)*))),
        }
    };
}

/// Create an error with the given message and category
///
/// # Examples
///
/// ```
/// use common_errors::{WritingError, error};
///
/// fn validate_input(input: &str) -> Result<(), WritingError> {
///     if input.is_empty() {
///         return Err(error!(validation, "Input cannot be empty"));
///     }
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! error {
    (config, $msg:expr) => {
        $crate::WritingError::config_error($msg)
    };
    (validation, $msg:expr) => {
        $crate::WritingError::validation_error($msg)
    };
    (topic, $msg:expr) => {
        $crate::WritingError::topic_error($msg)
    };
    (content, $msg:expr) => {
        $crate::WritingError::content_error($msg)
    };
    (io, $msg:expr) => {
        $crate::WritingError::io_error($msg)
    };
    (template, $msg:expr) => {
        $crate::WritingError::template_error($msg)
    };
    (format, $msg:expr) => {
        $crate::WritingError::format_error($msg)
    };
    (internal, $msg:expr) => {
        $crate::WritingError::internal_error($msg)
    };
    (not_found, $msg:expr) => {
        $crate::WritingError::not_found_error($msg)
    };
    ($category:expr, $msg:expr) => {
        $crate::WritingError::new($category, $msg)
    };
}