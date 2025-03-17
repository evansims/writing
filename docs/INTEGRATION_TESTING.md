# Integration Testing Patterns

This document outlines the standardized patterns for integration testing in the Writing project.

## Overview

Integration tests verify that different components of the system work together correctly. In the Writing project, integration tests focus on testing the command-line tools and their interactions with the file system, configuration, and other components.

## Test Utilities

The `common-test-utils` crate provides utilities for integration testing:

- `TestFixture`: Creates a temporary directory with a default configuration for testing
- `integration::TestCommand`: Represents a command to be tested
- `integration::InteractiveTest`: Helper for testing interactive commands

## Test Patterns

### Basic Command Testing

Test that a command executes successfully and produces the expected output:

```rust
use common_test_utils::integration::TestCommand;

#[test]
fn test_command_execution() {
    // Create a new test command
    let command = TestCommand::new("command-name").unwrap();
    
    // Test command execution
    let output = command.assert_success(&["--arg1", "value1"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Expected output"));
}
```

### Testing Command Failures

Test that a command fails appropriately when given invalid input:

```rust
#[test]
fn test_command_failure() {
    let command = TestCommand::new("command-name").unwrap();
    
    // Test command failure
    let output = command.assert_failure(&["--invalid-arg"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Expected error message"));
}
```

### Testing File Operations

Test that a command correctly modifies files:

```rust
#[test]
fn test_file_operations() {
    let command = TestCommand::new("command-name").unwrap();
    
    // Create test files
    let file_path = command.fixture.create_file("test.txt", "Test content").unwrap();
    assert!(file_path.exists());
    
    // Test command that modifies files
    command.assert_success(&["--file", "test.txt"]);
    
    // Verify file modifications
    let content = std::fs::read_to_string(file_path).unwrap();
    assert!(content.contains("Modified content"));
}
```

### Testing Content Operations

Test operations on content files:

```rust
#[test]
fn test_content_operations() {
    let command = TestCommand::new("command-name").unwrap();
    
    // Create test content
    let content_file = command.fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
    assert!(content_file.exists());
    
    // Test command that operates on content
    command.assert_success(&["--slug", "test-post", "--topic", "blog"]);
    
    // Verify content modifications
    // ...
}
```

### Testing Interactive Commands

Test commands that require user input:

```rust
#[test]
fn test_interactive_command() {
    let command = TestCommand::new("command-name").unwrap();
    
    // Test interactive command with input
    let output = command.run_with_input(&["--interactive"], "y\n").unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Expected output"));
}
```

### Testing Fully Interactive Commands

Test commands with multiple interactive prompts:

```rust
#[test]
fn test_fully_interactive_command() {
    let command = TestCommand::new("command-name").unwrap();
    
    // Create an interactive test
    let mut interactive = common_test_utils::integration::InteractiveTest::new(&command, &[]).unwrap();
    
    // Wait for prompts and send responses
    interactive.expect("First prompt").unwrap();
    interactive.send("first response").unwrap();
    
    interactive.expect("Second prompt").unwrap();
    interactive.send("second response").unwrap();
    
    // Close the interactive test
    let output = interactive.close().unwrap();
    assert!(output.status.success());
    
    // Verify results
    // ...
}
```

## Best Practices

1. **Isolation**: Each test should be isolated and not depend on the state from other tests.
2. **Cleanup**: Use the `TestFixture` to ensure that temporary files are cleaned up after tests.
3. **Assertions**: Make specific assertions about the expected behavior of commands.
4. **Coverage**: Test both success and failure cases for each command.
5. **Readability**: Use descriptive test names and comments to explain the purpose of each test.

## Example

See the integration tests in the `content-delete` crate for a complete example of integration testing. 