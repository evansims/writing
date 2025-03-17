//! # Utilities Module
//! 
//! This module provides utility functions used across the application.

use anyhow::Result;
use colored::*;
use crate::ui;

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