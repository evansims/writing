# Mocking Strategy

Recommended approach for using mocks in tests throughout the project.

## Core Principles

1. **Interface-Based Testing**: Test against trait interfaces
2. **Dependency Injection**: Inject dependencies instead of creating internally
3. **Consistent Approach**: Use the same mocking patterns across components
4. **Isolation**: Test each component in isolation from dependencies
5. **Minimal Mock Scope**: Only mock what's needed for isolation
6. **Readable Test Setups**: Keep setups clear and descriptive

## Mocking Tools

1. **Mockall**: Primary library for generating mock implementations of traits
2. **Common Test Utilities**: Infrastructure in `tools/common/test_utils`

## Key Patterns

### Trait-Based Mocking

Define traits for components that need to be mocked and implement for real and mock implementations.

### Constructor Injection

Provide dependencies through constructors rather than creating them internally.

### Test Fixtures

Use standard fixtures for common test scenarios to reduce setup duplication.
