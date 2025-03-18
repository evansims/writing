# LLMs Generate Tool

A specialized tool for generating training data for Large Language Models (LLMs) from your content.

## Features

- Generates LLM-friendly text formats
- Supports LLMS.txt format specification
- Includes metadata and context
- Handles draft content appropriately
- Configurable site URL
- Custom output directory support

## Usage

```bash
# Generate with default settings
llms-generate

# Include draft content
llms-generate --include-drafts

# Specify site URL
llms-generate --site-url "https://example.com"

# Custom output directory
llms-generate --output "./llm-data"
```

## Output Format

The tool generates two main files:

1. `llms.txt` - Core content in LLMS.txt format
2. `llms-full.txt` - Extended content with additional context

### LLMS.txt Format

```
# TITLE: Article Title
# URL: https://example.com/article
# TOPIC: Strategy
# TAGS: productivity, focus
# DATE: 2024-03-18

Article content in plain text format...

# SECTION: Subheading
Section content...
```

## Output Structure

```
[output_dir]/
  ├── llms.txt       # Core content
  ├── llms-full.txt  # Extended content
  └── metadata.json  # Build metadata
```

## Metadata

The generated metadata.json includes:

```json
{
  "site": "https://example.com",
  "include_drafts": false,
  "timestamp": "2024-03-18T12:00:00Z",
  "content_count": 42,
  "topics": ["strategy", "productivity"],
  "format_version": "1.0"
}
```
