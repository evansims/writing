//! # User Feedback Components
//!
//! This module provides functions for displaying user feedback such as
//! success messages, errors, warnings, and informational messages.

use colored::*;
use common_errors::{WritingError, print_error};

/// Display a success message
///
/// This function displays a success message with a green checkmark.
///
/// # Arguments
///
/// * `message` - The success message to display
///
/// # Examples
///
/// ```no_run
/// use write::ui::feedback::show_success;
///
/// show_success("Operation completed successfully");
/// ```
pub fn show_success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

/// Show an error message
///
/// This function displays an error message with a red "ERROR:" prefix.
///
/// # Arguments
///
/// * `message` - The error message to display
///
/// # Examples
///
/// ```no_run
/// use write::ui::feedback::show_error;
///
/// show_error("Failed to complete operation");
/// ```
pub fn show_error(message: &str) {
    eprintln!("{} {}", "ERROR:".red().bold(), message);
}

/// Show a detailed error with context and suggestions
///
/// This function displays a detailed error message with context and suggestions
/// using the common_errors framework.
///
/// # Arguments
///
/// * `error` - The WritingError to display
///
/// # Examples
///
/// ```no_run
/// use write::ui::feedback::show_detailed_error;
/// use common_errors::WritingError;
///
/// let error = WritingError::validation_error("Invalid input");
/// show_detailed_error(&error);
/// ```
pub fn show_detailed_error(error: &WritingError) {
    print_error(error);
}

/// Display a warning message
///
/// This function displays a warning message with a yellow exclamation mark.
///
/// # Arguments
///
/// * `message` - The warning message to display
///
/// # Examples
///
/// ```no_run
/// use write::ui::feedback::show_warning;
///
/// show_warning("This operation may take a long time");
/// ```
pub fn show_warning(message: &str) {
    println!("{} {}", "!".yellow(), message);
}

/// Display an info message
///
/// This function displays an informational message with a blue "i".
///
/// # Arguments
///
/// * `message` - The informational message to display
///
/// # Examples
///
/// ```no_run
/// use write::ui::feedback::show_info;
///
/// show_info("The operation is starting...");
/// ```
pub fn show_info(message: &str) {
    println!("{} {}", "i".blue(), message);
}

/// Display a debug message (only shown in debug mode)
///
/// This function displays a debug message with a magenta "DEBUG:" prefix.
/// These messages are intended for development and debugging purposes.
///
/// # Arguments
///
/// * `message` - The debug message to display
///
/// # Examples
///
/// ```no_run
/// use write::ui::feedback::show_debug;
///
/// show_debug("Internal state: count=5, mode=active");
/// ```
pub fn show_debug(message: &str) {
    if cfg!(debug_assertions) {
        println!("{} {}", "DEBUG:".magenta(), message);
    }
}