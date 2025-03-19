# Test Organization

Standardized approach for organizing tests throughout the project.

## Core Principles

1. **Consistent Structure**: Uniform organizational structure
2. **Clear Categories**: Distinct test categories
3. **Discoverable Tests**: Easy to find and run selectively
4. **Self-Explanatory**: Clear file names and module structures
5. **Isolation Aligned**: Supports tool isolation strategy

## Test Categories

### Unit Tests

- Located in `tests/unit` directory
- Test a single function, method, or struct
- Use mocks extensively
- Follow Arrange-Act-Assert pattern
- Run quickly (< 10ms per test)

### Integration Tests

- Located in `tests/integration` directory
- Test interactions between multiple components
- Minimize mocking where appropriate
- May use real file systems (in temporary directories)
- May have longer setup/teardown phases

### Property-Based Tests

- Located in `tests/property` directory
- Test properties or invariants about functions/components
- Generate random inputs using proptest
- Use common test utilities for property testing
- Focus on edge cases and broad input coverage
