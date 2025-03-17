//! # Mock Implementations
//! 
//! This module provides mock implementations for various components used in testing.

mod markdown;
mod filesystem;
mod config;
mod content;
mod command;

// Re-export the markdown mocks
pub use markdown::{MockMarkdown, MarkdownOperations};
pub use filesystem::{MockFileSystem, FileSystem};
pub use config::{MockConfigLoader, ConfigLoader};
pub use content::{MockContentOperations, ContentOperations};
pub use command::{MockCommandExecutor, CommandExecutor};

// Re-export all traits in a traits module
pub mod traits {
    pub use super::{FileSystem, ConfigLoader, ContentOperations, CommandExecutor, MarkdownOperations};
} 