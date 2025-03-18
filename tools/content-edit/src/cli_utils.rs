use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Split content into frontmatter and body.
///
/// This is a simplified version that doesn't require the full dependency stack.
/// It returns the raw frontmatter string and body string.
///
/// # Arguments
///
/// * `content` - The content string to split
///
/// # Returns
///
/// Returns a tuple containing the frontmatter string and body string.
pub fn split_frontmatter_and_body(content: &str) -> Option<(String, String)> {
    let content = content.trim_start();

    if !content.starts_with("---") {
        return None;
    }

    // Find the end of the frontmatter
    let rest = &content[3..];
    if let Some(end_index) = rest.find("---") {
        let frontmatter_str = rest[..end_index].trim().to_string();
        let body_start = end_index + 3;
        let body = if body_start < rest.len() {
            rest[body_start..].trim().to_string()
        } else {
            String::new()
        };

        Some((frontmatter_str, body))
    } else {
        None
    }
}

/// Extract frontmatter as key-value pairs
pub fn extract_frontmatter_as_map(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let (frontmatter, _) = split_frontmatter_and_body(content).unwrap_or(("".to_string(), content.to_string()));

    for line in frontmatter.lines() {
        if let Some(pos) = line.find(':') {
            let key = line[..pos].trim().to_string();
            let value = line[pos+1..].trim().to_string();

            // Remove quotes if present
            let clean_value = if value.starts_with('"') && value.ends_with('"') {
                value[1..value.len()-1].to_string()
            } else {
                value
            };

            map.insert(key, clean_value);
        }
    }

    map
}

/// Save content to a file
pub fn save_content(path: &Path, content: &str) -> std::io::Result<()> {
    fs::write(path, content)
}

/// Merge edited frontmatter with existing body
pub fn merge_frontmatter_and_body(frontmatter: &str, body: &str) -> String {
    format!("---\n{}\n---\n\n{}", frontmatter.trim(), body.trim())
}

/// Update content with edited frontmatter or body
pub fn update_content(
    path: &Path,
    content: &str,
    frontmatter_only: bool,
    content_only: bool
) -> std::io::Result<()> {
    // Read existing content
    let existing_content = fs::read_to_string(path)?;
    let (existing_frontmatter, existing_body) = split_frontmatter_and_body(&existing_content)
        .unwrap_or(("".to_string(), existing_content));

    let new_content = if frontmatter_only {
        // Get frontmatter from provided content
        let (new_frontmatter, _) = split_frontmatter_and_body(content)
            .unwrap_or((content.to_string(), "".to_string()));
        merge_frontmatter_and_body(&new_frontmatter, &existing_body)
    } else if content_only {
        // Preserve existing frontmatter, update only the body
        merge_frontmatter_and_body(&existing_frontmatter, content)
    } else {
        // Replace entire content
        content.to_string()
    };

    // Write content to file
    fs::write(path, new_content)
}

/// Commands supported by the CLI
pub enum Command {
    Read(PathBuf),
    Write(PathBuf, String),
    EditFrontmatter(PathBuf, String),
    EditContent(PathBuf, String),
    Extract(PathBuf, String), // Extract a specific frontmatter field
    Help,
    Unknown,
}

/// Print usage information
pub fn print_usage() {
    println!("Content Edit Tool");
    println!("Usage:");
    println!("  content-edit read <file>                   - Read a markdown file");
    println!("  content-edit write <file> <content>        - Write content to a file");
    println!("  content-edit frontmatter <file> <content>  - Update only frontmatter");
    println!("  content-edit content <file> <content>      - Update only content body");
    println!("  content-edit extract <file> <field>        - Extract a specific frontmatter field");
    println!("  content-edit help                          - Show this help message");
}

/// Parse command-line arguments
pub fn parse_args(args: Vec<String>) -> Command {
    if args.len() < 2 {
        return Command::Help;
    }

    match args[1].as_str() {
        "read" => {
            if args.len() < 3 {
                println!("Error: Missing file path");
                return Command::Unknown;
            }
            Command::Read(PathBuf::from(&args[2]))
        },
        "write" => {
            if args.len() < 4 {
                println!("Error: Missing file path or content");
                return Command::Unknown;
            }
            // Replace '\n' with actual newlines
            let content = args[3..].join(" ").replace("\\n", "\n");
            Command::Write(PathBuf::from(&args[2]), content)
        },
        "frontmatter" => {
            if args.len() < 4 {
                println!("Error: Missing file path or frontmatter content");
                return Command::Unknown;
            }
            // Replace '\n' with actual newlines
            let content = args[3..].join(" ").replace("\\n", "\n");
            Command::EditFrontmatter(PathBuf::from(&args[2]), content)
        },
        "content" => {
            if args.len() < 4 {
                println!("Error: Missing file path or content");
                return Command::Unknown;
            }
            // Replace '\n' with actual newlines
            let content = args[3..].join(" ").replace("\\n", "\n");
            Command::EditContent(PathBuf::from(&args[2]), content)
        },
        "extract" => {
            if args.len() < 4 {
                println!("Error: Missing file path or field name");
                return Command::Unknown;
            }
            Command::Extract(PathBuf::from(&args[2]), args[3].clone())
        },
        "help" | "--help" | "-h" => Command::Help,
        _ => {
            println!("Unknown command: {}", args[1]);
            Command::Unknown
        }
    }
}

/// Run a command
pub fn run_command(command: Command) -> i32 {
    match command {
        Command::Read(path) => {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    let (frontmatter, body) = split_frontmatter_and_body(&content)
                        .unwrap_or(("".to_string(), content));
                    println!("Frontmatter:\n---\n{}\n---\n", frontmatter);
                    println!("Content:\n{}", body);
                    0
                },
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    1
                }
            }
        },
        Command::Write(path, content) => {
            if let Err(e) = save_content(&path, &content) {
                eprintln!("Error writing to file: {}", e);
                return 1;
            }
            println!("Content written to {}", path.display());
            0
        },
        Command::EditFrontmatter(path, frontmatter) => {
            let frontmatter_content = format!("---\n{}\n---", frontmatter);
            if let Err(e) = update_content(&path, &frontmatter_content, true, false) {
                eprintln!("Error updating frontmatter: {}", e);
                return 1;
            }
            println!("Frontmatter updated in {}", path.display());
            0
        },
        Command::EditContent(path, content) => {
            if let Err(e) = update_content(&path, &content, false, true) {
                eprintln!("Error updating content: {}", e);
                return 1;
            }
            println!("Content body updated in {}", path.display());
            0
        },
        Command::Extract(path, field) => {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    let frontmatter = extract_frontmatter_as_map(&content);
                    match frontmatter.get(&field) {
                        Some(value) => {
                            println!("{}", value);
                            0
                        },
                        None => {
                            eprintln!("Field '{}' not found in frontmatter", field);
                            1
                        }
                    }
                },
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    1
                }
            }
        },
        Command::Help => {
            print_usage();
            0
        },
        Command::Unknown => {
            print_usage();
            1
        }
    }
}