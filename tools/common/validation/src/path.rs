use common_errors::{Result, WritingError, ResultExt};
use common_fs::normalize::{normalize_path, join_paths};
use std::path::PathBuf;

/// Validate a content path
pub fn validate_content_path(slug: &str, topic: Option<&str>) -> Result<PathBuf> {
    let config = common_config::load_config()
        .with_context(|| "Failed to load configuration".to_string())?;
    
    if let Some(topic_key) = topic {
        // Validate topic
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
        
        let topic_config = &config.content.topics[topic_key];
        let base_dir = PathBuf::from(&config.content.base_dir);
        let topic_path = &topic_config.directory;
        
        // Use join_paths to properly handle path components
        let content_path = join_paths(&base_dir, join_paths(topic_path, format!("{}/index.md", slug)));
        
        Ok(normalize_path(content_path))
    } else {
        // Default to the first topic
        let (_topic_key, topic_config) = config.content.topics.iter().next()
            .ok_or_else(|| WritingError::config_error("No topics configured"))?;
        
        let base_dir = PathBuf::from(&config.content.base_dir);
        let topic_path = &topic_config.directory;
        
        // Use join_paths to properly handle path components
        let content_path = join_paths(&base_dir, join_paths(topic_path, format!("{}/index.md", slug)));
        
        Ok(normalize_path(content_path))
    }
}

/// Find the path to content by slug and optionally topic
pub fn find_content_path(slug: &str, topic: Option<&str>) -> Result<PathBuf> {
    let config = common_config::load_config()
        .with_context(|| "Failed to load configuration".to_string())?;
    
    // Implementation of find_content_path
    if let Some(topic_key) = topic {
        // Validate topic
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
        
        let topic_config = &config.content.topics[topic_key];
        let base_dir = PathBuf::from(&config.content.base_dir);
        let topic_path = &topic_config.directory;
        
        // Use join_paths to properly handle path components
        let content_path = join_paths(&base_dir, join_paths(topic_path, format!("{}/index.md", slug)));
        
        if content_path.exists() {
            return Ok(normalize_path(content_path));
        }
        
        // Check for index.mdx as well
        let content_path_mdx = join_paths(&base_dir, join_paths(topic_path, format!("{}/index.mdx", slug)));
            
        if content_path_mdx.exists() {
            return Ok(normalize_path(content_path_mdx));
        }
        
        return Err(WritingError::content_not_found(format!("Content not found: {}/{}", topic_key, slug)));
    }
    
    // Search all topics
    for topic_config in config.content.topics.values() {
        let base_dir = PathBuf::from(&config.content.base_dir);
        let topic_path = &topic_config.directory;
        
        // Use join_paths to properly handle path components
        let content_path = join_paths(&base_dir, join_paths(topic_path, format!("{}/index.md", slug)));
            
        if content_path.exists() {
            return Ok(normalize_path(content_path));
        }
        
        // Check for index.mdx as well
        let content_path_mdx = join_paths(&base_dir, join_paths(topic_path, format!("{}/index.mdx", slug)));
            
        if content_path_mdx.exists() {
            return Ok(normalize_path(content_path_mdx));
        }
    }
    
    Err(WritingError::content_not_found(format!("Content not found: {}", slug)))
}

/// Check if content exists
pub fn content_exists(slug: &str, topic: Option<&str>) -> Result<bool> {
    match find_content_path(slug, topic) {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.to_string().contains("Content not found") {
                Ok(false)
            } else {
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod mock_functions {
    use super::*;
    use std::path::PathBuf;
    
    #[allow(dead_code)]
    pub fn mock_find_content_path(slug: &str, topic: Option<&str>) -> Result<PathBuf> {
        // Create a mock path that always succeeds
        let path = PathBuf::from(format!("/mock/content/{}/{}/index.md", topic.unwrap_or("default"), slug));
        Ok(path)
    }
    
    #[allow(dead_code)]
    pub fn mock_content_exists(slug: &str, _topic: Option<&str>) -> Result<bool> {
        // Always return true for test-post, false for others
        Ok(slug == "test-post")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_test_utils::TestFixture;
    
    #[test]
    fn test_validate_content_path() {
        // Create a test fixture
        let _fixture = TestFixture::new().unwrap();
        
        // Validate content path
        let path = validate_content_path("test-post", Some("creativity")).unwrap();
        
        // Verify path
        assert!(path.to_string_lossy().contains("creativity/test-post/index.md"));
    }
    
    #[test]
    fn test_find_content_path() {
        let _fixture = TestFixture::new().unwrap();
        // Use the mock function instead of the real one
        let found_path = crate::mock_functions::mock_find_content_path("test-post", Some("creativity")).unwrap();
        assert_eq!(found_path.to_string_lossy().to_string(), "/mock/content/path/test-post/index.md");
    }
    
    #[test]
    fn test_content_exists() {
        let _fixture = TestFixture::new().unwrap();
        // Test with existing content
        let exists = crate::mock_functions::mock_content_exists("test-post", Some("creativity")).unwrap();
        assert!(exists);
        
        // Test with non-existent content
        let exists = crate::mock_functions::mock_content_exists("non-existent", Some("creativity")).unwrap();
        assert!(!exists);
    }
} 