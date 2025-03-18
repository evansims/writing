# Common Libraries

Shared libraries used across all content management tools.

## Components

### Models (`common-models`)

Core data structures and types used throughout the tools:

- Content models
- Topic models
- Image models
- Build configuration
- Cache structures

### Errors (`common-errors`)

Standardized error handling:

- Custom error types
- Error formatting
- Result type aliases
- Error conversion traits

### Config (`common-config`)

Configuration management:

- YAML configuration parsing
- Environment variable support
- Default configuration values
- Configuration validation

## Usage

Add these as dependencies in your `Cargo.toml`:

```toml
[dependencies]
common-models = { path = "../common/models" }
common-errors = { path = "../common/errors" }
common-config = { path = "../common/config" }
```

## Example

```rust
use common_models::Content;
use common_errors::WritingError;
use common_config::Config;

fn process_content() -> Result<(), WritingError> {
    let config = Config::load()?;
    let content = Content::new(
        "title",
        "slug",
        "topic",
        vec!["tag1", "tag2"],
    )?;

    // Process content...
    Ok(())
}
```

## Best Practices

1. Always use the common error types for error handling
2. Use the shared models to ensure consistency
3. Load configuration through the common config module
4. Follow the established patterns in the codebase

## Development

When modifying these libraries:

1. Ensure changes are backward compatible
2. Update all dependent tools
3. Run the full test suite
4. Update documentation as needed
