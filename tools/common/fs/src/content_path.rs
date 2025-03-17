use std::path::PathBuf;
use common_models::Config;

/// Find the path to content by slug and optionally topic
///
/// This function locates a content file by its slug and optional topic, returning the path
/// to the content file.
///
/// # Arguments
///
/// * `slug` - The content slug
/// * `topic` - Optional topic name. If not provided, all topics will be searched.
/// * `config` - The configuration object containing topic information
///
/// # Returns
///
/// Returns a Result containing the path to the content file, or an error if not found.
pub fn find_content_path(slug: &str, topic: Option<&str>, config: &Config) -> Result<PathBuf, std::io::Error> {
    if let Some(topic_key) = topic {
        // Validate topic
        if !config.content.topics.contains_key(topic_key) {
            let valid_topics: Vec<String> = config.content.topics.keys()
                .map(|k| k.to_string())
                .collect();
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Invalid topic: {}. Valid topics are: {}", 
                    topic_key, 
                    valid_topics.join(", ")
                )
            ));
        }
        
        let topic_config = &config.content.topics[topic_key];
        let _base_dir = PathBuf::from(&config.content.base_dir);
        let topic_path = &topic_config.directory;
        
        // Use join_paths to properly handle path components
        let content_path = PathBuf::from(topic_path)
            .join(slug)
            .join("index.md");
        
        if content_path.exists() {
            return Ok(content_path);
        }
        
        // Check for index.mdx as well
        let content_path_mdx = PathBuf::from(topic_path)
            .join(slug)
            .join("index.mdx");
            
        if content_path_mdx.exists() {
            return Ok(content_path_mdx);
        }
        
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Content not found: {}/{}", topic_key, slug)
        ))
    } else {
        // Search all topics
        for topic_config in config.content.topics.values() {
            let _base_dir = PathBuf::from(&config.content.base_dir);
            let topic_path = &topic_config.directory;
            
            // Use join_paths to properly handle path components
            let content_path = PathBuf::from(topic_path)
                .join(slug)
                .join("index.md");
                
            if content_path.exists() {
                return Ok(content_path);
            }
            
            // Check for index.mdx as well
            let content_path_mdx = PathBuf::from(topic_path)
                .join(slug)
                .join("index.mdx");
                
            if content_path_mdx.exists() {
                return Ok(content_path_mdx);
            }
        }
        
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Content not found: {}", slug)
        ))
    }
} 