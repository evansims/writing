//! Unit tests extracted from lib.rs

use content-template::*;
mod tests {
    use super::*;
    use common_test_utils::with_temp_dir;
    use std::fs;

    #[test]
    fn test_create_template_validation() {
        // Test empty name validation
        let result = create_template(CreateTemplateOptions {
            name: "".to_string(),
            content_type: "article".to_string(),
            content: None,
        });

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Template name cannot be empty"));

        // Test invalid content type
        let result = create_template(CreateTemplateOptions {
            name: "test-template".to_string(),
            content_type: "invalid".to_string(),
            content: None,
        });

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid content type"));
    }

    #[test]
    fn test_template_lifecycle() {
        with_temp_dir(|temp_dir| {
            // Create a mock config file
            let config_path = temp_dir.join("config.yaml");
            let config_content = format!(
                "content:\n  base_dir: {}\n  topics: {{}}\nimages:\n  formats: []\n  sizes: {{}}\npublication:\n  author: Test Author\n  copyright: Test Copyright",
                temp_dir.to_string_lossy()
            );
            fs::write(&config_path, config_content).unwrap();

            // Create templates directory
            let templates_dir = temp_dir.join("templates");
            fs::create_dir_all(&templates_dir).unwrap();

            // Set the config path in the environment
            std::env::set_var("CONFIG_PATH", &config_path);

            // Create a template
            let result = create_template(CreateTemplateOptions {
                name: "test-template".to_string(),
                content_type: "article".to_string(),
                content: Some("Test content".to_string()),
            });

            if result.is_err() {
                println!("Template creation error: {}", result.as_ref().unwrap_err());
            }
            assert!(result.is_ok());
            let template = result.unwrap();
            assert_eq!(template.name, "test-template");
            assert_eq!(template.content_type, "article");

            // Get the template
            let content = get_template("test-template").unwrap();
            assert_eq!(content, "Test content");

            // List templates
            let templates = list_templates().unwrap();
            assert_eq!(templates.len(), 1);
            assert_eq!(templates[0].name, "test-template");

            // Delete the template
            let result = delete_template("test-template");
            assert!(result.is_ok());

            // Verify it's gone
            let result = get_template("test-template");
            assert!(result.is_err());

            Ok::<(), anyhow::Error>(())
        })
        .unwrap();
    }
}
