use anyhow::Result;
use common_test_utils::TestFixture;
use content_new::get_available_topics;

#[test]
fn test_get_available_topics() -> Result<()> {
    // Create a test fixture
    let fixture = TestFixture::new()?;

    // Register the test fixture's config
    fixture.register_test_config();

    // Now get available topics using the test config
    let topics = get_available_topics()?;

    // We should have at least one topic
    assert!(!topics.is_empty());

    // We expect to have topics defined in the TestFixture
    assert!(topics.iter().any(|(name, _)| name == "creativity"));
    assert!(topics.iter().any(|(name, _)| name == "strategy"));
    assert!(topics.iter().any(|(name, _)| name == "blog"));

    Ok(())
}
