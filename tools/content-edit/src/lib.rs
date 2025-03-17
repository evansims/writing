use common_errors::{Result, WritingError, ResultExt};
use common_fs::normalize::join_paths;
use common_models::Frontmatter;
use std::path::{Path, PathBuf};

/// Options for content editing
pub struct EditOptions {
    pub slug: Option<String>,
    pub topic: Option<String>,
    pub frontmatter_only: bool,
    pub content_only: bool,
}

/// Find the path to content by slug and optionally topic
pub fn find_content_path(slug: &str, topic: Option<&str>) -> Result<PathBuf> {
    let config = common_config::load_config()
        .map_err(|e| WritingError::config_error(format!("Failed to load config: {}", e)))?;
    
    // If topic is provided, look in that topic directory
    if let Some(topic_key) = topic {
        let topic_config = config.content.topics.get(topic_key)
            .ok_or_else(|| WritingError::validation_error(format!("Topic '{}' not found", topic_key)))?;
        
        let topic_dir = Path::new(&config.content.base_dir).join(&topic_config.directory);
        
        // Look for an article with the given slug
        let article_dirs = common_fs::find_dirs_with_depth(&topic_dir, 1, 1)
            .map_err(|e| WritingError::validation_error(format!("Failed to list article directories: {}", e)))?;
        
        for article_dir in article_dirs {
            let dir_name = article_dir.file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| WritingError::validation_error("Invalid directory name"))?;
            
            if dir_name == slug {
                // Found the article directory, now find the content file
                let content_path = article_dir.join("index.md");
                if common_fs::path_exists(&content_path) {
                    return Ok(content_path);
                }
            }
        }
        
        Err(WritingError::content_not_found(format!("Content with slug '{}' not found in topic '{}'", slug, topic_key)))
    } else {
        // No topic provided, search all topics
        let base_dir = &config.content.base_dir;
        
        for (topic_key, topic_config) in &config.content.topics {
            let topic_dir = join_paths(base_dir, &topic_config.directory);
            
            // Look for an article with the given slug
            let article_dirs = common_fs::find_dirs_with_depth(&topic_dir, 1, 1)
                .map_err(|e| WritingError::validation_error(format!("Failed to list article directories in topic '{}': {}", topic_key, e)))?;
            
            for article_dir in article_dirs {
                let slug = article_dir.file_name()
                    .and_then(|name| name.to_str())
                    .ok_or_else(|| WritingError::validation_error(format!("Invalid directory name in topic '{}'", topic_key)))?
                    .to_string();
                
                // Look for index.md or index.mdx
                let index_md = article_dir.join("index.md");
                let index_mdx = article_dir.join("index.mdx");
                
                let content_path = if index_md.exists() {
                    index_md
                } else if index_mdx.exists() {
                    index_mdx
                } else {
                    continue;
                };
                
                // Extract title from frontmatter
                let content = common_fs::read_file(&content_path)
                    .map_err(|e| WritingError::validation_error(format!("Failed to read content: {}", e)))?;
                let title = match common_markdown::extract_frontmatter_and_content(&content) {
                    Ok((fm, _)) => {
                        if fm.title.is_empty() {
                            slug.clone()
                        } else {
                            fm.title
                        }
                    },
                    _ => slug.clone(),
                };
                
                content_list.push((topic_key.clone(), title, content_path));
            }
        }
        
        Err(WritingError::content_not_found(format!("Content with slug '{}' not found in any topic", slug)))
    }
}

/// List all content in the repository
pub fn list_all_content() -> Result<Vec<(String, String, PathBuf)>> {
    let config = common_config::load_config()
        .with_context(|| "Failed to load configuration".to_string())?;
    let mut content_list = Vec::new();
    
    for (topic_key, topic_config) in &config.content.topics {
        let base_dir = PathBuf::from(&config.content.base_dir);
        let topic_dir = join_paths(base_dir, &topic_config.directory);
        
        if !topic_dir.exists() {
            continue;
        }
        
        // Find all subdirectories in the topic directory
        let article_dirs = common_fs::find_dirs_with_depth(&topic_dir, 1, 1)
            .map_err(|e| WritingError::validation_error(format!("Failed to list content: {}", e)))?;
        
        for article_dir in article_dirs {
            let slug = article_dir.file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| WritingError::validation_error(format!("Invalid directory name in topic '{}'", topic_key)))?
                .to_string();
            
            // Look for index.md or index.mdx
            let index_md = article_dir.join("index.md");
            let index_mdx = article_dir.join("index.mdx");
            
            let content_path = if index_md.exists() {
                index_md
            } else if index_mdx.exists() {
                index_mdx
            } else {
                continue;
            };
            
            // Extract title from frontmatter
            let content = common_fs::read_file(&content_path)
                .map_err(|e| WritingError::validation_error(format!("Failed to read content: {}", e)))?;
            let title = match common_markdown::extract_frontmatter_and_content(&content) {
                Ok((fm, _)) => {
                    if fm.title.is_empty() {
                        slug.clone()
                    } else {
                        fm.title
                    }
                },
                _ => slug.clone(),
            };
            
            content_list.push((topic_key.clone(), title, content_path));
        }
    }
    
    Ok(content_list)
}

/// Edit content with the given options
pub fn edit_content(options: &EditOptions) -> Result<()> {
    let _config = common_config::load_config()
        .map_err(|e| WritingError::config_error(format!("Failed to load config: {}", e)))?;
    
    let slug = options.slug
        .as_ref()
        .ok_or_else(|| WritingError::validation_error("Slug is required for editing content"))?;
    
    let content_path = find_content_path(slug, options.topic.as_deref())?;
    
    let content = common_fs::read_file(&content_path)
        .map_err(|e| WritingError::validation_error(format!("Failed to read content file: {}", e)))?;
    
    // Get the title from frontmatter
    let title = match common_markdown::extract_frontmatter_and_content(&content) {
        Ok((fm, _)) => fm.title,
        _ => "Untitled".to_string(),
    };
    
    // Serialize frontmatter to YAML
    let _frontmatter_yaml = serde_yaml::to_string(&title)
        .map_err(|e| WritingError::validation_error(format!("Failed to serialize frontmatter: {}", e)))?;
    
    // TODO: Open the content file in the user's editor
    
    Ok(())
}

/// Save edited content
pub fn save_edited_content(content_path: &Path, edited_content: &str) -> Result<()> {
    // Check if we're editing only frontmatter or only content
    let is_frontmatter_only = edited_content.starts_with("---\n") && 
                             edited_content.trim_end().ends_with("---");
    
    let is_content_only = !edited_content.contains("---");
    
    if is_frontmatter_only || is_content_only {
        // Read the original content
        let original_content = common_fs::read_file(content_path)
            .map_err(|e| WritingError::validation_error(format!("Failed to read original content: {}", e)))?;
        
        // Split the original content
        let (frontmatter, body) = split_frontmatter_and_body(&original_content)
            .map_err(|e| WritingError::validation_error(format!("Failed to parse original content: {}", e)))?;
        
        // Merge the edited part with the original
        if is_frontmatter_only {
            // Parse the edited frontmatter
            let edited_frontmatter = edited_content.trim_start_matches("---\n").trim_end_matches("---\n").trim();
            
            // Write the merged content
            common_fs::write_file(content_path, &format!("---\n{}---\n\n{}", edited_frontmatter, body))
                .map_err(|e| WritingError::validation_error(format!("Failed to write merged content: {}", e)))?;
        } else {
            // Edited content is body only
            let frontmatter_yaml = serde_yaml::to_string(&frontmatter)
                .map_err(|e| WritingError::validation_error(format!("Failed to serialize frontmatter: {}", e)))?;
            
            // Write the merged content
            common_fs::write_file(content_path, &format!("---\n{}---\n\n{}", frontmatter_yaml, edited_content))
                .map_err(|e| WritingError::validation_error(format!("Failed to write merged content: {}", e)))?;
        }
        
        Ok(())
    } else {
        // Full content edit, just write it directly
        common_fs::write_file(content_path, edited_content)
            .map_err(|e| WritingError::validation_error(format!("Failed to write edited content: {}", e)))?;
        
        Ok(())
    }
}

/// Extract frontmatter from a markdown file
#[allow(dead_code)]
fn extract_frontmatter(path: &Path) -> Result<Frontmatter> {
    let content = common_fs::read_file(path)?;
    let (frontmatter, _) = split_frontmatter_and_body(&content)?;
    Ok(frontmatter)
}

/// Split content into frontmatter and body
fn split_frontmatter_and_body(content: &str) -> Result<(Frontmatter, String)> {
    let mut lines = content.lines();
    let mut frontmatter_str = String::new();
    let mut body = String::new();
    
    // Check if the content starts with frontmatter delimiter
    if let Some(first_line) = lines.next() {
        if first_line.trim() == "---" {
            let mut in_frontmatter = true;
            
            // Extract frontmatter and body
            for line in lines {
                if in_frontmatter {
                    if line.trim() == "---" {
                        in_frontmatter = false;
                    } else {
                        frontmatter_str.push_str(line);
                        frontmatter_str.push('\n');
                    }
                } else {
                    body.push_str(line);
                    body.push('\n');
                }
            }
        } else {
            // No frontmatter, return default and original content
            return Ok((Frontmatter::default(), content.to_string()));
        }
    } else {
        // Empty content
        return Ok((Frontmatter::default(), String::new()));
    }
    
    // Parse frontmatter
    let frontmatter = match serde_yaml::from_str(&frontmatter_str) {
        Ok(fm) => fm,
        Err(e) => {
            return Err(WritingError::validation_error(format!("Invalid frontmatter: {}", e)));
        }
    };
    
    Ok((frontmatter, body))
}

/// Update content with new frontmatter and/or content
pub fn update_content(path: &Path, frontmatter: Option<Frontmatter>, content: Option<&str>) -> Result<()> {
    if frontmatter.is_none() && content.is_none() {
        return Ok(());
    }
    
    // Read the original content
    let original_content = common_fs::read_file(path)
        .map_err(|e| WritingError::validation_error(format!("Failed to read original content: {}", e)))?;
    
    // Parse the original content
    let (original_frontmatter, original_content_text) = common_markdown::extract_frontmatter_and_content(&original_content)
        .map_err(|e| WritingError::validation_error(format!("Failed to parse original content: {}", e)))?;
    
    // Merge frontmatter
    let merged_frontmatter = if let Some(new_frontmatter) = frontmatter {
        new_frontmatter
    } else {
        original_frontmatter
    };
    
    // Merge content
    let merged_content = if let Some(new_content) = content {
        new_content
    } else {
        &original_content_text
    };
    
    // Write the merged content
    let frontmatter_yaml = serde_yaml::to_string(&merged_frontmatter)
        .map_err(|e| WritingError::validation_error(format!("Failed to serialize frontmatter: {}", e)))?;
    
    let final_content = format!("---\n{}---\n\n{}", frontmatter_yaml, merged_content);
    
    common_fs::write_file(path, &final_content)
        .map_err(|e| WritingError::validation_error(format!("Failed to write merged content: {}", e)))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_test_utils::TestFixture;
    use common_fs::normalize::normalize_path;
    use std::fs;

    /// Test finding content path with topic
    #[test]
    fn test_find_content_path_with_topic() {
        // Create a test fixture
        let fixture = TestFixture::new().unwrap();
        
        // Create test content
        let content_file = fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
        
        // Find content path
        let found_path = find_content_path("test-post", Some("blog")).unwrap();
        
        // Verify path
        assert_eq!(normalize_path(found_path), normalize_path(content_file));
    }
    
    /// Test finding content path without topic
    #[test]
    fn test_find_content_path_without_topic() {
        // Create a test fixture
        let fixture = TestFixture::new().unwrap();
        
        // Create test content
        let content_file = fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
        
        // Find content path
        let found_path = find_content_path("test-post", None).unwrap();
        
        // Verify path
        assert_eq!(normalize_path(found_path), normalize_path(content_file));
    }
    
    /// Test finding content path with invalid topic
    #[test]
    fn test_find_content_path_invalid_topic() {
        // Create a test fixture
        let fixture = TestFixture::new().unwrap();
        
        // Find content path with invalid topic
        let result = find_content_path("test-post", Some("invalid-topic"));
        
        // Verify error
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Invalid topic"));
    }
    
    /// Test finding content path with nonexistent content
    #[test]
    fn test_find_content_path_nonexistent() {
        // Create a test fixture
        let fixture = TestFixture::new().unwrap();
        
        // Find nonexistent content
        let result = find_content_path("nonexistent-post", None);
        
        // Verify error
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Content not found"));
    }
    
    /// Test listing all content
    #[test]
    fn test_list_all_content() {
        // Create a test fixture
        let fixture = TestFixture::new().unwrap();
        
        // Create test content
        fixture.create_content("blog", "post1", "Post 1", false).unwrap();
        fixture.create_content("blog", "post2", "Post 2", false).unwrap();
        fixture.create_content("notes", "note1", "Note 1", false).unwrap();
        
        // List all content
        let content_list = list_all_content().unwrap();
        
        // Verify content list
        assert_eq!(content_list.len(), 3);
        
        // Verify content items
        let titles: Vec<String> = content_list.iter()
            .map(|(_, title, _)| title.clone())
            .collect();
        
        assert!(titles.contains(&"Post 1".to_string()));
        assert!(titles.contains(&"Post 2".to_string()));
        assert!(titles.contains(&"Note 1".to_string()));
    }
    
    /// Test editing content
    #[test]
    fn test_edit_content() {
        // Create a test fixture
        let fixture = TestFixture::new().unwrap();
        
        // Create test content
        let content_file = fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
        
        // Create edit options
        let options = EditOptions {
            slug: Some("test-post".to_string()),
            topic: Some("blog".to_string()),
            frontmatter_only: false,
            content_only: false,
        };
        
        // Edit content
        let result = edit_content(&options);
        
        // Verify result
        assert!(result.is_ok());
    }
    
    /// Test editing frontmatter only
    #[test]
    fn test_edit_frontmatter_only() {
        // Create a test fixture
        let fixture = TestFixture::new().unwrap();
        
        // Create test content
        fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
        
        // Create edit options
        let options = EditOptions {
            slug: Some("test-post".to_string()),
            topic: Some("blog".to_string()),
            frontmatter_only: true,
            content_only: false,
        };
        
        // Edit content
        let result = edit_content(&options);
        
        // Verify result
        assert!(result.is_ok());
    }
    
    /// Test editing content only
    #[test]
    fn test_edit_content_only() {
        // Create a test fixture
        let fixture = TestFixture::new().unwrap();
        
        // Create test content with body
        let content_file = fixture.content_dir.join("blog/test-post/index.mdx");
        let content = "---\ntitle: \"Test Post\"\n---\n\n# Test Post\n\nThis is a test post.";
        fs::create_dir_all(content_file.parent().unwrap()).unwrap();
        fs::write(&content_file, content).unwrap();
        
        // Create edit options
        let options = EditOptions {
            slug: Some("test-post".to_string()),
            topic: Some("blog".to_string()),
            frontmatter_only: false,
            content_only: true,
        };
        
        // Edit content
        let result = edit_content(&options);
        
        // Verify result
        assert!(result.is_ok());
    }
    
    /// Test saving edited content
    #[test]
    fn test_save_edited_content() {
        // Create a test fixture
        let fixture = TestFixture::new().unwrap();
        
        // Create test content
        let content_file = fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
        
        // Edit content
        let edited_content = "---\ntitle: \"Edited Post\"\n---\n\n# Edited Post\n\nThis post has been edited.";
        
        // Save edited content
        save_edited_content(&content_file, edited_content).unwrap();
        
        // Verify content was saved
        let saved_content = fs::read_to_string(content_file).unwrap();
        assert_eq!(saved_content, edited_content);
    }
    
    /// Test saving edited frontmatter
    #[test]
    fn test_save_edited_frontmatter() {
        // Create a test fixture
        let fixture = TestFixture::new().unwrap();
        
        // Create test content with body
        let content_file = fixture.content_dir.join("blog/test-post/index.mdx");
        let content = "---\ntitle: \"Test Post\"\n---\n\n# Test Post\n\nThis is a test post.";
        fs::create_dir_all(content_file.parent().unwrap()).unwrap();
        fs::write(&content_file, content).unwrap();
        
        // Edit frontmatter
        let edited_frontmatter = "---\ntitle: \"Edited Post\"\ntags:\n  - test\n---";
        
        // Save edited frontmatter
        save_edited_content(&content_file, edited_frontmatter).unwrap();
        
        // Verify content was saved with original body
        let saved_content = fs::read_to_string(content_file).unwrap();
        assert!(saved_content.contains("title: \"Edited Post\""));
        assert!(saved_content.contains("tags:"));
        assert!(saved_content.contains("# Test Post"));
        assert!(saved_content.contains("This is a test post."));
    }
    
    /// Test saving edited body
    #[test]
    fn test_save_edited_body() {
        // Create a test fixture
        let fixture = TestFixture::new().unwrap();
        
        // Create test content with body
        let content_file = fixture.content_dir.join("blog/test-post/index.mdx");
        let content = "---\ntitle: \"Test Post\"\n---\n\n# Test Post\n\nThis is a test post.";
        fs::create_dir_all(content_file.parent().unwrap()).unwrap();
        fs::write(&content_file, content).unwrap();
        
        // Edit body
        let edited_body = "# Edited Post\n\nThis post has been edited.";
        
        // Save edited body
        save_edited_content(&content_file, edited_body).unwrap();
        
        // Verify content was saved with original frontmatter
        let saved_content = fs::read_to_string(content_file).unwrap();
        assert!(saved_content.contains("title: \"Test Post\""));
        assert!(saved_content.contains("# Edited Post"));
        assert!(saved_content.contains("This post has been edited."));
        assert!(!saved_content.contains("This is a test post."));
    }
} 