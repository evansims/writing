use clap::Parser;
use content_edit::{EditOptions, find_content_path, list_all_content, edit_content, save_edited_content};
use common_errors::{Result, WritingError};
use std::process::Command;
use std::env;

#[derive(Parser)]
#[command(author, version, about = "Edit existing content")]
struct Args {
    /// Content slug to edit
    #[arg(short, long)]
    slug: Option<String>,
    
    /// Topic of the content
    #[arg(short, long)]
    topic: Option<String>,
    
    /// Edit only the frontmatter
    #[arg(long)]
    frontmatter_only: bool,
    
    /// Edit only the content
    #[arg(long)]
    content_only: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Create edit options
    let options = EditOptions {
        slug: args.slug.clone(),
        topic: args.topic.clone(),
        frontmatter_only: args.frontmatter_only,
        content_only: args.content_only,
    };
    
    // If no slug is provided, list all content for selection
    if options.slug.is_none() {
        let content_list = list_all_content()?;
        
        if content_list.is_empty() {
            return Err(WritingError::content_not_found("No content found"));
        }
        
        println!("Available content:");
        for (i, (topic, title, _)) in content_list.iter().enumerate() {
            println!("{}. {} - {}", i + 1, topic, title);
        }
        
        print!("Select content to edit (1-{}): ", content_list.len());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        
        let selection = input.trim().parse::<usize>()
            .map_err(|_| WritingError::invalid_argument("Invalid selection"))?;
        
        if selection < 1 || selection > content_list.len() {
            return Err(WritingError::invalid_argument(format!("Selection must be between 1 and {}", content_list.len())));
        }
        
        let (_, _, content_path) = &content_list[selection - 1];
        let content = std::fs::read_to_string(content_path)?;
        
        // Open the editor
        let edited_content = open_editor(content_path.to_str().unwrap(), &content)?;
        
        // Save the edited content
        save_edited_content(content_path, &edited_content)?;
        
        println!("Content saved successfully!");
        return Ok(());
    }
    
    // Edit content with the provided options
    edit_content(&options)?;
    
    // Find the content path
    let content_path = find_content_path(
        options.slug.as_ref().unwrap(),
        options.topic.as_deref()
    )?;
    
    // Read the content
    let content = std::fs::read_to_string(&content_path)?;
    
    // Open the editor
    let edited_content = open_editor(content_path.to_str().unwrap(), &content)?;
    
    // Save the edited content
    save_edited_content(&content_path, &edited_content)?;
    
    println!("Content saved successfully!");
    Ok(())
}

/// Open the editor with the content
fn open_editor(_file_path: &str, content: &str) -> Result<String> {
    // Get the editor from environment variable or use a default
    let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    
    // Create a temporary file
    let mut temp_file = tempfile::NamedTempFile::new()
        .map_err(WritingError::from)?;
    
    // Write the content to the temporary file
    std::io::Write::write_all(&mut temp_file, content.as_bytes())
        .map_err(WritingError::from)?;
    
    // Get the path to the temporary file
    let temp_path = temp_file.path();
    
    // Open the editor
    let status = Command::new(&editor)
        .arg(temp_path)
        .status()
        .map_err(|e| WritingError::command_error(format!("Failed to open editor: {}", e)))?;
    
    if !status.success() {
        return Err(WritingError::command_error(format!("Editor exited with non-zero status: {}", status)));
    }
    
    // Read the edited content
    let edited_content = std::fs::read_to_string(temp_path)
        .map_err(WritingError::from)?;
    
    Ok(edited_content)
} 