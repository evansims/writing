# Content Edit Tool

A tool for editing existing content and its metadata.

## Features

- Interactive content editing
- Metadata modification
- Content validation
- Draft management
- Tag management
- Asset handling
- Change tracking

## Usage

```bash
# Interactive mode
content-edit strategy/my-article

# Edit specific fields
content-edit strategy/my-article \
  --title "Updated Title" \
  --tagline "New tagline" \
  --tags "productivity,focus,strategy"

# Change status
content-edit strategy/my-article --status "draft"

# Update content file
content-edit strategy/my-article --edit

# Move to different topic
content-edit strategy/my-article --move-to "business"
```

## Editable Components

### Metadata (`metadata.yml`)

```yaml
title: "Article Title"
tagline: "Article tagline"
tags: ["tag1", "tag2"]
status: "draft|published"
updated: "2024-03-18T12:00:00Z"
```

### Content (`index.mdx`)

- Main content file
- Supports MDX format
- Asset references
- Code blocks
- Interactive components

### Assets

- Image management
- File attachments
- Resource linking

## Validation

The tool validates:

- Metadata format
- Required fields
- Tag format
- Status values
- Asset references
- MDX syntax

## Change Tracking

Each edit is tracked with:

- Timestamp
- Modified fields
- Previous values
- Editor information
