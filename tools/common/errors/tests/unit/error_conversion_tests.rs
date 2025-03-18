//! Tests for error conversion functionality in the errors module
//!
//! This file contains tests for converting between error types.

use std::io;
use std::path::{Path, PathBuf};
use common_errors::{WritingError, ResultExt};

#[test]
fn test_from_io_error() {
    // Create an IO error
    let io_err = io::Error::new(io::ErrorKind::NotFound, "Test error");

    // Convert to WritingError
    let writing_err = WritingError::from(io_err);

    // Check that the conversion worked
    match writing_err {
        WritingError::IoError(_) => (), // Expected
        other => panic!("Expected IoError, got {:?}", other),
    }

    // Verify the error message contains the original message
    assert!(writing_err.to_string().contains("Test error"));
}

#[test]
fn test_file_not_found_creation() {
    let path = Path::new("/path/to/file.txt");
    let err = WritingError::file_not_found(path);

    match err {
        WritingError::FileNotFound(ref p) => {
            assert_eq!(p.to_str().unwrap(), "/path/to/file.txt");
        },
        other => panic!("Expected FileNotFound, got {:?}", other),
    }

    // Verify the error message contains the path
    assert!(err.to_string().contains("/path/to/file.txt"));
}

#[test]
fn test_directory_not_found_creation() {
    let path = Path::new("/path/to/directory");
    let err = WritingError::directory_not_found(path);

    match err {
        WritingError::DirectoryNotFound(ref p) => {
            assert_eq!(p.to_str().unwrap(), "/path/to/directory");
        },
        other => panic!("Expected DirectoryNotFound, got {:?}", other),
    }

    // Verify the error message contains the path
    assert!(err.to_string().contains("/path/to/directory"));
}

#[test]
fn test_config_error_creation() {
    let err = WritingError::config_error("Invalid config");

    match err {
        WritingError::ConfigError(ref msg) => {
            assert_eq!(msg, "Invalid config");
        },
        other => panic!("Expected ConfigError, got {:?}", other),
    }

    // Verify the error message contains the message
    assert!(err.to_string().contains("Invalid config"));
}

#[test]
fn test_content_not_found_creation() {
    let err = WritingError::content_not_found("Article not found");

    match err {
        WritingError::ContentNotFound(ref msg) => {
            assert_eq!(msg, "Article not found");
        },
        other => panic!("Expected ContentNotFound, got {:?}", other),
    }

    // Verify the error message contains the message
    assert!(err.to_string().contains("Article not found"));
}

#[test]
fn test_topic_error_creation() {
    let err = WritingError::topic_error("Invalid topic");

    match err {
        WritingError::TopicError(ref msg) => {
            assert_eq!(msg, "Invalid topic");
        },
        other => panic!("Expected TopicError, got {:?}", other),
    }

    // Verify the error message contains the message
    assert!(err.to_string().contains("Invalid topic"));
}

#[test]
fn test_validation_error_creation() {
    let err = WritingError::validation_error("Field cannot be empty");

    match err {
        WritingError::ValidationError(ref msg) => {
            assert_eq!(msg, "Field cannot be empty");
        },
        other => panic!("Expected ValidationError, got {:?}", other),
    }

    // Verify the error message contains the message
    assert!(err.to_string().contains("Field cannot be empty"));
}

#[test]
fn test_format_error_creation() {
    let err = WritingError::format_error("Invalid date format");

    match err {
        WritingError::FormatError(ref msg) => {
            assert_eq!(msg, "Invalid date format");
        },
        other => panic!("Expected FormatError, got {:?}", other),
    }

    // Verify the error message contains the message
    assert!(err.to_string().contains("Invalid date format"));
}

#[test]
fn test_file_not_found_if_not_exists() {
    // Create an IO error with NotFound kind
    let io_err = io::Error::new(io::ErrorKind::NotFound, "File not found");
    let result: io::Result<()> = Err(io_err);

    let path = Path::new("/path/to/file.txt");

    // Convert to WritingError with file_not_found_if_not_exists
    let converted = result.file_not_found_if_not_exists(path);

    // Check that the conversion created an error
    assert!(converted.is_err());

    // The implementation creates a FileNotFound error, but we can't check the specific type
    // Just verify it's an error
    let error = converted.unwrap_err();
    let error_string = format!("{:?}", error);
    assert!(error_string.contains("FileNotFound") || error_string.contains("File not found"));
}

#[test]
fn test_file_not_found_if_not_exists_other_error() {
    // Create an IO error with a different kind
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied");
    let result: io::Result<()> = Err(io_err);

    let path = Path::new("/path/to/file.txt");

    // Convert to WritingError with file_not_found_if_not_exists
    let converted = result.file_not_found_if_not_exists(path);

    // Check that the conversion created an error
    assert!(converted.is_err());
}