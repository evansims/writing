use anyhow::Result;
use clap::{Parser, Subcommand};
use content_template::{create_template, delete_template, get_template, list_templates, CreateTemplateOptions};

/// Command-line tool for managing content templates
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new template
    Create {
        /// The name of the template
        #[arg(long, short)]
        name: String,
        
        /// The content type for the template (article, note, tutorial)
        #[arg(long, short)]
        content_type: String,
        
        /// The path to a file to use as content (optional)
        #[arg(long)]
        file: Option<String>,
    },
    
    /// List all templates
    List,
    
    /// Show a specific template
    Show {
        /// The name of the template to show
        name: String,
    },
    
    /// Delete a template
    Delete {
        /// The name of the template to delete
        name: String,
        
        /// Skip confirmation prompt
        #[arg(long, short)]
        force: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Create { name, content_type, file }) => {
            let content = if let Some(file_path) = file {
                Some(std::fs::read_to_string(&file_path)?)
            } else {
                None
            };
            
            let options = CreateTemplateOptions {
                name,
                content_type,
                content,
            };
            
            let template = create_template(options)?;
            println!("✓ Created template '{}' with content type '{}'", template.name, template.content_type);
            Ok(())
        },
        
        Some(Commands::List) => {
            let templates = list_templates()?;
            
            if templates.is_empty() {
                println!("No templates found.");
                return Ok(());
            }
            
            println!("Available templates:");
            for template in templates {
                println!("  - {} ({})", template.name, template.content_type);
            }
            
            Ok(())
        },
        
        Some(Commands::Show { name }) => {
            let content = get_template(&name)?;
            println!("{}", content);
            Ok(())
        },
        
        Some(Commands::Delete { name, force }) => {
            if !force {
                // Prompt for confirmation
                println!("Are you sure you want to delete template '{}'? (y/N)", name);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Template deletion cancelled.");
                    return Ok(());
                }
            }
            
            delete_template(&name)?;
            println!("✓ Deleted template '{}'", name);
            Ok(())
        },
        
        None => {
            // Default to list if no command is provided
            let templates = list_templates()?;
            
            if templates.is_empty() {
                println!("No templates found.");
                return Ok(());
            }
            
            println!("Available templates:");
            for template in templates {
                println!("  - {} ({})", template.name, template.content_type);
            }
            
            Ok(())
        },
    }
} 