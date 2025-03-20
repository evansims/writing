//! Tests for error context functionality
//!
//! This file contains tests for error context handling.

use crate::helpers::{ErrorContext, IoResultExt, Result, ResultExt, WritingError};
use std::fs;
use std::io;
use std::path::Path;

#[test]
fn test_with_context() {
    // Create an error
    let result: Result<()> = Err(WritingError::validation_error("Value is invalid"));

    // Add context
    let with_context = result.with_context(|| "Validation failed for user input");

    // Check that the error is still a ValidationError or contains the original message
    let error_message = with_context.unwrap_err().to_string();

    // Verify the error message contains the original error - we don't need to check the exact
    // format as that might change, just confirm the key information is preserved
    assert!(error_message.contains("Value is invalid"));
}

#[test]
fn test_with_context_ok_result() {
    // Create a successful result
    let result: Result<i32> = Ok(42);

    // Add context (should not affect the Ok value)
    let with_context = result.with_context(|| "This context won't be used");

    // Check that the result is still Ok with the same value
    assert_eq!(with_context, Ok(42));
}

#[test]
fn test_error_context_creation() {
    // Create a new ErrorContext
    let context = ErrorContext::new("load_config");

    // Check that the operation name is set correctly
    assert_eq!(context.operation, "load_config");
    assert!(context.file_path.is_none());
    assert!(context.details.is_none());

    // Test the string formatting of the context
    let context_string = context.format();
    assert!(context_string.contains("load_config"));
}

#[test]
fn test_error_context_with_file() {
    // Create a new ErrorContext with a file path
    let context = ErrorContext::new("read_file").with_file(Path::new("/path/to/file.txt"));

    // Check that both operation and file path are set correctly
    assert_eq!(context.operation, "read_file");
    assert_eq!(
        context.file_path.as_ref().unwrap().to_str().unwrap(),
        "/path/to/file.txt"
    );
    assert!(context.details.is_none());

    // Test the string formatting of the context
    let context_string = context.format();
    assert!(context_string.contains("read_file"));
    assert!(context_string.contains("/path/to/file.txt"));
}

#[test]
fn test_error_context_with_details() {
    // Create a new ErrorContext with details
    let context =
        ErrorContext::new("validate_input").with_details("Input validation failed for username");

    // Check that both operation and details are set correctly
    assert_eq!(context.operation, "validate_input");
    assert!(context.file_path.is_none());
    assert_eq!(
        context.details.as_ref().unwrap(),
        "Input validation failed for username"
    );

    // Test the string formatting of the context
    let context_string = context.format();
    assert!(context_string.contains("validate_input"));
    assert!(context_string.contains("Input validation failed for username"));
}

#[test]
fn test_error_context_complete() {
    // Create a new ErrorContext with both file path and details
    let context = ErrorContext::new("save_config")
        .with_file(Path::new("/path/to/config.yaml"))
        .with_details("Could not save configuration due to permission issues");

    // Check that all fields are set correctly
    assert_eq!(context.operation, "save_config");
    assert_eq!(
        context.file_path.as_ref().unwrap().to_str().unwrap(),
        "/path/to/config.yaml"
    );
    assert_eq!(
        context.details.as_ref().unwrap(),
        "Could not save configuration due to permission issues"
    );

    // Test the string formatting of the context
    let context_string = context.format();
    assert!(context_string.contains("save_config"));
    assert!(context_string.contains("/path/to/config.yaml"));
    assert!(context_string.contains("Could not save configuration due to permission issues"));
}

#[test]
fn test_with_enhanced_context() {
    // Create an IO error
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied");
    let io_result: io::Result<()> = Err(io_err);

    // Add enhanced context
    let result = io_result.with_enhanced_context(|| {
        ErrorContext::new("write_file")
            .with_file(Path::new("/path/to/file.txt"))
            .with_details("Could not write to file due to permissions")
    });

    // Check that the result is an error
    assert!(result.is_err());

    // For PermissionDenied errors with a file path, the implementation creates a PermissionDenied error
    // with just the path, not the full context
    let error_message = result.unwrap_err().to_string();
    assert!(error_message.contains("/path/to/file.txt"));
}

#[test]
fn test_with_enhanced_context_ok_result() {
    // Create a successful IO result
    let io_result: io::Result<i32> = Ok(42);

    // Add enhanced context (should not affect the Ok value)
    let result = io_result.with_enhanced_context(|| {
        ErrorContext::new("read_value").with_details("This context won't be used")
    });

    // Check that the result is still Ok with the same value
    assert_eq!(result, Ok(42));
}
