# Topic Add Tool

Tool for adding new topics to the content management system.

## Usage

```bash
# Interactive mode
./topic-add

# Direct creation
./topic-add --id "strategy" --title "Strategy" --description "Business strategy content"
```

## Features

- Interactive topic creation
- Automatic ID generation
- Metadata validation
- Duplicate detection
- Description management
- Parent topic support
- Icon assignment

## Topic Structure

```yaml
id: strategy
title: Strategy
description: Business strategy content
icon: chart-line
parent: business
metadata:
  tags:
    - business
    - planning
```

## Validation

- ID format and uniqueness
- Title and description length
- Valid parent topic
- Valid icon name
