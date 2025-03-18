# Writing

A personal writing collection exploring creativity, engineering, focus, mindset, strategy, and tools.

## Quick Start

```bash
# Show available commands
./write --help

# Launch interactive mode
./write interactive
```

## Commands

```bash
# Content
./write new --title "Article Title" --topic mindset
./write edit --slug article-slug
./write move --slug article-slug --new-slug new-slug
./write delete --slug article-slug
./write search --query "search term"
./write stats
./write template --name "template-name" --type mdx

# Images
./write image-optimize --source path/to/image.jpg --article article-slug
./write image-build --source path/to/image.jpg --article article-slug

# Organization
./write toc
./write list
./write topics

# Topics
./write topic-add --name "New Topic" --description "Description"
./write topic-edit --key topic-key --name "Updated Name"
./write topic-rename --key old-key --new-key new-key
./write topic-delete --key topic-key

# Build
./write build --site-url "https://example.com"
./write build benchmark --current ./target/criterion
```

## Structure

- `content/` - Articles by topic
- `docs/` - Documentation
- `templates/` - Article templates
- `tools/` - Content management tools
  - `content-*` - Content manipulation tools
  - `image-*` - Image processing tools
  - `benchmark-analyze/` - Performance analysis
  - `toc-generate/` - Table of contents generation
- `drafts/` - Work in progress
- `build/` - Generated files
- `config.yaml` - Configuration

## License

This project is licensed under the Creative Commons Attribution 4.0 International License - see the [LICENSE](LICENSE) file for details.
