# Test Organization Standard

This document outlines the standard organization for tests in the writing tool codebase.

## Directory Structure

Each tool should have a consistent test directory structure as follows:

```
/tools/tool-name/
  /src/            # Source code
  /tests/          # Tests
    mod.rs         # Test module declaration
    /unit/         # Unit tests
    /integration/  # Integration tests
    /property/     # Property-based tests
```

## Test Categories

Tests are organized into three main categories:

1. **Unit Tests** - Tests for individual functions and components in isolation

   - Located in `tests/unit/`
   - Test files should be named `*_tests.rs` (e.g., `validation_tests.rs`)
   - Should not require external resources or dependencies

2. **Integration Tests** - Tests that verify multiple components working together

   - Located in `tests/integration/`
   - Test files should be named `*_integration_tests.rs` (e.g., `content_integration_tests.rs`)
   - May require test fixtures and mocked external dependencies

3. **Property Tests** - Tests that verify properties of functions via randomized inputs
   - Located in `tests/property/`
   - Test files should be named `*_property_tests.rs` (e.g., `validation_property_tests.rs`)
   - Should use the proptest crate for property-based testing

## Test Module Structure

Each test directory should be declared as a module in `tests/mod.rs`:

```rust
//! Tests for the tool-name module
//!
//! This module contains tests for the tool-name tool.

// Unit tests
pub mod unit;

// Integration tests
pub mod integration;

// Property-based tests
pub mod property;
```

## Standardization Script

A script is provided to help standardize the test structure across all tools:

```bash
./tools/standardize_tests.sh
```

This script will:

1. Create the standard test directory structure for each tool
2. Move tests from non-standard locations to the standard locations
3. Create a `mod.rs` file if it doesn't exist

## Best Practices

1. **Keep Tests Separate** - Tests should not be included in the source code files
2. **Test Naming** - Use descriptive names for test functions: `test_<function_name>_<scenario>`
3. **Test Organization** - Group related tests in the same file
4. **Fixture Usage** - Use the `common_test_utils` module for test fixtures and helpers
5. **Test Coverage** - Aim for high test coverage, especially for critical functionality
6. **Documentation** - Include doc comments explaining the purpose of test modules and complex tests

## Migration Notes

If you encounter tests in non-standard locations:

1. **In-Source Tests** - Move tests from `#[cfg(test)]` modules in source files to `tests/unit/`
2. **Src/Tests Directory** - Move tests from `src/tests/` to `tests/unit/`
3. **Root Test Files** - Move tests from `tests/*.rs` to the appropriate category directory

Run the standardization script to handle this automatically.
