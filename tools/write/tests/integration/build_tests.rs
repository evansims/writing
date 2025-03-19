//! Tests for build-related commands in the Write CLI
//!
//! This file contains integration tests for the build command to ensure it properly
//! coordinates with other tools and produces expected outputs.

use anyhow::Result;
use common_test_utils::integration::TestCommand;
use std::path::PathBuf;

/// Helper function to ensure test directories exist
fn ensure_test_dirs(command: &TestCommand) -> Result<()> {
    let content_dir = command.fixture.temp_dir.path().join("content").join("blog");
    if !content_dir.exists() {
        std::fs::create_dir_all(&content_dir)?;
    }

    let build_dir = command.fixture.temp_dir.path().join("build");
    if !build_dir.exists() {
        std::fs::create_dir_all(&build_dir)?;
    }

    Ok(())
}

/// Helper to create test content
fn create_test_content(command: &TestCommand) -> Result<()> {
    // Create a blog post
    command.assert_success(&[
        "content", "new",
        "--title", "Build Test Post",
        "--topic", "blog",
        "--description", "This is a test post for build testing",
        "--content", "# Build Test Post\n\nThis is some test content for the build process."
    ]);

    Ok(())
}

#[test]
fn test_build_content_workflow() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;
    create_test_content(&command)?;

    // Act - Run the build command
    let output = command.assert_success(&[
        "content", "build",
    ]);

    // Assert
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("build") || stdout.contains("built"),
            "Output should indicate content was built");

    // Verify build output files were created
    let build_dir = command.fixture.temp_dir.path().join("build");
    let built_files = std::fs::read_dir(build_dir)?
        .filter_map(|e| e.ok())
        .collect::<Vec<_>>();

    assert!(!built_files.is_empty(), "Build should have produced output files");

    Ok(())
}

#[test]
fn test_build_with_validation_workflow() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;
    create_test_content(&command)?;

    // Act - Run the build command with validation
    let output = command.assert_success(&[
        "content", "build",
        "--validate",
    ]);

    // Assert
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("validated") || stdout.contains("validation"),
            "Output should indicate validation was performed");
    assert!(stdout.contains("build") || stdout.contains("built"),
            "Output should indicate content was built");

    Ok(())
}

#[test]
fn test_build_with_specific_output_dir() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;
    create_test_content(&command)?;

    // Create a custom output directory
    let custom_output_dir = command.fixture.temp_dir.path().join("custom-output");
    std::fs::create_dir_all(&custom_output_dir)?;

    // Act - Run the build command with custom output directory
    let output = command.assert_success(&[
        "content", "build",
        "--output", custom_output_dir.to_str().unwrap(),
    ]);

    // Assert
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("build") || stdout.contains("built"),
            "Output should indicate content was built");

    // Verify build output files were created in the custom directory
    let built_files = std::fs::read_dir(custom_output_dir)?
        .filter_map(|e| e.ok())
        .collect::<Vec<_>>();

    assert!(!built_files.is_empty(), "Build should have produced output files in custom directory");

    Ok(())
}

#[test]
fn test_build_sitemap_workflow() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;
    create_test_content(&command)?;

    // Act - Run the build command
    command.assert_success(&[
        "content", "build",
    ]);

    // Assert - Check for sitemap.xml
    let sitemap_path = command.fixture.temp_dir.path().join("build").join("sitemap.xml");
    assert!(sitemap_path.exists(), "sitemap.xml should be created during build");

    // Verify sitemap contains content
    let sitemap_content = std::fs::read_to_string(sitemap_path)?;
    assert!(!sitemap_content.is_empty(), "Sitemap should not be empty");
    assert!(sitemap_content.contains("<?xml"), "Sitemap should be valid XML");

    Ok(())
}

#[test]
fn test_build_rss_feed_workflow() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;
    create_test_content(&command)?;

    // Act - Run the build command
    command.assert_success(&[
        "content", "build",
    ]);

    // Assert - Check for feed.xml
    let feed_path = command.fixture.temp_dir.path().join("build").join("feed.xml");
    assert!(feed_path.exists(), "feed.xml should be created during build");

    // Verify feed contains content
    let feed_content = std::fs::read_to_string(feed_path)?;
    assert!(!feed_content.is_empty(), "RSS feed should not be empty");
    assert!(feed_content.contains("<?xml"), "RSS feed should be valid XML");

    Ok(())
}

#[test]
fn test_build_llm_friendly_output_workflow() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;
    create_test_content(&command)?;

    // Act - Run the build command
    command.assert_success(&[
        "content", "build",
    ]);

    // Assert - Check for llms.txt
    let llms_path = command.fixture.temp_dir.path().join("build").join("llms.txt");
    assert!(llms_path.exists(), "llms.txt should be created during build");

    // Verify llms file contains content
    let llms_content = std::fs::read_to_string(llms_path)?;
    assert!(!llms_content.is_empty(), "LLM friendly text should not be empty");

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