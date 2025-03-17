//! # Test Fixtures
//! 
//! This module provides reusable test fixtures for common testing patterns.

use common_errors::Result;
use common_models::Frontmatter;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::{tempdir, TempDir};

/// Fixture for validation testing
pub struct ValidationFixture {
    pub temp_dir: TempDir,
    pub valid_examples: HashMap<String, Vec<String>>,
    pub invalid_examples: HashMap<String, Vec<String>>,
}

impl ValidationFixture {
    /// Create a new validation fixture with default examples
    pub fn new() -> Result<Self> {
        let temp_dir = tempdir()?;
        
        // Initialize with some default examples
        let mut fixture = Self {
            temp_dir,
            valid_examples: HashMap::new(),
            invalid_examples: HashMap::new(),
        };
        
        // Add default valid examples
        fixture.add_valid_examples("slug", vec![
            "valid-slug".to_string(),
            "another-valid-slug".to_string(),
            "123-numbers-allowed".to_string(),
            "a".to_string(), // Single character slug
        ]);
        
        // Add default invalid examples
        fixture.add_invalid_examples("slug", vec![
            "".to_string(), // Empty string
            "Invalid Slug With Spaces".to_string(),
            "UPPERCASE-NOT-ALLOWED".to_string(),
            "special@characters!not$allowed".to_string(),
            "slug-with--consecutive-hyphens".to_string(),
            "very-long-slug-".to_string() + &"a".repeat(101), // Too long (just over the 100 char limit)
        ]);
        
        // Add valid title examples
        fixture.add_valid_examples("title", vec![
            "Valid Title".to_string(),
            "Another Valid Title with Punctuation!".to_string(),
            "Title with 123 Numbers".to_string(),
        ]);
        
        // Add invalid title examples
        fixture.add_invalid_examples("title", vec![
            "".to_string(), // Empty string
            "   ".to_string(), // Just whitespace
            "a".repeat(1000), // Too long
        ]);
        
        // Add valid date examples
        fixture.add_valid_examples("date", vec![
            "2023-01-01".to_string(),
            "2020-12-31".to_string(),
            "2000-02-29".to_string(), // Leap year
        ]);
        
        // Add invalid date examples
        fixture.add_invalid_examples("date", vec![
            "".to_string(), // Empty string
            "2023/01/01".to_string(), // Wrong format
            "2023-13-01".to_string(), // Invalid month
            "2023-01-32".to_string(), // Invalid day
            "23-1-1".to_string(), // Incorrect format
            "not-a-date".to_string(),
        ]);
        
        // Add valid tags examples
        fixture.add_valid_examples("tags", vec![
            "tag1, tag2, tag3".to_string(),
            "single-tag".to_string(),
            "".to_string(), // Empty tags are valid
        ]);
        
        // Add invalid tags examples
        fixture.add_invalid_examples("tags", vec![
            "Invalid Tag".to_string(), // Space in tag
            "tag1, UPPERCASE, tag3".to_string(), // Uppercase not allowed
            "tag1, tag2, tag2".to_string(), // Duplicate tags
        ]);
        
        Ok(fixture)
    }
    
    /// Add examples of valid inputs for a specific validation type
    pub fn add_valid_examples(&mut self, validation_type: &str, examples: Vec<String>) {
        self.valid_examples.insert(validation_type.to_string(), examples);
    }
    
    /// Add examples of invalid inputs for a specific validation type
    pub fn add_invalid_examples(&mut self, validation_type: &str, examples: Vec<String>) {
        self.invalid_examples.insert(validation_type.to_string(), examples);
    }
    
    /// Get valid examples for a specific validation type
    pub fn get_valid_examples(&self, validation_type: &str) -> Vec<String> {
        self.valid_examples
            .get(validation_type)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get invalid examples for a specific validation type
    pub fn get_invalid_examples(&self, validation_type: &str) -> Vec<String> {
        self.invalid_examples
            .get(validation_type)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Create a file with a specific example
    pub fn create_example_file(&self, validation_type: &str, example: &str, is_valid: bool) -> Result<PathBuf> {
        let validity = if is_valid { "valid" } else { "invalid" };
        let filename = format!("{}_{}.txt", validation_type, validity);
        let file_path = self.temp_dir.path().join(filename);
        
        let mut file = fs::File::create(&file_path)?;
        file.write_all(example.as_bytes())?;
        
        Ok(file_path)
    }
}

/// Fixture for file system operations testing
pub struct FileSystemFixture {
    pub temp_dir: TempDir,
    pub content_dir: PathBuf,
    pub config_dir: PathBuf,
}

impl FileSystemFixture {
    /// Create a new file system fixture
    pub fn new() -> Result<Self> {
        let temp_dir = tempdir()?;
        let content_dir = temp_dir.path().join("content");
        let config_dir = temp_dir.path().join("config");
        
        fs::create_dir_all(&content_dir)?;
        fs::create_dir_all(&config_dir)?;
        
        Ok(Self {
            temp_dir,
            content_dir,
            config_dir,
        })
    }
    
    /// Create a file with specific content
    pub fn create_file(&self, relative_path: &str, content: &str) -> Result<PathBuf> {
        let file_path = self.temp_dir.path().join(relative_path);
        
        // Create parent directories if needed
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let mut file = fs::File::create(&file_path)?;
        file.write_all(content.as_bytes())?;
        
        Ok(file_path)
    }
    
    /// Create a content file with frontmatter
    pub fn create_content_file(&self, topic: &str, slug: &str, frontmatter: &Frontmatter, content: &str) -> Result<PathBuf> {
        let relative_path = format!("content/{}/{}.md", topic, slug);
        
        // Combine frontmatter and content
        let full_content = format!(
            "---\ntitle: {}\n",
            frontmatter.title
        );
        
        let full_content = if let Some(published) = &frontmatter.published {
            format!("{}published: {}\n", full_content, published)
        } else {
            full_content
        };
        
        let full_content = if let Some(updated) = &frontmatter.updated {
            format!("{}updated: {}\n", full_content, updated)
        } else {
            full_content
        };
        
        let full_content = if let Some(tagline) = &frontmatter.tagline {
            format!("{}tagline: {}\n", full_content, tagline)
        } else {
            full_content
        };
        
        let full_content = if let Some(tags) = &frontmatter.tags {
            // Convert Vec<String> to a comma-separated string
            let tags_str = tags.join(", ");
            format!("{}tags: {}\n", full_content, tags_str)
        } else {
            full_content
        };
        
        let full_content = if let Some(draft) = frontmatter.draft {
            if draft {
                format!("{}draft: true\n", full_content)
            } else {
                full_content
            }
        } else {
            full_content
        };
        
        let full_content = format!("{}---\n\n{}", full_content, content);
        
        self.create_file(&relative_path, &full_content)
    }
    
    /// Create a directory structure
    pub fn create_directory(&self, relative_path: &str) -> Result<PathBuf> {
        let dir_path = self.temp_dir.path().join(relative_path);
        fs::create_dir_all(&dir_path)?;
        Ok(dir_path)
    }
    
    /// Get the absolute path to a relative path in the fixture
    pub fn abs_path(&self, relative_path: &str) -> PathBuf {
        self.temp_dir.path().join(relative_path)
    }
    
    /// Check if a file exists
    pub fn file_exists(&self, relative_path: &str) -> bool {
        self.abs_path(relative_path).exists() && self.abs_path(relative_path).is_file()
    }
    
    /// Check if a directory exists
    pub fn dir_exists(&self, relative_path: &str) -> bool {
        self.abs_path(relative_path).exists() && self.abs_path(relative_path).is_dir()
    }
    
    /// Read a file's contents
    pub fn read_file(&self, relative_path: &str) -> Result<String> {
        let content = fs::read_to_string(self.abs_path(relative_path))?;
        Ok(content)
    }
} 