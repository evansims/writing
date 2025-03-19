//! Tests for error handling in the Write CLI
//!
//! This file contains integration tests that focus on error handling
//! across tool boundaries within the Write CLI.

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

#[test]
fn test_content_edit_nonexistent_slug() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // Act - Try to edit a non-existent slug
    let output = command.assert_failure(&[
        "content", "edit",
        "--slug", "this-slug-does-not-exist",
        "--title", "Updated Title",
    ]);

    // Assert
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found") || stderr.contains("does not exist"),
            "Error should indicate that the slug doesn't exist");

    Ok(())
}

#[test]
fn test_content_delete_nonexistent_slug() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // Act - Try to delete a non-existent slug
    let output = command.assert_failure(&[
        "content", "delete",
        "--slug", "this-slug-does-not-exist",
        "--force", // Skip confirmation
    ]);

    // Assert
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found") || stderr.contains("does not exist"),
            "Error should indicate that the slug doesn't exist");

    Ok(())
}

#[test]
fn test_content_move_nonexistent_slug() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // Create a valid target topic
    command.assert_success(&[
        "topic", "add",
        "--key", "target",
        "--name", "Target Topic",
        "--description", "A topic to move content to"
    ]);

    // Act - Try to move a non-existent slug
    let output = command.assert_failure(&[
        "content", "move",
        "--slug", "this-slug-does-not-exist",
        "--to-topic", "target",
    ]);

    // Assert
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found") || stderr.contains("does not exist"),
            "Error should indicate that the slug doesn't exist");

    Ok(())
}

#[test]
fn test_content_move_nonexistent_topic() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // Create a valid content item
    command.assert_success(&[
        "content", "new",
        "--title", "Topic Move Test",
        "--topic", "blog",
        "--description", "This is a test post for testing moves"
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

    // Act - Try to move to a non-existent topic
    let output = command.assert_failure(&[
        "content", "move",
        "--slug", slug,
        "--to-topic", "nonexistent-topic",
    ]);

    // Assert
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found") || stderr.contains("does not exist"),
            "Error should indicate that the topic doesn't exist");

    Ok(())
}

#[test]
fn test_topic_delete_with_content() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // Create a new topic
    command.assert_success(&[
        "topic", "add",
        "--key", "delete-test",
        "--name", "Delete Test",
        "--description", "A topic to be deleted"
    ]);

    // Create content in that topic
    command.assert_success(&[
        "content", "new",
        "--title", "Topic Delete Test",
        "--topic", "delete-test",
        "--description", "This is a test post for topic deletion"
    ]);

    // Act - Try to delete the topic without force flag
    let output = command.assert_failure(&[
        "topic", "delete",
        "--key", "delete-test",
    ]);

    // Assert
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("content") || stderr.contains("not empty"),
            "Error should indicate that the topic contains content");

    // Should be able to delete with force flag
    command.assert_success(&[
        "topic", "delete",
        "--key", "delete-test",
        "--force",
    ]);

    Ok(())
}

#[test]
fn test_topic_rename_to_existing_key() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // Create another topic
    command.assert_success(&[
        "topic", "add",
        "--key", "target-topic",
        "--name", "Target Topic",
        "--description", "An existing topic"
    ]);

    // Act - Try to rename blog to the existing topic key
    let output = command.assert_failure(&[
        "topic", "rename",
        "--from", "blog",
        "--to", "target-topic",
    ]);

    // Assert
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("exists") || stderr.contains("already"),
            "Error should indicate that the target topic already exists");

    Ok(())
}

#[test]
fn test_content_validate_with_malformed_content() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // Create a content file with malformed markdown
    command.assert_success(&[
        "content", "new",
        "--title", "Malformed Content",
        "--topic", "blog",
        "--description", "This is a test post with malformed content",
        "--content", "# Title\n\nThis is [a broken link](http://example.com.\nThis is an invalid header\n===\nMore text."
    ]);

    // Act - Validate with strict mode
    let output = command.assert_failure(&[
        "content", "validate",
        "--strict",
    ]);

    // Assert
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error") || stderr.contains("invalid"),
            "Validation should report errors");

    Ok(())
}

#[test]
fn test_content_build_with_no_content() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // Create an empty content structure but no actual content files

    // Act - Run the build
    let output = command.assert_success(&[
        "content", "build",
    ]);

    // Assert - Should complete without errors but indicate no content
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("no content") || stdout.contains("0") || stdout.contains("empty"),
            "Output should indicate no content was found or built");

    Ok(())
}

#[test]
fn test_recovery_from_errors() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;
    ensure_test_dirs(&command)?;

    // First try an operation that should fail
    let _ = command.assert_failure(&[
        "content", "edit",
        "--slug", "nonexistent-slug",
        "--title", "This Should Fail",
    ]);

    // Act - Then try a valid operation
    let output = command.assert_success(&[
        "content", "new",
        "--title", "Recovery Test",
        "--topic", "blog",
        "--description", "This is a test of recovery after error"
    ]);

    // Assert - The valid operation should succeed despite previous error
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("created") || stdout.contains("success"),
            "Valid operation should succeed after an error");

    Ok(())
}

#[test]
fn test_invalid_command_parameters() -> Result<()> {
    // Arrange - Create the command and test environment
    let command = TestCommand::new("write")?;

    // Act & Assert - Test various invalid parameter combinations

    // Missing required parameter
    let output = command.assert_failure(&[
        "content", "new",
        // Missing title
        "--topic", "blog",
    ]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("title") || stderr.contains("required"),
            "Should error when missing required parameter");

    // Invalid topic key format
    let output = command.assert_failure(&[
        "topic", "add",
        "--key", "Invalid Topic Key with Spaces",
        "--name", "Invalid Topic",
    ]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("invalid") || stderr.contains("key"),
            "Should error with invalid topic key format");

    Ok(())
}