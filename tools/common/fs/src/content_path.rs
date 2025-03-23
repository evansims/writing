use common_models::Config;
use std::path::PathBuf;

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
pub fn find_content_path(
    slug: &str,
    topic: Option<&str>,
    config: &Config,
) -> Result<PathBuf, std::io::Error> {
    if let Some(topic_key) = topic {
        // Validate topic
        if !config.content.topics.contains_key(topic_key) {
            let valid_topics: Vec<String> = config
                .content
                .topics
                .keys()
                .map(|k| k.to_string())
                .collect();
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Invalid topic: {}. Valid topics are: {}",
                    topic_key,
                    valid_topics.join(", ")
                ),
            ));
        }

        let topic_config = &config.content.topics[topic_key];
        let topic_path = &topic_config.directory;

        // Check for the matching-name file with .md extension
        let content_path_md = PathBuf::from(&config.content.base_dir)
            .join(topic_path)
            .join(slug)
            .join(format!("{}.md", slug));

        if content_path_md.exists() {
            return Ok(content_path_md);
        }

        // Check for the matching-name file with .mdx extension
        let content_path_mdx = PathBuf::from(&config.content.base_dir)
            .join(topic_path)
            .join(slug)
            .join(format!("{}.mdx", slug));

        if content_path_mdx.exists() {
            return Ok(content_path_mdx);
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Content not found: {}/{}", topic_key, slug),
        ))
    } else {
        // Search all topics
        for topic_config in config.content.topics.values() {
            let topic_path = &topic_config.directory;

            // Check for the matching-name file with .md extension
            let content_path_md = PathBuf::from(&config.content.base_dir)
                .join(topic_path)
                .join(slug)
                .join(format!("{}.md", slug));

            if content_path_md.exists() {
                return Ok(content_path_md);
            }

            // Check for the matching-name file with .mdx extension
            let content_path_mdx = PathBuf::from(&config.content.base_dir)
                .join(topic_path)
                .join(slug)
                .join(format!("{}.mdx", slug));

            if content_path_mdx.exists() {
                return Ok(content_path_mdx);
            }
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Content not found: {}", slug),
        ))
    }
}
