# Content Validate Tool

Tool for validating content files and their structure.

## Usage

```bash
# Validate all content
./content-validate

# Validate specific topic
./content-validate --topic "strategy"

# Validate with detailed output
./content-validate --verbose
```

## Features

- Content validation
- Link checking
- Image validation
- Metadata validation
- Structure validation
- Detailed reporting

## Validation Checks

- Frontmatter format
- Required fields
- Link validity
- Image existence
- Topic references
- Content structure

## Output

```json
{
  "total_files": 100,
  "valid_files": 95,
  "errors": [
    {
      "file": "content/strategy/article.mdx",
      "error": "Missing required field: description"
    }
  ]
}
```

## Validation Checks

### Structure Validation

- Directory structure
- File naming
- Required files
- File permissions

### Metadata Validation

- Required fields
- Field formats
- Value constraints
- Date formats

### Content Validation

- MDX syntax
- Frontmatter
- Code blocks
- Interactive components

### Reference Validation

- Internal links
- Asset references
- Topic references
- Tag consistency

### Asset Validation

- Image existence
- File formats
- File sizes
- Alt text

## Custom Rules

Define custom validation rules in YAML:

```yaml
rules:
  metadata:
    required_fields:
      - title
      - slug
      - topic
    field_formats:
      slug: "^[a-z0-9-]+$"
      date: "^\\d{4}-\\d{2}-\\d{2}$"

  content:
    max_word_count: 5000
    min_sections: 2
    require_images: true
```

## Output Format

```json
{
  "valid": false,
  "errors": [
    {
      "type": "metadata",
      "file": "strategy/my-article/metadata.yml",
      "message": "Missing required field: topic",
      "line": 1
    }
  ],
  "warnings": [
    {
      "type": "content",
      "file": "strategy/my-article/index.mdx",
      "message": "No alt text for image",
      "line": 42
    }
  ],
  "fixed": [
    {
      "type": "metadata",
      "file": "strategy/my-article/metadata.yml",
      "message": "Fixed date format",
      "line": 5
    }
  ]
}
```

## Auto-fix Capabilities

The tool can automatically fix:

- Date format issues
- Slug formatting
- Tag normalization
- File permissions
- Basic MDX syntax
- Asset references
