# Write Tool

The main CLI tool for managing content, topics, and images.

## Usage

```bash
# Content operations
./write content new
./write content edit
./write content build
./write content validate
./write content search
./write content stats

# Topic operations
./write topic add
./write topic edit
./write topic delete
./write topic rename

# Image operations
./write image optimize
./write image build

# Build operations
./write build
```

## Features

- Content management (create, edit, move, delete)
- Topic management (add, edit, rename, delete)
- Image optimization and responsive generation
- Incremental builds with caching
- Content validation
- Full-text search
- Statistics and metrics

## Configuration

Reads configuration from:

1. Command line arguments
2. Environment variables
3. `.write.yaml` in project root

## Output

Built content is placed in the `public` directory:

```
public/
  content/     # Built content files
  topics/      # Topic metadata
  images/      # Optimized images
  toc.json     # Table of contents
  search.json  # Search index
```
