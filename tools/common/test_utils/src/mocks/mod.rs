//! Mock implementations for testing
//!
//! This module provides mock implementations of various components for testing.

mod fs;
mod config;
mod tools;

// Re-export filesystem mocks
pub use fs::{FileSystem, MockFileSystem, InMemoryFileSystem, create_test_fs};

// Re-export config mocks
pub use config::{ConfigLoader, MockConfigLoader, InMemoryConfigLoader, create_test_config_loader};

// Re-export tool mocks
pub use tools::{
    ContentCreatorMock, MockContentCreatorMock,
    ContentEditorMock, MockContentEditorMock,
    ContentValidatorMock, MockContentValidatorMock,
    ContentSearcherMock, MockContentSearcherMock,
    ContentMoverMock, MockContentMoverMock,
    ContentDeleterMock, MockContentDeleterMock,

    TestContentCreator, TestContentEditor, TestContentValidator,
    TestContentSearcher, TestContentMover, TestContentDeleter,

    create_test_tools
};