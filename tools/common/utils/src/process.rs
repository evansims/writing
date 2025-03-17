//! # Process Utilities
//! 
//! This module provides utilities for running external processes and tools.

use anyhow::Result;
use std::path::Path;
use std::process::Command;
use crate::ui;

/// Run a tool by name
pub fn run_tool(tool_name: &str, args: Vec<&str>) -> Result<()> {
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
    if !Path::new(&tool_path).exists() {
        return Err(anyhow::anyhow!("Tool not found: {}", tool_path));
    }
    
    // Show command being run
    ui::print_info(&format!("Running tool: {} {}", tool_path, args.join(" ")));
    
    // Run the command
    let status = Command::new(&tool_path)
        .args(args)
        .status()?;
        
    if !status.success() {
        return Err(anyhow::anyhow!("Tool execution failed with status: {}", status));
    }
    
    Ok(())
}

/// Run a shell command
pub fn run_command(command: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(command)
        .args(args)
        .output()?;
        
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Command execution failed: {}", stderr));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}

/// Run a shell command with input
pub fn run_command_with_input(command: &str, args: &[&str], input: &str) -> Result<String> {
    let mut child = Command::new(command)
        .args(args)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
        
    // Write to stdin
    if let Some(stdin) = child.stdin.as_mut() {
        use std::io::Write;
        stdin.write_all(input.as_bytes())?;
    }
    
    // Wait for the command to complete
    let output = child.wait_with_output()?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("Command execution failed: {}", stderr));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_run_command() {
        let result = run_command("echo", &["Hello, world!"]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "Hello, world!");
    }
    
    #[test]
    fn test_run_command_with_input() {
        let result = run_command_with_input("cat", &[], "Hello, world!");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, world!");
    }
} 