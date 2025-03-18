# Writing Tools Overview

A collection of Rust tools for managing content, images, and building a static site.

## Core Tools

### Content Management

- `write` - Main CLI tool for content operations
- `content-build` - Builds content into production formats
- `content-validate` - Validates content structure and links
- `content-search` - Full-text search across content
- `content-stats` - Content statistics and metrics

### Topic Management

- `topic-add` - Add new topics
- `topic-delete` - Remove topics
- `topic-edit` - Edit topic metadata
- `topic-rename` - Rename topics

### Image Management

- `image-optimize` - Optimize source images
- `image-build` - Generate responsive images
- `toc-generate` - Generate table of contents
- `llms-generate` - Generate training data

## Usage

```bash
# Content operations
./write content new
./write content edit
./write content build

# Topic operations
./write topic add
./write topic edit
./write topic delete

# Image operations
./write image optimize
./write image build

# Build operations
./write build
```

## Directory Structure

```
content/
  topic/
    article/
      index.mdx
      index.jpg
tools/
  write/
  content-build/
  topic-add/
  ...
docs/
  IMAGE-MANAGEMENT.md
  AVIF-GENERATION.md
  INCREMENTAL-BUILD.md
```

## Features

- MDX content with frontmatter
- Topic-based organization
- Responsive image generation
- Incremental builds
- Content validation
- Full-text search
- Statistics and metrics
