use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    // Get the command name from arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command-name>", args[0]);
        std::process::exit(1);
    }

    let command_name = &args[1];
    let command_camel = snake_to_camel_case(command_name);
    
    // Create lib.rs template
    let lib_template = format!(r#"use anyhow::Result;
use common_cli::{{Command, ContentCommand, DisplayResult}};
use common_errors::WritingError;
use clap::Parser;
use std::path::PathBuf;
use colored::*;

/// CLI arguments for the {command_name} command
#[derive(Parser, Debug)]
#[command(author, version, about = "Description for {command_name}")]
pub struct {command_camel}Args {{
    /// Example argument
    #[arg(short, long)]
    pub example: String,
    
    /// Optional argument
    #[arg(short, long)]
    pub optional: Option<String>,
}}

/// Command for {command_name}
pub struct {command_camel}Command {{
    args: {command_camel}Args,
}}

/// Result of the {command_name} operation
#[derive(Debug)]
pub struct {command_camel}Result {{
    pub message: String,
}}

impl DisplayResult for {command_camel}Result {{
    fn to_display(&self) -> String {{
        format!("{{}} {{}}",
            "SUCCESS:".green().bold(),
            self.message
        )
    }}
}}

impl Command for {command_camel}Command {{
    type Args = {command_camel}Args;
    type Output = {command_camel}Result;
    
    fn new(args: Self::Args) -> Self {{
        {command_camel}Command {{ args }}
    }}
    
    fn execute(&self) -> Result<Self::Output> {{
        // Implement command logic here
        
        Ok({command_camel}Result {{
            message: format!("Executed {{}} command", self.args.example),
        }})
    }}
    
    fn handle_result(result: Self::Output) {{
        result.print();
    }}
}}

impl ContentCommand for {command_camel}Command {{}}
"#);

    // Create main.rs template
    let main_template = format!(r#"use anyhow::Result;
use {command_name}_command::{{Command, {command_camel}Command, {command_camel}Args}};

fn main() -> Result<()> {{
    // Use the Command trait's run method as the standard entry point
    {command_camel}Command::run()
}}
"#);

    // Create Cargo.toml template
    let cargo_template = format!(r#"[package]
name = "{command_name}-command"
version = "0.1.0"
edition = "2021"
description = "Command for {command_name}"

[lib]
name = "{command_name}_command"
path = "src/lib.rs"

[[bin]]
name = "{command_name}"
path = "src/main.rs"

[dependencies]
clap.workspace = true
anyhow.workspace = true
serde.workspace = true
serde_yaml.workspace = true
colored.workspace = true
walkdir.workspace = true
dialoguer.workspace = true
common-models = {{ path = "../common/models" }}
common-config = {{ path = "../common/config" }}
common-fs = {{ path = "../common/fs" }}
common-markdown = {{ path = "../common/markdown" }}
common-errors = {{ path = "../common/errors" }}
common-cli = {{ path = "../common/cli" }}

[dev-dependencies]
tempfile.workspace = true
"#);

    // Create directory structure
    let dir_path = format!("../{command_name}-command/src");
    fs::create_dir_all(&dir_path)?;
    
    // Write files
    let lib_path = format!("../{command_name}-command/src/lib.rs");
    let main_path = format!("../{command_name}-command/src/main.rs");
    let cargo_path = format!("../{command_name}-command/Cargo.toml");
    
    fs::write(&lib_path, lib_template)?;
    fs::write(&main_path, main_template)?;
    fs::write(&cargo_path, cargo_template)?;
    
    println!("Created command template at:");
    println!("  Cargo.toml: {}", Path::new(&cargo_path).canonicalize()?.display());
    println!("  lib.rs: {}", Path::new(&lib_path).canonicalize()?.display());
    println!("  main.rs: {}", Path::new(&main_path).canonicalize()?.display());
    
    println!("\nRemember to add the new crate to the workspace members in tools/Cargo.toml");
    
    Ok(())
}

/// Convert snake_case to CamelCase
fn snake_to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    
    result
} 