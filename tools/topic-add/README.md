# Topic Add Tool

A tool for adding new topics to the content management system.

## Features

- Interactive topic creation
- Automatic ID generation
- Metadata validation
- Duplicate detection
- Description management
- Parent topic support
- Icon assignment

## Usage

```bash
# Interactive mode
topic-add

# Direct creation
topic-add \
  --id "strategy" \
  --title "Strategy" \
  --description "Articles about strategic thinking" \
  --icon "lightbulb"

# Create with parent topic
topic-add --parent "business" --id "strategy"

# Create with custom metadata
topic-add --id "strategy" --meta "featured:true"
```

## Topic Structure

Topics are stored with the following structure:

```yaml
id: "strategy"
title: "Strategy"
description: "Articles about strategic thinking"
icon: "lightbulb"
parent: "business" # Optional
metadata:
  featured: true # Optional custom metadata
created: "2024-03-18T12:00:00Z"
```

## Validation

The tool performs the following validations:

- ID format (lowercase, alphanumeric with hyphens)
- Unique ID check
- Title length and format
- Description length
- Valid parent topic (if specified)
- Valid icon name
