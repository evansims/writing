//! # Mock Command Executor Implementation
//! 
//! This module provides a mock implementation of command execution for testing.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use common_errors::{Result, WritingError};

/// A mock implementation of command execution
#[derive(Debug, Clone, Default)]
pub struct MockCommandExecutor {
    responses: Arc<Mutex<HashMap<String, (String, i32)>>>,
    executed_commands: Arc<Mutex<Vec<String>>>,
}

impl MockCommandExecutor {
    /// Create a new mock command executor implementation
    pub fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(HashMap::new())),
            executed_commands: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// Set the response for a specific command
    pub fn set_response(&mut self, command: &str, output: &str, exit_code: i32) {
        self.responses.lock().unwrap().insert(
            command.to_string(),
            (output.to_string(), exit_code)
        );
    }
    
    /// Execute a command and return the mocked output and exit code
    pub fn execute(&self, command: &str) -> Result<(String, i32)> {
        // Record the executed command
        self.executed_commands.lock().unwrap().push(command.to_string());
        
        // Get the response
        let responses = self.responses.lock().unwrap();
        if let Some((output, exit_code)) = responses.get(command) {
            Ok((output.clone(), *exit_code))
        } else {
            // Default response if not configured
            Err(WritingError::command_error(
                &format!("No mock response configured for command: {}", command)
            ))
        }
    }
    
    /// Get the list of executed commands
    pub fn get_executed_commands(&self) -> Vec<String> {
        self.executed_commands.lock().unwrap().clone()
    }
    
    /// Clear the list of executed commands
    pub fn clear_executed_commands(&mut self) {
        self.executed_commands.lock().unwrap().clear();
    }
}

/// Trait for command execution
pub trait CommandExecutor {
    /// Execute a command and return the output and exit code
    fn execute(&self, command: &str) -> Result<(String, i32)>;
}

// Implement the trait for the mock
impl CommandExecutor for MockCommandExecutor {
    fn execute(&self, command: &str) -> Result<(String, i32)> {
        self.execute(command)
    }
} 