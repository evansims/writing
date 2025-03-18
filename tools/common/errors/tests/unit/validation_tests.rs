//! Tests for validation extensions in the errors module
//!
//! This file contains tests for option validation extensions.

use crate::{WritingError, Result, OptionValidationExt, OptionExt};

#[test]
fn test_validate_required_with_some() {
    // Create an Option with a value
    let option: Option<String> = Some("test value".to_string());
    
    // Validate that it has a value
    let result = option.validate_required("Value is required");
    
    // Check that the result contains the expected value
    assert_eq!(result, Ok("test value".to_string()));
}

#[test]
fn test_validate_required_with_none() {
    // Create an Option without a value
    let option: Option<String> = None;
    
    // Validate that it has a value
    let result = option.validate_required("Value is required");
    
    // Check that the result is an error with the expected message
    match result {
        Err(WritingError::ValidationError(msg)) => {
            assert_eq!(msg, "Value is required");
        },
        other => panic!("Expected ValidationError, got {:?}", other),
    }
}

#[test]
fn test_validate_with_custom_error_some() {
    // Create an Option with a value
    let option: Option<String> = Some("test value".to_string());
    
    // Validate with a custom error function
    let result = option.validate_with(|| WritingError::content_not_found("Content not found"));
    
    // Check that the result contains the expected value
    assert_eq!(result, Ok("test value".to_string()));
}

#[test]
fn test_validate_with_custom_error_none() {
    // Create an Option without a value
    let option: Option<String> = None;
    
    // Validate with a custom error function
    let result = option.validate_with(|| WritingError::content_not_found("Content not found"));
    
    // Check that the result is the custom error
    match result {
        Err(WritingError::ContentNotFound(msg)) => {
            assert_eq!(msg, "Content not found");
        },
        other => panic!("Expected ContentNotFound, got {:?}", other),
    }
}

#[test]
fn test_content_not_found_extension_some() {
    // Create an Option with a value
    let option: Option<String> = Some("test value".to_string());
    
    // Use the content_not_found extension
    let result = option.content_not_found("Content not found");
    
    // Check that the result contains the expected value
    assert_eq!(result, Ok("test value".to_string()));
}

#[test]
fn test_content_not_found_extension_none() {
    // Create an Option without a value
    let option: Option<String> = None;
    
    // Use the content_not_found extension
    let result = option.content_not_found("Article not found");
    
    // Check that the result is a ContentNotFound error
    match result {
        Err(WritingError::ContentNotFound(msg)) => {
            assert_eq!(msg, "Article not found");
        },
        other => panic!("Expected ContentNotFound, got {:?}", other),
    }
}

#[test]
fn test_or_validation_error_extension_some() {
    // Create an Option with a value
    let option: Option<String> = Some("test value".to_string());
    
    // Use the or_validation_error extension
    let result = option.or_validation_error("Field is required");
    
    // Check that the result contains the expected value
    assert_eq!(result, Ok("test value".to_string()));
}

#[test]
fn test_or_validation_error_extension_none() {
    // Create an Option without a value
    let option: Option<String> = None;
    
    // Use the or_validation_error extension
    let result = option.or_validation_error("Field is required");
    
    // Check that the result is a ValidationError
    match result {
        Err(WritingError::ValidationError(msg)) => {
            assert_eq!(msg, "Field is required");
        },
        other => panic!("Expected ValidationError, got {:?}", other),
    }
}

#[test]
fn test_chained_validation() {
    // Create a function that uses multiple validations
    fn process_input(name: Option<String>, age: Option<i32>) -> Result<String> {
        let name = name.validate_required("Name is required")?;
        let age = age.validate_required("Age is required")?;
        
        if age < 18 {
            return Err(WritingError::validation_error("Age must be at least 18"));
        }
        
        Ok(format!("Processed: {} ({})", name, age))
    }
    
    // Test with valid inputs
    let result = process_input(Some("John".to_string()), Some(25));
    assert_eq!(result, Ok("Processed: John (25)".to_string()));
    
    // Test with missing name
    let result = process_input(None, Some(25));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Name is required"));
    
    // Test with missing age
    let result = process_input(Some("John".to_string()), None);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Age is required"));
    
    // Test with invalid age
    let result = process_input(Some("John".to_string()), Some(15));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Age must be at least 18"));
} 