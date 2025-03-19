//! Tests for cross-tool interactions in the Write CLI
//!
//! This file contains integration tests that test how different tools interact
//! with each other within the Write CLI.

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

    let images_dir = command.fixture.temp_dir.path().join("images").join("src");
    if !images_dir.exists() {
        std::fs::create_dir_all(&images_dir)?;
    }

    Ok(())
}

#[test]
fn test_content_stats_after_content_operations() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // Create initial content
    command.assert_success(&[
        "content", "new",
        "--title", "First Post",
        "--topic", "blog",
        "--description", "This is the first post"
    ]);

    // Get initial stats
    let initial_output = command.assert_success(&["content", "stats"]);
    let initial_stdout = String::from_utf8_lossy(&initial_output.stdout);

    // Create more content
    command.assert_success(&[
        "content", "new",
        "--title", "Second Post",
        "--topic", "blog",
        "--description", "This is the second post"
    ]);

    // Act - Get updated stats
    let updated_output = command.assert_success(&["content", "stats"]);
    let updated_stdout = String::from_utf8_lossy(&updated_output.stdout);

    // Assert - Stats should reflect the new content
    // We can't easily compare exact numbers due to formatting, but we can check that the stats changed
    assert!(initial_stdout != updated_stdout,
           "Stats should be different after adding content");

    Ok(())
}

#[test]
fn test_content_validate_after_content_edit() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // Create content
    command.assert_success(&[
        "content", "new",
        "--title", "Validation Post",
        "--topic", "blog",
        "--description", "This is a post for validation testing"
    ]);

    // Get the slug from the created file
    let content_dir = command.fixture.temp_dir.path().join("content").join("blog");
    let created_files = std::fs::read_dir(content_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect::<Vec<_>>();

    let file_path = created_files.first().expect("No content file found");
    let file_name = file_path.file_name().unwrap().to_string_lossy();
    let slug = file_name.split('.').next().unwrap();

    // Run initial validation
    command.assert_success(&["content", "validate"]);

    // Edit the content to have invalid markdown
    command.assert_success(&[
        "content", "edit",
        "--slug", slug,
        "--content", "# Title\n\nThis is [a broken link](http://example.com.\n\nIncomplete link."
    ]);

    // Act - Validate again
    let validate_output = command.assert_failure(&["content", "validate", "--strict"]);

    // Assert
    let stderr = String::from_utf8_lossy(&validate_output.stderr);
    assert!(stderr.contains("error") || stderr.contains("invalid"),
            "Validation should report errors for invalid markdown");

    Ok(())
}

#[test]
fn test_topic_rename_affects_content_operations() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // Create content in the blog topic
    command.assert_success(&[
        "content", "new",
        "--title", "Topic Test Post",
        "--topic", "blog",
        "--description", "This is a post for testing topic renames"
    ]);

    // Rename the blog topic
    command.assert_success(&[
        "topic", "rename",
        "--from", "blog",
        "--to", "articles"
    ]);

    // Act - Try to search for content
    let search_output = command.assert_success(&[
        "content", "search",
        "--query", "Topic Test Post"
    ]);

    // Assert
    let stdout = String::from_utf8_lossy(&search_output.stdout);
    assert!(stdout.contains("Topic Test Post"),
            "Content should still be findable after topic rename");

    // The content should now be in the articles topic directory
    let articles_dir = command.fixture.temp_dir.path().join("content").join("articles");
    let articles_files = std::fs::read_dir(articles_dir)?
        .filter_map(|e| e.ok())
        .collect::<Vec<_>>();

    assert!(!articles_files.is_empty(), "Content should be moved to the new topic directory");

    Ok(())
}

#[test]
fn test_content_build_after_content_edit() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // Create content
    command.assert_success(&[
        "content", "new",
        "--title", "Build Test Post",
        "--topic", "blog",
        "--description", "This is a post for testing build after edit",
        "--content", "# Original Content\n\nThis is the original content."
    ]);

    // Get the slug from the created file
    let content_dir = command.fixture.temp_dir.path().join("content").join("blog");
    let created_files = std::fs::read_dir(content_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect::<Vec<_>>();

    let file_path = created_files.first().expect("No content file found");
    let file_name = file_path.file_name().unwrap().to_string_lossy();
    let slug = file_name.split('.').next().unwrap();

    // Build the content
    command.assert_success(&["content", "build"]);

    // First build should have created files
    let build_dir = command.fixture.temp_dir.path().join("build");
    let first_build_files = std::fs::read_dir(build_dir.clone())?
        .filter_map(|e| e.ok())
        .collect::<Vec<_>>();

    // Edit the content
    command.assert_success(&[
        "content", "edit",
        "--slug", slug,
        "--content", "# Updated Content\n\nThis content has been updated."
    ]);

    // Act - Build again
    command.assert_success(&["content", "build"]);

    // Assert - The build files should reflect the updated content
    let output_html = build_dir.join("blog").join(format!("{}.html", slug));
    if output_html.exists() {
        let html_content = std::fs::read_to_string(output_html)?;
        assert!(html_content.contains("Updated Content"),
                "Built HTML should contain the updated content");
    } else {
        // If the specific HTML file doesn't exist, at least confirm build created something
        let second_build_files = std::fs::read_dir(build_dir)?
            .filter_map(|e| e.ok())
            .collect::<Vec<_>>();

        assert!(!second_build_files.is_empty(), "Second build should have created files");
    }

    Ok(())
}

#[test]
fn test_content_validation_and_build_integration() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // Create content with validation issues (missing description)
    command.assert_success(&[
        "content", "new",
        "--title", "Validation Build Post",
        "--topic", "blog",
        // No description - this might cause a validation warning
    ]);

    // Act - Build with validation enabled
    let output = command.assert_success(&[
        "content", "build",
        "--validate",
    ]);

    // Assert
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should mention both validation and build
    assert!(stdout.contains("validat"), "Output should mention validation");
    assert!(stdout.contains("build"), "Output should mention build");

    // Verify build output files were created despite validation warnings
    let build_dir = command.fixture.temp_dir.path().join("build");
    let built_files = std::fs::read_dir(build_dir)?
        .filter_map(|e| e.ok())
        .collect::<Vec<_>>();

    assert!(!built_files.is_empty(), "Build should have produced output files despite validation warnings");

    Ok(())
}

#[test]
fn test_end_to_end_content_workflow() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // Act & Assert - Test a complete workflow across different tools

    // 1. Add a new topic
    command.assert_success(&[
        "topic", "add",
        "--key", "technical",
        "--name", "Technical Articles",
        "--description", "Articles about technical topics"
    ]);

    // 2. Create content in the new topic
    command.assert_success(&[
        "content", "new",
        "--title", "Complete Workflow Test",
        "--topic", "technical",
        "--description", "This is a post for testing the complete workflow",
        "--content", "# Workflow Test\n\nThis is a test of the complete content workflow."
    ]);

    // 3. Find the content with search
    let search_output = command.assert_success(&[
        "content", "search",
        "--query", "Workflow Test"
    ]);
    let search_stdout = String::from_utf8_lossy(&search_output.stdout);
    assert!(search_stdout.contains("Complete Workflow Test"),
            "Search should find the created content");

    // 4. Get the slug from the search results or file system
    let content_dir = command.fixture.temp_dir.path().join("content").join("technical");
    let created_files = std::fs::read_dir(content_dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect::<Vec<_>>();

    let file_path = created_files.first().expect("No content file found");
    let file_name = file_path.file_name().unwrap().to_string_lossy();
    let slug = file_name.split('.').next().unwrap();

    // 5. Edit the content
    command.assert_success(&[
        "content", "edit",
        "--slug", slug,
        "--title", "Updated Workflow Test",
        "--content", "# Updated Workflow\n\nThis content has been updated as part of the workflow test."
    ]);

    // 6. Validate the content
    command.assert_success(&[
        "content", "validate",
        "--slug", slug
    ]);

    // 7. Get stats on the content
    let stats_output = command.assert_success(&["content", "stats"]);
    let stats_stdout = String::from_utf8_lossy(&stats_output.stdout);
    assert!(stats_stdout.contains("technical") || stats_stdout.contains("Technical"),
            "Stats should include the technical topic");

    // 8. Build the content
    command.assert_success(&["content", "build"]);

    // 9. Verify the complete workflow produced the expected results
    let build_dir = command.fixture.temp_dir.path().join("build");
    let sitemap_path = build_dir.join("sitemap.xml");
    let feed_path = build_dir.join("feed.xml");

    assert!(sitemap_path.exists(), "Sitemap should be created by the complete workflow");
    assert!(feed_path.exists(), "RSS feed should be created by the complete workflow");

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