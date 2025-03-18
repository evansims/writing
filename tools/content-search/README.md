# Content Search Tool

A tool for searching and discovering content within the content management system.

## Features

- Full-text search
- Topic-based filtering
- Tag-based filtering
- Status filtering
- Fuzzy matching
- Result highlighting
- Search history

## Usage

```bash
# Basic search
content-search "query"

# Search in specific topic
content-search "query" --topic "strategy"

# Search by tags
content-search "query" --tags "productivity,focus"

# Search with filters
content-search "query" \
  --topic "strategy" \
  --tags "productivity" \
  --status "published"

# Fuzzy search
content-search "query" --fuzzy

# Search with highlighting
content-search "query" --highlight
```

## Search Options

### Filters

- `--topic`: Filter by topic
- `--tags`: Filter by tags
- `--status`: Filter by status (draft/published)
- `--date`: Filter by date range
- `--author`: Filter by author

### Search Behavior

- `--fuzzy`: Enable fuzzy matching
- `--exact`: Require exact matches
- `--case-sensitive`: Case-sensitive search
- `--highlight`: Show matched terms
- `--limit`: Limit results count

## Output Format

```json
{
  "query": "search query",
  "total": 42,
  "results": [
    {
      "title": "Article Title",
      "slug": "article-slug",
      "topic": "strategy",
      "path": "/strategy/article-slug",
      "excerpt": "Highlighted excerpt...",
      "score": 0.95,
      "tags": ["productivity", "focus"],
      "status": "published"
    }
  ]
}
```

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
