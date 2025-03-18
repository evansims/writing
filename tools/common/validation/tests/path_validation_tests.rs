use common_validation::{validate_content_path, content_exists};

#[test]
fn test_validate_content_path_with_valid_topic() {
    // Test with a valid topic
    let path = validate_content_path("test-post", Some("blog"));
    assert!(path.is_ok());
    assert!(path.unwrap().to_string_lossy().contains("blog/test-post"));
}

#[test]
fn test_validate_content_path_with_default_topic() {
    // Test with no topic specified (should use default blog)
    let path = validate_content_path("test-post", None);
    assert!(path.is_ok());
    assert!(path.unwrap().to_string_lossy().contains("blog/test-post"));
}

#[test]
fn test_validate_content_path_with_invalid_topic() {
    // Test with an invalid topic
    let path = validate_content_path("test-post", Some("invalid-topic"));
    assert!(path.is_err());
    let err = path.unwrap_err();
    assert!(err.to_string().contains("Invalid topic"));
}

#[test]
fn test_path_for_slug_in_blog() {
    let slug = "my-post";

    let path = validate_content_path(slug, Some("blog")).unwrap();
    assert!(path.to_string_lossy().contains(&format!("blog/{}", slug)));
}

#[test]
fn test_path_for_slug_in_custom_topic() {
    let slug = "my-note";
    let topic = "notes";

    let path = validate_content_path(slug, Some(topic)).unwrap();
    assert!(path.to_string_lossy().contains(&format!("{}/{}", topic, slug)));
}