# Content Template Tool

Tool for managing content templates.

## Usage

```bash
# List templates
./content-template list

# Create template
./content-template create --name "article" --type "mdx"

# Apply template
./content-template apply --name "article" --target "content/strategy/new-article"
```

## Features

- Template creation
- Template editing
- Template deletion
- Template application
- Variable substitution
- Template validation

## Template Structure

```yaml
name: article
type: mdx
variables:
  - title
  - description
  - topic
content: |
  ---
  title: {{title}}
  description: {{description}}
  topic: {{topic}}
  ---

  # {{title}}
```

## Validation

- Template format
- Required variables
- Content structure
- Variable usage
- Output validation
