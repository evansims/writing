# Table of Contents Generator

Tool for generating a table of contents from content files.

## Usage

```bash
# Generate TOC
./toc-generate

# Generate for topic
./toc-generate --topic "strategy"

# Generate with custom output
./toc-generate --output "toc.json"
```

## Features

- TOC generation
- Topic organization
- Hierarchical structure
- Metadata inclusion
- Custom sorting
- Output formats

## Output Format

```json
{
  "topics": [
    {
      "id": "strategy",
      "title": "Strategy",
      "articles": [
        {
          "id": "strategic-planning",
          "title": "Strategic Planning",
          "description": "...",
          "date": "2024-03-18"
        }
      ]
    }
  ]
}
```

## Options

- Topic filtering
- Sort order
- Output format
- Metadata fields
- Custom fields
- Output location
