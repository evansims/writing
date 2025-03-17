//! # Error Reporting Tests
//! 
//! This module contains tests for the error reporting system.

use crate::{WritingError, ErrorReporter, ErrorDisplayStyle};
use std::path::Path;

#[test]
fn test_simple_error_formatting() {
    let error = WritingError::file_not_found("test.txt");
    let reporter = ErrorReporter::new();
    
    let result = reporter.format_error(&error, ErrorDisplayStyle::Simple);
    
    assert!(result.contains("File not found"));
    assert!(result.contains("test.txt"));
    
    // Simple style shouldn't have category or suggestion
    assert!(!result.contains("Category"));
    assert!(!result.contains("Suggestion"));
}

#[test]
fn test_detailed_error_formatting() {
    let error = WritingError::validation_error("Invalid input");
    let reporter = ErrorReporter::new();
    
    let result = reporter.format_error(&error, ErrorDisplayStyle::Detailed);
    
    assert!(result.contains("Error"));
    assert!(result.contains("Invalid input"));
    assert!(result.contains("Category"));
    assert!(result.contains("Validation"));
    assert!(result.contains("Suggestion"));
}

#[test]
fn test_debug_error_formatting() {
    let error = WritingError::content_not_found("article-123");
    let reporter = ErrorReporter::new();
    
    let result = reporter.format_error(&error, ErrorDisplayStyle::Debug);
    
    assert!(result.contains("Error"));
    assert!(result.contains("Content not found"));
    assert!(result.contains("article-123"));
    assert!(result.contains("Category"));
    assert!(result.contains("NotFound"));
    assert!(result.contains("Suggestion"));
    assert!(result.contains("Debug"));
}

#[test]
fn test_reporter_with_custom_settings() {
    // Reporter with suggestions disabled
    let reporter = ErrorReporter::with_settings(true, false, false);
    let error = WritingError::content_not_found("article-123");
    
    let result = reporter.format_error(&error, ErrorDisplayStyle::Detailed);
    
    assert!(result.contains("Content not found"));
    assert!(result.contains("Category"));
    assert!(!result.contains("Suggestion"));
}

#[test]
fn test_different_error_types_categorization() {
    let errors = vec![
        WritingError::config_error("Config error"),
        WritingError::content_not_found("Content not found"),
        WritingError::file_not_found("test.txt"),
        WritingError::validation_error("Validation error"),
        WritingError::permission_denied("private.txt"),
        WritingError::format_error("Format error"),
    ];
    
    let reporter = ErrorReporter::new();
    
    for error in errors {
        let result = reporter.format_error(&error, ErrorDisplayStyle::Detailed);
        
        // All detailed errors should have these components
        assert!(result.contains("Error"));
        assert!(result.contains("Category"));
        assert!(result.contains("Suggestion"));
    }
}

#[test]
fn test_default_reporter() {
    let reporter = crate::get_default_reporter();
    assert!(reporter.show_suggestions);
    assert!(reporter.show_error_codes);
    assert!(!reporter.show_debug);
} 