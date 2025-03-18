# Write CLI Architecture

## Testing Architecture

### Test Organization

The Write CLI project follows a standardized approach to test organization to ensure maintainability and clarity. Tests are organized into three main categories:

1. **Unit Tests**: Test individual components in isolation, usually located in the same file as the code or in a `tests/unit` directory.
2. **Integration Tests**: Test interactions between multiple components, located in a `tests/integration` directory.
3. **Property-Based Tests**: Test invariants and properties, located in a `tests/property` directory.

#### File Structure

```
component-name/
  ├── src/
  │    ├── lib.rs
  │    └── module.rs
  └── tests/
       ├── unit/
       │    └── module_tests.rs
       ├── integration/
       │    └── workflow_tests.rs
       └── property/
            └── module_properties.rs
```

#### Test Tagging

Tests can be tagged for selective execution using Cargo features:

```rust
// For tests that work with the file system
#[cfg(test)]
#[cfg_attr(feature = "fs_tests", test)]
fn test_file_operations() {
    // Test code
}
```

#### Helper Macros

The common-test-utils crate provides helper macros for cleaner tests:

```rust
// Using fixtures
with_test_fixture!(fixture => {
    // Test code using fixture
});

// Using mocks
with_mock!(MockFileSystem, mock_fs => {
    // Test code using mock_fs
});

// Property testing
test_property!(
    inputs = inputs,
    property = |input| property(input),
    description = "Property description"
);
```

#### Naming Conventions

- Unit test methods: `test_<function_name>_<scenario_description>`
- Integration test methods: `test_<workflow/feature>_<scenario_description>`
- Property test methods: `prop_<function/feature>_<property_description>`

For more details, see [TEST_ORGANIZATION.md](TEST_ORGANIZATION.md).

### Tool Isolation

All tools in the Write CLI ecosystem are designed to be independently testable following the principles outlined in [TOOL_ISOLATION.md](TOOL_ISOLATION.md).

### Mocking Strategy

The Write CLI uses a consistent approach to mocking, using Mockall for trait-based mocking as described in [MOCKING_GUIDE.md](MOCKING_GUIDE.md).
