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
//! - Standard assertion helpers for common test patterns
//! - Test environment setup helpers
//! - Test helper macros for common patterns
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
use std::path::{Path, PathBuf};
use tempfile::tempdir;

// Export the modules
pub mod mocks;
pub mod proptest;
pub mod fixtures;
pub mod assertions;
pub mod test_environment;
pub mod macros;

// Also re-export key fixtures for easier access
pub use fixtures::{ValidationFixture, FileSystemFixture, TestFixture};
pub use test_environment::{TestEnvironment, TestEnvironmentConfig, with_test_environment, with_custom_test_environment};
pub use assertions::*;
pub use proptest::TestScenario;

// Re-export key mocks for easier access
pub use mocks::{
    // File system mocks
    FileSystem, MockFileSystem,
    // Config mocks
    ConfigLoader, MockConfigLoader,
    // Tool mocks
    ContentCreator, ContentEditor, ContentValidator, ContentSearcher,
    ContentMover, ContentDeleter
};

// Re-export macros
// Note: Macros are automatically exported through the #[macro_export] attribute,
// so we don't need to re-export them here explicitly

/// Integration test utilities for command-line tools
pub mod integration {
    use super::*;

    use std::io::{self, BufRead, BufReader, Write};
    use std::process::{Child, Stdio, Output, Command};

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
            // Get the current working directory
            let current_dir = std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."));

            // Find the command in the target directory
            let target_dir = std::env::var("CARGO_TARGET_DIR")
                .unwrap_or_else(|_| "target".to_string());

            // Try to find the executable in several likely locations
            let mut paths = Vec::new();

            // Direct absolute path from current directory
            paths.push(current_dir.join("target").join("debug").join(name));

            // If we're in a subdirectory of the project, try going up to find the target directory
            let mut up_dir = current_dir.clone();
            for _ in 0..3 {
                // Go up one level
                if let Some(parent) = up_dir.parent() {
                    up_dir = parent.to_path_buf();
                    paths.push(up_dir.join("target").join("debug").join(name));
                }
            }

            // Try using the CARGO_TARGET_DIR environment variable if set
            paths.push(PathBuf::from(&target_dir).join("debug").join(name));

            // Use the first path that exists
            let path = paths.into_iter()
                .find(|p| p.exists())
                .ok_or_else(|| {
                    common_errors::WritingError::validation_error(
                        format!("Command executable not found: {}", name)
                    )
                })?;

            eprintln!("Debug: Found command at: {}", path.display());

            let fixture = TestFixture::new()?;

            Ok(Self {
                name: name.to_string(),
                path,
                fixture,
            })
        }

        /// Run the command with the given arguments
        pub fn run(&self, args: &[&str]) -> std::io::Result<Output> {
            // Print debug information
            eprintln!("Debug: Running command: {} {:?}", self.path.display(), args);
            eprintln!("Debug: Current dir: {}", self.fixture.temp_dir.path().display());
            eprintln!("Debug: Config path: {}", self.fixture.temp_dir.path().join("config.yaml").display());

            // Ensure the content directory exists
            let content_dir = self.fixture.temp_dir.path().join("content");
            if !content_dir.exists() {
                std::fs::create_dir_all(&content_dir).expect("Failed to create content directory");
            }

            // Run the command
            Command::new(&self.path)
                .args(args)
                .current_dir(self.fixture.temp_dir.path())
                .env("CONFIG_PATH", self.fixture.temp_dir.path().join("config.yaml"))
                .env("TEST_MODE", "1")
                .output()
        }

        /// Run the command with the given input
        pub fn run_with_input(&self, args: &[&str], input: &str) -> std::io::Result<Output> {
            // Print debug information
            eprintln!("Debug: Running command with input: {} {:?}", self.path.display(), args);
            eprintln!("Debug: Input: {}", input);
            eprintln!("Debug: Current dir: {}", self.fixture.temp_dir.path().display());
            eprintln!("Debug: Config path: {}", self.fixture.temp_dir.path().join("config.yaml").display());

            // Ensure the content directory exists
            let content_dir = self.fixture.temp_dir.path().join("content");
            if !content_dir.exists() {
                std::fs::create_dir_all(&content_dir).expect("Failed to create content directory");
            }

            let mut child = Command::new(&self.path)
                .args(args)
                .current_dir(self.fixture.temp_dir.path())
                .env("CONFIG_PATH", self.fixture.temp_dir.path().join("config.yaml"))
                .env("TEST_MODE", "1")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?;

            {
                let stdin = child.stdin.as_mut().expect("Failed to open stdin");
                stdin.write_all(input.as_bytes())?;
            }

            child.wait_with_output()
        }

        /// Spawn the command with the given arguments
        pub fn spawn(&self, args: &[&str]) -> std::io::Result<Child> {
            // Print debug information
            eprintln!("Debug: Spawning command: {} {:?}", self.path.display(), args);
            eprintln!("Debug: Current dir: {}", self.fixture.temp_dir.path().display());
            eprintln!("Debug: Config path: {}", self.fixture.temp_dir.path().join("config.yaml").display());

            // Ensure the content directory exists
            let content_dir = self.fixture.temp_dir.path().join("content");
            if !content_dir.exists() {
                std::fs::create_dir_all(&content_dir).expect("Failed to create content directory");
            }

            // Run the command
            Command::new(&self.path)
                .args(args)
                .current_dir(self.fixture.temp_dir.path())
                .env("CONFIG_PATH", self.fixture.temp_dir.path().join("config.yaml"))
                .env("TEST_MODE", "1")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
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