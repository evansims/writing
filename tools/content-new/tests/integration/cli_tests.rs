use anyhow::Result;
use std::process::{Command, Output};
use std::path::PathBuf;
use common_test_utils::TestFixture;

// Helper function to run the command
fn run_command(args: &[&str], test_dir: &PathBuf) -> Result<Output> {
    let cargo_bin = std::env::var("CARGO_BIN_EXE_content-new")
        .unwrap_or_else(|_| "target/debug/content-new".to_string());

    let output = Command::new(cargo_bin)
        .args(args)
        .env("TEST_MODE", "1")
        .current_dir(test_dir)
        .output()?;

    Ok(output)
}

#[test]
fn test_cli_create_content_with_args() -> Result<()> {
    // Arrange: Set up test environment
    let fixture = TestFixture::new()?;
    fixture.register_test_config();

    // Act: Run the CLI command with args
    let output = run_command(&[
        "--topic", "blog",
        "-T", "Test Title",
        "-g", "Test Description",
        "-a", "test,example",
        "-d", // draft flag
    ], &PathBuf::from(fixture.path()))?;

    // Assert: Command executed successfully
    assert!(output.status.success(),
        "Command failed with: {}", String::from_utf8_lossy(&output.stderr));

    // Get the output message
    let stdout = String::from_utf8_lossy(&output.stdout);

    // The slug would be 'test-title' derived from the title
    let expected_path = PathBuf::from(fixture.path())
        .join("content")
        .join("blog")
        .join("test-title")
        .join("index.mdx");

    // Verify the file was created
    assert!(expected_path.exists(),
        "Expected file not created at {}", expected_path.display());

    // Verify content
    let content = std::fs::read_to_string(&expected_path)?;
    assert!(content.contains("title: \"Test Title\""));
    assert!(content.contains("tagline: \"Test Description\""));
    assert!(content.contains("# Test Title"));
    assert!(content.contains("date: DRAFT") || content.contains("draft: true"));
    assert!(content.contains("\"test\"") && content.contains("\"example\""));

    Ok(())
}

#[test]
fn test_cli_without_required_args() -> Result<()> {
    // Since the CLI prompts for missing required args, this test
    // is limited to checking that the command fails without input
    // when not in interactive mode

    // Arrange: Set up test environment
    let fixture = TestFixture::new()?;
    fixture.register_test_config();

    // Use a special variable to disable interactive prompts
    let output = Command::new("cargo")
        .args(["run", "--bin", "content-new"])
        .env("TEST_MODE", "1")
        .env("CI", "1") // Disable interactive prompts
        .current_dir(fixture.path())
        .output()?;

    // Assert: Should fail or handle gracefully without required args
    // In this case, we expect a non-zero exit code since user input would be required
    assert!(!output.status.success() || output.stdout.is_empty());

    Ok(())
}

// We would add more integration tests here to cover different CLI scenarios