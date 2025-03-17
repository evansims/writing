# Command Pattern Implementation Guide

This document outlines the standardized command pattern used across all tools in the writing project.

## Core Components

### 1. Command Args

Every command should define a CLI arguments struct using `clap::Parser`:

```rust
#[derive(Parser, Debug)]
#[command(author, version, about = "Description")]
pub struct ExampleArgs {
    /// Slug for the content
    #[arg(short, long)]
    pub slug: Option<String>,
    
    // Other arguments...
}
```

### 2. Command Implementation

Each tool should define a command struct that implements the `Command` trait:

```rust
pub struct ExampleCommand {
    args: ExampleArgs,
}

impl Command for ExampleCommand {
    type Args = ExampleArgs;
    type Output = ExampleResult;
    
    fn new(args: Self::Args) -> Self {
        ExampleCommand { args }
    }
    
    fn execute(&self) -> Result<Self::Output> {
        // Command implementation
        Ok(ExampleResult { /* ... */ })
    }
    
    fn handle_result(result: Self::Output) {
        result.print();
    }
}
```

### 3. Command Result

Each command should define a result struct that implements the `DisplayResult` trait:

```rust
#[derive(Debug)]
pub struct ExampleResult {
    pub message: String,
}

impl DisplayResult for ExampleResult {
    fn to_display(&self) -> String {
        format!("{} {}", "SUCCESS:".green().bold(), self.message)
    }
}
```

### 4. Content Commands

For commands that operate on content, implement the `ContentCommand` trait:

```rust
impl ContentCommand for ExampleCommand {}
```

This provides helper methods for validating slugs and topics, and finding content paths.

### 5. Main Entry Point

The main entry point should be simple and use the command's `run()` method:

```rust
fn main() -> Result<()> {
    ExampleCommand::run()
}
```

## Interactive Commands

For commands that require interactive selection (like selecting content without a slug):

1. Implement interactive selection in the main.rs file, not in the command itself
2. Return an appropriate error from the command if a required argument is not provided
3. Handle the interactive selection in main.rs, then call the command with the selected arguments

Example:

```rust 
// In lib.rs command implementation
fn execute(&self) -> Result<Self::Output> {
    if self.args.slug.is_none() {
        return Err(WritingError::validation_error("No slug provided. Use the CLI to select content interactively.").into());
    }
    
    // Rest of implementation
}

// In main.rs
fn main() -> Result<()> {
    let args = ExampleArgs::parse();
    
    if args.slug.is_none() {
        handle_interactive_selection(args)
    } else {
        ExampleCommand::new(args).execute()
            .map(|result| ExampleCommand::handle_result(result))
    }
}

fn handle_interactive_selection(args: ExampleArgs) -> Result<()> {
    // Interactive selection logic
    // Then create a new args struct with the selection
    let selected_args = ExampleArgs {
        slug: Some(selected_slug),
        // Other args
    };
    
    // Execute the command with the selected args
    let result = ExampleCommand::new(selected_args).execute()?;
    ExampleCommand::handle_result(result);
    Ok(())
}
```

## Creating New Commands

Use the `generate_command` tool to create new command templates:

```bash
cargo run --bin generate_command example-name
```

This will create a new command with the following structure:
- `example-name-command/src/lib.rs` - Command implementation
- `example-name-command/src/main.rs` - CLI entry point
- `example-name-command/Cargo.toml` - Package configuration

Then customize the generated templates to implement your specific command functionality. 