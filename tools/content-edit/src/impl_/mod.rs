//! Implementation details for the content-edit module.
//!
//! This module contains the implementation of the public API functions.
//! It has been refactored into sub-modules for each functionality area.

// Export the implementation modules
pub mod find;
pub mod list;
pub mod edit;
pub mod frontmatter;
pub mod editor;

// Re-export the public functions for use by the lib.rs module
pub use find::find_content_path;
pub use list::list_all_content;
pub use edit::{edit_content, save_edited_content, update_content, update_frontmatter_field};
pub use frontmatter::{extract_frontmatter, extract_frontmatter_from_string, split_frontmatter_and_body};
pub use editor::ContentEditorImpl;