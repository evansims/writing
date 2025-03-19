# Mocking Guide: Write CLI Testing Strategy

This guide outlines the recommended approach for using mocks in tests throughout the Write CLI ecosystem. It serves as a reference for implementing testable code and effectively using mocks to isolate test units.

## Core Principles

1. **Interface-Based Testing**: Test against trait interfaces, not concrete implementations.
2. **Dependency Injection**: Inject dependencies rather than creating them internally.
3. **Consistent Mocking Approach**: Use the same mocking patterns across all components.
4. **Isolation**: Test each component in isolation from its dependencies.
5. **Minimal Mock Scope**: Only mock what you need to isolate the system under test (SUT).
6. **Readable Test Setups**: Keep test setups clear and descriptive.

## Mocking Tools

The Write CLI uses the following tools for mocking:

1. **Mockall**: For generating mock implementations of traits. This is the primary mocking library.
2. **Common Test Utilities**: Found in `tools/common/test_utils`, this provides infrastructure for mocking common components.

## Mocking Patterns

### Trait-Based Mocking

Always define traits for components that need to be mocked:

```rust
// Define a trait for a component
pub trait ContentProcessor {
    fn process_content(&self, options: &ContentOptions) -> Result<PathBuf>;
}

// Implement the trait for the real implementation
impl ContentProcessor for DefaultContentProcessor {
    fn process_content(&self, options: &ContentOptions) -> Result<PathBuf> {
        // Real implementation
    }
}
```

Then create a mock implementation using Mockall:

```rust
use mockall::automock;

#[automock]
pub trait ContentProcessorMock: ContentProcessor {}
```

### Dependency Injection

Always inject dependencies rather than creating them internally:

```rust
// Bad: Hard to test
struct ContentManager {
    processor: DefaultContentProcessor,
}

// Good: Testable through dependency injection
struct ContentManager {
    processor: Box<dyn ContentProcessor>,
}

impl ContentManager {
    // Constructor that accepts dependencies
    fn new(processor: Box<dyn ContentProcessor>) -> Self {
        Self { processor }
    }
}
```

### Setting Up Test Expectations

When setting up mocks, be explicit about expectations:

```rust
#[test]
fn test_content_manager() {
    // Create a mock processor
    let mut mock_processor = MockContentProcessorMock::new();

    // Set up expectations
    mock_processor.expect_process_content()
        .with(predicate::function(|options| options.slug == Some("test".to_string())))
        .times(1)
        .returning(|_| Ok(PathBuf::from("content/test.md")));

    // Create the system under test with the mock
    let manager = ContentManager::new(Box::new(mock_processor));

    // Test the manager
    let result = manager.create_content("test", "blog");
    assert!(result.is_ok());
}
```

### Mocking External Services

For external services like the file system or network calls:

```rust
// Create a mock file system
let mut mock_fs = MockFileSystem::new();

// Set up expectations
mock_fs.expect_file_exists()
    .with(predicate::eq(PathBuf::from("content/test.md")))
    .returning(|_| Ok(true));

// Use the mock in your test
let system_under_test = SystemUnderTest::new(Box::new(mock_fs));
```

### Wrapper Traits for External Services

For external services, create wrapper traits and mock those:

```rust
// Define trait for the service
pub trait HttpClient {
    fn get(&self, url: &str) -> Result<String>;
    fn post(&self, url: &str, body: &str) -> Result<String>;
}

// Real implementation using reqwest
pub struct ReqwestHttpClient;

impl HttpClient for ReqwestHttpClient {
    fn get(&self, url: &str) -> Result<String> {
        // Real implementation
    }

    fn post(&self, url: &str, body: &str) -> Result<String> {
        // Real implementation
    }
}

// Mock implementation for testing
#[automock]
pub trait HttpClientMock {
    fn get(&self, url: &str) -> Result<String>;
    fn post(&self, url: &str, body: &str) -> Result<String>;
}

impl HttpClient for MockHttpClientMock {
    fn get(&self, url: &str) -> Result<String> {
        self.get(url)
    }

    fn post(&self, url: &str, body: &str) -> Result<String> {
        self.post(url, body)
    }
}
```

## Frequently Used Mocks

The Write CLI includes these commonly used mocks:

| Mock                       | Purpose                          | Location                                             |
| -------------------------- | -------------------------------- | ---------------------------------------------------- |
| `MockFileSystem`           | Mocking filesystem operations    | `common_test_utils::mocks::MockFileSystem`           |
| `MockConfigLoader`         | Mocking configuration operations | `common_test_utils::mocks::MockConfigLoader`         |
| `MockContentCreatorMock`   | Mocking content creation         | `common_test_utils::mocks::MockContentCreatorMock`   |
| `MockContentEditorMock`    | Mocking content editing          | `common_test_utils::mocks::MockContentEditorMock`    |
| `MockContentValidatorMock` | Mocking content validation       | `common_test_utils::mocks::MockContentValidatorMock` |
| `MockContentSearcherMock`  | Mocking content searching        | `common_test_utils::mocks::MockContentSearcherMock`  |
| `MockContentMoverMock`     | Mocking content moving           | `common_test_utils::mocks::MockContentMoverMock`     |
| `MockContentDeleterMock`   | Mocking content deletion         | `common_test_utils::mocks::MockContentDeleterMock`   |

## Example Test Cases

### Unit Test Example

```rust
#[test]
fn test_content_validator() {
    // Arrange - Set up mocks
    let mut mock_creator = MockContentCreatorMock::new();
    let mut mock_validator = MockContentValidatorMock::new();

    // Set expectations
    mock_creator.expect_create_content()
        .returning(|_| Ok(PathBuf::from("content/blog/test.md")));

    mock_validator.expect_validate_content()
        .returning(|_| Ok(vec![]));

    // Create the system under test
    let content_manager = ContentManager::new(
        Box::new(mock_creator),
        Box::new(mock_validator),
    );

    // Act - Call the method
    let result = content_manager.create_validated_content("Test", "blog");

    // Assert - Verify the result
    assert!(result.is_ok());
}
```

### Integration Test Example

```rust
#[test]
fn test_content_workflow_integration() {
    // Create test fixture with a real filesystem in a temp directory
    let fixture = TestFixture::new().unwrap();
    let fs = fixture.get_filesystem();

    // Create test implementations that use the real filesystem
    let creator = RealContentCreator::new(fs.clone());
    let validator = RealContentValidator::new(fs.clone());

    // Create the system under test
    let content_manager = ContentManager::new(
        Box::new(creator),
        Box::new(validator),
    );

    // Test the complete workflow
    let result = content_manager.create_validated_content("Test", "blog");
    assert!(result.is_ok());

    // Verify the file was created on the filesystem
    let path = result.unwrap();
    assert!(fixture.path().join(path).exists());
}
```

## Common Test Patterns

### Pattern 1: Basic Mocking with Expectations

Set up basic expectations for mocks to return predefined values.

```rust
// Arrange: Create mock instances
let mut mock_creator = MockContentCreatorMock::new();

// Set expectations for the mock
mock_creator.expect_create_content()
    .with(predicate::always())
    .times(1)
    .returning(|_| Ok(PathBuf::from("content/blog/test-article.md")));

// Create the system under test
let sut = SystemUnderTest::new(Box::new(mock_creator));

// Act & Assert
let result = sut.do_something();
assert!(result.is_ok());
```

### Pattern 2: Testing Error Conditions

Configure mocks to simulate error scenarios.

```rust
// Arrange: Create mock instances with error returns
let mut mock_fs = MockFileSystem::new();
mock_fs.expect_file_exists()
    .returning(|_| Err(common_errors::WritingError::not_found("File not found")));

// Create the system under test
let sut = SystemUnderTest::new(Box::new(mock_fs));

// Act & Assert
let result = sut.do_something();
assert!(result.is_err());
assert!(result.unwrap_err().to_string().contains("File not found"));
```

### Pattern 3: Sequence of Method Calls

Test operations that involve multiple dependent method calls.

```rust
// Arrange - Set up mocks with a sequence of calls
let mut mock_creator = MockContentCreatorMock::new();
let mut mock_validator = MockContentValidatorMock::new();

// First call - create content
mock_creator.expect_create_content()
    .times(1)
    .returning(|_| Ok(PathBuf::from("content/blog/test-article.md")));

// Second call - validate content (with issues)
mock_validator.expect_validate_content()
    .times(1)
    .returning(|_| Ok(vec!["Missing description".to_string()]));

// Third call - fix validation issues
mock_validator.expect_fix_validation_issues()
    .times(1)
    .returning(|_| Ok(vec!["Fixed: Added description".to_string()]));

// Create SUT, Act & Assert
let sut = SystemUnderTest::new(Box::new(mock_creator), Box::new(mock_validator));
let result = sut.create_and_validate();
assert!(result.is_ok());
```

### Pattern 4: Verifying Side Effects

Capture and verify arguments passed to mocked methods.

```rust
// Arrange
let captured_options = std::sync::Arc::new(std::sync::Mutex::new(None));
let captured_for_assert = captured_options.clone();

let mut mock_creator = MockContentCreatorMock::new();
mock_creator.expect_create_content()
    .times(1)
    .returning(move |options| {
        // Capture the options for later verification
        let mut options_storage = captured_options.lock().unwrap();
        *options_storage = Some(options.clone());
        Ok(PathBuf::from("content/blog/test-article.md"))
    });

// Act
let sut = SystemUnderTest::new(Box::new(mock_creator));
sut.create_content("Test Article", "blog");

// Assert - Verify captured options
let captured = captured_for_assert.lock().unwrap();
let options = captured.as_ref().expect("Options should have been captured");
assert_eq!(options.title.as_ref().unwrap(), "Test Article");
```

## Best Practices

1. **Mock at the Right Level**: Mock at the trait interface level, not at the method level.
2. **Verify Behavior, Not Implementation**: Test that components behave correctly, not how they're implemented.
3. **Keep Mocks Simple**: Don't add complex logic to mocks; they should return predefined values.
4. **Test Edge Cases**: Use mocks to test error paths and edge cases easily.
5. **Limit Mock Scope**: Only mock what's necessary for the test, use real implementations where possible.
6. **Use Test Helpers**: Create helper functions for common mock setups.
7. **Document Mock Usage**: Add comments explaining the purpose of each mock setup.
8. **Consistent Naming**: Use consistent naming conventions for mocks (e.g., `MockTraitName`).
9. **Format Test Steps**: Use Arrange, Act, Assert pattern for clear test structure.

## Common Pitfalls

1. **Over-Mocking**: Mocking too many components can lead to brittle tests.
2. **Tight Coupling to Implementation**: Mocks that depend on implementation details will break when implementation changes.
3. **Missing Expectations**: Not setting up all required expectations can lead to unexpected behavior.
4. **Incorrect Return Types**: Ensure mocks return the correct types with appropriate values.
5. **Not Handling Error Cases**: Make sure to test failure paths as well as success paths.
6. **Unrealistic Mocks**: Ensure mocks behave reasonably similarly to real implementations.
7. **Overspecified Tests**: Being too specific about how something is achieved rather than what is achieved.

## Reference Implementation

For a complete reference implementation of proper mocking patterns, see:

1. `tools/common/test_utils/examples/mock_examples.rs` - Examples of how to use mocks with dependency injection
2. `tools/common/test_utils/examples/mock_test_patterns.rs` - Common test patterns using mocks

## Additional Resources

- [Mockall Documentation](https://docs.rs/mockall/latest/mockall/)
- [TOOL_ISOLATION.md](TOOL_ISOLATION.md) - Explains the tool isolation strategy
- `common_test_utils::mocks` module - Contains all mock implementations

By following these guidelines, you'll ensure that tests throughout the Write CLI ecosystem are consistent, maintainable, and effective at catching bugs before they reach production.
