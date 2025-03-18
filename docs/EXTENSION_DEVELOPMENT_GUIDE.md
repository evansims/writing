# Extension Development Guide

This guide provides detailed information for developers who want to extend the writing tools system. It covers creating plugins, using extension points, and best practices for extending the system.

## Table of Contents

1. [Introduction](#introduction)
2. [Plugin Development](#plugin-development)
   - [Plugin Structure](#plugin-structure)
   - [Plugin Manifest](#plugin-manifest)
   - [Plugin API](#plugin-api)
   - [Hook Implementation](#hook-implementation)
3. [Extension Points](#extension-points)
   - [Content Extension Points](#content-extension-points)
   - [Build Extension Points](#build-extension-points)
   - [Image Extension Points](#image-extension-points)
   - [UI Extension Points](#ui-extension-points)
4. [Configuration Extension Points](#configuration-extension-points)
5. [Testing Extensions](#testing-extensions)
6. [Debugging Extensions](#debugging-extensions)
7. [Distribution](#distribution)
8. [Best Practices](#best-practices)
9. [Examples](#examples)

## Introduction

The writing tools system is designed to be extensible through a plugin architecture. Plugins can hook into various extension points to modify behavior, add new commands, process content, and customize the system's functionality.

This guide will help you understand how to develop effective extensions for the writing tools system.

## Plugin Development

### Plugin Structure

A plugin is a Rust crate that implements the `Plugin` trait and is compiled as a dynamic library. The basic structure of a plugin project is:

```
my-plugin/
├── Cargo.toml           # Plugin crate configuration
├── plugin.toml          # Plugin manifest
└── src/
    ├── lib.rs           # Plugin entry point
    ├── content_hooks.rs # Content hook implementations
    ├── build_hooks.rs   # Build hook implementations
    └── commands.rs      # Custom commands
```

### Plugin Manifest

Every plugin requires a `plugin.toml` file that defines its metadata and capabilities:

```toml
[plugin]
name = "my-plugin"
version = "1.0.0"
description = "A plugin that enhances markdown processing"
author = "Plugin Author <author@example.com>"
repository = "https://github.com/author/my-plugin"
api_version = "1.0"

[dependencies]
other-plugin = "^1.0.0"

[capabilities]
content_hooks = true
build_hooks = true
image_hooks = false
ui_hooks = true
custom_commands = true
```

The manifest fields are:

- **name**: Unique identifier for the plugin
- **version**: Plugin version following semantic versioning
- **description**: Brief description of the plugin's functionality
- **author**: Plugin author information
- **repository**: URL to the plugin's source repository
- **api_version**: Version of the plugin API this plugin targets
- **dependencies**: Other plugins this plugin depends on
- **capabilities**: The types of hooks and features this plugin provides

### Plugin API

The plugin API is provided by the `writing-tools-plugin-api` crate, which you should include as a dependency in your plugin's `Cargo.toml`:

```toml
[package]
name = "my-plugin"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
writing-tools-plugin-api = "1.0"
```

The API provides traits and types needed to implement plugin functionality:

```rust
use writing_tools_plugin_api::{
    Plugin, PluginRegistry, Content, ContentHooks, BuildHooks,
    Result, Error, ErrorKind, ContentType
};
```

### Hook Implementation

Plugins implement one or more hook traits to extend functionality:

```rust
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn register(&self, registry: &mut PluginRegistry) -> Result<()> {
        // Register content hooks
        registry.register_content_hooks(MyContentHooks::new())?;

        // Register build hooks
        registry.register_build_hooks(MyBuildHooks::new())?;

        // Register custom commands
        registry.register_command("extend-markdown", ExtendMarkdownCommand::new())?;

        Ok(())
    }
}

// Content hooks implementation
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

    // Other content hook implementations...
}

// Build hooks implementation
pub struct MyBuildHooks;

impl MyBuildHooks {
    pub fn new() -> Self {
        Self
    }
}

impl BuildHooks for MyBuildHooks {
    fn post_content_process(&self, content: &dyn Content, output: &mut String) -> Result<()> {
        // Add custom processing for markdown content
        if content.content_type() == ContentType::Article {
            *output = output.replace("[[TOC]]", &generate_toc(content)?);
        }
        Ok(())
    }

    // Other build hook implementations...
}

// Plugin entry point
#[no_mangle]
pub fn init_plugin() -> Box<dyn Plugin> {
    Box::new(MyPlugin)
}
```

## Extension Points

The system provides multiple extension points categorized by functionality.

### Content Extension Points

Content hooks allow plugins to interact with content creation, modification, and deletion:

```rust
pub trait ContentHooks {
    fn pre_content_create(&self, content_type: &ContentType, slug: &str, options: &ContentCreationOptions) -> Result<()>;
    fn post_content_create(&self, content: &dyn Content) -> Result<()>;
    fn pre_content_update(&self, content: &dyn Content, updates: &ContentUpdateOptions) -> Result<()>;
    fn post_content_update(&self, content: &dyn Content) -> Result<()>;
    fn pre_content_delete(&self, content: &dyn Content) -> Result<()>;
    fn post_content_delete(&self, path: &Path) -> Result<()>;
    fn validate_content(&self, content: &dyn Content) -> Result<ValidationReport>;
}
```

#### Use Cases

- Add default frontmatter to new content
- Validate content against custom rules
- Process content after creation (e.g., add related links)
- Perform cleanup actions after content deletion

#### Example: Adding Default Frontmatter

```rust
fn pre_content_create(&self, content_type: &ContentType, slug: &str, options: &mut ContentCreationOptions) -> Result<()> {
    if *content_type == ContentType::Article {
        // Add default reading time estimate field
        options.frontmatter.insert("reading_time_minutes".to_string(), 5.into());

        // Add default featured flag
        options.frontmatter.insert("featured".to_string(), false.into());
    }
    Ok(())
}
```

### Build Extension Points

Build hooks allow plugins to influence the build process:

```rust
pub trait BuildHooks {
    fn pre_build(&self, options: &BuildOptions) -> Result<()>;
    fn post_content_process(&self, content: &dyn Content, output: &mut String) -> Result<()>;
    fn post_build(&self, output_dir: &Path) -> Result<()>;
    fn contribute_build_artifacts(&self, output_dir: &Path) -> Result<Vec<PathBuf>>;
    fn modify_sitemap(&self, sitemap: &mut Sitemap) -> Result<()>;
    fn modify_rss(&self, rss: &mut RssFeed) -> Result<()>;
}
```

#### Use Cases

- Enhance content during build (e.g., syntax highlighting)
- Generate additional files (e.g., search index)
- Modify generated artifacts (e.g., add items to sitemap)
- Perform post-build operations (e.g., deploy to CDN)

#### Example: Syntax Highlighting

```rust
fn post_content_process(&self, content: &dyn Content, output: &mut String) -> Result<()> {
    // Apply syntax highlighting to code blocks
    *output = replace_code_blocks(output, |language, code| {
        syntax_highlight(language, code)
    })?;

    Ok(())
}
```

### Image Extension Points

Image hooks allow plugins to interact with image processing:

```rust
pub trait ImageHooks {
    fn pre_image_optimize(&self, source_path: &Path, options: &mut ImageOptimizationOptions) -> Result<()>;
    fn post_image_optimize(&self, optimized_image: &OptimizedImage) -> Result<()>;
    fn contribute_image_formats(&self) -> Result<Vec<ImageFormat>>;
    fn process_image(&self, image: &Image, format: &ImageFormat) -> Result<Option<ProcessedImage>>;
}
```

#### Use Cases

- Add custom image formats
- Apply custom image processing filters
- Adjust optimization parameters based on image content
- Generate additional metadata for images

#### Example: Custom Watermarking

```rust
fn process_image(&self, image: &Image, format: &ImageFormat) -> Result<Option<ProcessedImage>> {
    if *format == ImageFormat::Jpeg && image.width() > 1000 {
        // Apply watermark to large JPEG images
        let processed = image.clone();
        processed.watermark("© My Blog", Position::BottomRight, 0.5)?;
        return Ok(Some(processed));
    }

    // Return None to let the default processing handle other cases
    Ok(None)
}
```

### UI Extension Points

UI hooks allow plugins to enhance the user interface:

```rust
pub trait UiHooks {
    fn register_commands(&self, registry: &mut CommandRegistry) -> Result<()>;
    fn customize_progress(&self, progress: &mut ProgressBar) -> Result<()>;
    fn contribute_templates(&self) -> Result<HashMap<String, Template>>;
    fn modify_help_text(&self, command: &str, help_text: &mut String) -> Result<()>;
}
```

#### Use Cases

- Add custom commands to the CLI
- Customize progress indicators
- Provide custom templates for content creation
- Enhance help documentation

#### Example: Custom Command

```rust
fn register_commands(&self, registry: &mut CommandRegistry) -> Result<()> {
    registry.register_command("wordcount", WordCountCommand::new())?;
    Ok(())
}

struct WordCountCommand;

impl WordCountCommand {
    fn new() -> Self {
        Self
    }
}

impl Command for WordCountCommand {
    fn execute(&self, args: &[String]) -> Result<()> {
        if args.is_empty() {
            return Err(Error::new(ErrorKind::InvalidArgument, "No content specified"));
        }

        let path = Path::new(&args[0]);
        let content = read_content(path)?;
        let word_count = count_words(&content.body());

        println!("{}: {} words", content.title(), word_count);
        Ok(())
    }

    fn name(&self) -> &'static str {
        "wordcount"
    }

    fn description(&self) -> &'static str {
        "Count words in content"
    }

    fn usage(&self) -> &'static str {
        "wordcount <content-path>"
    }
}
```

## Configuration Extension Points

Plugins can define their own configuration options and access the system configuration:

```rust
// Plugin configuration
let config = registry.get_plugin_config()?;
let highlight_style = config.get::<String>("highlight_style").unwrap_or_else(|_| "monokai".to_string());

// System configuration
let system_config = registry.get_system_config()?;
let content_dir = system_config.get::<PathBuf>("content_dir")?;
```

### Custom Configuration Schema

Plugins can define a schema for their configuration:

```rust
fn register(&self, registry: &mut PluginRegistry) -> Result<()> {
    // Register configuration schema
    let schema = ConfigSchema::new()
        .add_field("highlight_style", ConfigFieldType::String, "Syntax highlighting style")
        .add_field("line_numbers", ConfigFieldType::Boolean, "Show line numbers in code blocks")
        .set_defaults([
            ("highlight_style", "monokai".into()),
            ("line_numbers", true.into()),
        ]);

    registry.register_config_schema(schema)?;

    // Register hooks
    // ...

    Ok(())
}
```

## Testing Extensions

The plugin API provides testing utilities for plugins:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use writing_tools_plugin_api::testing::{TestPluginRegistry, TestContent, TestBuildOptions};

    #[test]
    fn test_content_hooks() {
        let plugin = MyPlugin;
        let mut registry = TestPluginRegistry::new();

        plugin.register(&mut registry).unwrap();

        let test_content = TestContent::new("Test", "Test content");

        // Test the post_content_create hook
        registry.trigger_post_content_create(&test_content).unwrap();
    }

    #[test]
    fn test_build_hooks() {
        let plugin = MyPlugin;
        let mut registry = TestPluginRegistry::new();

        plugin.register(&mut registry).unwrap();

        let test_content = TestContent::new("Test", "# Heading\n\n[[TOC]]\n\nContent");
        let mut output = test_content.body().to_string();

        // Test the post_content_process hook
        registry.trigger_post_content_process(&test_content, &mut output).unwrap();

        assert!(output.contains("<ul>"));
        assert!(output.contains("<li>Heading</li>"));
    }
}
```

## Debugging Extensions

### Plugin Logging

Plugins can use the provided logging utilities:

```rust
use writing_tools_plugin_api::log;

fn post_content_process(&self, content: &dyn Content, output: &mut String) -> Result<()> {
    log::info!("Processing content: {}", content.title());

    // Processing logic...

    log::debug!("Applied transformations: {}", transformations_applied);
    Ok(())
}
```

### Development Mode

You can enable development mode for easier plugin debugging:

```bash
write --dev-mode plugin-test my-plugin
```

In development mode:

- Plugin errors include more detail
- Plugins can be reloaded without restarting the application
- Logging is more verbose

## Distribution

### Packaging Plugins

To package a plugin for distribution:

```bash
write plugin package my-plugin
```

This creates a `.wtplugin` file containing:

- Compiled plugin binary for all supported platforms
- Plugin manifest
- Documentation
- Example configurations

### Publishing Plugins

Plugins can be published to the plugin repository:

```bash
write plugin publish my-plugin
```

This requires:

- A valid developer account
- Signed plugin package
- Successful validation checks

### Installation

Users install plugins using:

```bash
write plugin install my-plugin
```

## Best Practices

### Performance Considerations

- Cache expensive operations
- Use incremental processing where possible
- Avoid blocking operations
- Use concurrency for CPU-intensive tasks

### Security Considerations

- Validate all user input
- Use safe APIs for file system operations
- Request only the permissions your plugin needs
- Document security considerations

### Compatibility

- Target a specific API version
- Test with multiple versions of the writing tools
- Document minimum required version
- Follow semantic versioning for your plugin

### Documentation

- Document all configuration options
- Provide usage examples
- Include a clear description of your plugin's functionality
- Document any changes to default behavior

## Examples

### Markdown Extension Plugin

This example plugin enhances markdown processing:

```rust
use writing_tools_plugin_api::{Plugin, PluginRegistry, Content, BuildHooks, Result, Error};

pub struct MarkdownExtensionPlugin;

impl Plugin for MarkdownExtensionPlugin {
    fn register(&self, registry: &mut PluginRegistry) -> Result<()> {
        registry.register_build_hooks(MarkdownBuildHooks::new())?;
        Ok(())
    }
}

struct MarkdownBuildHooks;

impl MarkdownBuildHooks {
    fn new() -> Self {
        Self
    }
}

impl BuildHooks for MarkdownBuildHooks {
    fn post_content_process(&self, content: &dyn Content, output: &mut String) -> Result<()> {
        // Process custom syntax
        *output = output
            .replace("++", "<ins>")
            .replace("++", "</ins>")
            .replace("==", "<mark>")
            .replace("==", "</mark>");

        // Add footnote handling
        *output = process_footnotes(output)?;

        Ok(())
    }
}

fn process_footnotes(content: &str) -> Result<String> {
    // Implementation details...
    Ok(content.to_string())
}

#[no_mangle]
pub fn init_plugin() -> Box<dyn Plugin> {
    Box::new(MarkdownExtensionPlugin)
}
```

### Image Gallery Plugin

This example plugin adds support for image galleries:

```rust
use writing_tools_plugin_api::{Plugin, PluginRegistry, Content, ContentHooks, BuildHooks, Result, Error};

pub struct GalleryPlugin;

impl Plugin for GalleryPlugin {
    fn register(&self, registry: &mut PluginRegistry) -> Result<()> {
        registry.register_content_hooks(GalleryContentHooks::new())?;
        registry.register_build_hooks(GalleryBuildHooks::new())?;
        Ok(())
    }
}

struct GalleryContentHooks;

impl GalleryContentHooks {
    fn new() -> Self {
        Self
    }
}

impl ContentHooks for GalleryContentHooks {
    fn validate_content(&self, content: &dyn Content) -> Result<ValidationReport> {
        let mut report = ValidationReport::new();

        // Validate gallery directives
        if content.body().contains("{{gallery:") {
            for gallery in extract_galleries(content.body()) {
                if !gallery_exists(&gallery) {
                    report.add_warning(format!("Gallery '{}' not found", gallery));
                }
            }
        }

        Ok(report)
    }
}

struct GalleryBuildHooks;

impl GalleryBuildHooks {
    fn new() -> Self {
        Self
    }
}

impl BuildHooks for GalleryBuildHooks {
    fn post_content_process(&self, content: &dyn Content, output: &mut String) -> Result<()> {
        // Replace gallery directives with HTML galleries
        *output = process_gallery_directives(output)?;
        Ok(())
    }

    fn contribute_build_artifacts(&self, output_dir: &Path) -> Result<Vec<PathBuf>> {
        // Generate gallery.js and gallery.css
        let js_path = output_dir.join("assets/js/gallery.js");
        let css_path = output_dir.join("assets/css/gallery.css");

        generate_gallery_js(&js_path)?;
        generate_gallery_css(&css_path)?;

        Ok(vec![js_path, css_path])
    }
}

#[no_mangle]
pub fn init_plugin() -> Box<dyn Plugin> {
    Box::new(GalleryPlugin)
}
```

## Conclusion

This guide covers the essentials of extending the writing tools system through plugins. By following these patterns and best practices, you can create powerful extensions that enhance the functionality of the system while maintaining compatibility and performance.

For more information, refer to the [Plugin System Architecture](./PLUGIN_SYSTEM_ARCHITECTURE.md) document and explore the examples in the `examples/plugins` directory.
