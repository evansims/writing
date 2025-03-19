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

## Testing

We use Cargo Nextest for running tests and LLVM-based coverage tools for measuring test coverage.

### Running Tests

```bash
# Run all tests using Nextest
cargo nextest run --workspace

# Run tests for a specific tool
cargo nextest run -p content-new

# Run tests with specific features
cargo nextest run --workspace --all-features

# Run tests with a specific profile
cargo nextest run --workspace --profile local
```

### Measuring Test Coverage

We provide a coverage script to simplify generating coverage reports:

```bash
# Generate summary coverage report
./coverage.sh

# Generate HTML coverage report
./coverage.sh html

# Generate HTML report and open it in browser
./coverage.sh open

# Generate LCOV report
./coverage.sh lcov

# Show help for coverage script
./coverage.sh help
```

### Coverage Thresholds

We aim for the following coverage targets:

- Common libraries: 90%+ coverage
- Individual tools: 80%+ coverage
- Overall project: 80%+ coverage

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
3. **Testing**:
   - Write unit tests for all critical functionality
   - Keep tests isolated from other tools
   - Aim for 80%+ code coverage
   - Follow TDD principles where possible
4. **Documentation**: Document all public functions and types

## Direct Usage (No Installation Required)

You can run the CLI directly from the repository root without installing it:

```bash
# From the repository root
./write --help

# Run interactive mode
./write interactive

# Create new content
./write new --title "My Article" --topic "mindset" --description "My description" --tags "tag1,tag2" --content-type "article"
```

The `./write` script is a wrapper that automatically builds the CLI if needed and executes it directly from its build location.

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
./write new --help
```

## Recent Improvements

### Testing Framework Update

We've updated our testing framework to use Cargo Nextest, which provides:

- Faster test execution through parallelization
- Better test organization and filtering
- Improved test output and reporting
- Test stability improvements

### Code Coverage Tools

We now use cargo-llvm-cov for coverage measurement, which:

- Provides more accurate coverage data
- Integrates better with Rust tooling
- Generates HTML reports for easy inspection
- Supports coverage thresholds and gating

### Incremental Building

The `write build` command now supports incremental building, which significantly improves performance when working with large content repositories. Only files that have changed since the last build will be processed, resulting in much faster builds.

To force a full rebuild of all content:

```bash
./write build --rebuild
```

See [docs/INCREMENTAL-BUILD.md](../docs/INCREMENTAL-BUILD.md) for more details.

### Lazy Configuration Loading

Configuration is now loaded lazily, which improves memory usage and startup time. The configuration is only loaded when it's actually needed, rather than at application startup.

### Parallel Image Processing

Image operations now use parallel processing, which significantly improves performance when working with multiple images.
