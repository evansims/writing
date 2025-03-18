use clap::Parser;
use content_edit::{
    EditOptions,
    list_all_content, edit_content, save_edited_content
};
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
        let content_list = list_all_content()
            .map_err(|e| WritingError::from(e))?;

        if content_list.is_empty() {
            return Err(WritingError::content_not_found("No content found"));
        }

        println!("Available content:");
        for (i, content) in content_list.iter().enumerate() {
            println!("{}. {} - {}", i + 1, content.topic, content.title);
        }

        print!("Select content to edit (1-{}): ", content_list.len());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let selection = input.trim().parse::<usize>()
            .map_err(|_| WritingError::invalid_argument("Invalid selection"))?;

        if selection < 1 || selection > content_list.len() {
            return Err(WritingError::invalid_argument(format!("Selection must be between 1 and {}", content_list.len())));
        }

        let selected_content = &content_list[selection - 1];
        let content = std::fs::read_to_string(&selected_content.path)?;

        // Convert path to string safely
        let path_str = selected_content.path.to_str()
            .ok_or_else(|| WritingError::other("Path contains invalid characters"))?;

        // Open the editor
        let edited_content = open_editor(path_str, &content)?;

        // Save the edited content
        save_edited_content(&selected_content.path, &edited_content)
            .map_err(|e| WritingError::from(e))?;

        println!("Content saved successfully!");
        return Ok(());
    }

    // Edit content with the provided options
    let content = edit_content(&options)
        .map_err(|e| WritingError::from(e))?;

    // Read the content
    let content_text = std::fs::read_to_string(&content.path)?;

    // Convert path to string safely
    let path_str = content.path.to_str()
        .ok_or_else(|| WritingError::other("Path contains invalid characters"))?;

    // Open the editor
    let edited_content = open_editor(path_str, &content_text)?;

    // Save the edited content
    save_edited_content(&content.path, &edited_content)
        .map_err(|e| WritingError::from(e))?;

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