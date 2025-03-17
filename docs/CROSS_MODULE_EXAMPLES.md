# Cross-Module Examples

This document provides examples of how different modules in the Writing project interact with each other. These examples demonstrate common usage patterns and help developers understand how to use the various components together.

## Table of Contents

1. [Content Creation and Management](#content-creation-and-management)
2. [Error Handling and Validation](#error-handling-and-validation)
3. [Configuration and File System](#configuration-and-file-system)
4. [Command Line Interface](#command-line-interface)
5. [Image Processing](#image-processing)
6. [Testing](#testing)

## Content Creation and Management

### Creating and Editing Content

This example shows how to create and edit content using the `content-new` and `content-edit` tools, along with the common libraries.

```rust
use common_config::Config;
use common_fs::find_content_path;
use common_errors::{Result, WritingError, ErrorContext, IoResultExt};
use common_markdown::frontmatter::{parse_frontmatter, update_frontmatter};
use std::fs;
use std::path::Path;

/// Create a new content file
fn create_content(config: &Config, topic: &str, slug: &str, title: &str) -> Result<()> {
    // Validate that the topic exists
    let topic_config = config.content.topics.get(topic)
        .ok_or_else(|| WritingError::topic_error(format!("Topic '{}' not found", topic)))?;
    
    // Create the content directory
    let content_dir = Path::new(&config.content.base_dir)
        .join(&topic_config.path)
        .join(slug);
    
    fs::create_dir_all(&content_dir)
        .with_enhanced_context(|| {
            ErrorContext::new("create_content_directory")
                .with_file(&content_dir)
        })?;
    
    // Create the content file with frontmatter
    let content_file = content_dir.join("index.mdx");
    let frontmatter = format!(
        r#"---
title: "{}"
date: "{}"
draft: true
---

# {}

Write your content here.
"#,
        title,
        chrono::Local::now().format("%Y-%m-%d"),
        title
    );
    
    fs::write(&content_file, frontmatter)
        .with_enhanced_context(|| {
            ErrorContext::new("write_content_file")
                .with_file(&content_file)
        })?;
    
    println!("Created content at {}", content_file.display());
    Ok(())
}

/// Edit content frontmatter
fn update_content_title(config: &Config, topic: &str, slug: &str, new_title: &str) -> Result<()> {
    // Find the content path
    let content_path = find_content_path(config, topic, slug)?;
    
    // Read the content file
    let content = fs::read_to_string(&content_path)
        .with_enhanced_context(|| {
            ErrorContext::new("read_content_file")
                .with_file(&content_path)
        })?;
    
    // Parse and update the frontmatter
    let (frontmatter, content_body) = parse_frontmatter(&content)?;
    let mut frontmatter = frontmatter.clone();
    frontmatter.insert("title".to_string(), new_title.to_string());
    
    // Write the updated content
    let updated_content = update_frontmatter(&frontmatter, content_body);
    fs::write(&content_path, updated_content)
        .with_enhanced_context(|| {
            ErrorContext::new("write_updated_content")
                .with_file(&content_path)
        })?;
    
    println!("Updated title to '{}' in {}", new_title, content_path.display());
    Ok(())
}
```

## Error Handling and Validation

### Comprehensive Error Handling

This example demonstrates how to use the error handling and validation utilities together.

```rust
use common_errors::{Result, WritingError, ResultExt, OptionValidationExt, ErrorContext, IoResultExt};
use common_config::Config;
use std::fs;
use std::path::Path;

/// Process a configuration file with comprehensive error handling
fn process_config(config_path: &Path) -> Result<Config> {
    // Check if the file exists
    if !config_path.exists() {
        return Err(WritingError::file_not_found(config_path));
    }
    
    // Read the config file with context
    let config_str = fs::read_to_string(config_path)
        .with_enhanced_context(|| {
            ErrorContext::new("read_config_file")
                .with_file(config_path)
                .with_details("Failed to read configuration file")
        })?;
    
    // Parse the config with context
    let config: Config = serde_yaml::from_str(&config_str)
        .with_context(|| format!("Failed to parse config file: {}", config_path.display()))?;
    
    // Validate required fields
    let base_dir = config.content.base_dir.clone();
    if base_dir.is_empty() {
        return Err(WritingError::validation_error("Content base directory cannot be empty"));
    }
    
    // Validate that the base directory exists
    let base_dir_path = Path::new(&base_dir);
    if !base_dir_path.exists() {
        return Err(WritingError::directory_not_found(base_dir_path));
    }
    
    // Validate that there is at least one topic
    if config.content.topics.is_empty() {
        return Err(WritingError::validation_error("At least one topic must be defined"));
    }
    
    // Process optional fields with validation
    let site_url = config.publication.site
        .validate_with(|| WritingError::validation_error("Site URL is required"))?;
    
    println!("Config validated successfully. Site URL: {}", site_url);
    Ok(config)
}
```

## Configuration and File System

### Loading Configuration and Working with Files

This example shows how to load configuration and work with files using the common libraries.

```rust
use common_config::{Config, load_config};
use common_fs::{find_content_files, read_content_file};
use common_errors::{Result, ErrorContext, IoResultExt};
use std::fs;
use std::path::Path;

/// List all content files for a topic
fn list_topic_content(config_path: &Path, topic: &str) -> Result<()> {
    // Load the configuration
    let config = load_config(config_path)?;
    
    // Validate that the topic exists
    let topic_config = config.content.topics.get(topic)
        .ok_or_else(|| common_errors::WritingError::topic_error(format!("Topic '{}' not found", topic)))?;
    
    // Find all content files for the topic
    let topic_path = Path::new(&config.content.base_dir).join(&topic_config.path);
    let content_files = find_content_files(&topic_path)?;
    
    println!("Content files for topic '{}':", topic);
    for file in content_files {
        // Read the content file
        let content = read_content_file(&file)?;
        
        // Extract the title from frontmatter
        let title = content.frontmatter.title.clone();
        let published = content.frontmatter.published
            .as_deref()
            .unwrap_or("Unpublished");
        
        println!("- {} ({}): {}", file.display(), published, title);
    }
    
    Ok(())
}

/// Create a backup of all content
fn backup_content(config_path: &Path, backup_dir: &Path) -> Result<()> {
    // Load the configuration
    let config = load_config(config_path)?;
    
    // Create the backup directory
    fs::create_dir_all(backup_dir)
        .with_enhanced_context(|| {
            ErrorContext::new("create_backup_directory")
                .with_file(backup_dir)
        })?;
    
    // Iterate through all topics
    for (topic_id, topic_config) in &config.content.topics {
        let topic_path = Path::new(&config.content.base_dir).join(&topic_config.path);
        let backup_topic_path = backup_dir.join(topic_id);
        
        // Create the topic backup directory
        fs::create_dir_all(&backup_topic_path)
            .with_enhanced_context(|| {
                ErrorContext::new("create_backup_topic_directory")
                    .with_file(&backup_topic_path)
            })?;
        
        // Find all content files for the topic
        let content_files = find_content_files(&topic_path)?;
        
        // Copy each content file to the backup directory
        for file in content_files {
            let relative_path = file.strip_prefix(&topic_path).unwrap();
            let backup_file = backup_topic_path.join(relative_path);
            
            // Create parent directories
            if let Some(parent) = backup_file.parent() {
                fs::create_dir_all(parent)
                    .with_enhanced_context(|| {
                        ErrorContext::new("create_backup_parent_directory")
                            .with_file(parent)
                    })?;
            }
            
            // Copy the file
            fs::copy(&file, &backup_file)
                .with_enhanced_context(|| {
                    ErrorContext::new("copy_file_to_backup")
                        .with_file(&file)
                        .with_details(format!("Copying to {}", backup_file.display()))
                })?;
        }
    }
    
    println!("Backup completed to {}", backup_dir.display());
    Ok(())
}
```

## Command Line Interface

### Creating a Command with the Command Pattern

This example demonstrates how to create a command using the standardized command pattern.

```rust
use common_cli::{Command, DisplayResult};
use common_config::Config;
use common_errors::Result;
use clap::{Parser, Subcommand};

/// Command line arguments
#[derive(Parser)]
struct Args {
    /// Path to the configuration file
    #[clap(short, long, default_value = "config.yaml")]
    config: String,
    
    #[clap(subcommand)]
    command: Commands,
}

/// Available commands
#[derive(Subcommand)]
enum Commands {
    /// List all topics
    ListTopics,
    
    /// List content for a topic
    ListContent {
        /// Topic ID
        #[clap(short, long)]
        topic: String,
    },
}

/// List topics command
struct ListTopicsCommand {
    config: Config,
}

impl Command for ListTopicsCommand {
    type Args = ();
    
    fn new(_args: Self::Args, config: Config) -> Result<Self> {
        Ok(Self { config })
    }
    
    fn execute(&self) -> Result<DisplayResult> {
        let mut topics = Vec::new();
        
        for (id, topic) in &self.config.content.topics {
            topics.push(format!("{}: {}", id, topic.name));
        }
        
        Ok(DisplayResult::List {
            title: "Available Topics".to_string(),
            items: topics,
        })
    }
}

/// List content command
struct ListContentCommand {
    config: Config,
    topic: String,
}

impl Command for ListContentCommand {
    type Args = String;
    
    fn new(topic: Self::Args, config: Config) -> Result<Self> {
        Ok(Self { config, topic })
    }
    
    fn execute(&self) -> Result<DisplayResult> {
        // Validate that the topic exists
        let topic_config = self.config.content.topics.get(&self.topic)
            .ok_or_else(|| common_errors::WritingError::topic_error(format!("Topic '{}' not found", self.topic)))?;
        
        // Find all content files for the topic
        let topic_path = std::path::Path::new(&self.config.content.base_dir).join(&topic_config.path);
        let content_files = common_fs::find_content_files(&topic_path)?;
        
        let mut items = Vec::new();
        for file in content_files {
            // Read the content file
            let content = common_fs::read_content_file(&file)?;
            
            // Extract the title from frontmatter
            let title = content.frontmatter.title.clone();
            let published = content.frontmatter.published
                .as_deref()
                .unwrap_or("Unpublished");
            
            items.push(format!("{} ({})", title, published));
        }
        
        Ok(DisplayResult::List {
            title: format!("Content for topic '{}'", topic_config.name),
            items,
        })
    }
}

/// Main function
fn main() -> Result<()> {
    let args = Args::parse();
    
    // Load the configuration
    let config = common_config::load_config(&args.config)?;
    
    // Execute the appropriate command
    match args.command {
        Commands::ListTopics => {
            let command = ListTopicsCommand::new((), config)?;
            let result = command.execute()?;
            println!("{}", result);
        },
        Commands::ListContent { topic } => {
            let command = ListContentCommand::new(topic, config)?;
            let result = command.execute()?;
            println!("{}", result);
        },
    }
    
    Ok(())
}
```

## Image Processing

### Optimizing Images with Feature Flags

This example demonstrates how to use the image optimization functionality with feature flags.

```rust
use common_config::Config;
use common_errors::{Result, ErrorContext, IoResultExt};
use std::path::Path;

/// Optimize images for a content directory
fn optimize_images(config_path: &Path, topic: &str, slug: &str) -> Result<()> {
    // Load the configuration
    let config = common_config::load_config(config_path)?;
    
    // Find the content path
    let content_path = common_fs::find_content_path(&config, topic, slug)?;
    let images_dir = content_path.parent().unwrap().join("images");
    
    // Check if the images directory exists
    if !images_dir.exists() {
        return Ok(());
    }
    
    // Find all image files
    let image_files = std::fs::read_dir(&images_dir)
        .with_enhanced_context(|| {
            ErrorContext::new("read_images_directory")
                .with_file(&images_dir)
        })?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && is_image_file(&path) {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    
    // Optimize each image
    for image_path in image_files {
        println!("Optimizing image: {}", image_path.display());
        
        // Create optimized versions for each size
        for (size_id, size_config) in &config.images.sizes {
            // Create the optimized image for each format
            for format in &config.images.formats {
                #[cfg(feature = "webp")]
                if format == "webp" {
                    optimize_webp(&image_path, &images_dir, size_id, size_config, &config)?;
                }
                
                #[cfg(feature = "avif")]
                if format == "avif" {
                    optimize_avif(&image_path, &images_dir, size_id, size_config, &config)?;
                }
                
                // JPEG is always supported
                if format == "jpg" || format == "jpeg" {
                    optimize_jpeg(&image_path, &images_dir, size_id, size_config, &config)?;
                }
            }
        }
    }
    
    Ok(())
}

/// Check if a file is an image
fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        return ext == "jpg" || ext == "jpeg" || ext == "png" || ext == "gif";
    }
    false
}

/// Optimize an image as JPEG
fn optimize_jpeg(
    image_path: &Path,
    output_dir: &Path,
    size_id: &str,
    size_config: &common_models::ImageSize,
    config: &Config,
) -> Result<()> {
    // Implementation details...
    Ok(())
}

#[cfg(feature = "webp")]
fn optimize_webp(
    image_path: &Path,
    output_dir: &Path,
    size_id: &str,
    size_config: &common_models::ImageSize,
    config: &Config,
) -> Result<()> {
    // Implementation details...
    Ok(())
}

#[cfg(feature = "avif")]
fn optimize_avif(
    image_path: &Path,
    output_dir: &Path,
    size_id: &str,
    size_config: &common_models::ImageSize,
    config: &Config,
) -> Result<()> {
    // Implementation details...
    Ok(())
}
```

## Testing

### Integration Testing with Mock Objects

This example demonstrates how to use the testing utilities for integration testing with mock objects.

```rust
use common_test_utils::mocks::{MockFileSystem, MockConfigLoader, MockContentOperations};
use common_test_utils::mocks::traits::{FileSystem, ConfigLoader, ContentOperations};
use common_models::{Config, ContentConfig, PublicationConfig, Article, Frontmatter};
use common_errors::Result;
use std::collections::HashMap;
use std::path::Path;

/// Content processor that can be tested with mocks
struct ContentProcessor<F: FileSystem, C: ConfigLoader, O: ContentOperations> {
    fs: F,
    config_loader: C,
    content_ops: O,
}

impl<F: FileSystem, C: ConfigLoader, O: ContentOperations> ContentProcessor<F, C, O> {
    /// Create a new content processor
    fn new(fs: F, config_loader: C, content_ops: O) -> Self {
        Self { fs, config_loader, content_ops }
    }
    
    /// Process content
    fn process_content(&mut self, config_path: &Path, topic: &str, slug: &str) -> Result<()> {
        // Load the configuration
        let config = self.config_loader.load_config(config_path)?;
        
        // Get the article
        let article = self.content_ops.get_article(topic, slug)
            .ok_or_else(|| common_errors::WritingError::content_not_found(format!("Article {}/{} not found", topic, slug)))?;
        
        // Process the content
        let processed_content = format!("# {}\n\n{}", article.frontmatter.title, article.content);
        
        // Write the processed content
        let output_path = Path::new("output").join(topic).join(slug).join("index.html");
        self.fs.write_file(output_path, processed_content)?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_process_content() {
        // Create mock objects
        let mut mock_fs = MockFileSystem::new();
        
        let config = Config {
            content: ContentConfig {
                base_dir: "/content".to_string(),
                topics: HashMap::new(),
                tags: None,
            },
            images: common_models::ImageConfig {
                formats: vec!["jpg".to_string()],
                format_descriptions: None,
                sizes: HashMap::new(),
                naming: None,
                quality: None,
            },
            publication: PublicationConfig {
                author: "Test Author".to_string(),
                copyright: "Test Copyright".to_string(),
                site: None,
            },
        };
        let mock_config = MockConfigLoader::new(config);
        
        let mut mock_content = MockContentOperations::new();
        
        // Add a test article
        let article = Article {
            frontmatter: Frontmatter {
                title: "Test Article".to_string(),
                published: Some("2023-01-01".to_string()),
                updated: None,
                slug: Some("test-article".to_string()),
                tagline: None,
                tags: Some(vec!["test".to_string()]),
                topics: Some(vec!["blog".to_string()]),
                draft: Some(false),
                featured_image: None,
            },
            content: "This is a test article.".to_string(),
            slug: "test-article".to_string(),
            topic: "blog".to_string(),
            path: "/content/blog/test-article".to_string(),
            word_count: Some(7),
            reading_time: Some(1),
        };
        mock_content.add_article(article);
        
        // Create the processor
        let mut processor = ContentProcessor::new(mock_fs.clone(), mock_config, mock_content);
        
        // Process the content
        let result = processor.process_content(Path::new("/config.yaml"), "blog", "test-article");
        assert!(result.is_ok());
        
        // Verify the output
        assert!(mock_fs.file_exists("/output/blog/test-article/index.html"));
        let content = mock_fs.read_file("/output/blog/test-article/index.html").unwrap();
        assert!(content.contains("# Test Article"));
        assert!(content.contains("This is a test article."));
    }
}
```

These examples demonstrate how different modules in the Writing project interact with each other. They provide a starting point for developers to understand how to use the various components together and implement common functionality. 