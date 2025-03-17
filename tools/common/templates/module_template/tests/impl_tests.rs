//! Tests for the template module implementation.
//!
//! This file contains unit tests for the implementation details of the template module.

use super::super::*;

#[test]
fn test_create_template_with_valid_name() {
    let result = create_template("test-template", models::TemplateConfig::empty());
    
    assert!(result.is_ok());
    let template = result.unwrap();
    assert_eq!(template.name, "test-template");
    assert_eq!(template.path.to_str().unwrap(), "templates/test-template");
}

#[test]
fn test_create_template_with_empty_name() {
    let result = create_template("", models::TemplateConfig::empty());
    
    assert!(result.is_err());
    match result.unwrap_err() {
        errors::TemplateError::InvalidName(_) => (), // Expected error
        err => panic!("Unexpected error: {:?}", err),
    }
}

#[test]
fn test_get_template_existing() {
    let result = get_template("example-1");
    
    assert!(result.is_ok());
    let template = result.unwrap();
    assert_eq!(template.name, "example-1");
}

#[test]
fn test_get_template_non_existent() {
    let result = get_template("non-existent-template");
    
    assert!(result.is_err());
    match result.unwrap_err() {
        errors::TemplateError::NotFound(name) => assert_eq!(name, "non-existent-template"),
        err => panic!("Unexpected error: {:?}", err),
    }
}

#[test]
fn test_list_templates() {
    let result = list_templates();
    
    assert!(result.is_ok());
    let templates = result.unwrap();
    assert_eq!(templates.len(), 2);
    assert_eq!(templates[0].name, "example-1");
    assert_eq!(templates[1].name, "example-2");
} 