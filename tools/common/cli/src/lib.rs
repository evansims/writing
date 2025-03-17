use anyhow::Result;
#[cfg(feature = "command")]
use clap::Parser;
use common_errors::WritingError;
use std::fmt::Debug;
use std::path::PathBuf;

// Add the factory module
pub mod factory;
// Add the args module
pub mod args;

// Re-export common argument structs for easier access
pub use args::{
    ContentArgs, DraftArgs, TemplateArgs, TagArgs, EditArgs, CreateArgs,
    ForceArgs, OutputFormatArgs, VerboseArgs, FileArgs, DirectoryArgs,
    RecursiveArgs, LimitArgs, SearchArgs, SortArgs, PaginationArgs,
};

/// Common trait for command execution
#[cfg(feature = "command")]
pub trait Command: Sized {
    /// The CLI arguments type that this command accepts
    type Args: Parser + Debug;
    
    /// The result type returned by this command
    type Output;
    
    /// Create a new command instance from parsed arguments
    fn new(args: Self::Args) -> Self;
    
    /// Execute the command
    fn execute(&self) -> Result<Self::Output>;
    
    /// Run the command from the command line
    /// This is the standard entry point that should be called from main()
    fn run() -> Result<()> {
        // Parse command line arguments
        let args = Self::Args::parse();
        
        // Create the command
        let command = Self::new(args);
        
        // Execute the command
        let result = command.execute()?;
        
        // Handle the result (default implementation just prints success)
        Self::handle_result(result);
        
        Ok(())
    }
    
    /// Handle the command output (can be overridden by implementing commands)
    fn handle_result(_output: Self::Output) {
        println!("Command executed successfully");
    }
}

/// Common traits for commands that operate on content
#[cfg(feature = "content-commands")]
pub trait ContentCommand: Command {
    /// Validate that a slug is provided
    fn validate_slug(&self, slug: &Option<String>) -> Result<String> {
        slug.as_deref()
            .ok_or_else(|| WritingError::validation_error("No slug provided").into())
            .map(|s| s.to_string())
    }
    
    /// Validate that a topic exists
    fn validate_topic(&self, topic: &Option<String>) -> Result<Option<String>> {
        if let Some(topic) = topic {
            let config = common_config::load_config()?;
            
            if !config.content.topics.contains_key(topic) {
                let valid_topics: Vec<String> = config.content.topics.keys()
                    .map(|k| k.to_string())
                    .collect();
                
                return Err(WritingError::topic_error(format!(
                    "Invalid topic: {}. Valid topics are: {}", 
                    topic, 
                    valid_topics.join(", ")
                )).into());
            }
            
            Ok(Some(topic.clone()))
        } else {
            Ok(None)
        }
    }
    
    /// Find content path by slug and optional topic
    fn find_content_path(&self, slug: &str, topic: Option<&str>) -> Result<PathBuf> {
        let config = common_config::load_config()?;
        common_fs::find_content_path(slug, topic, &config)
            .map_err(|e| WritingError::content_not_found(format!("Failed to find content: {}", e)).into())
    }
}

/// Common interface for displaying command results
pub trait DisplayResult {
    /// Convert the result to a displayable format
    fn to_display(&self) -> String;
    
    /// Print the result to the terminal
    fn print(&self) {
        println!("{}", self.to_display());
    }
}

/// Utility functions for CLI output
#[cfg(feature = "colored")]
pub mod util {
    use colored::*;
    
    pub fn print_success(message: &str) {
        println!("{} {}", "SUCCESS:".green().bold(), message);
    }
    
    pub fn print_error(message: &str) {
        eprintln!("{} {}", "ERROR:".red().bold(), message);
    }
    
    pub fn print_warning(message: &str) {
        println!("{} {}", "WARNING:".yellow().bold(), message);
    }
    
    pub fn print_info(message: &str) {
        println!("{} {}", "INFO:".blue().bold(), message);
    }
}

/// Utility functions for CLI output (without colors)
#[cfg(not(feature = "colored"))]
pub mod util {
    pub fn print_success(message: &str) {
        println!("SUCCESS: {}", message);
    }
    
    pub fn print_error(message: &str) {
        eprintln!("ERROR: {}", message);
    }
    
    pub fn print_warning(message: &str) {
        println!("WARNING: {}", message);
    }
    
    pub fn print_info(message: &str) {
        println!("INFO: {}", message);
    }
} 