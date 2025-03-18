# Tool Isolation Strategy

This document outlines the approach for ensuring all tools can be tested and run independently, while still integrating seamlessly within the Write CLI ecosystem.

## Core Principles

1. **Clear Boundaries**: Each tool should have well-defined boundaries with explicit interfaces.
2. **Dependency Inversion**: Tools should depend on abstractions (traits), not on concrete implementations.
3. **No Circular Dependencies**: Ensure no circular dependencies between tools.
4. **Shared Code in Common Libraries**: Extract shared functionality into common libraries.
5. **Independent Testing**: Each tool must be testable in isolation from other tools.

## Current Architecture Analysis

The current architecture has several challenges:

1. The Write CLI directly imports and uses other tools, creating tight coupling
2. Tools have implicit dependencies on each other
3. Some functionality is duplicated across tools
4. Testing is difficult because tools can't be easily mocked

## Refactoring Strategy

### 1. Extract Common Interfaces

Define common traits in `common/traits` that represent the capabilities of each tool:

```rust
// Example trait for content-new
pub trait ContentCreator {
    fn create_content(&self, options: &ContentOptions) -> Result<PathBuf>;
    fn list_templates(&self) -> Result<Vec<Template>>;
    fn get_available_topics(&self) -> Result<Vec<(String, TopicConfig)>>;
}

// Example trait for content-edit
pub trait ContentEditor {
    fn edit_content(&self, options: &EditOptions) -> Result<PathBuf>;
    fn update_frontmatter(&self, slug: &str, updates: HashMap<String, String>) -> Result<()>;
}
```

### 2. Implement Traits for Each Tool

Each tool should implement the appropriate trait:

```rust
// In content-new/src/lib.rs
pub struct ContentNew {
    config: Config,
    fs: Box<dyn FileSystem>,
}

impl ContentCreator for ContentNew {
    fn create_content(&self, options: &ContentOptions) -> Result<PathBuf> {
        // Implementation here
    }

    // Other methods
}
```

### 3. Dependency Injection in the Write CLI

The Write CLI should use dependency injection to access tools:

```rust
// In write/src/tools/content.rs
pub struct ContentTools {
    creator: Box<dyn ContentCreator>,
    editor: Box<dyn ContentEditor>,
    // Other tools
}

impl ContentTools {
    pub fn new(
        creator: Box<dyn ContentCreator>,
        editor: Box<dyn ContentEditor>,
        // Other tools
    ) -> Self {
        Self { creator, editor, /* ... */ }
    }

    pub fn create_content(&self, options: ContentOptions) -> Result<()> {
        self.creator.create_content(&options)?;
        // Additional Write CLI logic
        Ok(())
    }
}
```

### 4. Factory Pattern for Tool Creation

Use a factory pattern to create the appropriate tool implementations:

```rust
// In write/src/tools/factory.rs
pub struct ToolFactory;

impl ToolFactory {
    pub fn create_content_creator() -> Box<dyn ContentCreator> {
        Box::new(ContentNew::new())
    }

    pub fn create_content_editor() -> Box<dyn ContentEditor> {
        Box::new(ContentEdit::new())
    }

    // Other tools
}
```

### 5. Mock Implementations for Testing

Create mock implementations for testing:

```rust
// In common/test_utils/src/mocks/content.rs
pub struct MockContentCreator {
    // Mock implementation using mockall
}

impl ContentCreator for MockContentCreator {
    // Mock implementations
}
```

## Implementation Plan

1. Define traits for each tool category in `common/traits`
2. Refactor each tool to implement these traits
3. Update the Write CLI to use dependency injection
4. Create mock implementations for testing
5. Update tests to use mock implementations

## Benefits

1. **Improved Testability**: Each tool can be tested in isolation
2. **Clear Interfaces**: Well-defined interfaces between components
3. **Flexibility**: Easily swap implementations
4. **Reduced Duplication**: Common code extracted to shared libraries
5. **Enhanced Maintainability**: Better separation of concerns
