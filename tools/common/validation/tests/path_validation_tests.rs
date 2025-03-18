use common_validation::{validate_content_path};
use common_test_utils::TestFixture;

#[test]
fn test_validate_content_path() -> Result<(), Box<dyn std::error::Error>> {
    // Create a test fixture that sets up a mock environment
    let fixture = TestFixture::new()?;

    // Set known topics
    fixture.config.set_expected_topics(vec![
        ("creativity".to_string(), common_models::TopicConfig {
            name: "Creativity".to_string(),
            description: "Creative content".to_string(),
            directory: "creativity".to_string(),
        })
    ]);

    // Register the test config
    fixture.register_test_config();

    // Use the mock functions to avoid file system interaction
    let path = validate_content_path("test-post", Some("creativity"))?;

    // The path should contain the topic directory and slug
    assert!(path.to_string_lossy().contains("creativity/test-post/index.md"));

    Ok(())
}

#[test]
fn test_validate_content_path_with_valid_topic() {
    // Create a test fixture
    let fixture = TestFixture::new().unwrap();

    // Set known topics
    fixture.config.set_expected_topics(vec![
        ("creativity".to_string(), common_models::TopicConfig {
            name: "Creativity".to_string(),
            description: "Creative content".to_string(),
            directory: "creativity".to_string(),
        })
    ]);

    // Register the test config
    fixture.register_test_config();

    // Test with a valid topic
    let path = validate_content_path("test-post", Some("creativity"));
    assert!(path.is_ok());
    assert!(path.unwrap().to_string_lossy().contains("creativity/test-post"));
}

#[test]
fn test_validate_content_path_with_default_topic() {
    // Create a test fixture
    let fixture = TestFixture::new().unwrap();

    // Set known topics with creativity as the first one (default)
    fixture.config.set_expected_topics(vec![
        ("creativity".to_string(), common_models::TopicConfig {
            name: "Creativity".to_string(),
            description: "Creative content".to_string(),
            directory: "creativity".to_string(),
        })
    ]);

    // Register the test config
    fixture.register_test_config();

    // Test with no topic specified (should use default creativity)
    let path = validate_content_path("test-post", None);
    assert!(path.is_ok());

    // Check that the path contains the expected directory and slug
    let path_str = path.unwrap().to_string_lossy().to_string();
    println!("Path: {}", path_str);
    assert!(path_str.contains("creativity") && path_str.contains("test-post"));
}

#[test]
fn test_validate_content_path_with_invalid_topic() {
    // Create a test fixture
    let fixture = TestFixture::new().unwrap();

    // Set known topics
    fixture.config.set_expected_topics(vec![
        ("creativity".to_string(), common_models::TopicConfig {
            name: "Creativity".to_string(),
            description: "Creative content".to_string(),
            directory: "creativity".to_string(),
        })
    ]);

    // Register the test config
    fixture.register_test_config();

    // Test with an invalid topic
    let path = validate_content_path("test-post", Some("invalid-topic"));
    assert!(path.is_err());
    let err = path.unwrap_err();
    assert!(err.to_string().contains("Invalid topic"));
}

#[test]
fn test_path_for_slug_in_blog() {
    let fixture = TestFixture::new().unwrap();

    // Set known topics
    fixture.config.set_expected_topics(vec![
        ("creativity".to_string(), common_models::TopicConfig {
            name: "Creativity".to_string(),
            description: "Creative content".to_string(),
            directory: "creativity".to_string(),
        })
    ]);

    // Register the test config
    fixture.register_test_config();

    let slug = "my-post";

    let path = validate_content_path(slug, Some("creativity")).unwrap();
    assert!(path.to_string_lossy().contains(&format!("creativity/{}", slug)));
}

#[test]
fn test_path_for_slug_in_custom_topic() {
    let fixture = TestFixture::new().unwrap();

    // Set known topics
    fixture.config.set_expected_topics(vec![
        ("strategy".to_string(), common_models::TopicConfig {
            name: "Strategy".to_string(),
            description: "Strategy posts".to_string(),
            directory: "strategy".to_string(),
        })
    ]);

    // Register the test config
    fixture.register_test_config();

    let slug = "my-note";
    let topic = "strategy";

    let path = validate_content_path(slug, Some(topic)).unwrap();
    assert!(path.to_string_lossy().contains(&format!("{}/{}", topic, slug)));
}