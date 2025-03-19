# Testing Infrastructure Guide

This document provides an overview of the testing infrastructure for the Write CLI ecosystem, including available tools, commands, and best practices.

## Overview

The Write CLI testing approach is based on the following principles:

1. **Comprehensive Test Coverage**: Aim for 80%+ code coverage across all components
2. **Test Isolation**: Each tool can be tested independently without dependencies
3. **Multiple Test Types**: Unit, Integration, and Property-based tests
4. **Consistent Structure**: Standardized test organization across all components
5. **Test Quality Validation**: Mutation testing to verify test effectiveness
6. **Performance Optimization**: Efficient test execution with caching

## Test Structure

Tests are organized according to the standard defined in [TEST_ORGANIZATION.md](TEST_ORGANIZATION.md):

```
/tools/tool-name/
  /src/            # Source code
  /tests/          # Tests
    mod.rs         # Test module declaration
    /unit/         # Unit tests
    /integration/  # Integration tests
    /property/     # Property-based tests
```

## Available Tools

### Core Testing

- **cargo nextest**: Primary test runner

  ```bash
  # Run all tests
  cargo nextest run --workspace --all-features

  # Run with specific profile
  cargo nextest run --workspace --profile local
  ```

- **Coverage Measurement**

  ```bash
  # Generate coverage summary
  ./coverage.sh

  # Generate HTML report
  ./coverage.sh html

  # Generate and open HTML report
  ./coverage.sh open
  ```

### Advanced Testing

- **Mutation Testing**

  ```bash
  # Run mutation tests on all crates
  ./mutation.sh

  # Run mutation tests on a specific crate
  ./mutation.sh content-new
  ```

- **Test Optimization**

  ```bash
  # Analyze test execution time
  ./optimize-tests.sh analyze

  # Run fast tests only
  ./optimize-tests.sh fast

  # Setup test caching
  ./optimize-tests.sh cache
  ```

## CI/CD Integration

The testing infrastructure is integrated with CI/CD through several GitHub Actions workflows:

1. **[ci.yml](.github/workflows/ci.yml)**: Main CI workflow for testing all changes
2. **[test-coverage.yml](.github/workflows/test-coverage.yml)**: Coverage measurement and monitoring
3. **[mutation-testing.yml](.github/workflows/mutation-testing.yml)**: Quality assessment of tests
4. **[test-optimization.yml](.github/workflows/test-optimization.yml)**: Performance optimization

## Test Configuration

Test profiles are defined in `Cargo.toml` and in the `.config/nextest/` directory:

- **ci**: For continuous integration (fails fast, minimal output)
- **coverage**: For coverage measurement (runs all tests, no fail-fast)
- **local**: For local development (immediate output for debugging)
- **ci-optimized**: Performance-optimized profile for CI

## Testing Standards

### Mocking

Refer to [MOCKING_GUIDE.md](MOCKING_GUIDE.md) for detailed guidelines on:

- Using mockall for trait-based mocking
- Setting up test expectations
- Injecting dependencies for testability

### Tool Isolation

Refer to [TOOL_ISOLATION.md](TOOL_ISOLATION.md) for information on:

- Testing tools independently
- Mocking dependencies between tools
- Dependency injection patterns

## Performance Standards

The test suite is expected to meet these performance targets:

- Unit test suite: < 10s execution time
- Integration test suite: < 60s execution time
- Full test suite with coverage: < 2m execution time

## Coverage Goals

The codebase aims to maintain these coverage levels:

- Common libraries: 90%+ coverage
- Individual tools: 80%+ coverage for each tool in isolation
- Command line interfaces: 85%+ coverage
- Business logic: 85%+ coverage
- Integration points: 80%+ coverage
- Overall project: 80%+ coverage

## Getting Started

For new developers, we recommend:

1. Start by reading the [TEST_ORGANIZATION.md](TEST_ORGANIZATION.md) and [MOCKING_GUIDE.md](MOCKING_GUIDE.md)
2. Run `cargo nextest run --workspace --profile local` to check existing tests
3. Run `./coverage.sh` to see current coverage levels
4. When adding new code, follow TDD principles by writing tests first

## Troubleshooting

If you encounter issues with the testing infrastructure:

1. **Tests are failing in CI but pass locally**

   - Run with `cargo nextest run --workspace --profile ci` to simulate CI environment

2. **Coverage is lower than expected**

   - Check for excluded files in `.coveragerc`
   - Ensure all code paths are being tested

3. **Slow test execution**

   - Run `./optimize-tests.sh analyze` to identify slow tests
   - Consider parallelizing or optimizing slow tests

4. **Mutation tests failing**
   - Review the test assertions to ensure they verify behavior correctly
   - Add test cases for edge conditions flagged by mutation testing
