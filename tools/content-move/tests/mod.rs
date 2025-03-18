//! Test modules for content-move

// Unit tests
#[path = "unit/move_options_tests.rs"]
mod move_options_tests;

#[path = "unit/find_content_tests.rs"]
mod find_content_tests;

#[path = "unit/move_content_tests.rs"]
mod move_content_tests;

#[path = "unit/tests_tests.rs"]
mod tests_tests;

// Property tests
#[path = "property/move_content_properties.rs"]
mod move_content_properties;

// Integration tests
#[path = "integration/cli_tests.rs"]
mod cli_tests;
