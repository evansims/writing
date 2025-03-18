//! Tests for error formatting functionality
//!
//! This file contains tests for the error formatter and related functionality.

use crate::{
    WritingError, ErrorFormatter, ErrorFormatterExt, Verbosity,
    print_error
};
use std::path::PathBuf;

#[test]
fn test_error_formatter_basic() {
    let err = WritingError::file_not_found("/path/to/file.txt");
    let formatter = ErrorFormatter::new(&err);

    let formatted = formatter.format();
    assert!(formatted.contains("File not found"));
    assert!(formatted.contains("/path/to/file.txt"));
}

#[test]
fn test_error_formatter_with_context() {
    let err = WritingError::file_not_found("/path/to/file.txt")
        .with_context(|| "Failed to read configuration");
    let formatter = ErrorFormatter::new(&err);

    let formatted = formatter.format();
    assert!(formatted.contains("File not found"));
    assert!(formatted.contains("/path/to/file.txt"));
    assert!(formatted.contains("Failed to read configuration"));
}

#[test]
fn test_error_formatter_minimal_verbosity() {
    let err = WritingError::file_not_found("/path/to/file.txt")
        .with_context(|| "Failed to read configuration");
    let formatter = ErrorFormatter::new(&err).verbosity(Verbosity::Minimal);

    let formatted = formatter.format();
    assert!(formatted.contains("File not found"));
    // In minimal verbosity, we should not see the context
    assert!(!formatted.contains("Failed to read configuration"));
}

#[test]
fn test_error_formatter_detailed_verbosity() {
    let err = WritingError::file_not_found("/path/to/file.txt")
        .with_context(|| "Failed to read configuration");
    let formatter = ErrorFormatter::new(&err).verbosity(Verbosity::Detailed);

    let formatted = formatter.format();
    assert!(formatted.contains("File not found"));
    assert!(formatted.contains("/path/to/file.txt"));
    assert!(formatted.contains("Failed to read configuration"));
    // In detailed verbosity, we should see more information about the error
    assert!(formatted.contains("Error Type:"));
}

#[test]
fn test_error_formatter_debug_verbosity() {
    let err = WritingError::file_not_found("/path/to/file.txt")
        .with_context(|| "Failed to read configuration");
    let formatter = ErrorFormatter::new(&err).verbosity(Verbosity::Debug);

    let formatted = formatter.format();
    assert!(formatted.contains("File not found"));
    assert!(formatted.contains("/path/to/file.txt"));
    assert!(formatted.contains("Failed to read configuration"));
    // In debug verbosity, we should see even more details
    assert!(formatted.contains("Debug Details:"));
}

#[test]
fn test_error_formatter_extension_trait() {
    let err = WritingError::file_not_found("/path/to/file.txt");

    // Test the extension trait for formatting
    let formatted = err.format();
    assert!(formatted.contains("File not found"));
    assert!(formatted.contains("/path/to/file.txt"));

    // Test changing verbosity
    let formatted_minimal = err.format_with_verbosity(Verbosity::Minimal);
    assert!(formatted_minimal.contains("File not found"));
    assert!(formatted_minimal.len() < formatted.len()); // Minimal should be shorter
}

#[test]
fn test_print_error_function() {
    let err = WritingError::file_not_found("/path/to/file.txt");

    // We can't easily test the actual printing, but we can ensure the function doesn't panic
    print_error(&err, Verbosity::Normal);
    print_error(&err, Verbosity::Minimal);
    print_error(&err, Verbosity::Detailed);
    print_error(&err, Verbosity::Debug);

    // No assertion needed; if we get here without panicking, the test passes
}

#[test]
fn test_nested_error_formatting() {
    // Create a nested error scenario
    let inner_err = WritingError::file_not_found("/inner/file.txt");
    let middle_err = inner_err.with_context(|| "Middle layer error");
    let outer_err = middle_err.with_context(|| "Outer layer error");

    let formatter = ErrorFormatter::new(&outer_err).verbosity(Verbosity::Detailed);
    let formatted = formatter.format();

    // Check that all layers are present in the detailed output
    assert!(formatted.contains("Outer layer error"));
    assert!(formatted.contains("Middle layer error"));
    assert!(formatted.contains("/inner/file.txt"));
}

#[test]
fn test_formatting_different_error_types() {
    // Test formatting for different error types
    let errors = vec![
        WritingError::file_not_found("/path/to/file.txt"),
        WritingError::directory_not_found("/path/to/dir"),
        WritingError::config_error("Invalid configuration"),
        WritingError::content_not_found("Article 'test' not found"),
        WritingError::topic_error("Invalid topic"),
        WritingError::validation_error("Validation failed"),
        WritingError::format_error("Invalid format"),
        WritingError::permission_denied("/path/to/protected"),
        WritingError::content_already_exists("Content already exists"),
        WritingError::invalid_argument("Invalid argument"),
        WritingError::command_error("Command failed"),
        WritingError::template_error("Template error"),
        WritingError::content_parsing_error("Parsing error"),
        WritingError::other("Other error"),
    ];

    for err in errors {
        let formatted = err.format();
        // Each error should have a non-empty formatted string
        assert!(!formatted.is_empty());
        // Each error should contain its specific message
        match &err {
            WritingError::FileNotFound(path) => {
                assert!(formatted.contains(&path.display().to_string()));
            }
            WritingError::DirectoryNotFound(path) => {
                assert!(formatted.contains(&path.display().to_string()));
            }
            _ => {
                // For other error types, check that the message is included
                assert!(formatted.contains(&err.message()));
            }
        }
    }
}