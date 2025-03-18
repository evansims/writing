use common_errors::Result;
use common_test_utils::TestFixture;
use std::fs;

#[test]
fn test_basic_config_loading() -> Result<()> {
    // Set up a fixture
    let fixture = TestFixture::new()?;

    // Print the current working directory
    println!("Current working directory: {:?}", std::env::current_dir()?);

    // Print environment variable
    println!("CONFIG_PATH env var: {:?}", std::env::var("CONFIG_PATH"));

    // Read the config file directly
    let config_path = std::env::var("CONFIG_PATH").unwrap();
    let config_content = fs::read_to_string(&config_path)?;
    println!("Config file content from filesystem:\n{}", config_content);

    // Try parsing it ourselves
    let parsed_config: common_models::Config = serde_yaml::from_str(&config_content)?;
    println!("Topics in parsed config: {:?}", parsed_config.content.topics.keys().collect::<Vec<_>>());

    // Test that fields exist
    for (key, topic) in &parsed_config.content.topics {
        println!("Topic {}: name={}, directory={}, description={}",
                 key, topic.name, topic.directory, topic.description);
    }

    // Try to load the config through the library
    let config = common_config::load_config()?;

    // Check the topics
    assert!(config.content.topics.contains_key("creativity"));
    assert!(config.content.topics.contains_key("strategy"));

    Ok(())
}