# Writing Tools

This directory contains the tools used for managing the writing project. The tools are organized in a modular architecture with shared libraries to reduce code duplication and improve maintainability.

## Architecture

The tools follow a layered architecture:

```
tools/
├── common/                  # Shared libraries
│   ├── models/              # Common data structures
│   ├── config/              # Configuration handling
│   ├── fs/                  # Filesystem operations
│   └── markdown/            # Markdown processing utilities
├── content-*/               # Content management tools
├── image-*/                 # Image processing tools
├── topic-*/                 # Topic management tools
└── write/                   # Main CLI tool
```

### Common Libraries

- **common-models**: Shared data structures used across tools
- **common-config**: Configuration loading and parsing
- **common-fs**: Filesystem operations
- **common-markdown**: Markdown processing and frontmatter handling

### Tools

Each tool follows a consistent structure:

- **Binary**: Command-line interface for standalone use
- **Library**: Core functionality that can be imported by other tools

## Building

```bash
# Build all tools
cargo build --release

# Build a specific tool
cargo build --release -p content-new
```

## Development

### Adding a New Tool

1. Create a new directory for your tool
2. Create a Cargo.toml with both lib and bin targets
3. Implement lib.rs with core functionality
4. Implement main.rs with the CLI interface

Example Cargo.toml:

```toml
[package]
name = "new-tool"
version = "0.1.0"
edition = "2021"
description = "Description of the tool"

[lib]
name = "new_tool"
path = "src/lib.rs"

[[bin]]
name = "new-tool"
path = "src/main.rs"

[dependencies]
clap.workspace = true
anyhow.workspace = true
common-models = { path = "../common/models" }
common-config = { path = "../common/config" }
common-fs = { path = "../common/fs" }
common-markdown = { path = "../common/markdown" }
```

### Best Practices

1. **DRY Principle**: Use common libraries instead of duplicating code
2. **Error Handling**: Use anyhow for error propagation
3. **Testing**: Add unit tests for critical functionality
4. **Documentation**: Document all public functions and types

## Direct Usage (No Installation Required)

You can run the CLI directly from the repository root without installing it:

```bash
# From the repository root
./writing --help

# Run interactive mode
./writing interactive

# Create new content
./writing new --title "My Article" --topic "mindset" --tagline "My tagline" --tags "tag1,tag2" --content-type "article"
```

The `./writing` script is a wrapper that automatically builds the CLI if needed and executes it directly from its build location.

## Building from Source

If you want to build the CLI manually:

```bash
cd tools
cargo build --release
```

The binaries will be available in `tools/target/release/`.

## How it Works

The CLI now directly executes the individual tool binaries without using the Makefile. This:

1. Removes the dependency on `make`
2. Improves performance by eliminating process overhead
3. Provides better error handling and output
4. Makes the CLI more self-contained

Each command in the CLI maps to a specific tool binary in the `tools/target/release/` directory.

## Available Commands

- `interactive` - Launch the interactive TUI
- `new` - Create new content

For more details on each command, use `--help`:

```bash
./writing new --help
``` 