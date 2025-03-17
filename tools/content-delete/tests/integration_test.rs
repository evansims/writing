use common_test_utils::integration::TestCommand;

#[test]
fn test_delete_content() {
    // Create a new test command for content-delete
    let command = TestCommand::new("content-delete").unwrap();
    
    // Create test content
    let content_file = command.fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
    assert!(content_file.exists());
    
    // Test deleting content with force flag
    let output = command.assert_success(&["-s", "test-post", "-t", "blog", "-f"]);
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Deleted"));
    assert!(stdout.contains("Test Post"));
    
    // Verify content is deleted
    assert!(!content_file.exists());
}

#[test]
fn test_delete_nonexistent_content() {
    // Create a new test command for content-delete
    let command = TestCommand::new("content-delete").unwrap();
    
    // Test deleting nonexistent content
    let output = command.assert_failure(&["-s", "nonexistent-post", "-t", "blog", "-f"]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found"));
}

#[test]
fn test_interactive_delete() {
    // Create a new test command for content-delete
    let command = TestCommand::new("content-delete").unwrap();
    
    // Create test content
    let content_file = command.fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
    assert!(content_file.exists());
    
    // Test interactive deletion with confirmation
    let output = command.run_with_input(&["-s", "test-post", "-t", "blog"], "y\n").unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Deleted"));
    
    // Verify content is deleted
    assert!(!content_file.exists());
}

#[test]
fn test_interactive_delete_cancel() {
    // Create a new test command for content-delete
    let command = TestCommand::new("content-delete").unwrap();
    
    // Create test content
    let content_file = command.fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
    assert!(content_file.exists());
    
    // Test interactive deletion with cancellation
    let output = command.run_with_input(&["-s", "test-post", "-t", "blog"], "n\n").unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Cancelled"));
    
    // Verify content is not deleted
    assert!(content_file.exists());
}

#[test]
fn test_fully_interactive_delete() {
    // Create a new test command for content-delete
    let command = TestCommand::new("content-delete").unwrap();
    
    // Create test content
    let content_file = command.fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
    assert!(content_file.exists());
    
    // Test fully interactive deletion using the InteractiveTest helper
    let mut interactive = common_test_utils::integration::InteractiveTest::new(&command, &[]).unwrap();
    
    // Wait for the prompt and select the content
    interactive.expect("Select content to delete").unwrap();
    interactive.send("1").unwrap(); // Assuming the first item in the list
    
    // Confirm deletion
    interactive.expect("Are you sure").unwrap();
    interactive.send("y").unwrap();
    
    // Close the interactive test
    let output = interactive.close().unwrap();
    assert!(output.status.success());
    
    // Verify content is deleted
    assert!(!content_file.exists());
} 