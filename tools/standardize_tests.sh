#!/bin/bash

# Standardize test structure across all tools
# This script creates a consistent test directory structure for all tools
# and moves tests from non-standard locations to the standard locations.

# Standard test directory structure:
# /tests
#   - /unit - unit tests
#   - /integration - integration tests
#   - /property - property-based tests
#   - mod.rs - test module definition

# Set the base directory for tools
TOOLS_DIR="./tools"

# Find all tools (excluding common/ directory which has special handling)
TOOLS=$(find "$TOOLS_DIR" -mindepth 1 -maxdepth 1 -type d | grep -v "common" | grep -v "target")

# Create a standard test structure for each tool
for TOOL in $TOOLS; do
  echo "Processing $TOOL..."

  # Create standard test directories if they don't exist
  mkdir -p "$TOOL/tests/unit"
  mkdir -p "$TOOL/tests/integration"
  mkdir -p "$TOOL/tests/property"

  # Create a mod.rs file if it doesn't exist
  if [ ! -f "$TOOL/tests/mod.rs" ]; then
    cat > "$TOOL/tests/mod.rs" << EOF
//! Tests for the $(basename "$TOOL") module
//!
//! This module contains tests for the $(basename "$TOOL") tool.

// Unit tests
pub mod unit;

// Integration tests
pub mod integration;

// Property-based tests
pub mod property;
EOF
  fi

  # Move any integration tests in the root tests directory
  INTEGRATION_TESTS=$(find "$TOOL/tests" -maxdepth 1 -name "*_test.rs" -o -name "*_tests.rs" -o -name "integration_*.rs" -o -name "integration.rs")
  for TEST in $INTEGRATION_TESTS; do
    TEST_NAME=$(basename "$TEST")
    echo "  Moving integration test: $TEST_NAME"
    cp "$TEST" "$TOOL/tests/integration/${TEST_NAME}"
    rm "$TEST"
  done

  # Check if there are tests in src directory that need to be moved
  if [ -d "$TOOL/src/tests" ]; then
    echo "  Found tests in src/tests directory, moving to standard location"

    # Copy unit tests to the unit tests directory
    find "$TOOL/src/tests" -name "*.rs" -not -name "mod.rs" -exec cp {} "$TOOL/tests/unit/" \;

    # Remove tests from src directory (only after confirming they exist in the new location)
    TEST_COUNT=$(find "$TOOL/tests/unit" -name "*.rs" | wc -l)
    if [ $TEST_COUNT -gt 0 ]; then
      echo "  Removing tests from src/tests directory"
      rm -rf "$TOOL/src/tests"
    fi
  fi

  # Check for inline tests in lib.rs
  if grep -q "#\[cfg(test)\]" "$TOOL/src/lib.rs"; then
    echo "  Found inline tests in lib.rs, extracting to unit tests"

    # Extract the test module content using a more robust approach
    # This uses awk to extract everything between #[cfg(test)] and the end of the module
    awk '/#\[cfg\(test\)]/{flag=1;next} /^}$/{if(flag){flag=0;print;next}} flag{print}' "$TOOL/src/lib.rs" > "$TOOL/temp_test_module.txt"

    # Get the module name
    MODULE_NAME=$(grep -A 1 "#\[cfg(test)\]" "$TOOL/src/lib.rs" | grep "mod" | sed 's/mod //g' | sed 's/{//g' | tr -d ' ' | tr -d '\n')

    if [ -n "$MODULE_NAME" ]; then
      # Create a new unit test file
      echo "//! Unit tests extracted from lib.rs" > "$TOOL/tests/unit/${MODULE_NAME}_tests.rs"
      echo "" >> "$TOOL/tests/unit/${MODULE_NAME}_tests.rs"
      echo "use $(basename "$TOOL")::*;" >> "$TOOL/tests/unit/${MODULE_NAME}_tests.rs"

      # Add the test module content
      cat "$TOOL/temp_test_module.txt" >> "$TOOL/tests/unit/${MODULE_NAME}_tests.rs"

      # Remove the temporary file
      rm "$TOOL/temp_test_module.txt"

      # Remove the test module from lib.rs
      # First make a backup
      cp "$TOOL/src/lib.rs" "$TOOL/src/lib.rs.bak"

      # Remove the test module using sed
      sed -i.bak '/#\[cfg(test\)]/,/^}$/d' "$TOOL/src/lib.rs"
      rm "$TOOL/src/lib.rs.bak"
    fi
  fi
done

echo "Test standardization complete!"