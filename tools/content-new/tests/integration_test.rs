use common_test_utils::integration::TestCommand;
use std::fs;

// Tests are temporarily disabled to avoid hanging issues
#[test]
fn test_create_new_content() {
    // Create a new test command for content-new
    let command = TestCommand::new("content-new").unwrap();
    
    // Test creating new content with required arguments
    let output = command.assert_success(&[
        "--title", "Test Article",
        "--topic", "blog",
        "--tagline", "A test article",
        "--content-type", "article"
    ]);
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created"));
    assert!(stdout.contains("Test Article"));
    
    // Verify content was created
    let content_path = command.fixture.content_dir.join("blog/test-article/index.mdx");
    assert!(content_path.exists());
    
    // Verify content has expected structure
    let content = fs::read_to_string(content_path).unwrap();
    assert!(content.contains("title: \"Test Article\""));
    assert!(content.contains("tagline: \"A test article\""));
}

#[test]
fn test_create_new_content_with_tags() {
    // Create a new test command for content-new
    let command = TestCommand::new("content-new").unwrap();
    
    // Test creating new content with tags
    let output = command.assert_success(&[
        "--title", "Tagged Article",
        "--topic", "blog",
        "--tagline", "An article with tags",
        "--content-type", "article",
        "--tags", "tag1,tag2,tag3"
    ]);
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created"));
    assert!(stdout.contains("Tagged Article"));
    
    // Verify content was created
    let content_path = command.fixture.content_dir.join("blog/tagged-article/index.mdx");
    assert!(content_path.exists());
    
    // Verify content has expected structure
    let content = fs::read_to_string(content_path).unwrap();
    assert!(content.contains("title: \"Tagged Article\""));
    assert!(content.contains("tagline: \"An article with tags\""));
    assert!(content.contains("\"tag1\","));
    assert!(content.contains("\"tag2\","));
    assert!(content.contains("\"tag3\","));
}

#[test]
fn test_create_new_content_as_draft() {
    // Create a new test command for content-new
    let command = TestCommand::new("content-new").unwrap();
    
    // Test creating new content as draft
    let output = command.assert_success(&[
        "--title", "Draft Article",
        "--topic", "blog",
        "--tagline", "A draft article",
        "--content-type", "article",
        "--draft"
    ]);
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created"));
    assert!(stdout.contains("Draft Article"));
    
    // Verify content was created
    let content_path = command.fixture.content_dir.join("blog/draft-article/index.mdx");
    assert!(content_path.exists());
    
    // Verify content has expected structure
    let content = fs::read_to_string(content_path).unwrap();
    assert!(content.contains("title: \"Draft Article\""));
    assert!(content.contains("tagline: \"A draft article\""));
    assert!(content.contains("draft: true"));
    assert!(content.contains("date: DRAFT"));
}

#[test]
fn test_create_new_content_with_custom_template() {
    // Create a new test command for content-new
    let command = TestCommand::new("content-new").unwrap();
    
    // Create a custom template
    let template_dir = command.fixture.temp_dir.path().join("templates");
    fs::create_dir_all(&template_dir).unwrap();
    let template_path = template_dir.join("custom-template.hbs");
    fs::write(template_path, "---\ntitle: \"{{title}}\"\ntagline: \"{{tagline}}\"\n---\n\n# {{title}}\n\n{{introduction}}").unwrap();
    
    // Set template directory in config
    let config_path = command.fixture.config_path.clone();
    let mut config_content = fs::read_to_string(&config_path).unwrap();
    config_content = config_content.replace("templates_dir: templates", &format!("templates_dir: {}", template_dir.to_string_lossy()));
    fs::write(&config_path, config_content).unwrap();
    
    // Test creating new content with custom template
    let output = command.assert_success(&[
        "--title", "Custom Template Article",
        "--topic", "blog",
        "--tagline", "Using a custom template",
        "--content-type", "article",
        "--template", "custom-template"
    ]);
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Created"));
    assert!(stdout.contains("Custom Template Article"));
    
    // Verify content was created
    let content_path = command.fixture.content_dir.join("blog/custom-template-article/index.mdx");
    assert!(content_path.exists());
    
    // Verify content has expected structure
    let content = fs::read_to_string(content_path).unwrap();
    assert!(content.contains("title: \"Custom Template Article\""));
    assert!(content.contains("tagline: \"Using a custom template\""));
    assert!(content.contains("# Custom Template Article"));
}

#[test]
fn test_create_new_content_with_invalid_topic() {
    // Create a new test command for content-new
    let command = TestCommand::new("content-new").unwrap();
    
    // Test creating new content with invalid topic
    let output = command.assert_failure(&[
        "--title", "Invalid Topic Article",
        "--topic", "invalid-topic",
        "--tagline", "An article with invalid topic",
        "--content-type", "article"
    ]);
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid topic"));
    assert!(stderr.contains("invalid-topic"));
    
    // Verify content was not created
    let content_path = command.fixture.content_dir.join("invalid-topic/invalid-topic-article/index.mdx");
    assert!(!content_path.exists());
}

#[test]
fn test_interactive_content_creation() {
    // Create a new test command for content-new
    let command = TestCommand::new("content-new").unwrap();
    
    // Test interactive content creation
    let mut interactive = common_test_utils::integration::InteractiveTest::new(&command, &[]).unwrap();
    
    // Enter title
    interactive.expect("Enter title").unwrap();
    interactive.send("Interactive Article").unwrap();
    
    // Select topic
    interactive.expect("Select topic").unwrap();
    interactive.send("1").unwrap(); // Assuming the first item is "blog"
    
    // Enter tagline
    interactive.expect("Enter tagline").unwrap();
    interactive.send("An interactive article").unwrap();
    
    // Enter content type
    interactive.expect("Enter content type").unwrap();
    interactive.send("article").unwrap();
    
    // Enter tags
    interactive.expect("Enter tags").unwrap();
    interactive.send("interactive,test").unwrap();
    
    // Select draft status
    interactive.expect("Create as draft").unwrap();
    interactive.send("n").unwrap();
    
    // Close the interactive test
    let output = interactive.close().unwrap();
    assert!(output.status.success());
    
    // Verify content was created
    let content_path = command.fixture.content_dir.join("blog/interactive-article/index.mdx");
    assert!(content_path.exists());
    
    // Verify content has expected structure
    let content = fs::read_to_string(content_path).unwrap();
    assert!(content.contains("title: \"Interactive Article\""));
    assert!(content.contains("tagline: \"An interactive article\""));
    assert!(content.contains("\"interactive\","));
    assert!(content.contains("\"test\","));
} 
