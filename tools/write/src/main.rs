use anyhow::Result;
use clap::Parser;
use colored::*;

mod cli;
mod ui;
mod tools;
mod config;

use cli::{Cli, Commands};

fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // If no command is provided, show the interactive menu
    let command = match cli.command {
        Some(cmd) => Some(cmd),
        None => ui::show_main_menu()?,
    };
    
    // Execute the command if one was provided or selected
    if let Some(cmd) = command {
        execute_command(cmd)?;
    }
    
    Ok(())
}

/// Execute a command
fn execute_command(command: Commands) -> Result<()> {
    match command {
        Commands::Content(cmd) => tools::execute_content_command(cmd),
        Commands::Topic(cmd) => tools::execute_topic_command(cmd),
        Commands::Image(cmd) => tools::execute_image_command(cmd),
        Commands::Build(cmd) => tools::execute_build_command(cmd),
        Commands::Stats { slug, topic, include_drafts, sort_by, detailed } => {
            tools::execute_stats_command(slug, topic, include_drafts, &sort_by, detailed)
        }
    }
}
