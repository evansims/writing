# Configuration Key Migration Guide

This document maps current configuration keys to their standardized versions following the conventions defined in [CONFIG_NAMING_CONVENTIONS.md](./CONFIG_NAMING_CONVENTIONS.md).

## Migration Strategy

All configuration keys have been reviewed and standardized according to our naming conventions. This document serves as a reference for:

1. Identifying which keys need to be updated
2. Finding the correct standardized key names
3. Understanding the rationale for changes

## Content Configuration

| Current Key                 | Standardized Key                 | Change Rationale                                 |
| --------------------------- | -------------------------------- | ------------------------------------------------ |
| `content.base_dir`          | No change                        | Already follows convention                       |
| `content.topics`            | No change                        | Already follows convention                       |
| `content.topics.<key>.path` | `content.topics.<key>.directory` | More descriptive of purpose; "path" is ambiguous |
| `content.tags`              | No change                        | Already follows convention                       |

## Image Configuration

| Current Key                  | Standardized Key               | Change Rationale           |
| ---------------------------- | ------------------------------ | -------------------------- |
| `images.formats`             | No change                      | Already follows convention |
| `images.format_descriptions` | No change                      | Already follows convention |
| `images.sizes`               | No change                      | Already follows convention |
| `images.sizes.<key>.width`   | `images.sizes.<key>.width_px`  | Add units for clarity      |
| `images.sizes.<key>.height`  | `images.sizes.<key>.height_px` | Add units for clarity      |
| `images.naming.pattern`      | No change                      | Already follows convention |
| `images.naming.examples`     | No change                      | Already follows convention |
| `images.quality`             | No change                      | Already follows convention |

## Publication Configuration

| Current Key             | Standardized Key       | Change Rationale           |
| ----------------------- | ---------------------- | -------------------------- |
| `publication.author`    | No change              | Already follows convention |
| `publication.copyright` | No change              | Already follows convention |
| `publication.site`      | `publication.site_url` | Clarify that this is a URL |

## Frontmatter Configuration

| Current Key                  | Standardized Key                  | Change Rationale            |
| ---------------------------- | --------------------------------- | --------------------------- |
| `frontmatter.title`          | No change                         | Already follows convention  |
| `frontmatter.published`      | `frontmatter.published_at`        | Clarify this is a date/time |
| `frontmatter.updated`        | `frontmatter.updated_at`          | Clarify this is a date/time |
| `frontmatter.slug`           | No change                         | Already follows convention  |
| `frontmatter.tagline`        | No change                         | Already follows convention  |
| `frontmatter.tags`           | No change                         | Already follows convention  |
| `frontmatter.topics`         | No change                         | Already follows convention  |
| `frontmatter.draft`          | `frontmatter.is_draft`            | Boolean flag convention     |
| `frontmatter.featured_image` | `frontmatter.featured_image_path` | Clarify this is a path      |

## Implementation Details

For each key that requires changes:

1. Update the model struct field name
2. Add a serde rename attribute to maintain backward compatibility:
   ```rust
   #[serde(rename = "old_name")]
   pub new_name: Type,
   ```
3. Add documentation comments explaining both old and new names
4. Update any direct field access in the code

## Backward Compatibility

All renamed fields will continue to work with the old configuration format through the use of serde's `rename` attribute. This ensures existing configuration files will continue to work without modification, while allowing new configuration files to use the standardized names.

## Deprecation Timeline

- **Current Release (v1.x)**: Support both old and new key formats
- **Next Major Release (v2.0)**: Log deprecation warnings for old key formats
- **Future Release (v3.0)**: Remove support for old key formats

## Field Update Examples

### Model Updates

```rust
// Before
pub struct TopicConfig {
    pub name: String,
    pub description: String,
    #[serde(rename = "path")]
    pub directory: String,
}

// After
pub struct TopicConfig {
    pub name: String,
    pub description: String,
    #[serde(rename = "path")]
    pub directory: String, // No change needed, already correct
}
```

### Publication Config Update

```rust
// Before
pub struct PublicationConfig {
    pub author: String,
    pub copyright: String,
    pub site: Option<String>,
}

// After
pub struct PublicationConfig {
    pub author: String,
    pub copyright: String,
    #[serde(rename = "site")]
    pub site_url: Option<String>,
}
```
