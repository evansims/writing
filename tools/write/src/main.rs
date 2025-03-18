//! # Write CLI Tool
//!
//! This is a CLI tool for managing writing content.
use common_errors::{WritingError, print_error};
use clap::Parser;

mod cli;
mod tools;
mod commands;
mod ui;

use cli::Cli;
use commands::executor::execute_command;

/// Main entry point for the Write CLI tool
fn main() {
    // Parse command line arguments and execute the
    // corresponding command or return an error.
    if let Err(e) = run() {
        print_error(&e);
        std::process::exit(1);
    }
}

/// Run the CLI tool with the provided arguments
fn run() -> Result<(), WritingError> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Set up error handling
    let result = match cli.command {
        // Execute the command using our command executor
        command => execute_command(command),
    };

    // Convert anyhow::Error to WritingError
    result.map_err(|e| WritingError::format_error(format!("{:#}", e)))
}
