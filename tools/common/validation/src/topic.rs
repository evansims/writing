use common_errors::{Result, WritingError, ResultExt};
use common_models::TopicConfig;

/// Validate that a topic exists in the configuration
#[allow(dead_code)]
pub fn validate_topic(topic: Option<&str>) -> Result<Option<String>> {
    if let Some(topic_key) = topic {
        let config = common_config::load_config()
            .with_context(|| "Failed to load configuration")?;
        
        if !config.content.topics.contains_key(topic_key) {
            let valid_topics: Vec<String> = config.content.topics.keys()
                .map(|k| k.to_string())
                .collect();
            
            return Err(WritingError::topic_error(format!(
                "Invalid topic: {}. Valid topics are: {}", 
                topic_key, 
                valid_topics.join(", ")
            )));
        }
        
        Ok(Some(topic_key.to_string()))
    } else {
        Ok(None)
    }
}

/// Get available topics from the configuration
#[allow(dead_code)]
pub fn get_available_topics() -> Result<Vec<(String, TopicConfig)>> {
    let config = common_config::load_config()
        .with_context(|| "Failed to load configuration")?;
    
    let topics: Vec<(String, TopicConfig)> = config.content.topics
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    
    Ok(topics)
}

/// Get a specific topic configuration
pub fn get_topic_config(topic: &str) -> Result<TopicConfig> {
    let config = common_config::load_config()
        .with_context(|| "Failed to load configuration")?;
    
    if !config.content.topics.contains_key(topic) {
        let valid_topics: Vec<String> = config.content.topics.keys()
            .map(|k| k.to_string())
            .collect();
        
        return Err(WritingError::topic_error(format!(
            "Invalid topic: {}. Valid topics are: {}", 
            topic, 
            valid_topics.join(", ")
        )));
    }
    
    Ok(config.content.topics[topic].clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_test_utils::TestFixture;
    
    #[test]
    fn test_validate_topic_valid() {
        let _fixture = TestFixture::new().unwrap();
        
        // Test with valid topic (using a topic that exists in the actual configuration)
        let result = validate_topic(Some("creativity"));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("creativity".to_string()));
    }
    
    #[test]
    fn test_validate_topic_invalid() {
        let _fixture = TestFixture::new().unwrap();
        
        // Test with invalid topic
        let result = validate_topic(Some("invalid-topic"));
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid topic"));
    }
    
    #[test]
    fn test_validate_topic_none() {
        // Test with None
        let result = validate_topic(None);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }
    
    #[test]
    fn test_get_available_topics() {
        let _fixture = TestFixture::new().unwrap();
        
        // Get available topics
        let topics = get_available_topics().unwrap();
        
        // Check that we have at least some topics
        assert!(!topics.is_empty());
        assert!(topics.iter().any(|(name, _)| name == "creativity"));
        assert!(topics.iter().any(|(name, _)| name == "strategy"));
        
        // Check that the topic configs are valid
        for (_, config) in topics {
            assert!(!config.name.is_empty());
            assert!(!config.directory.is_empty());
        }
    }
    
    #[test]
    fn test_get_topic_config() {
        let _fixture = TestFixture::new().unwrap();
        
        // Get topic config for a valid topic
        let config = get_topic_config("creativity").unwrap();
        
        // Verify config
        assert!(!config.name.is_empty());
        assert!(!config.directory.is_empty());
    }
} 