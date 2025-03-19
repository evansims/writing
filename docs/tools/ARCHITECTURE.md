# Content Management System Architecture

High-level overview of how the various tools work together.

## Tool Organization

### Primary Interface

The `write` tool serves as the main entry point:

- Command routing
- Configuration management
- Common functionality

### Content Management Tools

Tools for content lifecycle:

- `content-new`: Content creation
- `content-edit`: Content modification
- `content-move`: Content organization
- `content-delete`: Content removal
- `content-search`: Content discovery
- `content-validate`: Content validation
- `content-stats`: Content statistics

### Topic Management Tools

Tools for topic management:

- `topic-add`: Topic creation
- `topic-edit`: Topic modification
- `topic-rename`: Topic renaming
- `topic-delete`: Topic removal

### Image Management Tools

Tools for image processing:

- `image-optimize`: Source image optimization
- `image-build`: Responsive image generation

### Build Tools

Tools for building the site:

- `content-build`: Content processing
- `toc-generate`: Table of contents generation
- `llms-generate`: LLMs data generation

## Data Flow

1. Content and configuration management
2. Content processing and transformation
3. Image processing and optimization
4. Site generation with auxiliary files
