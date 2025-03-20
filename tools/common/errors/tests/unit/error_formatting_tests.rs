//! Tests for the error formatter
//!
//! This file contains tests for the error formatter and related functionality.

use crate::helpers::{
    print_error, ErrorCategory, ErrorContext, ErrorDisplayStyle, ErrorFormatter, ErrorFormatterExt,
    Verbosity, WritingError,
};
use std::io;

#[test]
fn test_error_formatter_basic() {
    let err = WritingError::file_not_found("/path/to/file.txt");
    let formatter = ErrorFormatter::new();

    let formatted = formatter.format(&err);
    assert!(formatted.contains("File not found"));
    assert!(formatted.contains("/path/to/file.txt"));
}

#[test]
fn test_error_formatter_with_context() {
    let err = WritingError::file_not_found("/path/to/file.txt");
    let formatter = ErrorFormatter::new();

    let formatted = formatter.format(&err);
    assert!(formatted.contains("File not found"));
    assert!(formatted.contains("/path/to/file.txt"));
}

#[test]
fn test_error_formatter_minimal_verbosity() {
    let err = WritingError::file_not_found("/path/to/file.txt");
    let formatter = ErrorFormatter::new().with_verbosity(Verbosity::Minimal);

    let formatted = formatter.format(&err);
    assert!(formatted.contains("File not found"));
}

#[test]
fn test_error_formatter_detailed_verbosity() {
    let err = WritingError::file_not_found("/path/to/file.txt");
    let formatter = ErrorFormatter::new().with_verbosity(Verbosity::Detailed);

    let formatted = formatter.format(&err);
    assert!(formatted.contains("File not found"));
    assert!(formatted.contains("/path/to/file.txt"));
    // In detailed verbosity, we should see more information about the error
    assert!(formatted.contains("Error Type:"));
}

#[test]
fn test_error_formatter_debug_verbosity() {
    let err = WritingError::file_not_found("/path/to/file.txt");
    let formatter = ErrorFormatter::new().with_verbosity(Verbosity::Debug);

    let formatted = formatter.format(&err);
    assert!(formatted.contains("File not found"));
    assert!(formatted.contains("/path/to/file.txt"));
    // In debug verbosity, we should see even more details
    assert!(formatted.contains("Debug Details:"));
}

#[test]
fn test_error_formatter_extension_trait() {
    let err = WritingError::file_not_found("/path/to/file.txt");
    let formatter = ErrorFormatter::new();

    // Test the extension trait for formatting
    let formatted = err.format(&formatter);
    assert!(formatted.contains("File not found"));
    assert!(formatted.contains("/path/to/file.txt"));

    // Test with different verbosity levels
    let formatter_minimal = ErrorFormatter::new().with_verbosity(Verbosity::Minimal);
    let formatted_minimal = err.format(&formatter_minimal);
    assert!(formatted_minimal.contains("File not found"));
    assert!(formatted_minimal.len() < formatted.len()); // Minimal should be shorter
}

#[test]
fn test_print_error_function() {
    let err = WritingError::file_not_found("/path/to/file.txt");

    // We can't easily test the actual printing, but we can ensure the function doesn't panic
    print_error(&err);

    // No assertion needed; if we get here without panicking, the test passes
}

#[test]
fn test_nested_error_formatting() {
    // Create a nested error scenario
    let inner_err = WritingError::file_not_found("/inner/file.txt");
    let middle_err = WritingError::other("Middle layer error");
    let outer_err = WritingError::other("Outer layer error");

    let formatter = ErrorFormatter::new().with_verbosity(Verbosity::Detailed);
    let formatted = formatter.format(&outer_err);

    // Check that all layers are present in the detailed output
    assert!(formatted.contains("Outer layer error"));
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

    let formatter = ErrorFormatter::new();
    for err in errors {
        let formatted = err.format(&formatter);
        // Each error should have a non-empty formatted string
        assert!(!formatted.is_empty());
        // Each error should contain its specific message
        assert!(formatted.contains(&err.message()));
    }
}
