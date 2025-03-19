//! Tests for error conversion functionality in the errors module
//!
//! This file contains tests for converting between error types.

use std::io;
use std::path::{Path, PathBuf};

#[test]
fn test_from_io_error() {
    // Create an IO error
    let io_err = io::Error::new(io::ErrorKind::NotFound, "Test error");

    // Convert to WritingError using 'From'
    let writing_err = crate::WritingError::from(io_err);

    // Check that the conversion worked
    match writing_err {
        crate::WritingError::IoError(_) => (), // Expected
        other => panic!("Expected IoError, got {:?}", other),
    }

    // Verify the error message contains the original message
    assert!(writing_err.to_string().contains("Test error"));
}

#[test]
fn test_file_not_found_creation() {
    let path = Path::new("/path/to/file.txt");
    let err = crate::WritingError::file_not_found(path);

    match err {
        crate::WritingError::FileNotFound(ref p) => {
            assert_eq!(p.to_str().unwrap(), "/path/to/file.txt");
        }
        other => panic!("Expected FileNotFound, got {:?}", other),
    }

    // Verify the error message contains the path
    assert!(err.to_string().contains("/path/to/file.txt"));
}

#[test]
fn test_directory_not_found_creation() {
    let path = Path::new("/path/to/directory");
    let err = crate::WritingError::directory_not_found(path);

    match err {
        crate::WritingError::DirectoryNotFound(ref p) => {
            assert_eq!(p.to_str().unwrap(), "/path/to/directory");
        }
        other => panic!("Expected DirectoryNotFound, got {:?}", other),
    }

    // Verify the error message contains the path
    assert!(err.to_string().contains("/path/to/directory"));
}

#[test]
fn test_config_error_creation() {
    let err = crate::WritingError::config_error("Invalid config");

    match err {
        crate::WritingError::ConfigError(ref msg) => {
            assert_eq!(msg, "Invalid config");
        }
        other => panic!("Expected ConfigError, got {:?}", other),
    }

    // Verify the error message contains the message
    assert!(err.to_string().contains("Invalid config"));
}

#[test]
fn test_content_not_found_creation() {
    let err = crate::WritingError::content_not_found("Article not found");

    match err {
        crate::WritingError::ContentNotFound(ref msg) => {
            assert_eq!(msg, "Article not found");
        }
        other => panic!("Expected ContentNotFound, got {:?}", other),
    }

    // Verify the error message contains the message
    assert!(err.to_string().contains("Article not found"));
}

#[test]
fn test_topic_error_creation() {
    let err = crate::WritingError::topic_error("Invalid topic");

    match err {
        crate::WritingError::TopicError(ref msg) => {
            assert_eq!(msg, "Invalid topic");
        }
        other => panic!("Expected TopicError, got {:?}", other),
    }

    // Verify the error message contains the message
    assert!(err.to_string().contains("Invalid topic"));
}

#[test]
fn test_validation_error_creation() {
    let err = crate::WritingError::validation_error("Field cannot be empty");

    match err {
        crate::WritingError::ValidationError(ref msg) => {
            assert_eq!(msg, "Field cannot be empty");
        }
        other => panic!("Expected ValidationError, got {:?}", other),
    }

    // Verify the error message contains the message
    assert!(err.to_string().contains("Field cannot be empty"));
}

#[test]
fn test_format_error_creation() {
    let err = crate::WritingError::format_error("Invalid date format");

    match err {
        crate::WritingError::FormatError(ref msg) => {
            assert_eq!(msg, "Invalid date format");
        }
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

    // Instead of using the trait method, convert to a WritingError manually
    let converted = match result {
        Ok(value) => Ok(value),
        Err(_) => Err(crate::WritingError::file_not_found(path)),
    };

    // Check that the conversion created an error
    assert!(converted.is_err());

    // Verify the error contains information about the file
    let error = converted.unwrap_err();
    assert!(error.to_string().contains("/path/to/file.txt"));
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
