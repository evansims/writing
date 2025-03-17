# Factory Pattern for Tool Creation

This document describes the factory pattern implementation for tool creation in the Writing project.

## Overview

The factory pattern is a creational design pattern that provides an interface for creating objects without specifying their concrete classes. In the Writing project, we use the factory pattern to standardize the creation of tool instances, making it easier to manage and extend the available tools.

## Benefits

- **Centralized Tool Creation**: All tools are created through a single factory, providing a consistent interface.
- **Simplified Configuration Management**: The factory handles loading and passing configuration to tools.
- **Reduced Coupling**: Client code doesn't need to know the concrete classes of tools, only their interfaces.
- **Easier Testing**: The factory pattern makes it easier to mock tools for testing.
- **Extensibility**: New tools can be added by extending the factory, without changing client code.

## Implementation

The factory pattern is implemented in the `common_cli::factory` module, which provides:

1. **ToolType Enum**: Defines all available tool types.
2. **Tool Trait**: Defines the interface that all tools must implement.
3. **ToolFactory Class**: Provides methods for creating tool instances.

### ToolType Enum

The `ToolType` enum defines all the available tools that can be created through the factory:

```rust
pub enum ToolType {
    ContentNew,
    ContentEdit,
    ContentDelete,
    // ... other tool types ...
}
```

It also provides methods for converting between string names and enum values:

```rust
impl ToolType {
    pub fn from_str(name: &str) -> Result<Self> {
        // ...
    }
    
    pub fn to_str(&self) -> &'static str {
        // ...
    }
}
```

### Tool Trait

The `Tool` trait defines the interface that all tools must implement:

```rust
pub trait Tool {
    fn execute(&self, args: Vec<&str>) -> Result<()>;
}
```

This provides a consistent interface for executing tools with command-line arguments.

### ToolFactory Class

The `ToolFactory` class provides methods for creating tool instances:

```rust
pub struct ToolFactory {
    config: Config,
    config_path: PathBuf,
}

impl ToolFactory {
    pub fn new() -> Result<Self> {
        // ...
    }
    
    pub fn with_config_path<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        // ...
    }
    
    pub fn create_tool(&self, tool_type: ToolType) -> Result<Box<dyn Tool>> {
        // ...
    }
    
    pub fn create_tool_by_name(&self, tool_name: &str) -> Result<Box<dyn Tool>> {
        // ...
    }
}
```

## Usage

### Creating a Tool Factory

```rust
use common_cli::factory::ToolFactory;
use common_errors::Result;

fn run_tool() -> Result<()> {
    // Create a factory with the default configuration
    let factory = ToolFactory::new()?;
    
    // Or create a factory with a specific configuration file
    let factory = ToolFactory::with_config_path("custom-config.yaml")?;
    
    // ...
}
```

### Creating a Tool by Type

```rust
use common_cli::factory::{ToolFactory, ToolType};
use common_errors::Result;

fn run_tool() -> Result<()> {
    let factory = ToolFactory::new()?;
    
    // Create a tool by type
    let tool = factory.create_tool(ToolType::ContentEdit)?;
    
    // Execute the tool with arguments
    tool.execute(vec!["--topic", "blog", "--slug", "my-post"])?;
    
    Ok(())
}
```

### Creating a Tool by Name

```rust
use common_cli::factory::ToolFactory;
use common_errors::Result;

fn run_tool() -> Result<()> {
    let factory = ToolFactory::new()?;
    
    // Create a tool by name
    let tool = factory.create_tool_by_name("content-edit")?;
    
    // Execute the tool with arguments
    tool.execute(vec!["--topic", "blog", "--slug", "my-post"])?;
    
    Ok(())
}
```

## Extending with New Tools

To add a new tool to the factory:

1. Add a new variant to the `ToolType` enum.
2. Add a new case to the `from_str` and `to_str` methods.
3. Add a new private method to create the tool in `ToolFactory`.
4. Add a new case to the `create_tool` method.
5. Implement the `Tool` trait for the new tool.

Example:

```rust
// 1. Add a new variant to the ToolType enum
pub enum ToolType {
    // ... existing types ...
    NewTool,
}

impl ToolType {
    // 2. Add a new case to the from_str method
    pub fn from_str(name: &str) -> Result<Self> {
        match name {
            // ... existing cases ...
            "new-tool" => Ok(ToolType::NewTool),
            _ => Err(WritingError::invalid_argument(format!("Unknown tool: {}", name))),
        }
    }
    
    // 2. Add a new case to the to_str method
    pub fn to_str(&self) -> &'static str {
        match self {
            // ... existing cases ...
            ToolType::NewTool => "new-tool",
        }
    }
}

impl ToolFactory {
    // 3. Add a new private method to create the tool
    fn create_new_tool(&self) -> Result<Box<dyn Tool>> {
        Ok(Box::new(NewTool {
            config: self.config.clone(),
            config_path: self.config_path.clone(),
        }))
    }
    
    // 4. Add a new case to the create_tool method
    pub fn create_tool(&self, tool_type: ToolType) -> Result<Box<dyn Tool>> {
        match tool_type {
            // ... existing cases ...
            ToolType::NewTool => self.create_new_tool(),
        }
    }
}

// 5. Implement the Tool trait for the new tool
struct NewTool {
    config: Config,
    config_path: PathBuf,
}

impl Tool for NewTool {
    fn execute(&self, args: Vec<&str>) -> Result<()> {
        // Implementation
        Ok(())
    }
}
```

## Best Practices

1. **Keep the Tool Interface Simple**: The `Tool` trait should have a minimal interface to make it easy to implement for new tools.
2. **Use Dependency Injection**: Pass dependencies like configuration to tools through the factory.
3. **Handle Errors Gracefully**: Use the `Result` type to handle errors and provide meaningful error messages.
4. **Document New Tools**: Update the documentation when adding new tools to the factory.
5. **Write Tests**: Write tests for new tools and factory methods to ensure they work correctly. 