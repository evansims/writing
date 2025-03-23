use anyhow::Result;
use common_fs::normalize::{join_paths, normalize_path};
use fs_extra::dir::{copy, CopyOptions};
use std::fs;
use std::path::{Path, PathBuf};
/// Extension trait for Option to validate required fields
pub trait OptionValidationExt<T> {
    fn validate_required(self, message: &'static str) -> Result<T>;
}

impl<T> OptionValidationExt<T> for Option<T> {
    fn validate_required(self, message: &'static str) -> Result<T> {
        self.ok_or_else(|| anyhow::anyhow!(message))
    }
}

/// Options for content movement
#[derive(Clone)]
pub struct MoveOptions {
    pub slug: Option<String>,
    pub new_slug: Option<String>,
    pub topic: Option<String>,
    pub new_topic: Option<String>,
    pub update_frontmatter: bool,
}

/// Find the directory containing the content
pub fn find_content_dir(slug: &str, topic: Option<&str>) -> Result<(PathBuf, String)> {
    let config = common_config::load_config()?;

    if let Some(topic_key) = topic {
        // Validate topic
        if !config.content.topics.contains_key(topic_key) {
            let valid_topics: Vec<String> = config
                .content
                .topics
                .keys()
                .map(|k| k.to_string())
                .collect();
            return Err(anyhow::anyhow!(
                "Invalid topic: {}. Valid topics are: {}",
                topic_key,
                valid_topics.join(", ")
            ));
        }

        let topic_config = &config.content.topics[topic_key];
        let base_dir = PathBuf::from(&config.content.base_dir);
        let topic_path = &topic_config.directory;

        // Use join_paths to properly handle path components
        let content_dir = join_paths(&base_dir, join_paths(topic_path, slug));

        if content_dir.exists() {
            return Ok((normalize_path(content_dir), topic_key.to_string()));
        }

        return Err(anyhow::anyhow!(
            "Content not found: {}",
            content_dir.display()
        ));
    }

    // Search for content in all topics
    for (topic_key, topic_config) in &config.content.topics {
        let base_dir = PathBuf::from(&config.content.base_dir);
        let topic_path = &topic_config.directory;

        // Use join_paths to properly handle path components
        let content_dir = join_paths(&base_dir, join_paths(topic_path, slug));

        if content_dir.exists() {
            return Ok((normalize_path(content_dir), topic_key.clone()));
        }
    }

    Err(anyhow::anyhow!("Content not found for slug: {}", slug))
}

/// List all content in the repository
pub fn list_all_content() -> Result<Vec<(String, String, PathBuf)>> {
    let config = common_config::load_config()?;
    let mut content_list = Vec::new();

    for (topic_key, topic_config) in &config.content.topics {
        let base_dir = PathBuf::from(&config.content.base_dir);
        let topic_dir = join_paths(&base_dir, &topic_config.directory);

        if !topic_dir.exists() {
            continue;
        }

        // Find all subdirectories in the topic directory
        let article_dirs = common_fs::find_dirs_with_depth(&topic_dir, 1, 1)?;

        for article_dir in article_dirs {
            let slug = article_dir
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("")
                .to_string();

            // Check for the matching-name file with .md extension
            let md_file_path = join_paths(&article_dir, format!("{}.md", slug));
            let mdx_file_path = join_paths(&article_dir, format!("{}.mdx", slug));

            // If any of the content files exist, add to the list
            if md_file_path.exists() || mdx_file_path.exists() {
                content_list.push((topic_key.clone(), slug, normalize_path(article_dir)));
            }
        }
    }

    Ok(content_list)
}

/// Update content references
pub fn update_content_references(
    content_path: &Path,
    old_slug: &str,
    new_slug: &str,
) -> Result<()> {
    // Normalize the path before reading the file
    let normalized_path = normalize_path(content_path);
    let content = common_fs::read_file(&normalized_path)?;

    // Replace old slug with new slug
    let updated_content = content.replace(old_slug, new_slug);

    if content != updated_content {
        common_fs::write_file(&normalized_path, &updated_content)?;
    }

    Ok(())
}

/// Move content to a new location and/or rename it
pub fn move_content(options: &MoveOptions) -> Result<()> {
    // Validate options
    let slug = options
        .slug
        .clone()
        .validate_required("Content slug is required")?;
    let current_topic = options
        .topic
        .clone()
        .validate_required("Current topic is required")?;
    let new_topic = options
        .new_topic
        .clone()
        .validate_required("New topic is required")?;

    // Load config
    let config = common_config::load_config()?;

    // Validate current topic
    if !config.content.topics.contains_key(&current_topic) {
        return Err(anyhow::anyhow!(
            "Current topic not found: {}",
            current_topic
        ));
    }

    // Validate new topic
    if !config.content.topics.contains_key(&new_topic) {
        return Err(anyhow::anyhow!("New topic not found: {}", new_topic));
    }

    // Get topic configs
    let current_topic_config = &config.content.topics[&current_topic];
    let new_topic_config = &config.content.topics[&new_topic];

    // Get paths
    let base_dir = PathBuf::from(&config.content.base_dir);
    let current_topic_path = &current_topic_config.directory;
    let new_topic_path = &new_topic_config.directory;

    // Find content path
    let content_path = join_paths(&base_dir, join_paths(current_topic_path, &slug));

    if !content_path.exists() {
        return Err(anyhow::anyhow!(
            "Content not found: {}/{}",
            current_topic,
            slug
        ));
    }

    // Create new content path
    let new_content_path = join_paths(&base_dir, join_paths(new_topic_path, &slug));

    if new_content_path.exists() {
        return Err(anyhow::anyhow!(
            "Content already exists in target topic: {}/{}",
            new_topic,
            slug
        ));
    }

    // Create parent directory if it doesn't exist
    if let Some(parent) = new_content_path.parent() {
        common_fs::create_dir_all(parent)?;
    }

    // Move content
    move_dir(&content_path, &new_content_path)?;

    // Update frontmatter if requested
    if options.update_frontmatter {
        update_frontmatter(&new_content_path, &current_topic, &new_topic)?;
    }

    Ok(())
}

/// Move a directory from one location to another
///
/// This function moves a directory from one location to another.
///
/// # Parameters
///
/// * `from` - Source directory
/// * `to` - Target directory
///
/// # Returns
///
/// Returns Ok(()) if successful
///
/// # Errors
///
/// Returns an error if the directory cannot be moved
fn move_dir(from: &Path, to: &Path) -> Result<()> {
    // Try to use fs::rename first (fast path)
    if let Ok(()) = fs::rename(from, to) {
        return Ok(());
    }

    // If rename fails (e.g., across filesystems), fall back to copy and remove
    copy_dir_all(from, to)?;
    fs::remove_dir_all(from)?;

    Ok(())
}

/// Copy a directory recursively
///
/// This function copies a directory recursively from one location to another.
///
/// # Parameters
///
/// * `from` - Source directory
/// * `to` - Target directory
///
/// # Returns
///
/// Returns Ok(()) if successful
///
/// # Errors
///
/// Returns an error if the directory cannot be copied
fn copy_dir_all(from: &Path, to: &Path) -> Result<()> {
    let mut options = CopyOptions::new();
    options.copy_inside = true;

    // Create target directory if it doesn't exist
    if !to.exists() {
        common_fs::create_dir_all(to)?;
    }

    // Copy directory
    copy(from, to, &options)?;

    Ok(())
}

/// Update frontmatter after moving content
///
/// This function updates the frontmatter of a content file after moving it.
///
/// # Parameters
///
/// * `content_path` - Path to the content directory
/// * `old_topic` - Old topic key
/// * `new_topic` - New topic key
///
/// # Returns
///
/// Returns Ok(()) if successful
///
/// # Errors
///
/// Returns an error if the frontmatter cannot be updated
fn update_frontmatter(content_path: &Path, old_topic: &str, new_topic: &str) -> Result<()> {
    // Get slug from the content directory name
    let slug = content_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid content directory name"))?;

    // Check for the matching-name file with .md extension
    let md_path = content_path.join(format!("{}.md", slug));
    if md_path.exists() {
        return update_frontmatter_file(&md_path, old_topic, new_topic);
    }

    // Check for the matching-name file with .mdx extension
    let mdx_path = content_path.join(format!("{}.mdx", slug));
    if mdx_path.exists() {
        return update_frontmatter_file(&mdx_path, old_topic, new_topic);
    }

    // No content file found
    Err(anyhow::anyhow!(
        "Content file not found in {}",
        content_path.display()
    ))
}

/// Update frontmatter in a file
///
/// This function updates the frontmatter in a file.
///
/// # Parameters
///
/// * `file_path` - Path to the file
/// * `old_topic` - Old topic key
/// * `new_topic` - New topic key
///
/// # Returns
///
/// Returns Ok(()) if successful
///
/// # Errors
///
/// Returns an error if the frontmatter cannot be updated
fn update_frontmatter_file(file_path: &Path, old_topic: &str, new_topic: &str) -> Result<()> {
    // Read the file
    let content = common_fs::read_file(file_path)?;

    // Extract frontmatter
    let (frontmatter, content_without_frontmatter) =
        common_markdown::extract_frontmatter(&content)?;

    // Update topics in frontmatter
    let mut updated_frontmatter = frontmatter.clone();

    // Check if topics field exists
    if let Some(topics) = updated_frontmatter.get_mut("topics") {
        if let Some(topics_array) = topics.as_sequence_mut() {
            // Replace old topic with new topic
            for topic_value in topics_array.iter_mut() {
                if let Some(topic_str) = topic_value.as_str() {
                    if topic_str == old_topic {
                        *topic_value = serde_yaml::Value::String(new_topic.to_string());
                    }
                }
            }
        }
    }

    // Convert frontmatter back to YAML
    let updated_frontmatter_str = serde_yaml::to_string(&updated_frontmatter)?;

    // Combine updated frontmatter with content
    let updated_content = format!(
        "---\n{}---\n{}",
        updated_frontmatter_str, content_without_frontmatter
    );

    // Write updated content back to file
    common_fs::write_file(file_path, &updated_content)?;

    Ok(())
}
