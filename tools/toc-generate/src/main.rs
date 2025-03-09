use anyhow::Result;
use clap::Parser;
use colored::*;
use std::path::PathBuf;
use toc_generate::{TocOptions, generate_toc};

#[derive(Parser)]
#[command(author, version, about = "Generate table of contents")]
struct Args {
    /// Output file path (default: index.md)
    #[arg(short, long, default_value = "index.md")]
    output: PathBuf,
    
    /// Custom title for the table of contents
    #[arg(short, long)]
    title: Option<String>,
    
    /// Custom description for the table of contents
    #[arg(short, long)]
    description: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Create options from CLI arguments
    let options = TocOptions {
        output: args.output.clone(),
        title: args.title,
        description: args.description,
    };
    
    // Generate table of contents
    match generate_toc(&options) {
        Ok(output_path) => {
            println!("{} Table of contents generated at: {:?}", "SUCCESS:".green().bold(), output_path);
            Ok(())
        },
        Err(e) => Err(e),
    }
} 