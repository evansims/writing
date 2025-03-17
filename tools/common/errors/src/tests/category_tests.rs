//! # Error Category Tests
//! 
//! This module contains tests for the error category system.

use crate::{WritingError, ErrorCategory};
use std::path::Path;

#[test]
fn test_error_category_from_error() {
    // Test mapping from error to category
    let config_error = WritingError::config_error("Config error");
    assert_eq!(ErrorCategory::from(&config_error), ErrorCategory::Configuration);
    
    let not_found_error = WritingError::content_not_found("Content not found");
    assert_eq!(ErrorCategory::from(&not_found_error), ErrorCategory::NotFound);
    
    let file_not_found = WritingError::file_not_found("test.txt");
    assert_eq!(ErrorCategory::from(&file_not_found), ErrorCategory::NotFound);
    
    let validation_error = WritingError::validation_error("Validation error");
    assert_eq!(ErrorCategory::from(&validation_error), ErrorCategory::Validation);
    
    let permission_error = WritingError::permission_denied("private.txt");
    assert_eq!(ErrorCategory::from(&permission_error), ErrorCategory::Permission);
    
    let format_error = WritingError::format_error("Format error");
    assert_eq!(ErrorCategory::from(&format_error), ErrorCategory::Format);
}

#[test]
fn test_user_message() {
    // Test user messages for each category
    assert!(!ErrorCategory::Configuration.user_message().is_empty());
    assert!(!ErrorCategory::Validation.user_message().is_empty());
    assert!(!ErrorCategory::NotFound.user_message().is_empty());
    assert!(!ErrorCategory::Permission.user_message().is_empty());
    assert!(!ErrorCategory::Format.user_message().is_empty());
    assert!(!ErrorCategory::Io.user_message().is_empty());
    assert!(!ErrorCategory::Command.user_message().is_empty());
    assert!(!ErrorCategory::Template.user_message().is_empty());
    assert!(!ErrorCategory::Parsing.user_message().is_empty());
    assert!(!ErrorCategory::Unexpected.user_message().is_empty());
}

#[test]
fn test_user_suggestion() {
    // Test user suggestions for each category
    assert!(!ErrorCategory::Configuration.user_suggestion().is_empty());
    assert!(!ErrorCategory::Validation.user_suggestion().is_empty());
    assert!(!ErrorCategory::NotFound.user_suggestion().is_empty());
    assert!(!ErrorCategory::Permission.user_suggestion().is_empty());
    assert!(!ErrorCategory::Format.user_suggestion().is_empty());
    assert!(!ErrorCategory::Io.user_suggestion().is_empty());
    assert!(!ErrorCategory::Command.user_suggestion().is_empty());
    assert!(!ErrorCategory::Template.user_suggestion().is_empty());
    assert!(!ErrorCategory::Parsing.user_suggestion().is_empty());
    assert!(!ErrorCategory::Unexpected.user_suggestion().is_empty());
} 