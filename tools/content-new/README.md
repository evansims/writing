# Content New Tool

A tool for creating new content with proper structure and metadata.

## Features

- Interactive content creation wizard
- Template-based content generation
- Automatic slug generation
- Topic validation and selection
- Tag management
- Draft status handling
- Custom metadata support

## Usage

```bash
# Interactive mode
content-new

# Direct creation
content-new \
  --title "My New Article" \
  --topic "strategy" \
  --tagline "A great article about strategy" \
  --tags "productivity,focus" \
  --type "article"

# Create from template
content-new --template "blog-post"

# Create as draft
content-new --draft
```

## Content Structure

New content is created with the following structure:

```
content/
  └── [topic]/
      └── [slug]/
          ├── index.mdx    # Main content file
          ├── metadata.yml # Content metadata
          └── assets/      # Content-specific assets
```

## Templates

The tool supports various content templates:

- `article` - Standard article format
- `blog-post` - Blog post with metadata
- `tutorial` - Step-by-step tutorial format
- `review` - Product/service review format

## Metadata

Generated metadata includes:

```yaml
title: "My New Article"
slug: "my-new-article"
topic: "strategy"
tagline: "A great article about strategy"
tags: ["productivity", "focus"]
type: "article"
created: "2024-03-18T12:00:00Z"
status: "draft"
```
