# Writing

A curated collection of personal writings exploring creativity, engineering, focus, mindset, strategy, and tools. Originally published on my personal site.

## Getting Started

This repository uses a CLI tool to provide a simple interface for common tasks. Before you begin, make sure you have the necessary prerequisites installed.

### Prerequisites

1. **Rust** - Required for building the tools
   ```bash
   # Install Rust on macOS, Linux, or WSL
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Follow the prompts and restart your terminal or run:
   source "$HOME/.cargo/env"
   ```

2. **ImageMagick** - Required for image processing
   ```bash
   # macOS
   brew install imagemagick
   
   # Ubuntu/Debian
   sudo apt-get install imagemagick
   ```

### Using the CLI

The CLI is available directly from the repository root - no installation needed:

```bash
# Show available commands
./write --help

# Launch the interactive CLI
./write interactive
```

The CLI provides both command-line and interactive interfaces for managing your writing project. See [tools/README.md](tools/README.md) for more details.

## Common Tasks

### Creating a New Article

```bash
./write new --title "Article Title" --topic mindset --tagline "A compelling description"
```

### Editing an Existing Article

```bash
./write edit --slug article-slug
```

### Moving an Article

```bash
./write move --slug article-slug --new-slug new-slug
```

### Deleting an Article

```bash
./write delete --slug article-slug
```

### Managing Images

Place a high-quality source image named `index.jpg` in your article directory, then run:

```bash
./write image-optimize --source path/to/image.jpg --article article-slug
```

For more details on image management, see [docs/IMAGE-MANAGEMENT.md](docs/IMAGE-MANAGEMENT.md).

### Generating Table of Contents

```bash
./write toc
```

This generates a table of contents in `build/index.md`.

### Viewing Article Statistics

```bash
./write stats
```

This displays statistics for all articles. For a specific article:

```bash
./write stats --slug article-slug
```

For detailed statistics:

```bash
./write stats --detailed
```

### Listing Content

```bash
# List all articles
./write list

# List all topics with descriptions
./write topics
```

### Managing Topics

```bash
# Add a new topic
./write topic-add --name "New Topic" --description "Description of the new topic"

# Edit a topic
./write topic-edit --key topic-key --name "Updated Name" --description "Updated description"

# Rename a topic
./write topic-rename --key old-key --new-key new-key --new-name "New Name" --new-path new-path

# Delete a topic
./write topic-delete --key topic-key
```

### Generating LLM Files

```bash
./write llms --site-url "https://example.com"
```

This generates `llms.txt` and `llms-full.txt` files in the `build/` directory according to the [llmstxt.org](https://llmstxt.org) standard.

### Building Content

```bash
./write build --site-url "https://example.com"
```

## Repository Structure

- `content/` - Articles organized by topic
- `docs/` - Documentation files
- `templates/` - Templates for new articles
- `tools/` - Rust-based tools for managing the writing collection
- `drafts/` - Work-in-progress articles
- `build/` - Output directory for generated files (not committed)
- `config.yaml` - Configuration file

## Documentation

Detailed documentation is available in the `docs/` directory:

- [Image Management](docs/IMAGE-MANAGEMENT.md) - How images are managed and optimized
- [AVIF Generation](docs/AVIF-GENERATION.md) - How AVIF images are generated

## Troubleshooting

### Rust Not Installed

If you see an error about Rust not being installed:

```
Error: Rust is not installed
```

Install Rust using the command in the Prerequisites section.

### ImageMagick Not Installed

If you see an error about ImageMagick not being installed:

```
Error: ImageMagick is not installed
```

Install ImageMagick using the command in the Prerequisites section.

### CLI Not Building

If the CLI isn't building automatically, you can build it manually:

```bash
cd tools
cargo build --release
```

### AVIF Support in ImageMagick

If you have issues with AVIF generation, ensure your ImageMagick installation supports AVIF:

```bash
convert -list format | grep AVIF
```

If AVIF is not listed, update ImageMagick:

```bash
# macOS
brew upgrade imagemagick

# Ubuntu/Debian
sudo apt-get update && sudo apt-get upgrade imagemagick
```

## License

This repository's content is licensed under [Creative Commons Attribution 4.0 International (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/).
