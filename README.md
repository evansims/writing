# Writing

A personal writing collection exploring creativity, engineering, focus, mindset, strategy, and tools.

## Quick Start

```bash
# Show available commands
./write --help

# Launch interactive mode
./write interactive
```

## Common Commands

```bash
# Content Management
./write new --title "Article Title" --topic mindset
./write edit --slug article-slug
./write move --slug article-slug --new-slug new-slug
./write delete --slug article-slug

# Image Management
./write image-optimize --source path/to/image.jpg --article article-slug

# Content Organization
./write toc
./write stats
./write list
./write topics

# Topic Management
./write topic-add --name "New Topic" --description "Description"
./write topic-edit --key topic-key --name "Updated Name"
./write topic-rename --key old-key --new-key new-key
./write topic-delete --key topic-key

# Build
./write build --site-url "https://example.com"
```

## Project Structure

- `content/` - Articles by topic
- `docs/` - Documentation
- `templates/` - Article templates
- `tools/` - Content management tools
- `drafts/` - Work in progress
- `build/` - Generated files
- `config.yaml` - Configuration
