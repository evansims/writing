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
use std::path::{Path, PathBuf};
use tempfile::tempdir;

// Export the mocks module
pub mod mocks;

// Export the proptest module
pub mod proptest;

// Export the fixtures module
pub mod fixtures;

// Also re-export key fixtures for easier access
pub use fixtures::{ValidationFixture, FileSystemFixture, TestFixture};

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
                .env("CONFIG_PATH", self.fixture.path().join("config.yaml"))
                .output()
        }

        /// Spawn the command as a child process
        pub fn spawn(&self, args: &[&str]) -> std::io::Result<Child> {
            Command::new(&self.path)
                .args(args)
                .current_dir(self.fixture.path())
                .env("CONFIG_PATH", self.fixture.path().join("config.yaml"))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
        }

        /// Run the command with the given input
        pub fn run_with_input(&self, args: &[&str], input: &str) -> std::io::Result<Output> {
            let mut child = Command::new(&self.path)
                .args(args)
                .current_dir(self.fixture.path())
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