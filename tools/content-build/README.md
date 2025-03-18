# Content Build Tool

Specialized tool for building content files into production formats.

## Usage

```bash
# Build all content
./content-build

# Build specific topic
./content-build --topic "strategy"

# Include drafts
./content-build --include-drafts

# Force rebuild
./content-build --force

# Skip formats
./content-build --skip-html --skip-json

# Custom output
./content-build --output "./dist"
```

## Features

- MDX to HTML/JSON/RSS/Sitemap conversion
- Draft content handling
- Topic-based filtering
- Incremental builds via caching
- Verbose output option

## Output Structure

```
output/
  content/     # Built content files
  feed/        # RSS feed
  sitemap/     # Sitemap files
```
