# Topic Edit Tool

A tool for modifying existing topic metadata and properties.

## Features

- Interactive topic editing
- Metadata modification
- Description updates
- Icon changes
- Parent topic management
- Custom metadata editing
- Change validation

## Usage

```bash
# Interactive mode
topic-edit strategy

# Update specific fields
topic-edit strategy \
  --title "Strategic Planning" \
  --description "Updated description"

# Update icon
topic-edit strategy --icon "chart"

# Change parent topic
topic-edit strategy --parent "business"

# Update custom metadata
topic-edit strategy --meta "featured:false"

# Remove parent topic
topic-edit strategy --remove-parent
```

## Editable Fields

The following fields can be modified:

- Title
- Description
- Icon
- Parent topic
- Custom metadata

## Validation

All changes undergo the same validation as topic creation:

- Title format and length
- Description length
- Valid icon name
- Valid parent topic
- Metadata format

## Change History

The tool maintains a history of changes:

```yaml
history:
  - timestamp: "2024-03-18T12:00:00Z"
    field: "title"
    old: "Strategy"
    new: "Strategic Planning"
  - timestamp: "2024-03-18T12:00:00Z"
    field: "description"
    old: "Old description"
    new: "Updated description"
```
