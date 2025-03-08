# Writing CLI

A beautiful, interactive CLI for managing your writing project.

## Features

- Interactive TUI (Text User Interface) for easy navigation
- Command-line interface for scripting and automation
- Integrates with all existing writing management tools
- Beautiful, colorful output
- Easy to use and install

## Installation

To install the Writing CLI, run the installation script:

```bash
cd tools
./install.sh
```

This will:
1. Build the CLI tool
2. Install it to `~/.local/bin/writing`
3. Add `~/.local/bin` to your PATH (if needed)

## Usage

### Interactive Mode

To launch the interactive TUI:

```bash
writing interactive
```

This will open a beautiful, interactive interface where you can:
- Manage content (create, edit, move, delete, list)
- Manage topics (list, add, edit, rename, delete)
- Manage images (optimize, build)
- Build operations (content, table of contents, LLMs)
- View content statistics

### Command-line Mode

You can also use the CLI in command-line mode for scripting and automation:

```bash
# Create new content
writing new --title "My New Article" --topic "strategy" --tagline "A great article" --tags "productivity,focus" --content-type "article"

# Get help
writing --help
```

## Development

To build the CLI tool:

```bash
cd tools
cargo build --release
```

The binary will be available at `tools/target/release/writing-cli`.

## License

This project is licensed under the same license as the main writing project. 