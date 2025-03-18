# Content Stats Tool

Tool for generating statistics about content.

## Usage

```bash
# Generate stats
./content-stats

# Stats for topic
./content-stats --topic "strategy"

# Stats with options
./content-stats --format "json" --include-drafts
```

## Features

- Content metrics
- Topic statistics
- Word counts
- Image counts
- Link analysis
- Status breakdown

## Output Format

```json
{
  "total_articles": 100,
  "total_words": 50000,
  "total_images": 200,
  "topics": {
    "strategy": {
      "articles": 20,
      "words": 10000,
      "images": 40
    }
  }
}
```

## Metrics

- Article counts
- Word counts
- Image counts
- Link counts
- Topic distribution
- Status distribution
