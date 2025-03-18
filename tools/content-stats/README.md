# Content Stats Tool

A tool for analyzing and reporting content statistics and metrics.

## Features

- Content analytics
- Topic distribution
- Tag analysis
- Word counts
- Asset usage
- Publication trends
- Custom reporting

## Usage

```bash
# Generate all stats
content-stats

# Stats for specific topic
content-stats --topic "strategy"

# Custom date range
content-stats --from "2024-01-01" --to "2024-03-18"

# Export to JSON
content-stats --format json

# Detailed word counts
content-stats --word-counts

# Asset usage report
content-stats --assets
```

## Available Metrics

### Content Overview

- Total content count
- Published vs draft ratio
- Content by topic
- Content by type
- Average word count

### Topic Analysis

- Topic distribution
- Topic growth rate
- Topic engagement
- Topic relationships

### Tag Analysis

- Tag frequency
- Tag combinations
- Tag trends
- Tag coverage

### Asset Usage

- Image counts
- File types
- Asset sizes
- Usage patterns

## Output Formats

### JSON Format

```json
{
  "total_content": 42,
  "topics": {
    "strategy": {
      "count": 15,
      "published": 12,
      "drafts": 3,
      "avg_words": 1500
    }
  },
  "tags": {
    "productivity": 25,
    "focus": 18
  },
  "assets": {
    "images": 156,
    "files": 42
  }
}
```

### CSV Format

```csv
metric,value
total_content,42
published_content,35
draft_content,7
avg_word_count,1250
```

## Custom Reports

Create custom reports using:

```bash
# Custom metric selection
content-stats --metrics "word_count,tag_count,asset_count"

# Custom grouping
content-stats --group-by "topic,status"

# Custom sorting
content-stats --sort-by "word_count" --order "desc"
```
