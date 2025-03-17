//! Integration tests for the template module.
//!
//! This file contains integration tests that test the module as a whole.

use anyhow::Result;
use std::path::Path;
use tempfile::TempDir;

use super::super::*;

/// A test fixture for integration tests.
struct TestFixture {
    /// The temporary directory
    _temp_dir: TempDir,
    /// The path to the templates directory
    templates_dir: std::path::PathBuf,
}

impl TestFixture {
    /// Creates a new test fixture.
    fn new() -> Result<Self> {
        let temp_dir = tempfile::tempdir()?;
        let templates_dir = temp_dir.path().join("templates");
        std::fs::create_dir_all(&templates_dir)?;
        
        Ok(Self {
            _temp_dir: temp_dir,
            templates_dir,
        })
    }
    
    /// Creates a test template.
    fn create_test_template(&self, name: &str, description: Option<&str>) -> Result<()> {
        let template_dir = self.templates_dir.join(name);
        std::fs::create_dir_all(&template_dir)?;
        
        let config = models::TemplateConfig {
            description: description.map(|s| s.to_string()),
        };
        
        // In a real implementation, this would write the config to a file
        let _config = config;
        
        Ok(())
    }
}

#[test]
fn test_template_workflow() -> Result<()> {
    let fixture = TestFixture::new()?;
    
    // Create a template
    let config = models::TemplateConfig::new(Some("Test template".to_string()));
    let template = create_template("integration-test", config.clone())?;
    
    // Verify the template was created
    assert_eq!(template.name, "integration-test");
    assert_eq!(template.config.description, Some("Test template".to_string()));
    
    // Pre-create a template for testing updates
    fixture.create_test_template("existing-template", Some("Original description"))?;
    
    // Update an existing template
    let updated_config = models::TemplateConfig::new(Some("Updated description".to_string()));
    let updated = update_template("existing-template", updated_config)?;
    
    // Verify the update
    assert_eq!(updated.name, "existing-template");
    assert_eq!(updated.config.description, Some("Updated description".to_string()));
    
    // Test deleting a template
    let result = delete_template("existing-template");
    assert!(result.is_ok());
    
    // Verify the template was deleted
    let result = get_template("existing-template");
    assert!(result.is_err());
    
    Ok(())
} 