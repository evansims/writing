//! Tests for configuration passing between tools in the Write CLI
//!
//! This file contains integration tests that ensure configuration is correctly
//! passed between tools.

use anyhow::Result;
use common_test_utils::integration::TestCommand;
use std::path::PathBuf;
use std::io::Write;
use std::process::Command;

/// Helper function to create a custom config file
fn create_custom_config(command: &TestCommand, config_content: &str) -> Result<PathBuf> {
    let config_path = command.fixture.temp_dir.path().join("config.yaml");
    let mut file = std::fs::File::create(&config_path)?;
    file.write_all(config_content.as_bytes())?;
    Ok(config_path)
}

#[test]
fn test_topic_config_affects_content_creation() -> Result<()> {
    // Arrange - Create the command and test environment with custom config
    let command = TestCommand::new("write")?;

    // Custom config with template setting for the blog topic
    let config_content = r#"
content:
  base_dir: content
  topics:
    blog:
      name: Blog
      description: Blog posts
      directory: blog
      template: custom-template.md
images:
  formats: ["jpg"]
publication:
  author: Test Author
  copyright: Test Copyright
  site: https://example.com
"#;

    let config_path = create_custom_config(&command, config_content)?;

    // Create a custom template in the expected location
    let templates_dir = command.fixture.temp_dir.path().join("templates");
    std::fs::create_dir_all(&templates_dir)?;

    let template_content = r#"---
title: "{{ title }}"
description: "{{ description }}"
date: "{{ date }}"
custom_field: "This is from the custom template"
---

# {{ title }}

{{ description }}
"#;

    let template_path = templates_dir.join("custom-template.md");
    std::fs::write(&template_path, template_content)?;

    // Create content dir
    let content_dir = command.fixture.temp_dir.path().join("content").join("blog");
    std::fs::create_dir_all(&content_dir)?;

    // Act - Create new content using the config with custom template
    let output = command.assert_success(&[
        "content", "new",
        "--title", "Template Test Post",
        "--topic", "blog",
        "--description", "This is a test of template configuration"
    ]);

    // Assert
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("created"), "Output should indicate content was created");

    // Check that the content file was created with the custom template
    let created_files = std::fs::read_dir(content_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect::<Vec<_>>();

    let file_path = created_files.first().expect("No content file found");
    let content = std::fs::read_to_string(file_path)?;

    assert!(content.contains("This is from the custom template"),
            "Content should be created using the custom template from config");

    Ok(())
}

#[test]
fn test_build_config_affects_output() -> Result<()> {
    // Arrange - Create the command and test environment with custom config
    let command = TestCommand::new("write")?;

    // Custom config with build settings
    let config_content = r#"
content:
  base_dir: content
  topics:
    blog:
      name: Blog
      description: Blog posts
      directory: blog
build:
  output_dir: custom-build-output
  site_url: https://example.com/custom-path
  title: Custom Site Title
  description: Custom site description
images:
  formats: ["jpg"]
publication:
  author: Test Author
  copyright: Test Copyright
  site: https://example.com
"#;

    let config_path = create_custom_config(&command, config_content)?;

    // Create content dir and post
    let content_dir = command.fixture.temp_dir.path().join("content").join("blog");
    std::fs::create_dir_all(&content_dir)?;

    // Create a post
    command.assert_success(&[
        "content", "new",
        "--title", "Config Build Test",
        "--topic", "blog",
        "--description", "Testing build configuration"
    ]);

    // Act - Build content
    let output = command.assert_success(&[
        "content", "build",
    ]);

    // Assert
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("build") || stdout.contains("built"),
            "Output should indicate content was built");

    // Check that the build used the custom output directory
    let custom_build_dir = command.fixture.temp_dir.path().join("custom-build-output");
    assert!(custom_build_dir.exists(), "Custom build directory should exist");

    // Check that the sitemap has the custom URL
    let sitemap_path = custom_build_dir.join("sitemap.xml");
    if sitemap_path.exists() {
        let sitemap_content = std::fs::read_to_string(sitemap_path)?;
        assert!(sitemap_content.contains("https://example.com/custom-path"),
                "Sitemap should contain the custom site URL from config");
    }

    Ok(())
}

#[test]
fn test_image_config_affects_processing() -> Result<()> {
    // Arrange - Create the command and test environment with custom config
    let command = TestCommand::new("write")?;

    // Custom config with image settings
    let config_content = r#"
content:
  base_dir: content
  topics:
    blog:
      name: Blog
      description: Blog posts
      directory: blog
images:
  base_dir: custom-images
  src_dir: custom-src
  build_dir: custom-build
  formats: ["webp", "avif"]
  sizes: [100, 200, 300]
publication:
  author: Test Author
  copyright: Test Copyright
  site: https://example.com
"#;

    let config_path = create_custom_config(&command, config_content)?;

    // Create custom image directories
    let images_base_dir = command.fixture.temp_dir.path().join("custom-images");
    let images_src_dir = images_base_dir.join("custom-src");
    std::fs::create_dir_all(&images_src_dir)?;

    // Create a simple test image
    let image_path = images_src_dir.join("test-image.png");

    // Simple 1x1 PNG content
    let png_data: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A,
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53,
        0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41,
        0x54, 0x08, 0xD7, 0x63, 0xF8, 0xCF, 0xC0, 0x00,
        0x00, 0x03, 0x01, 0x01, 0x00, 0x18, 0xDD, 0x8D,
        0xB0, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E,
        0x44, 0xAE, 0x42, 0x60, 0x82
    ];

    let mut file = std::fs::File::create(&image_path)?;
    file.write_all(png_data)?;

    // Act - Build images
    let output = command.assert_success(&[
        "image", "build",
    ]);

    // Assert
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("build") || stdout.contains("built") || stdout.contains("processed"),
            "Output should indicate images were built/processed");

    // Check that the custom build directory was used
    let custom_build_dir = images_base_dir.join("custom-build");
    assert!(custom_build_dir.exists(), "Custom image build directory should exist");

    Ok(())
}

#[test]
fn test_config_modifications_affect_tools() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;

    // Initial config
    let initial_config = r#"
content:
  base_dir: content
  topics:
    blog:
      name: Blog
      description: Blog posts
      directory: blog
images:
  formats: ["jpg"]
publication:
  author: Initial Author
  copyright: Initial Copyright
  site: https://example.com
"#;

    let config_path = create_custom_config(&command, initial_config)?;

    // Create content dir
    let content_dir = command.fixture.temp_dir.path().join("content").join("blog");
    std::fs::create_dir_all(&content_dir)?;

    // Create content with initial config
    command.assert_success(&[
        "content", "new",
        "--title", "Initial Config Post",
        "--topic", "blog",
        "--description", "This is created with the initial config"
    ]);

    // Now modify the config to change the author
    let modified_config = r#"
content:
  base_dir: content
  topics:
    blog:
      name: Blog
      description: Blog posts
      directory: blog
images:
  formats: ["jpg"]
publication:
  author: Modified Author
  copyright: Modified Copyright
  site: https://example.com
"#;

    std::fs::write(config_path, modified_config)?;

    // Act - Create new content after config modification
    command.assert_success(&[
        "content", "new",
        "--title", "Modified Config Post",
        "--topic", "blog",
        "--description", "This is created with the modified config"
    ]);

    // Get all content files
    let content_files = std::fs::read_dir(content_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect::<Vec<_>>();

    // Need to find the newest file, which should be the one created with modified config
    let newest_file = content_files.iter()
        .filter_map(|path| {
            path.metadata().ok().map(|meta| (path, meta.modified().ok()))
        })
        .filter_map(|(path, modified)| modified.map(|m| (path, m)))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(path, _)| path);

    if let Some(newest_path) = newest_file {
        // Assert
        let content = std::fs::read_to_string(newest_path)?;
        assert!(content.contains("Modified Author"),
                "Content created after config modification should use the new author");
    } else {
        panic!("Could not find the newest content file");
    }

    Ok(())
}

#[test]
fn test_environment_variables_override_config() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;

    // Base config
    let config_content = r#"
content:
  base_dir: content
  topics:
    blog:
      name: Blog
      description: Blog posts
      directory: blog
images:
  formats: ["jpg"]
publication:
  author: Config Author
  copyright: Config Copyright
  site: https://example.com
"#;

    let config_path = create_custom_config(&command, config_content)?;

    // Create content dir
    let content_dir = command.fixture.temp_dir.path().join("content").join("blog");
    std::fs::create_dir_all(&content_dir)?;

    // Act - Create content with environment variable override for author
    // Note: We're using let binding because we need to drop everything before asserting
    let stdout = {
        let output = Command::new(&command.path)
            .args(&[
                "content", "new",
                "--title", "Env Var Test Post",
                "--topic", "blog",
                "--description", "Testing environment variable overrides"
            ])
            .current_dir(command.fixture.temp_dir.path())
            .env("CONFIG_PATH", config_path)
            .env("WRITING_PUBLICATION_AUTHOR", "Environment Author")
            .output()?;

        String::from_utf8_lossy(&output.stdout).to_string()
    };

    // Assert
    assert!(stdout.contains("created"), "Output should indicate content was created");

    // Check the content to see if it has the environment variable author
    let content_files = std::fs::read_dir(content_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect::<Vec<_>>();

    if let Some(file_path) = content_files.first() {
        let content = std::fs::read_to_string(file_path)?;
        assert!(content.contains("Environment Author"),
                "Content should use the author from environment variable override");
    } else {
        panic!("Could not find created content file");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tempfile::NamedTempFile;

    // A simple placeholder test that doesn't rely on any missing functions
    #[test]
    fn test_placeholder() -> Result<()> {
        // This is a placeholder test to ensure the integration tests compile
        // The actual tests will be implemented once the TestCommand is properly available
        let _output_file = NamedTempFile::new()?;
        Ok(())
    }
}