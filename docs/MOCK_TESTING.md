# Mock Testing

This document outlines the standardized mock objects for unit testing in the Writing project.

## Overview

Mock objects are used in unit testing to isolate the code being tested from its dependencies. In the Writing project, mock objects are provided for filesystem operations, config loading, content operations, and command execution.

## Mock Implementations

The `common-test-utils` crate provides the following mock implementations:

- `MockFileSystem`: Mock implementation of filesystem operations
- `MockConfigLoader`: Mock implementation of config loading
- `MockContentOperations`: Mock implementation of content operations
- `MockCommandExecutor`: Mock implementation of command execution

## Traits

The mock implementations implement the following traits:

- `FileSystem`: Trait for filesystem operations
- `ConfigLoader`: Trait for config loading
- `ContentOperations`: Trait for content operations
- `CommandExecutor`: Trait for command execution

These traits can be used to create dependency injection in your code, making it easier to test.

## Usage Examples

### Mock Filesystem

```rust
use common_test_utils::mocks::MockFileSystem;

#[test]
fn test_with_mock_fs() {
    let mut mock_fs = MockFileSystem::new();
    
    // Set up mock files
    mock_fs.add_file("/content/blog/post.md", "# Test Post\n\nContent");
    
    // Test something that uses the filesystem
    assert!(mock_fs.file_exists("/content/blog/post.md"));
    assert_eq!(mock_fs.read_file("/content/blog/post.md").unwrap(), "# Test Post\n\nContent");
}
```

### Mock Config Loader

```rust
use common_test_utils::mocks::MockConfigLoader;
use common_models::{Config, ContentConfig, PublicationConfig};
use std::collections::HashMap;

#[test]
fn test_with_mock_config() {
    // Create a simple config
    let config = Config {
        content: ContentConfig {
            base_dir: "/content".to_string(),
            topics: HashMap::new(),
            tags: None,
        },
        images: common_models::ImageConfig {
            formats: vec!["jpg".to_string()],
            format_descriptions: None,
            sizes: HashMap::new(),
            naming: None,
            quality: None,
        },
        publication: PublicationConfig {
            author: "Test Author".to_string(),
            copyright: "Test Copyright".to_string(),
            site: None,
        },
    };
    
    let mock_config = MockConfigLoader::new(config);
    
    // Test something that uses the config
    let loaded_config = mock_config.load_config("/config.yaml").unwrap();
    assert_eq!(loaded_config.publication.author, "Test Author");
}
```

### Mock Content Operations

```rust
use common_test_utils::mocks::MockContentOperations;
use common_models::{Article, Frontmatter};

#[test]
fn test_with_mock_content() {
    let mut mock_content = MockContentOperations::new();
    
    // Create a test article
    let article = Article {
        frontmatter: Frontmatter {
            title: "Test Article".to_string(),
            published: Some("2023-01-01".to_string()),
            updated: None,
            slug: Some("test-article".to_string()),
            tagline: None,
            tags: Some(vec!["test".to_string()]),
            topics: Some(vec!["blog".to_string()]),
            draft: Some(false),
            featured_image: None,
        },
        content: "# Test Article\n\nThis is a test article.".to_string(),
        slug: "test-article".to_string(),
        topic: "blog".to_string(),
        path: "/content/blog/test-article".to_string(),
        word_count: Some(7),
        reading_time: Some(1),
    };
    
    // Add the article
    mock_content.add_article(article);
    
    // Test something that uses content operations
    let retrieved = mock_content.get_article("blog", "test-article").unwrap();
    assert_eq!(retrieved.frontmatter.title, "Test Article");
}
```

### Mock Command Executor

```rust
use common_test_utils::mocks::MockCommandExecutor;

#[test]
fn test_with_mock_command() {
    let mut mock_cmd = MockCommandExecutor::new();
    
    // Set up command responses
    mock_cmd.set_response("ls -la", "file1.txt\nfile2.txt", 0);
    
    // Test something that executes commands
    let (output, exit_code) = mock_cmd.execute("ls -la").unwrap();
    assert_eq!(output, "file1.txt\nfile2.txt");
    assert_eq!(exit_code, 0);
}
```

### Using Traits for Dependency Injection

```rust
use common_test_utils::mocks::traits::{FileSystem, ConfigLoader};
use common_test_utils::mocks::{MockFileSystem, MockConfigLoader};

// Function that uses the FileSystem trait
fn read_and_process<F: FileSystem>(fs: &F, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs.read_file(path)?;
    Ok(content.to_uppercase())
}

#[test]
fn test_read_and_process() {
    let mut mock_fs = MockFileSystem::new();
    mock_fs.add_file("/test.txt", "test content");
    
    let result = read_and_process(&mock_fs, "/test.txt").unwrap();
    assert_eq!(result, "TEST CONTENT");
}
```

## Best Practices

1. **Dependency Injection**: Use traits for dependency injection to make your code testable.
2. **Minimal Mocking**: Only mock what you need to test your code.
3. **Realistic Behavior**: Make your mocks behave realistically to catch bugs early.
4. **Clear Setup**: Set up your mocks clearly at the beginning of your tests.
5. **Verify Interactions**: Verify that your code interacts with the mocks as expected.

## Example

See the tests in the `common-test-utils` crate for examples of using the mock implementations. 