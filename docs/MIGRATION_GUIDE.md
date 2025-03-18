# Migration Guide for v2.0.0

This guide helps users migrate from previous versions of the writing tools to v2.0.0, which includes several important changes and improvements.

## Table of Contents

1. [Overview of Changes](#overview-of-changes)
2. [Breaking Changes](#breaking-changes)
3. [Configuration Changes](#configuration-changes)
4. [Command Line Interface Changes](#command-line-interface-changes)
5. [Content Format Changes](#content-format-changes)
6. [Plugin API Changes](#plugin-api-changes)
7. [Migration Steps](#migration-steps)
8. [Troubleshooting](#troubleshooting)

## Overview of Changes

Version 2.0.0 represents a major update to the writing tools, with significant improvements in:

- **Performance**: Faster content processing and incremental building
- **Extensibility**: New plugin system with a stable API
- **Multilingual Support**: Full support for multilingual content
- **Search Capabilities**: Advanced search features for content
- **Code Organization**: Improved code structure and error handling
- **Documentation**: Comprehensive documentation for all components

## Breaking Changes

### Command Naming

Command names have been standardized to follow a consistent `verb-noun` pattern:

| Old Command | New Command      | Notes                         |
| ----------- | ---------------- | ----------------------------- |
| `new`       | `content-new`    | Creates new content           |
| `edit`      | `content-edit`   | Edits existing content        |
| `delete`    | `content-delete` | Deletes content               |
| `add-topic` | `topic-add`      | Adds a new topic              |
| `build`     | `content-build`  | Builds content for publishing |

The main `write` command still functions as a wrapper for these specialized commands, so you can continue to use:

```bash
write new article my-article
```

But for scripting or automation, you should update to use the full command names:

```bash
content-new article my-article
```

### Configuration File Structure

The configuration file structure has been updated to use standardized naming conventions:

- YAML files use `snake_case` for keys
- JSON files use `camelCase` for keys

Example:

```diff
# config.yaml
- contentDir: ./content
- topicDir: ./topics
+ content_dir: ./content
+ topic_dir: ./topics
```

The automatic migration tool will update your configuration files, but custom scripts referencing configuration values will need to be updated.

### Removed Features

The following features have been removed:

- The `legacy-import` command (use the new plugin-based importers instead)
- The `stats-basic` command (replaced by the more comprehensive `content-stats` command)
- Direct WordPress XML export (now available as a plugin)

## Configuration Changes

### New Configuration Options

Many new configuration options are available in v2.0.0:

```yaml
# config.yaml
plugin_dir: ~/.config/writing-tools/plugins # Plugin directory
search:
  enabled: true # Enable search capabilities
  index_file: ./public/search-index.json # Search index output
multilingual:
  enabled: false # Enable multilingual support
  default_language: en # Default language
  languages: [en, es, fr] # Supported languages
```

### Automated Configuration Migration

A migration tool is provided to automatically update your configuration:

```bash
write config migrate
```

This tool:

1. Creates a backup of your existing configuration
2. Migrates settings to the new format
3. Adds new default settings
4. Reports changes made

## Command Line Interface Changes

### New Commands

Several new commands have been added:

- `write plugin install <plugin-name>`: Install a plugin
- `write plugin list`: List installed plugins
- `write plugin search <query>`: Search for plugins
- `write search index`: Build a search index
- `write search query <query>`: Search content
- `write lang add <language>`: Add a new language
- `write lang status`: Show translation status

### Modified Command Options

Some commands have new or modified options:

#### content-build (formerly build)

```diff
# Building content
- write build --output ./public
+ write build --output ./public --incremental --optimize
```

New options:

- `--incremental`: Only rebuild changed content
- `--optimize`: Apply additional optimizations
- `--languages`: Build specific languages only

#### content-new (formerly new)

```diff
# Creating new content
- write new article my-article "My Article"
+ write new article my-article "My Article" --template blog --language en
```

New options:

- `--template`: Specify a template (was previously positional)
- `--language`: Specify content language
- `--topics`: Specify multiple topics in one command

## Content Format Changes

### Multilingual Content

Multilingual content uses a new directory structure:

```
content/
├── en/
│   ├── articles/
│   │   └── my-article.md
│   └── pages/
│       └── about.md
├── es/
│   ├── articles/
│   │   └── my-article.md
│   └── pages/
│       └── about.md
```

A migration tool is provided to help organize existing content:

```bash
write lang init --default en
```

This will:

1. Move existing content to the default language directory
2. Update internal references
3. Generate language configuration

### Frontmatter Changes

New standardized frontmatter fields are available:

```yaml
---
title: My Article
date: 2023-01-15
topics: [rust, programming]
language: en
translations:
  es: /es/articles/my-article
  fr: /fr/articles/my-article
featured: false
---
```

## Plugin API Changes

If you've developed custom plugins for earlier versions, significant changes to the plugin API will require updates:

### Plugin Manifest

Plugins now require a `plugin.toml` manifest:

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
build_hooks = true
image_hooks = false
```

### Hook Registration

Hook registration now uses a registry pattern:

```diff
- export_hooks! {
-     pre_content_create: my_pre_content_create,
-     post_build: my_post_build,
- }
+ fn register(&self, registry: &mut PluginRegistry) -> Result<()> {
+     registry.register_content_hooks(MyContentHooks::new())?;
+     registry.register_build_hooks(MyBuildHooks::new())?;
+     Ok(())
+ }
```

See the [Extension Development Guide](./EXTENSION_DEVELOPMENT_GUIDE.md) for complete details on the new plugin API.

## Migration Steps

Follow these steps to migrate to v2.0.0:

### 1. Backup Your Data

Before migrating, create a backup of your content and configuration:

```bash
mkdir -p ~/writing-backup
cp -r ~/.config/writing-tools ~/writing-backup/config
cp -r ./content ~/writing-backup/content
cp -r ./topics ~/writing-backup/topics
```

### 2. Update the Writing Tools

Install the latest version:

```bash
# Using the install script
curl -sSL https://writing-tools.example.com/install.sh | sh

# Or using cargo
cargo install writing-tools --version 2.0.0
```

### 3. Migrate Configuration

Run the configuration migration tool:

```bash
write config migrate
```

### 4. Update Content (if using multilingual support)

If you plan to use multilingual support:

```bash
write lang init --default en
```

### 5. Rebuild Content

Rebuild your content with the new tools:

```bash
write build --incremental
```

### 6. Update Scripts and Automation

Update any scripts or automation to use the new command names and options.

### 7. Install Desired Plugins

Browse and install plugins for additional functionality:

```bash
write plugin search
write plugin install markdown-extended
```

## Troubleshooting

### Missing Commands

If you encounter "command not found" errors:

```bash
write path-update
```

This updates your PATH to include the new command locations.

### Configuration Errors

If you encounter configuration errors:

```bash
write config validate --verbose
```

This will check your configuration and suggest fixes.

### Content Processing Errors

If you encounter content processing errors:

```bash
write content-validate --fix
```

This will validate your content against the new requirements and fix common issues.

### Plugin Compatibility

If you encounter plugin compatibility issues:

```bash
write plugin compat-check
```

This will check your plugins for compatibility with v2.0.0 and suggest updates.

## Need Help?

If you encounter issues not covered in this guide:

- Check the [Frequently Asked Questions](./FAQ.md)
- Visit the [GitHub Issues](https://github.com/writing-tools/writing-tools/issues)
- Join the [Community Discord](https://discord.gg/writing-tools)

## Additional Resources

- [Extension Development Guide](./EXTENSION_DEVELOPMENT_GUIDE.md)
- [Plugin System Architecture](./PLUGIN_SYSTEM_ARCHITECTURE.md)
- [Incremental Build System](./INCREMENTAL-BUILD.md)
- [Configuration Key Migration](./CONFIG_KEY_MIGRATION.md)
