# Common Validation Module

This module provides common validation functions for the writing tools.

## Features

- Content validation: Validate content files for proper structure and required fields
- Frontmatter validation: Ensure frontmatter contains required fields and proper formatting
- Slug validation: Validate and generate slugs for content
- Path validation: Validate and generate paths for content files
- Tag validation: Validate and format tags for content
- Topic validation: Validate topics against configuration

## Usage

```rust
use common_validation::{validate_slug, validate_topic, validate_content_path};
use common_errors::Result;

fn validate_content(slug: &str, topic: Option<&str>) -> Result<()> {
    // Validate slug
    validate_slug(slug)?;
    
    // Validate topic
    let topic = validate_topic(topic)?;
    
    // Validate content path
    let content_path = validate_content_path(slug, topic.as_deref())?;
    
    Ok(())
}
```

## API Reference

### Slug Validation

- `validate_slug(slug: &str) -> Result<String>`: Validates that a slug is properly formatted
- `slugify(title: &str) -> String`: Generates a slug from a title

### Topic Validation

- `validate_topic(topic: Option<&str>) -> Result<Option<String>>`: Validates that a topic exists in the configuration
- `get_available_topics() -> Result<Vec<(String, TopicConfig)>>`: Gets a list of available topics from the configuration

### Path Validation

- `validate_content_path(slug: &str, topic: Option<&str>) -> Result<PathBuf>`: Validates that a content path is valid
- `find_content_path(slug: &str, topic: Option<&str>) -> Result<PathBuf>`: Finds the path to content by slug and optionally topic
- `content_exists(slug: &str, topic: Option<&str>) -> Result<bool>`: Checks if content exists by slug and optionally topic

### Content Validation

- `validate_content(content: &str) -> Result<Frontmatter>`: Validates that content is properly formatted
- `validate_content_body(body: &str) -> Result<()>`: Validates that content body is properly formatted
- `validate_content_title(title: &str) -> Result<()>`: Validates that content title is properly formatted
- `validate_content_tagline(tagline: &str) -> Result<()>`: Validates that content tagline is properly formatted
- `validate_content_date(date: &str) -> Result<()>`: Validates that content date is properly formatted
- `validate_content_type(content_type: &str) -> Result<String>`: Validates that content type is supported

### Frontmatter Validation

- `validate_frontmatter(frontmatter: &str) -> Result<Frontmatter>`: Validates that frontmatter is properly formatted
- `extract_frontmatter(content: &str) -> Result<(Frontmatter, String)>`: Extracts frontmatter from content
- `combine_frontmatter_and_body(frontmatter: &Frontmatter, body: &str) -> String`: Combines frontmatter and body into content

### Tag Validation

- `validate_tags(tags: &str) -> Result<Vec<String>>`: Validates that tags are properly formatted
- `format_tags(tags: &str) -> String`: Formats tags for frontmatter 