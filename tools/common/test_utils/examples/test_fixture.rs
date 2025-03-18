/// Example of how to create and use a TestFixture
///
/// This example demonstrates how to create a test fixture that includes
/// configuration and a temporary file system for testing.

use std::collections::HashMap;
use std::path::Path;
use anyhow::Result;
use common_models::{Config, ContentConfig, PublicationConfig, ImageConfig};
use common_test_utils::mocks::{MockFileSystem, MockConfigLoader};
use common_test_utils::mocks::traits::{FileSystem, ConfigLoader};

/// A simple test fixture structure to replace the one used in tests
pub struct TestFixture {
    pub fs: MockFileSystem,
    pub config: MockConfigLoader,
    pub temp_dir: tempfile::TempDir,
}

impl TestFixture {
    /// Create a new test fixture with default configuration
    pub fn new() -> Result<Self> {
        let temp_dir = tempfile::tempdir()?;

        // Create a basic file system mock
        let fs = MockFileSystem::new();

        // Create basic configuration
        let config = Config {
            content: ContentConfig {
                base_dir: "content".to_string(),
                topics: HashMap::new(),
                tags: None,
            },
            images: ImageConfig {
                formats: vec!["jpg".to_string()],
                format_descriptions: None,
                sizes: HashMap::new(),
                naming: None,
                quality: None,
            },
            publication: PublicationConfig {
                author: "Test Author".to_string(),
                copyright: "Test Copyright".to_string(),
                site_url: None,
            },
        };

        let config_loader = MockConfigLoader::new(config);

        Ok(Self {
            fs,
            config: config_loader,
            temp_dir,
        })
    }

    /// Add a custom configuration to the fixture
    pub fn with_config(&mut self, config_yaml: &str) -> Result<&mut Self> {
        // Create a config file
        let config_path = self.temp_dir.path().join("config.yaml");
        std::fs::write(&config_path, config_yaml)?;

        // Parse the YAML into a Config object
        let config = serde_yaml::from_str::<Config>(config_yaml)?;

        // Update the mock config loader
        self.config.set_config(config);

        Ok(self)
    }

    /// Get the path to the temporary directory
    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }
}

fn main() -> Result<()> {
    // Create a new test fixture
    let mut fixture = TestFixture::new()?;

    // Add a custom configuration
    fixture.with_config(r#"
content:
  base_dir: content
  topics:
    blog:
      name: Blog
      description: Blog posts
      directory: content/blog
    notes:
      name: Notes
      description: Notes and thoughts
      directory: content/notes
  tags:
    categories:
      - writing
      - tech
      - personal
images:
  formats:
    - jpg
    - webp
  sizes:
    standard:
      width: 800
      height: 600
      description: Standard size
    thumbnail:
      width: 200
      height: 150
      description: Thumbnail size
publication:
  author: Example Author
  copyright: Copyright 2023
  site_url: https://example.com
    "#)?;

    // Create some test directories in our mock file system
    fixture.fs.create_directory(Path::new("/content/blog"));
    fixture.fs.create_directory(Path::new("/content/notes"));

    // Create some test files
    fixture.fs.add_file("/content/blog/post1.md",
        "---\ntitle: Test Post\n---\n\nThis is a test post");

    // Use the fixture in tests
    let fs: &dyn FileSystem = &fixture.fs;
    let config: &dyn ConfigLoader = &fixture.config;

    // Test file operations
    assert!(fs.file_exists(Path::new("/content/blog/post1.md")));

    // Test config operations
    let loaded_config = config.load_config(Path::new("/config.yaml"))?;
    assert_eq!(loaded_config.publication.author, "Example Author");

    println!("All tests passed!");

    Ok(())
}