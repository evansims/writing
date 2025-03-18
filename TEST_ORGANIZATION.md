# Test Organization Guide

This document outlines the standardized approach for organizing tests throughout the Write CLI ecosystem, ensuring consistency and maintainability across all components.

## Core Principles

1. **Consistent Structure**: All tests follow the same organizational structure
2. **Clear Categories**: Tests are organized into distinct categories (unit, integration, property-based)
3. **Discoverable Tests**: Tests are easy to find and run selectively
4. **Self-Explanatory**: Test file names and module structures clearly indicate what is being tested
5. **Isolation Aligned**: Test organization supports the tool isolation strategy

## Test Categories

All tests in the Write CLI ecosystem are organized into three main categories:

### 1. Unit Tests

Unit tests focus on testing individual components in isolation:

- Located in a dedicated `tests/unit` directory
- Test a single function, method, or struct
- Use mocks extensively as outlined in the [Mocking Guide](MOCKING_GUIDE.md)
- Follow a clear Arrange-Act-Assert pattern
- Should run quickly (< 10ms per test)

### 2. Integration Tests

Integration tests focus on testing how components work together:

- Located in a dedicated `tests/integration` directory
- Test interactions between multiple components
- Minimize mocking where appropriate
- May use real file systems (in temporary directories)
- May have longer setup/teardown phases

### 3. Property-Based Tests

Property-based tests focus on validating invariants and properties:

- Located in a dedicated `tests/property` directory
- Test properties or invariants about functions/components
- Generate random inputs using proptest
- Use the common test utilities for property testing
- Focus on edge cases and broad input coverage

## File Structure and Naming

### Unit Tests Inside Source Files

For unit tests inside source files (`#[cfg(test)]` modules):

```rust
// src/some_module.rs

// Module code...

#[cfg(test)]
mod tests {
    use super::*;
    use common_test_utils::mocks::*;

    // For simple tests
    #[test]
    fn test_function_name_scenario_description() {
        // Test code
    }

    // For grouped tests
    mod function_name_tests {
        use super::*;

        #[test]
        fn scenario_1() {
            // Test code
        }

        #[test]
        fn scenario_2() {
            // Test code
        }
    }
}
```

### Dedicated Test Files

For tests in dedicated test directories:

```
tests/
  ├── unit/
  │    ├── module_name_tests.rs
  │    └── another_module_tests.rs
  ├── integration/
  │    ├── workflow_tests.rs
  │    └── cross_tool_tests.rs
  └── property/
       ├── module_name_properties.rs
       └── another_module_properties.rs
```

## Test Tagging

To facilitate selective test runs, we use test attributes for tagging:

```rust
// For tests that work with the file system
#[cfg(test)]
#[cfg_attr(test, test)]
#[cfg_attr(feature = "fs_tests", test)]
fn test_file_operations() {
    // Test code
}

// For tests that take longer to run
#[test]
#[cfg_attr(feature = "slow_tests", ignore)]
fn test_expensive_operation() {
    // Test code
}
```

Common tags include:

- `slow_tests`: For tests that take longer to run
- `fs_tests`: For tests that interact with the file system
- `network_tests`: For tests that require network access
- `integration_tests`: For broader integration tests

## Helper Macros

Common test helper macros are provided in the `common_test_utils` crate:

### Fixture Setup

```rust
use common_test_utils::with_test_fixture;

#[test]
fn test_with_fixture() {
    with_test_fixture!(fixture => {
        // Test code using fixture
        let content_path = fixture.create_content("blog", "test-post", "Test Post", false).unwrap();
        assert!(content_path.exists());
    });
}
```

### Mock Setup

```rust
use common_test_utils::with_mocks;

#[test]
fn test_with_mocks() {
    with_mocks!(MockFileSystem, mock_fs => {
        // Setup expectations
        mock_fs.expect_file_exists()
            .returning(|_| Ok(true));

        // Test code using mock_fs
    });
}
```

### Common Assertions

```rust
use common_test_utils::assert_contains;

#[test]
fn test_with_common_assertions() {
    let result = some_function();
    assert_contains!(result, "expected text");
}
```

## Test Naming Conventions

- Unit test methods: `test_<function_name>_<scenario_description>`
- Integration test methods: `test_<workflow/feature>_<scenario_description>`
- Property test methods: `prop_<function/feature>_<property_description>`
- Test files: `<module_name>_tests.rs` or `<feature_name>_tests.rs`

## Best Practices

1. **Arrange-Act-Assert**: Structure tests with a clear separation between setup, action, and verification
2. **Single Assertion Focus**: Each test should focus on verifying one specific behavior
3. **Descriptive Names**: Use descriptive test names that explain what is being tested
4. **Minimal Setup**: Keep test setup as minimal as possible while still being clear
5. **Independent Tests**: Tests should be independent and not rely on the state from other tests
6. **Test Data**: Use test fixtures and factories for consistent test data
7. **Test Coverage**: Aim for comprehensive coverage of code paths and edge cases
8. **Test Readability**: Prioritize test readability over minimal code duplication

## Integration with Cargo Nextest

For running specific categories of tests with Cargo Nextest:

```bash
# Run all unit tests
cargo nextest run --no-default-features

# Run integration tests
cargo nextest run --test '*' --workspace

# Run property tests
cargo nextest run --test '*_properties'

# Run tests with specific tags
cargo nextest run --features fs_tests
```

## Example Test Structure

For a typical component, the test structure might look like:

```
component-name/
  ├── src/
  │    ├── lib.rs
  │    ├── module1.rs
  │    └── module2.rs
  └── tests/
       ├── unit/
       │    ├── module1_tests.rs
       │    └── module2_tests.rs
       ├── integration/
       │    └── workflow_tests.rs
       └── property/
            └── module1_properties.rs
```

## References

- [TOOL_ISOLATION.md](TOOL_ISOLATION.md) - Guide for tool isolation strategy
- [MOCKING_GUIDE.md](MOCKING_GUIDE.md) - Guide for mocking approach
