//! Mock implementations for testing
//!
//! This module provides mock implementations of various components for testing.

pub mod config;
pub mod fs;
pub mod tools;

// Re-export types
pub use config::{ConfigLoader, MockConfigLoader};
pub use fs::{FileSystem, MockFileSystem};
pub use common_traits::tools::{
    ContentCreator, ContentEditor, ContentValidator, ContentSearcher,
    ContentMover, ContentDeleter
};

// Re-export config mocks
pub use config::{
    InMemoryConfigLoader, create_test_config_loader
};

// Re-export tool mocks
pub use tools::{
    ContentCreatorMock,
    ContentEditorMock,
    ContentValidatorMock,
    ContentSearcherMock,
    ContentMoverMock,
    ContentDeleterMock,
    TestContentCreator,
    TestContentEditor,
    TestContentValidator,
    TestContentSearcher,
    TestContentMover,
    TestContentDeleter,
    create_test_tools
};