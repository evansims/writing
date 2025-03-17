# Context-Specific Configuration Views

This document describes the context-specific configuration views provided by the `common_config::views` module in the Writing project.

## Overview

Context-specific configuration views provide a simplified and focused interface to the configuration for different tools. Each view exposes only the configuration properties relevant to a specific context, making it easier to work with the configuration in a type-safe and context-aware manner.

## Benefits

- **Simplified Access**: Provides a simplified interface to the configuration
- **Type Safety**: Ensures type-safe access to configuration properties
- **Context Awareness**: Exposes only the configuration properties relevant to a specific context
- **Improved Readability**: Makes code more readable by using domain-specific methods
- **Cached Access**: Uses the configuration cache for improved performance

## Available Views

### ContentView

A view for content-related configuration, providing access to the base directory, topics, and other content-related properties.

```rust
pub struct ContentView {
    config: Config,
}
```

#### Methods

- **new**: Creates a new content view
- **from_path**: Creates a new content view from a specific path
- **base_dir**: Gets the base directory for content
- **base_dir_path**: Gets the base directory as a path
- **topics**: Gets all topics
- **topic_keys**: Gets all topic keys
- **topic**: Gets a specific topic by key
- **validate_topic**: Validates that a topic exists
- **topic_path**: Gets the path for a specific topic
- **topic_full_path**: Gets the full path for a specific topic

### ImageView

A view for image-related configuration, providing access to supported formats, sizes, and other image-related properties.

```rust
pub struct ImageView {
    config: Config,
}
```

#### Methods

- **new**: Creates a new image view
- **from_path**: Creates a new image view from a specific path
- **formats**: Gets the supported image formats
- **sizes**: Gets all image sizes
- **size_keys**: Gets all image size keys
- **size**: Gets a specific image size by key
- **validate_size**: Validates that an image size exists

### PublicationView

A view for publication-related configuration, providing access to the author, copyright, site URL, and other publication-related properties.

```rust
pub struct PublicationView {
    config: Config,
}
```

#### Methods

- **new**: Creates a new publication view
- **from_path**: Creates a new publication view from a specific path
- **author**: Gets the author
- **copyright**: Gets the copyright
- **site**: Gets the site URL

## Usage Examples

### Using ContentView

```rust
use common_config::views::ContentView;
use common_errors::Result;

fn list_topics() -> Result<()> {
    let view = ContentView::new()?;
    
    println!("Base directory: {}", view.base_dir());
    
    for key in view.topic_keys() {
        let topic = view.topic(&key).unwrap();
        println!("Topic: {} - {}", topic.name, topic.description);
    }
    
    Ok(())
}

fn get_topic_path(key: &str) -> Result<()> {
    let view = ContentView::new()?;
    
    if let Some(path) = view.topic_full_path(key) {
        println!("Full path for {}: {}", key, path.display());
    } else {
        println!("Topic not found: {}", key);
    }
    
    Ok(())
}
```

### Using ImageView

```rust
use common_config::views::ImageView;
use common_errors::Result;

fn list_image_sizes() -> Result<()> {
    let view = ImageView::new()?;
    
    println!("Supported formats: {:?}", view.formats());
    
    for key in view.size_keys() {
        let size = view.size(&key).unwrap();
        println!("Size {}: {}x{} - {}", key, size.width, size.height, size.description);
    }
    
    Ok(())
}

fn get_image_size(key: &str) -> Result<()> {
    let view = ImageView::new()?;
    
    match view.validate_size(key) {
        Ok(size) => println!("Size {}: {}x{}", key, size.width, size.height),
        Err(_) => println!("Size not found: {}", key),
    }
    
    Ok(())
}
```

### Using PublicationView

```rust
use common_config::views::PublicationView;
use common_errors::Result;

fn show_publication_info() -> Result<()> {
    let view = PublicationView::new()?;
    
    println!("Author: {}", view.author());
    println!("Copyright: {}", view.copyright());
    
    if let Some(site) = view.site() {
        println!("Site: {}", site);
    }
    
    Ok(())
}
```

### Creating Custom Views

You can create custom views by implementing the `ConfigView` trait:

```rust
use common_config::views::ConfigView;
use common_models::Config;
use common_errors::Result;

struct CustomView {
    config: Config,
}

impl ConfigView for CustomView {
    fn config(&self) -> &Config {
        &self.config
    }
    
    fn from_config(config: Config) -> Self {
        CustomView { config }
    }
}

impl CustomView {
    fn new() -> Result<Self> {
        let config = common_config::load_config()?;
        Ok(CustomView { config })
    }
    
    fn custom_property(&self) -> &str {
        &self.config.custom.property
    }
}
```

## Best Practices

1. **Use the Appropriate View**: Choose the view that best matches your context to keep your code focused and readable.

2. **Prefer Views Over Direct Access**: Use views instead of directly accessing the configuration to benefit from type safety and context awareness.

3. **Create Custom Views for Specific Needs**: If the existing views don't meet your needs, create a custom view that exposes only the properties you need.

4. **Validate Input**: Use the validation methods provided by the views to ensure that the configuration properties you're accessing exist.

5. **Handle Errors Gracefully**: Always handle errors from view methods gracefully, providing meaningful error messages to users.

## Implementation Details

The context-specific configuration views are implemented in the `common_config::views` module. Each view is a struct that wraps a `Config` instance and provides methods for accessing specific parts of the configuration.

The views use the configuration cache internally, ensuring that they benefit from the same performance improvements as direct configuration access.

## Testing

The context-specific configuration views are thoroughly tested to ensure they work correctly. The tests cover various scenarios, including:

- Creating views from the default configuration
- Creating views from a specific path
- Accessing configuration properties
- Validating configuration properties
- Handling missing properties

## Conclusion

The context-specific configuration views provided by the `common_config::views` module are essential for simplifying access to the configuration and ensuring type safety and context awareness. By using these views, you can make your code more readable, maintainable, and robust. 