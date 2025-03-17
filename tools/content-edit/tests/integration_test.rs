use common_test_utils::integration::TestCommand;
use std::path::Path;
use std::fs;

#[test]
fn test_edit_content() {
    // Create a new test command for content-edit
    let command = TestCommand::new("content-edit").unwrap();
    
    // Create test content
    let content_file = command.fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
    let original_content = fs::read_to_string(&content_file).unwrap();
    
    // Create a temporary file with edited content
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_file = temp_dir.path().join("edited.mdx");
    let edited_content = original_content.replace("Test Post", "Edited Post");
    fs::write(&temp_file, &edited_content).unwrap();
    
    // Set up environment to use our test editor
    let editor_script = temp_dir.path().join("editor.sh");
    fs::write(&editor_script, format!(
        "#!/bin/sh\ncp \"{}\" \"$1\"",
        temp_file.to_string_lossy()
    )).unwrap();
    
    // Make the script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&editor_script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&editor_script, perms).unwrap();
    }
    
    // Test editing content with slug and topic
    let output = command.run_with_input(
        &["--slug", "test-post", "--topic", "blog"],
        &format!("EDITOR={}", editor_script.to_str().unwrap())
    ).unwrap();
    
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("saved successfully"));
    
    // Verify content was edited
    let updated_content = fs::read_to_string(&content_file).unwrap();
    assert!(updated_content.contains("Edited Post"));
    assert!(!updated_content.contains("Test Post"));
}

#[test]
fn test_edit_frontmatter_only() {
    // Create a new test command for content-edit
    let command = TestCommand::new("content-edit").unwrap();
    
    // Create test content
    let content_file = command.fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
    
    // Add some content to the file
    let original_content = fs::read_to_string(&content_file).unwrap();
    let content_with_body = format!("{}\n\n# Test Post\n\nThis is a test post.", original_content);
    fs::write(&content_file, &content_with_body).unwrap();
    
    // Create a temporary file with edited frontmatter
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_file = temp_dir.path().join("edited.mdx");
    let frontmatter = original_content.replace("Test Post", "Edited Post");
    fs::write(&temp_file, &frontmatter).unwrap();
    
    // Set up environment to use our test editor
    let editor_script = temp_dir.path().join("editor.sh");
    fs::write(&editor_script, format!(
        "#!/bin/sh\ncp \"{}\" \"$1\"",
        temp_file.to_string_lossy()
    )).unwrap();
    
    // Make the script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&editor_script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&editor_script, perms).unwrap();
    }
    
    // Test editing frontmatter only
    let output = command.run_with_input(
        &["--slug", "test-post", "--topic", "blog", "--frontmatter-only"],
        &format!("EDITOR={}", editor_script.to_str().unwrap())
    ).unwrap();
    
    assert!(output.status.success());
    
    // Verify frontmatter was edited but body remains
    let updated_content = fs::read_to_string(&content_file).unwrap();
    assert!(updated_content.contains("Edited Post"));
    assert!(!updated_content.contains("title: \"Test Post\""));
    assert!(updated_content.contains("# Test Post"));
    assert!(updated_content.contains("This is a test post."));
}

#[test]
fn test_edit_content_only() {
    // Create a new test command for content-edit
    let command = TestCommand::new("content-edit").unwrap();
    
    // Create test content
    let content_file = command.fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
    
    // Add some content to the file
    let original_content = fs::read_to_string(&content_file).unwrap();
    let content_with_body = format!("{}\n\n# Test Post\n\nThis is a test post.", original_content);
    fs::write(&content_file, &content_with_body).unwrap();
    
    // Create a temporary file with edited body
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_file = temp_dir.path().join("edited.mdx");
    let edited_body = "# Edited Post\n\nThis post has been edited.";
    fs::write(&temp_file, &edited_body).unwrap();
    
    // Set up environment to use our test editor
    let editor_script = temp_dir.path().join("editor.sh");
    fs::write(&editor_script, format!(
        "#!/bin/sh\ncp \"{}\" \"$1\"",
        temp_file.to_string_lossy()
    )).unwrap();
    
    // Make the script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&editor_script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&editor_script, perms).unwrap();
    }
    
    // Test editing content only
    let output = command.run_with_input(
        &["--slug", "test-post", "--topic", "blog", "--content-only"],
        &format!("EDITOR={}", editor_script.to_str().unwrap())
    ).unwrap();
    
    assert!(output.status.success());
    
    // Verify body was edited but frontmatter remains
    let updated_content = fs::read_to_string(&content_file).unwrap();
    assert!(updated_content.contains("title: \"Test Post\""));
    assert!(updated_content.contains("# Edited Post"));
    assert!(updated_content.contains("This post has been edited."));
    assert!(!updated_content.contains("This is a test post."));
}

#[test]
fn test_edit_nonexistent_content() {
    // Create a new test command for content-edit
    let command = TestCommand::new("content-edit").unwrap();
    
    // Test editing nonexistent content
    let output = command.assert_failure(&["--slug", "nonexistent-post", "--topic", "blog"]);
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Content not found"));
}

#[test]
fn test_interactive_content_selection() {
    // Create a new test command for content-edit
    let command = TestCommand::new("content-edit").unwrap();
    
    // Create test content
    let content_file = command.fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
    
    // Create a temporary file with edited content
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_file = temp_dir.path().join("edited.mdx");
    let original_content = fs::read_to_string(&content_file).unwrap();
    let edited_content = original_content.replace("Test Post", "Edited Post");
    fs::write(&temp_file, &edited_content).unwrap();
    
    // Set up environment to use our test editor
    let editor_script = temp_dir.path().join("editor.sh");
    fs::write(&editor_script, format!(
        "#!/bin/sh\ncp \"{}\" \"$1\"",
        temp_file.to_string_lossy()
    )).unwrap();
    
    // Make the script executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&editor_script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&editor_script, perms).unwrap();
    }
    
    // Test interactive content selection
    let mut interactive = common_test_utils::integration::InteractiveTest::new(
        &command,
        &[]
    ).unwrap();
    
    // Set the EDITOR environment variable
    std::env::set_var("EDITOR", editor_script.to_str().unwrap());
    
    // Select content
    interactive.expect("Select content to edit").unwrap();
    interactive.send("1").unwrap(); // Assuming the first item in the list
    
    // Close the interactive test
    let output = interactive.close().unwrap();
    assert!(output.status.success());
    
    // Verify content was edited
    let updated_content = fs::read_to_string(&content_file).unwrap();
    assert!(updated_content.contains("Edited Post"));
    assert!(!updated_content.contains("Test Post"));
} 