# Extended Architecture Reference

Detailed architecture information for the content management system.

## Architecture Details

### Common Libraries

- **models**: Core data structures like Content, Topic, and Config
- **config**: Configuration loading from YAML files and environment variables
- **fs**: File system operations with proper error handling
- **markdown**: MDX parsing, frontmatter extraction, and rendering
- **utils**: Common utilities for string manipulation, logging, etc.
- **validation**: Input validation, path validation, and schema validation
- **errors**: Error types, formatting, and handling

### Tool Responsibilities

#### Content Tools

- **content-new**: Creates new content with templates and metadata
- **content-edit**: Modifies content with frontmatter preservation
- **content-move**: Relocates content with reference updating
- **content-delete**: Removes content with safety checks
- **content-search**: Full-text and metadata search
- **content-validate**: Validates content against schemas
- **content-stats**: Generates statistics about content
- **content-template**: Manages content templates

#### Topic Tools

- **topic-add**: Creates and registers new topics
- **topic-edit**: Updates topic metadata and properties
- **topic-rename**: Renames topics with reference updating
- **topic-delete**: Removes topics with content reassignment

#### Image Tools

- **image-optimize**: Processes source images for web
- **image-build**: Generates responsive images in multiple formats

#### Build Tools

- **content-build**: Processes content for static site
- **toc-generate**: Builds content hierarchy
- **llms-generate**: Creates LLM-friendly content formats

### Directory Structure

The tools follow a consistent directory structure:

```
tools/<tool-name>/
  src/
    lib.rs       # Library interface
    main.rs      # CLI implementation
    actions/     # Core functionality
    models/      # Tool-specific models
    cli/         # CLI parsing
  tests/
    unit/        # Unit tests
    integration/ # Integration tests
    property/    # Property-based tests
  Cargo.toml     # Dependencies
  README.md      # Quick reference
```
