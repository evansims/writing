//! # Write CLI Tool
//!
//! This is a CLI tool for managing writing content.
use common_errors::{WritingError, print_error};
use clap::Parser;
use anyhow::Result;
use cli::{Cli, Commands, BuildCommands};
use std::path::PathBuf;

mod cli;
mod tools;
mod commands;
mod ui;
mod content;
mod topic;
mod image;
mod build;
mod benchmark;

use commands::executor::execute_command;

/// Main entry point for the Write CLI tool
fn main() -> Result<()> {
    let cli = Cli::parse();
    run(cli)
}

/// Run the CLI tool with the provided arguments
fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Build { command } => match command {
            BuildCommands::Content { topic, rebuild } => {
                build::build_content(topic.as_deref(), rebuild)
            }
            BuildCommands::Toc { topic } => build::generate_toc(topic.as_deref()),
            BuildCommands::Benchmark {
                baseline,
                current,
                threshold,
                report,
                json,
                verbose,
            } => {
                let baseline = baseline.map(PathBuf::from);
                let current = PathBuf::from(current);
                let report = PathBuf::from(report);
                benchmark::analyze_benchmarks(baseline, current, threshold, report, json, verbose)
            }
        },
        // Execute the command using our command executor
        command => execute_command(command),
    }
}
