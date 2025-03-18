//! Content editing functionality for the writing tools.
//!
//! This module provides functionality for editing content in the content repository.
//! It supports finding, listing, and editing content, with options for modifying
//! frontmatter and content separately.
//!
//! # Examples
//!
//! ## Finding and editing content
//!
//! ```no_run
//! use content_edit::{EditOptions, edit_content, save_edited_content};
//!
//! // Create options for editing content
//! let options = EditOptions::for_full_edit("my-post", Some("blog".to_string()));
//!
//! // Get the content to edit
//! let content = edit_content(&options).expect("Failed to get content");
//!
//! // Read the current content
//! let current_content = std::fs::read_to_string(&content.path).expect("Failed to read content");
//!
//! // Make changes to the content (in this example, we're just adding a comment)
//! let edited_content = format!("{}\n<!-- Edited content -->", current_content);
//!
//! // Save the edited content
//! save_edited_content(&content.path, &edited_content).expect("Failed to save content");
//! ```
//!
//! ## Listing all content
//!
//! ```no_run
//! use content_edit::list_all_content;
//!
//! // Get all content in the repository
//! let content_list = list_all_content().expect("Failed to list content");
//!
//! // Print information about each content item
//! for content in content_list {
//!     println!("Title: {}", content.title);
//!     println!("Topic: {}", content.topic);
//!     println!("Slug: {}", content.slug);
//!     println!("Path: {:?}", content.path);
//!     println!("---");
//! }
//! ```
//!
//! ## Editing only the frontmatter
//!
//! ```no_run
//! use content_edit::{EditOptions, edit_content, save_edited_content};
//!
//! // Create options for editing only the frontmatter
//! let options = EditOptions::for_frontmatter("my-post", Some("blog".to_string()));
//!
//! // Get the content to edit
//! let content = edit_content(&options).expect("Failed to get content");
//!
//! // Create edited frontmatter
//! let edited_frontmatter = r#"---
//! title: "Updated Title"
//! date: "2023-01-01"
//! tags: ["example", "documentation"]
//! ---"#;
//!
//! // Save only the frontmatter (the body will be preserved)
//! save_edited_content(&content.path, edited_frontmatter).expect("Failed to save frontmatter");
//! ```

mod impl_;
mod models;
mod errors;

#[cfg(test)]
mod tests;

// Re-export public types
pub use errors::ContentEditError;
pub use models::{EditOptions, EditableContent};

// Re-export public functions from implementation
pub use impl_::{
    find_content_path,
    list_all_content,
    edit_content,
    save_edited_content,
    extract_frontmatter,
    extract_frontmatter_from_string,
    split_frontmatter_and_body,
    update_content,
};

// Constants that should be available to users of this module
pub const DEFAULT_CONTENT_DIR: &str = "content";