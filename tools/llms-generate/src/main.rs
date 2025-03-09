use anyhow::Result;
use clap::Parser;
use colored::*;
use llms_generate::{LlmsOptions, generate_llms};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Generate llms.txt and llms-full.txt files according to the llmstxt.org standard")]
struct Args {
    /// Output directory for llms.txt and llms-full.txt (default: current directory)
    #[arg(short, long, default_value = ".")]
    output_dir: PathBuf,

    /// Site URL (required for generating absolute URLs)
    #[arg(short, long)]
    site_url: Option<String>,

    /// Include drafts in the output
    #[arg(short, long)]
    include_drafts: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Create options from CLI arguments
    let options = LlmsOptions {
        output_dir: args.output_dir,
        site_url: args.site_url,
        include_drafts: args.include_drafts,
    };
    
    // Generate LLMS files
    match generate_llms(&options) {
        Ok((llms_txt_path, llms_full_txt_path)) => {
            println!("{} Generated {} file", "SUCCESS:".green().bold(), llms_txt_path.display());
            println!("{} Generated {} file", "SUCCESS:".green().bold(), llms_full_txt_path.display());
            Ok(())
        },
        Err(e) => {
            eprintln!("{} {}", "ERROR:".red().bold(), e);
            Err(e)
        }
    }
} 