# Configuration Naming Conventions

This document outlines the standard naming conventions for configuration keys in the writing tools codebase. Following these conventions ensures consistency across the configuration system and improves maintainability.

## General Principles

1. **Consistency**: Use consistent patterns throughout the configuration
2. **Clarity**: Names should clearly indicate their purpose
3. **Specificity**: Be specific but not verbose
4. **Hierarchy**: Use structure to indicate relationships between settings

## Naming Formats

### YAML Configuration Files

- Use `snake_case` for all keys
- Use plural form for collections
- Use descriptive names (avoid abbreviations)
- Group related settings under a common parent

### Rust Struct Fields

- Use `snake_case` for struct fields
- Add clear documentation comments for each field
- Use consistent terminology between YAML keys and struct fields
- Use appropriate type names that match their purpose

## Standard Key Patterns

### Directory Paths

- Always use `_dir` or `_directory` suffix for directory paths
- Example: `base_dir`, `content_directory`

### URLs and URIs

- Always use `_url` or `_uri` suffix
- Example: `site_url`, `api_uri`

### Collections

- Use plural form for collections
- Example: `formats`, `topics`, `tags`

### Maps/Dictionaries

- Use singular naming for the map itself, plural for the keys inside
- Example: `topic` (map) containing multiple `topics` (keys)

### Boolean Flags

- Use affirmative phrasing for boolean settings
- Example: `is_published`, `has_features`

### Size and Dimension Settings

- Use explicit naming with appropriate units
- Example: `width_px`, `max_size_mb`

## Examples

### Before Standardization

```yaml
content:
  baseDir: "content"
  topic:
    blog:
      name: "Blog"
      desc: "Blog posts"
      path: "blog"
```

### After Standardization

```yaml
content:
  base_dir: "content"
  topics:
    blog:
      name: "Blog"
      description: "Blog posts"
      directory: "blog"
```

## Backwards Compatibility

During the transition to standardized naming conventions, we will:

1. Support both old and new key formats
2. Log deprecation warnings when old formats are used
3. Document all key changes in the migration guide
4. Provide utility functions to convert between formats

## Implementation Timeline

- Phase 1: Document all current keys and their standardized versions
- Phase 2: Update model structs with appropriate field names
- Phase 3: Implement backward compatibility layer
- Phase 4: Update all code references to use new standardized names
- Phase 5: Remove backward compatibility after deprecation period
