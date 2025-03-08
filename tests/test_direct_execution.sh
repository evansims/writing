#!/bin/bash

echo "Testing direct tool execution in the CLI..."
echo "========================================="

# Create test directory
cd /Users/evan/Developer/evansims/writing
mkdir -p test_output

# Try to use the CLI's "new" command with --help to see its output
echo -e "\nRunning './write new --help':"
./write new --help 2>&1 | tee test_output/new_help.log

# Check if we have a "new" subcommand
echo -e "\nDoes the CLI have the 'new' subcommand?"
if grep -q "Create new content" test_output/new_help.log; then
  echo "Yes, the 'new' subcommand exists."
else
  echo "No, the 'new' subcommand doesn't seem to exist or doesn't have expected help text."
fi

# Try to create a simple test article
echo -e "\nTrying to create a test article (will redirect output):"
./write new --title "Test Article" --topic "mindset" --tagline "A test article" --tags "test,article" --content-type "article" 2>&1 | tee test_output/new_article.log

# Check if it's using direct tool execution
echo -e "\nChecking if direct tool execution was used:"
if grep -q "Running tool:" test_output/new_article.log; then
  echo "SUCCESS: CLI is using direct tool execution"
  cat test_output/new_article.log
else
  echo "FAILURE: CLI might still be using make or doesn't show execution details"
  cat test_output/new_article.log
fi

# Clean up
rm -rf test_output 