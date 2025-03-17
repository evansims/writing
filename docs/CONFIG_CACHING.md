# Configuration Caching

This document describes the configuration caching mechanism provided by the `common_config::cache` module in the Writing project.

## Overview

Configuration caching is essential for improving performance by avoiding repeated filesystem access when loading configuration. The `common_config::cache` module provides a thread-safe caching mechanism that automatically invalidates the cache when the configuration file is modified or after a specified time period.

## Benefits

- **Improved Performance**: Reduces filesystem access by caching configuration in memory
- **Automatic Cache Invalidation**: Detects file modifications and refreshes the cache when needed
- **Thread Safety**: Provides thread-safe access to the configuration cache
- **Configurable Cache Lifetime**: Allows specifying the maximum age of cached entries
- **Lazy Loading**: Only loads the configuration when it's actually needed

## Available Types and Functions

### ConfigCache

A thread-safe configuration cache that automatically invalidates entries based on file modification time and age.

```rust
pub struct ConfigCache {
    cache: Mutex<Option<CacheEntry>>,
    max_age: Duration,
    check_modifications: bool,
}
```

#### Methods

- **new**: Creates a new configuration cache with the specified maximum age and modification checking
- **global**: Returns a reference to the global configuration cache instance
- **get_config**: Gets the cached configuration, loading it from disk if necessary
- **get_config_from_path**: Gets the cached configuration from a specific path
- **clear**: Clears the configuration cache

### Global Functions

- **load_config**: Loads the configuration using the global cache instance
- **load_config_from_path**: Loads the configuration from a specific path, using the cache if enabled
- **clear_config_cache**: Clears the global configuration cache

## Usage Examples

### Basic Usage

```rust
use common_config::{load_config, clear_config_cache};

fn get_config_example() -> common_errors::Result<()> {
    // Load the configuration (uses the cache)
    let config = load_config()?;
    
    println!("Author: {}", config.publication.author);
    
    // Subsequent calls will use the cached config
    let config2 = load_config()?;
    
    // Clear the cache if needed
    clear_config_cache();
    
    // This will reload the config from disk
    let config3 = load_config()?;
    
    Ok(())
}
```

### Using the Cache Directly

```rust
use common_config::cache::ConfigCache;
use std::time::Duration;

fn custom_cache_example() -> common_errors::Result<()> {
    // Create a custom cache with a 1-minute max age
    let cache = ConfigCache::new(Duration::from_secs(60), true);
    
    // Get the config from the cache
    let config = cache.get_config()?;
    
    println!("Author: {}", config.publication.author);
    
    // Clear the cache if needed
    cache.clear();
    
    Ok(())
}
```

### Loading from a Specific Path

```rust
use common_config::{load_config_from_path, clear_config_cache};
use std::path::Path;

fn specific_path_example() -> common_errors::Result<()> {
    // Load the configuration from a specific path
    let config = load_config_from_path(Path::new("config.yaml"))?;
    
    println!("Author: {}", config.publication.author);
    
    // Clear the cache if needed
    clear_config_cache();
    
    Ok(())
}
```

### Disabling Caching

Caching is enabled by default, but can be disabled by disabling the `cache` feature:

```toml
# In Cargo.toml
[dependencies]
common-config = { path = "../common/config", default-features = false }
```

## Best Practices

1. **Use the Global Cache**: For most use cases, the global cache instance provided by `ConfigCache::global()` is sufficient.

2. **Clear the Cache When Needed**: If you know the configuration file has been modified, call `clear_config_cache()` to force a reload.

3. **Consider Cache Lifetime**: The default cache lifetime is 5 minutes, which is suitable for most use cases. If you need a different lifetime, create a custom cache instance.

4. **Be Aware of File Modifications**: By default, the cache checks for file modifications, but this can be disabled if needed.

5. **Use Feature Flags**: If you don't need caching, you can disable it by disabling the `cache` feature.

## Implementation Details

The configuration caching mechanism is implemented in the `common_config::cache` module. The implementation uses a `Mutex` to provide thread-safe access to the cache, and a `CacheEntry` struct to store the cached configuration along with metadata like the file path, last modification time, and creation time.

The cache is automatically invalidated when:

1. The configuration file is modified (if `check_modifications` is `true`)
2. The cache entry is older than the specified maximum age

The global cache instance is created using the singleton pattern, ensuring that there's only one instance of the cache throughout the application.

## Testing

The configuration caching mechanism is thoroughly tested to ensure it works correctly. The tests cover various scenarios, including:

- Creating and using the cache
- Accessing the global cache instance
- Loading configuration from a specific path
- Clearing the cache
- Handling file modifications
- Cache invalidation based on age

## Conclusion

The configuration caching mechanism provided by the `common_config::cache` module is essential for improving performance by avoiding repeated filesystem access when loading configuration. By using this mechanism, you can ensure that your application loads configuration efficiently while still detecting changes to the configuration file. 