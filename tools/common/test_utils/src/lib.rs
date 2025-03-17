//! # Common Test Utilities
//! 
//! This module provides common test utilities for the writing tools.
//! 
//! ## Features
//! 
//! - Test fixture creation with temporary directories
//! - Configuration generation for tests
//! - Content file creation for tests
//! - Integration test patterns and utilities
//! - Mock implementations for unit testing
//! - Property-based testing utilities
//! - Specialized test fixtures for validation and file system testing
//! 
//! ## Example
//! 
//! ```rust
//! use common_test_utils::TestFixture;
//! 
//! #[test]
//! fn test_something() {
//!     let fixture = TestFixture::new().unwrap();
//!     
//!     // Create test content
//!     let content_file = fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
//!     
//!     // Test something with the content
//!     assert!(content_file.exists());
//!     
//!     // The fixture will be cleaned up automatically when it goes out of scope
//! }
//! ```

use common_errors::Result;
use common_models::{Config, ContentConfig, PublicationConfig, TopicConfig, ImageConfig, ImageSize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};
use std::process::{Command, Output};
use std::io::Write;

// Export the mocks module
pub mod mocks;

// Export the proptest module
pub mod proptest;

// Export the fixtures module
pub mod fixtures;

// Also re-export key fixtures for easier access
pub use fixtures::{ValidationFixture, FileSystemFixture};

/// A test fixture with a temporary directory and configuration
pub struct TestFixture {
    pub temp_dir: TempDir,
    pub config_path: PathBuf,
    pub content_dir: PathBuf,
}

impl TestFixture {
    /// Create a new test fixture with default configuration
    pub fn new() -> Result<Self> {
        let temp_dir = tempdir()?;
        let config_path = temp_dir.path().join("config.yaml");
        let content_dir = temp_dir.path().join("content");
        let fixture = Self {
            temp_dir,
            config_path,
            content_dir,
        };
        
        fixture.create_default_config()?;
        fs::create_dir_all(&fixture.content_dir)?;
        
        Ok(fixture)
    }
    
    /// Create default configuration
    pub fn create_default_config(&self) -> Result<()> {
        // Create topics
        let mut topics = HashMap::new();
        topics.insert(
            "blog".to_string(),
            TopicConfig {
                name: "Blog".to_string(),
                description: "Blog posts".to_string(),
                directory: "blog".to_string(),
            },
        );
        topics.insert(
            "notes".to_string(),
            TopicConfig {
                name: "Notes".to_string(),
                description: "Quick notes".to_string(),
                directory: "notes".to_string(),
            },
        );
        
        // Create image sizes
        let mut sizes = HashMap::new();
        sizes.insert(
            "small".to_string(),
            ImageSize {
                width: 480,
                height: 320,
                description: "Small image".to_string(),
            },
        );
        sizes.insert(
            "medium".to_string(),
            ImageSize {
                width: 800,
                height: 600,
                description: "Medium image".to_string(),
            },
        );
        
        // Create config
        let config = Config {
            content: ContentConfig {
                base_dir: self.content_dir.to_string_lossy().to_string(),
                topics,
                tags: None,
            },
            images: ImageConfig {
                formats: vec!["webp".to_string(), "jpg".to_string()],
                format_descriptions: None,
                sizes,
                naming: None,
                quality: None,
            },
            publication: PublicationConfig {
                author: "Test Author".to_string(),
                copyright: "Test Copyright".to_string(),
                site: Some("https://example.com".to_string()),
            },
        };
        
        let config_yaml = serde_yaml::to_string(&config)?;
        fs::write(&self.config_path, config_yaml)?;
        
        Ok(())
    }
    
    /// Create a test content file
    pub fn create_content(&self, topic: &str, slug: &str, title: &str, is_draft: bool) -> Result<PathBuf> {
        let topic_dir = self.content_dir.join(topic);
        let content_dir = topic_dir.join(slug);
        let content_file = content_dir.join("index.mdx");
        
        fs::create_dir_all(&content_dir)?;
        
        let frontmatter = format!(
            r#"---
title: "{}"
date: "2023-01-01"
draft: {}
---
"#,
            title, is_draft
        );
        
        let content = format!("{}\n\n# {}\n\nThis is test content.", frontmatter, title);
        fs::write(&content_file, content)?;
        
        Ok(content_file)
    }
    
    /// Get the path to this test fixture
    pub fn path(&self) -> &Path {
        self.temp_dir.path()
    }
    
    /// Create a subdirectory in the test fixture
    pub fn create_dir(&self, path: &str) -> Result<PathBuf> {
        let dir_path = self.temp_dir.path().join(path);
        fs::create_dir_all(&dir_path)?;
        Ok(dir_path)
    }
    
    /// Create a file in the test fixture
    pub fn create_file(&self, path: &str, content: &str) -> Result<PathBuf> {
        let file_path = self.temp_dir.path().join(path);
        
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(&file_path, content)?;
        Ok(file_path)
    }

    /// Run a command in the test fixture directory
    pub fn run_command(&self, program: &str, args: &[&str]) -> std::io::Result<Output> {
        Command::new(program)
            .args(args)
            .current_dir(self.path())
            .env("CONFIG_PATH", &self.config_path)
            .output()
    }
}

/// Integration test utilities for command-line tools
pub mod integration {
    use super::*;
    
    use std::io::{self, BufRead, BufReader};
    use std::process::{Child, Stdio};

    /// Represents a command to be tested
    pub struct TestCommand {
        /// The name of the command executable
        pub name: String,
        /// The path to the command executable
        pub path: PathBuf,
        /// The test fixture to use
        pub fixture: TestFixture,
    }

    impl TestCommand {
        /// Create a new test command
        pub fn new(name: &str) -> Result<Self> {
            // Find the command in the target directory
            let target_dir = std::env::var("CARGO_TARGET_DIR")
                .unwrap_or_else(|_| "target".to_string());
            
            let path = PathBuf::from(target_dir)
                .join("debug")
                .join(name);
            
            if !path.exists() {
                return Err(common_errors::WritingError::validation_error(
                    format!("Command executable not found: {}", path.display())
                ));
            }
            
            Ok(Self {
                name: name.to_string(),
                path,
                fixture: TestFixture::new()?,
            })
        }
        
        /// Run the command with the given arguments
        pub fn run(&self, args: &[&str]) -> std::io::Result<Output> {
            Command::new(&self.path)
                .args(args)
                .current_dir(self.fixture.path())
                .env("CONFIG_PATH", &self.fixture.config_path)
                .output()
        }
        
        /// Run the command with the given arguments and return a child process
        pub fn spawn(&self, args: &[&str]) -> std::io::Result<Child> {
            Command::new(&self.path)
                .args(args)
                .current_dir(self.fixture.path())
                .env("CONFIG_PATH", &self.fixture.config_path)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
        }
        
        /// Run the command with the given arguments and provide input
        pub fn run_with_input(&self, args: &[&str], input: &str) -> std::io::Result<Output> {
            let mut child = self.spawn(args)?;
            
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(input.as_bytes())?;
            }
            
            child.wait_with_output()
        }
        
        /// Assert that the command succeeds with the given arguments
        pub fn assert_success(&self, args: &[&str]) -> Output {
            let output = self.run(args).expect("Failed to run command");
            assert!(
                output.status.success(),
                "Command failed with status: {}\nstdout: {}\nstderr: {}",
                output.status,
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            output
        }
        
        /// Assert that the command fails with the given arguments
        pub fn assert_failure(&self, args: &[&str]) -> Output {
            let output = self.run(args).expect("Failed to run command");
            assert!(
                !output.status.success(),
                "Command succeeded unexpectedly\nstdout: {}\nstderr: {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            output
        }
        
        /// Assert that the command output contains the given text
        pub fn assert_output_contains(&self, args: &[&str], text: &str) -> Output {
            let output = self.assert_success(args);
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(
                stdout.contains(text),
                "Command output does not contain '{}'\nOutput: {}",
                text,
                stdout
            );
            output
        }
        
        /// Assert that the command error output contains the given text
        pub fn assert_error_contains(&self, args: &[&str], text: &str) -> Output {
            let output = self.assert_failure(args);
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(
                stderr.contains(text),
                "Command error output does not contain '{}'\nError: {}",
                text,
                stderr
            );
            output
        }
    }

    /// Helper for interactive command testing
    pub struct InteractiveTest {
        child: Child,
        reader: BufReader<std::process::ChildStdout>,
    }

    impl InteractiveTest {
        /// Create a new interactive test
        pub fn new(command: &TestCommand, args: &[&str]) -> io::Result<Self> {
            let mut child = command.spawn(args)?;
            let stdout = child.stdout.take().expect("Failed to capture stdout");
            let reader = BufReader::new(stdout);
            
            Ok(Self { child, reader })
        }
        
        /// Wait for the given text to appear in the output
        pub fn expect(&mut self, text: &str) -> io::Result<()> {
            let mut buffer = String::new();
            loop {
                let bytes_read = self.reader.read_line(&mut buffer)?;
                if bytes_read == 0 {
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        format!("Expected '{}' but got EOF", text),
                    ));
                }
                
                if buffer.contains(text) {
                    return Ok(());
                }
            }
        }
        
        /// Send input to the command
        pub fn send(&mut self, input: &str) -> io::Result<()> {
            if let Some(stdin) = &mut self.child.stdin {
                stdin.write_all(input.as_bytes())?;
                stdin.write_all(b"\n")?;
                stdin.flush()?;
                Ok(())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "Failed to write to stdin",
                ))
            }
        }
        
        /// Close the interactive test
        pub fn close(mut self) -> io::Result<Output> {
            if let Some(mut stdin) = self.child.stdin.take() {
                let _ = stdin.write_all(b"q\n");
            }
            
            self.child.wait_with_output()
        }
    }
}

/// Run a test with a temporary directory
pub fn with_temp_dir<F, T>(f: F) -> T
where
    F: FnOnce(&Path) -> T,
{
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    f(temp_dir.path())
} 