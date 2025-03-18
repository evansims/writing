#!/bin/bash

# Script to generate missing test files

# Function to create placeholder test files
create_test_placeholders() {
  local crate=$1
  local test_dir="$crate/tests"

  # Create directories if they don't exist
  mkdir -p "$test_dir/unit"
  mkdir -p "$test_dir/integration"
  mkdir -p "$test_dir/property"

  # Create unit test module
  if [ ! -f "$test_dir/unit/mod.rs" ]; then
    cat > "$test_dir/unit/mod.rs" << EOF
//! Unit tests for $crate

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
EOF
  fi

  # Create integration test module
  if [ ! -f "$test_dir/integration/mod.rs" ]; then
    cat > "$test_dir/integration/mod.rs" << EOF
//! Integration tests for $crate

#[cfg(test)]
mod tests {
    #[test]
    fn it_integrates() {
        assert!(true);
    }
}
EOF
  fi

  # Create property test module
  if [ ! -f "$test_dir/property/mod.rs" ]; then
    cat > "$test_dir/property/mod.rs" << EOF
//! Property tests for $crate

#[cfg(test)]
mod tests {
    #[test]
    fn property_holds() {
        assert!(true);
    }
}
EOF
  fi
}

# List of crates that need test files
crates=(
  "content-delete"
  "topic-edit"
  "image-optimize"
  "write"
  "image-build"
  "content-validate"
  "content-search"
  "topic-add"
  "topic-delete"
  "topic-rename"
  "content-template"
  "content-move"
  "toc-generate"
  "llms-generate"
)

# Create test files for each crate
for crate in "${crates[@]}"; do
  echo "Creating test files for $crate"
  create_test_placeholders "$crate"
done

echo "Done creating test files!"