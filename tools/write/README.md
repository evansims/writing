# Write Tool

The `write` tool is the primary interface for managing content in this project. It provides a comprehensive set of commands for creating, editing, building, and managing content, topics, and images.

## Features

- Content Management

  - Create new content
  - Edit existing content
  - Move/rename content
  - Delete content
  - Search content
  - View content statistics
  - Validate content
  - Build content for production

- Topic Management

  - Add new topics
  - Edit topic metadata
  - Rename topics
  - Delete topics

- Image Management

  - Optimize images
  - Build image assets

- Build Tools
  - Generate table of contents
  - Generate LLM training data
  - Build search indices
  - Build content for production

## Usage

The tool can be used either interactively through menus or directly via command-line arguments:

```bash
# Interactive mode
write

# Direct commands
write content new      # Create new content
write content edit    # Edit existing content
write content build   # Build content for production
write topic add      # Add a new topic
write image optimize # Optimize images
```

## Build Cache

The tool implements a sophisticated build cache system to improve performance:

- LazyBuildCache: Provides thread-safe caching with automatic invalidation
- Intelligent rebuild detection: Only rebuilds modified content
- Cache persistence: Saves build state between runs

## Configuration

The tool reads configuration from the following sources (in order of precedence):

1. Command line arguments
2. Environment variables
3. Configuration file (`.write.yaml` in project root)

## Output

By default, all built content is placed in the `public` directory with the following structure:

```
public/
  ├── content/      # Built content files
  ├── topics/       # Topic metadata and indices
  ├── images/       # Optimized images
  ├── toc.json      # Table of contents
  ├── search.json   # Search index
  └── llm/          # LLM training data
```
