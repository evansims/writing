# Content Build Tool

A specialized tool for building content files into their final formats for production use.

## Features

- Converts content from MDX to various output formats:
  - HTML for web display
  - JSON for API consumption
  - RSS for feed readers
  - Sitemap for search engines
- Supports draft content handling
- Topic-based filtering
- Incremental builds via caching
- Verbose output option for debugging

## Usage

```bash
# Build all content
content-build

# Build specific topic
content-build --topic "strategy"

# Build with drafts included
content-build --include-drafts

# Force rebuild all content
content-build --force

# Skip certain output formats
content-build --skip-html --skip-json

# Specify custom output directory
content-build --output "./dist"
```

## Output Structure

```
[output_dir]/
  ├── content/
  │   ├── [topic]/
  │   │   └── [slug]/
  │   │       ├── index.html
  │   │       └── index.json
  ├── feed.xml
  └── sitemap.xml
```
