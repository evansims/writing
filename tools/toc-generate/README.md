# TOC Generate Tool

A tool for generating a comprehensive table of contents for all content.

## Features

- Automatic TOC generation
- Topic-based organization
- Support for nested structures
- Metadata inclusion
- Draft content handling
- JSON output format
- Custom output path

## Usage

```bash
# Generate TOC with default settings
toc-generate

# Include draft content
toc-generate --include-drafts

# Custom output file
toc-generate --output "./public/toc.json"

# Generate with specific topic
toc-generate --topic "strategy"
```

## Output Format

The tool generates a JSON file with the following structure:

```json
{
  "version": "1.0",
  "generated": "2024-03-18T12:00:00Z",
  "topics": [
    {
      "id": "strategy",
      "title": "Strategy",
      "description": "Articles about strategic thinking",
      "count": 42
    }
  ],
  "content": [
    {
      "title": "Article Title",
      "slug": "article-slug",
      "topic": "strategy",
      "path": "/strategy/article-slug",
      "created": "2024-03-18T12:00:00Z",
      "updated": "2024-03-18T12:00:00Z",
      "tags": ["productivity", "focus"],
      "type": "article",
      "status": "published"
    }
  ]
}
```

## Use Cases

The generated TOC can be used for:

- Website navigation
- Content discovery
- Search functionality
- RSS feed generation
- Site analytics
- Content auditing
