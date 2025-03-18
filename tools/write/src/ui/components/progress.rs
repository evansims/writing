//! # Progress Bar Component
//!
//! This module provides functionality for creating progress bars for long-running operations.

use indicatif::{ProgressBar, ProgressStyle};
use common_errors::WritingError;

/// Create a progress bar with a specific style
///
/// This function creates a progress bar with a style that includes elapsed time,
/// a progress bar, current position, total length, and estimated time remaining.
///
/// # Arguments
///
/// * `len` - The total length of the progress bar
///
/// # Returns
///
/// A configured progress bar
///
/// # Examples
///
/// ```no_run
/// use write::ui::components::progress::create_progress_bar;
///
/// let pb = create_progress_bar(100);
/// for i in 0..100 {
///     pb.inc(1);
///     // Do work...
/// }
/// pb.finish_with_message("Done!");
/// ```
pub fn create_progress_bar(len: u64) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .map_err(|err| {
                WritingError::format_error(format!("Failed to create progress bar template: {}", err))
            })
            .expect("Progress bar template should be valid") // This is a developer error rather than a runtime error
            .progress_chars("#>-"),
    );
    pb
}

/// Create a spinner progress bar for operations with unknown length
///
/// This function creates a spinner-style progress bar that doesn't show a fixed
/// progress bar, useful for operations where the total length is unknown.
///
/// # Returns
///
/// A configured spinner progress bar
///
/// # Examples
///
/// ```no_run
/// use write::ui::components::progress::create_spinner;
///
/// let spinner = create_spinner();
/// spinner.set_message("Processing...");
/// // Do work...
/// spinner.finish_with_message("Done!");
/// ```
pub fn create_spinner() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {elapsed_precise} {msg}")
            .map_err(|err| {
                WritingError::format_error(format!("Failed to create spinner template: {}", err))
            })
            .expect("Spinner template should be valid") // This is a developer error rather than a runtime error
    );
    pb
}