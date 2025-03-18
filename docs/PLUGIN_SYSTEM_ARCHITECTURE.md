# Plugin System Architecture

This document describes the architecture and implementation of the plugin system that enables extending the writing tools with custom functionality.

## Overview

The plugin system allows developers to extend the core functionality of the writing tools without modifying the source code. It provides a stable, versioned API for plugins to interact with the core system and hook into various extension points.

## Design Goals

The plugin system was designed with the following goals in mind:

1. **Safety**: Plugins should not be able to compromise the system or cause data loss
2. **Stability**: The plugin API should be stable and versioned
3. **Simplicity**: Creating plugins should be straightforward
4. **Performance**: Plugins should have minimal performance impact
5. **Isolation**: Plugins should be isolated from each other
6. **Discoverability**: Plugins should be easy to discover and install

## High-Level Architecture

The plugin system consists of the following components:

1. **Plugin Registry**: Manages plugin registration and discovery
2. **Plugin Loader**: Loads plugin code and initializes plugins
3. **Hook System**: Defines extension points and manages hook registration
4. **Plugin API**: Provides a stable interface for plugins to interact with the core system
5. **Plugin Sandbox**: Provides an isolated environment for plugin execution
6. **Plugin Manifest**: Defines plugin metadata and capabilities

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│                 │    │                 │    │                 │
│   Plugin One    │    │   Plugin Two    │    │   Plugin Three  │
│                 │    │                 │    │                 │
└────────┬────────┘    └────────┬────────┘    └────────┬────────┘
         │                      │                      │
         │                      │                      │
         ▼                      ▼                      ▼
┌────────────────────────────────────────────────────────────────┐
│                          Plugin API                            │
└────────────────────────────────────────────────────────────────┘
         │                      │                      │
         │                      │                      │
         ▼                      ▼                      ▼
┌────────────────────────────────────────────────────────────────┐
│                        Plugin Registry                         │
└────────────────────────────────────────────────────────────────┘
         │                      │                      │
         │                      │                      │
         ▼                      ▼                      ▼
┌────────────────────────────────────────────────────────────────┐
│                          Hook System                           │
└────────────────────────────────────────────────────────────────┘
         │                      │                      │
         │                      │                      │
         ▼                      ▼                      ▼
┌────────────────────────────────────────────────────────────────┐
│                           Core System                          │
└────────────────────────────────────────────────────────────────┘
```

## Plugin Discovery and Loading

Plugins are discovered and loaded in the following sequence:

1. Scan plugin directories for plugin manifests
2. Parse and validate plugin manifests
3. Sort plugins by dependencies and load order
4. Load plugins in the correct order
5. Initialize each plugin
6. Register plugin hooks

### Plugin Directories

Plugins are discovered in the following locations:

- `~/.config/writing-tools/plugins/`: User-installed plugins
- `<installation_dir>/plugins/`: System-wide plugins

### Plugin Manifest

Each plugin must have a `plugin.toml` file at its root with the following structure:

```toml
[plugin]
name = "my-plugin"
version = "1.0.0"
description = "A sample plugin"
author = "Plugin Author"
repository = "https://github.com/author/my-plugin"
api_version = "1.0"

[dependencies]
other-plugin = "^1.0.0"

[capabilities]
content_hooks = true
build_hooks = true
image_hooks = false
```

## Plugin API

The plugin API is versioned to ensure compatibility between plugins and the core system. The API includes:

- Core data structures
- Hook registration
- Configuration access
- Content manipulation
- Utility functions

### API Versioning

Plugin API versions follow semantic versioning:

- **Major version**: Breaking changes
- **Minor version**: Non-breaking additions
- **Patch version**: Bug fixes

Plugins specify the API version they target in their manifest.

## Hook System

The hook system defines extension points where plugins can register callbacks. Hooks are categorized by functionality:

### Content Hooks

```rust
pub trait ContentHooks {
    fn pre_content_create(&self, content_type: &ContentType, slug: &str, options: &ContentCreationOptions) -> Result<(), Error>;
    fn post_content_create(&self, content: &dyn Content) -> Result<(), Error>;
    fn pre_content_update(&self, content: &dyn Content, updates: &ContentUpdateOptions) -> Result<(), Error>;
    fn post_content_update(&self, content: &dyn Content) -> Result<(), Error>;
    fn pre_content_delete(&self, content: &dyn Content) -> Result<(), Error>;
    fn post_content_delete(&self, path: &Path) -> Result<(), Error>;
}
```

### Build Hooks

```rust
pub trait BuildHooks {
    fn pre_build(&self, options: &BuildOptions) -> Result<(), Error>;
    fn post_content_process(&self, content: &dyn Content, output: &mut String) -> Result<(), Error>;
    fn post_build(&self, output_dir: &Path) -> Result<(), Error>;
}
```

### Image Hooks

```rust
pub trait ImageHooks {
    fn pre_image_optimize(&self, source_path: &Path, options: &ImageOptimizationOptions) -> Result<(), Error>;
    fn post_image_optimize(&self, optimized_image: &OptimizedImage) -> Result<(), Error>;
}
```

### UI Hooks

```rust
pub trait UiHooks {
    fn register_commands(&self, registry: &mut CommandRegistry) -> Result<(), Error>;
    fn customize_progress(&self, progress: &mut ProgressBar) -> Result<(), Error>;
}
```

## Plugin Sandbox

Plugins run in a sandboxed environment to ensure they cannot compromise system security or stability:

1. **File System Access**: Limited to specific directories
2. **Network Access**: Disabled by default, can be enabled with explicit permission
3. **Process Spawning**: Disabled
4. **Memory Limits**: Enforced to prevent resource exhaustion

## Plugin Registration

Plugins register their capabilities with the plugin registry:

```rust
pub fn register(registry: &mut PluginRegistry) -> Result<(), Error> {
    // Register content hooks
    registry.register_content_hooks(MyContentHooks::new())?;

    // Register build hooks
    registry.register_build_hooks(MyBuildHooks::new())?;

    // Register commands
    registry.register_command("my-command", MyCommand::new())?;

    Ok(())
}
```

## Plugin Lifecycle

Plugins go through the following lifecycle:

1. **Discovery**: Plugin is discovered in a plugin directory
2. **Validation**: Plugin manifest is validated
3. **Loading**: Plugin code is loaded
4. **Initialization**: Plugin is initialized
5. **Registration**: Plugin registers hooks and capabilities
6. **Runtime**: Plugin hooks are invoked during system operation
7. **Shutdown**: Plugin resources are released during system shutdown

## Hook Execution

When a hook point is reached, the system:

1. Identifies all registered hooks for the extension point
2. Sorts hooks by priority
3. Executes each hook in order
4. Continues execution if all hooks succeed, or aborts if any hook fails

## Plugin Configuration

Plugins can have their own configuration, stored in `~/.config/writing-tools/plugins/<plugin-name>/config.toml`.

The plugin API provides access to this configuration:

```rust
let config = registry.get_plugin_config()?;
let value = config.get::<String>("my-setting")?;
```

## Error Handling

Plugins must handle errors gracefully and return structured errors:

```rust
fn pre_content_create(&self, content_type: &ContentType, slug: &str, options: &ContentCreationOptions) -> Result<(), Error> {
    if slug.contains("forbidden") {
        return Err(Error::new(ErrorKind::ValidationError, "Slug contains forbidden word"));
    }
    Ok(())
}
```

## Plugin Development Guide

### Creating a Basic Plugin

1. Create a new Rust crate with the following structure:

```
my-plugin/
├── Cargo.toml
├── plugin.toml
└── src/
    └── lib.rs
```

2. Define the plugin manifest in `plugin.toml`:

```toml
[plugin]
name = "my-plugin"
version = "1.0.0"
description = "A sample plugin"
author = "Plugin Author"
repository = "https://github.com/author/my-plugin"
api_version = "1.0"

[capabilities]
content_hooks = true
```

3. Implement the plugin in `lib.rs`:

```rust
use writing_tools_plugin_api::{Plugin, PluginRegistry, Content, ContentHooks, Result, Error};

pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn register(&self, registry: &mut PluginRegistry) -> Result<()> {
        registry.register_content_hooks(MyContentHooks::new())?;
        Ok(())
    }
}

pub struct MyContentHooks;

impl MyContentHooks {
    pub fn new() -> Self {
        Self
    }
}

impl ContentHooks for MyContentHooks {
    fn post_content_create(&self, content: &dyn Content) -> Result<()> {
        println!("Created content: {}", content.title());
        Ok(())
    }

    // Implement other hooks as needed
}

// Plugin entry point
#[no_mangle]
pub fn init_plugin() -> Box<dyn Plugin> {
    Box::new(MyPlugin)
}
```

4. Build the plugin:

```bash
cargo build --release
```

5. Install the plugin:

```bash
mkdir -p ~/.config/writing-tools/plugins/my-plugin
cp target/release/libmy_plugin.so ~/.config/writing-tools/plugins/my-plugin/
cp plugin.toml ~/.config/writing-tools/plugins/my-plugin/
```

### Testing Plugins

The plugin API provides a test harness for testing plugins:

```rust
#[test]
fn test_content_hooks() {
    let plugin = MyPlugin;
    let mut registry = TestPluginRegistry::new();

    plugin.register(&mut registry).unwrap();

    let test_content = TestContent::new("Test", "Test content");

    // Test the post_content_create hook
    registry.trigger_post_content_create(&test_content).unwrap();
}
```

## Security Considerations

### Plugin Verification

Plugins can be signed to verify their authenticity:

1. Plugin author generates a key pair
2. Plugin is signed with the private key
3. Plugin is distributed with its signature
4. Plugin loader verifies the signature before loading

### User Permissions

Users must explicitly enable plugins and grant them permissions:

```bash
write plugin enable my-plugin
write plugin grant my-plugin network
```

## Plugin Distribution

Plugins can be distributed in the following ways:

1. **Source Code**: Users build the plugin from source
2. **Binary Package**: Pre-built plugin binaries
3. **Plugin Repository**: Central repository of verified plugins

### Plugin Repository

The plugin repository is a centralized location for discovering and installing plugins:

```bash
write plugin search markdown
write plugin install markdown-extended
```

## Conclusion

The plugin system provides a powerful and flexible way to extend the writing tools without modifying the core system. By following a well-defined architecture with clear extension points and a stable API, plugins can enhance the system's functionality while maintaining security and stability.

For more detailed examples and API reference, see the [Extension Development Guide](./EXTENSION_DEVELOPMENT_GUIDE.md).
