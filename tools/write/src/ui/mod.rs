//! # UI Components
//!
//! This module provides UI components for the interactive CLI experience.
//! It has been refactored into sub-modules for different UI component groups.

pub mod menus;
pub mod components;
pub mod feedback;

// Re-export the public UI components for backward compatibility
pub use menus::{
    show_main_menu,
    show_content_menu,
    show_topic_menu,
    show_image_menu,
    show_build_menu,
};

pub use components::progress::create_progress_bar;

pub use feedback::{
    show_success,
    show_error,
    show_detailed_error,
    show_warning,
    show_info,
};