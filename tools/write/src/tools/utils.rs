//! # Utilities Module
//!
//! This module provides utility functions used across the application.

use anyhow::Result;
use crate::ui;
use rayon::prelude::*;
use std::time::Instant;
use indicatif::{ProgressBar, ProgressStyle};

/// Run a tool by name
pub fn run_tool(tool_name: &str, args: Vec<&str>) -> Result<()> {
    run_tool_by_name(tool_name, args)
}

/// Run a tool by name (implementation)
pub fn run_tool_by_name(tool_name: &str, args: Vec<&str>) -> Result<()> {
    // Convert args to String for run_tool_command
    let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    run_tool_command(tool_name, &args)
}

/// Run a tool command with string arguments
pub fn run_tool_command(tool_name: &str, args: &[String]) -> Result<()> {
    // Call with default tools directory
    run_tool_command_with_dir(tool_name, args, None)
}

/// Run a tool command with string arguments and optional tools directory
pub fn run_tool_command_with_dir(tool_name: &str, args: &[String], tools_dir: Option<&str>) -> Result<()> {
    // Get the tools directory
    let tools_dir = tools_dir.unwrap_or("tools");

    // Get the full path to the tool
    let tool_path = format!("{}/{}", tools_dir, tool_name);

    // Check if the tool exists
    if !std::path::Path::new(&tool_path).exists() {
        return Err(anyhow::anyhow!("Tool not found: {}", tool_path));
    }

    // Show command being run
    ui::show_info(&format!("Running tool: {} {}", tool_path, args.join(" ")));

    // Run the command
    let status = std::process::Command::new(&tool_path)
        .args(args)
        .status()?;

    if !status.success() {
        return Err(anyhow::anyhow!("Tool execution failed with status: {}", status));
    }

    Ok(())
}

/// Process a collection of items in parallel with automatic batching
///
/// This utility function processes a collection of items in parallel using rayon,
/// with automatic batching to avoid excessive memory usage for large collections.
///
/// # Type Parameters
///
/// * `T` - The type of items to process
/// * `R` - The result type of the processing function
/// * `E` - The error type that can be returned by the processing function
///
/// # Parameters
///
/// * `items` - Collection of items to process
/// * `batch_size` - Maximum items to process in a single batch
/// * `show_progress` - Whether to display a progress bar
/// * `progress_message` - Message to display on the progress bar
/// * `processor` - Function to process each item
///
/// # Returns
///
/// A tuple containing the results and the elapsed time
pub fn process_in_parallel<T, R, E, F>(
    items: Vec<T>,
    batch_size: usize,
    show_progress: bool,
    progress_message: &str,
    processor: F,
) -> (Vec<Result<R, E>>, std::time::Duration)
where
    T: Send + Sync,
    R: Send,
    E: Send,
    F: Fn(&T) -> Result<R, E> + Send + Sync,
{
    let item_count = items.len();
    let actual_batch_size = batch_size.min(item_count).max(1);
    let start_time = Instant::now();

    // Create progress bar if requested
    let progress = if show_progress {
        let pb = ProgressBar::new(item_count as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
                .map_err(|err| {
                    anyhow::anyhow!(format!("Failed to create progress bar template: {}", err))
                })
                .expect("Progress bar template should be valid")
                .progress_chars("#>-"),
        );
        pb.set_message(progress_message.to_string());
        Some(pb)
    } else {
        None
    };

    // Process items in parallel with batching
    let results: Vec<Result<R, E>> = items
        .par_chunks(actual_batch_size)
        .flat_map(|chunk| {
            chunk.par_iter().map(|item| {
                let result = processor(item);

                // Increment progress bar if it exists
                if let Some(pb) = &progress {
                    pb.inc(1);
                }

                result
            }).collect::<Vec<_>>()
        })
        .collect();

    // Finish progress bar if it exists
    if let Some(pb) = progress {
        pb.finish_with_message("Processing completed");
    }

    let elapsed = start_time.elapsed();

    (results, elapsed)
}

/// Calculate success ratio from results
///
/// This utility function calculates the success ratio from a collection of results.
///
/// # Type Parameters
///
/// * `T` - The success type
/// * `E` - The error type
///
/// # Parameters
///
/// * `results` - Collection of results to analyze
///
/// # Returns
///
/// A tuple containing the number of successes, failures, and total items
pub fn calculate_success_ratio<T, E>(results: &[Result<T, E>]) -> (usize, usize, usize) {
    let total = results.len();
    let successes = results.iter().filter(|r| r.is_ok()).count();
    let failures = total - successes;

    (successes, failures, total)
}