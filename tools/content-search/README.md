# Content Search Tool

Tool for searching through content files.

## Usage

```bash
# Search content
./content-search "query"

# Search with filters
./content-search "query" --topic "strategy" --type "article"

# Search with options
./content-search "query" --limit 10 --sort "date"
```

## Features

- Full-text search
- Topic filtering
- Type filtering
- Status filtering
- Date filtering
- Result highlighting

## Output Format

```json
{
  "query": "strategy",
  "results": [
    {
      "file": "content/strategy/article.mdx",
      "title": "Strategic Planning",
      "excerpt": "...",
      "score": 0.95
    }
  ]
}
```

## Search Options

- Case sensitivity
- Word boundaries
- Fuzzy matching
- Result limit
- Sort order
- Output format

## Search Index

The tool maintains a search index for efficient searching:

- Full-text content
- Metadata fields
- Asset references
- Topic relationships

## Performance

- Incremental index updates
- Cached search results
- Optimized search algorithms
- Background indexing
