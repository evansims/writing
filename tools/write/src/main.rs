//! # Write CLI Tool
//!
//! This is a CLI tool for managing writing content.
use clap::Parser;
use cli::{Cli, Commands};
use common_errors::{Result, WritingError};
use std::path::PathBuf;
use crate::tools::build;

mod cli;
mod config;
mod commands;
mod ui;
mod tools;

/// Main entry point for the Write CLI tool
fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

/// Run the CLI tool with the provided arguments
fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Build(command) => match command {
            cli::BuildCommands::Content { topic, rebuild } => {
                build::build_content(
                    None,                   // output_dir
                    None,                   // slug
                    topic.map(String::from), // topic
                    false,                  // include_drafts
                    false,                  // skip_html
                    false,                  // skip_json
                    false,                  // skip_rss
                    false,                  // skip_sitemap
                    rebuild,                // force_rebuild
                    false,                  // verbose
                ).map_err(|e| WritingError::validation_error(format!("Build error: {}", e)))?;
                Ok(())
            }
            cli::BuildCommands::Toc { topic } => {
                build::generate_toc(topic.map(String::from))
                    .map_err(|e| WritingError::validation_error(format!("TOC generation error: {}", e)))?;
                Ok(())
            }
            cli::BuildCommands::Benchmark {
                baseline,
                current,
                threshold,
                report,
                json,
                verbose,
            } => {
                commands::executor::execute_command(Commands::Build(cli::BuildCommands::Benchmark {
                    baseline,
                    current,
                    threshold,
                    report: report,
                    json,
                    verbose,
                })).map_err(|e| WritingError::validation_error(format!("Benchmark error: {}", e)))
            }
        },
        // Execute the command using our command executor
        command => commands::executor::execute_command(command)
            .map_err(|e| WritingError::validation_error(format!("Command execution error: {}", e)))
    }
}
