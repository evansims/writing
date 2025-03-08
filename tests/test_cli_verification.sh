#!/bin/bash

echo "Verifying CLI direct tool execution..."
echo "======================================"

# Create test directory
cd /Users/evan/Developer/evansims/writing
mkdir -p test_output

# Redirect CLI output to see its execution path
echo -e "\nRunning topics command:"
./write topics 2>&1 | tee test_output/list_topics.log

# Check if the output mentions running a tool directly
echo -e "\nChecking if direct tool execution was used:"
if grep -q "Running tool:" test_output/list_topics.log; then
  echo "SUCCESS: CLI is using direct tool execution"
else
  echo "FAILURE: CLI might still be using make"
fi

# Clean up
rm -rf test_output 