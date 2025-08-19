#!/bin/bash

# Define the list of features to test
FEATURES=("v6_1" "v7_0" "v7_1" "v7_2")

# Exit immediately if a command exits with a non-zero status
set -e

# Iterate over each feature
for FEATURE in "${FEATURES[@]}"; do
    echo "Testing with feature: $FEATURE"

    # Run tests with the current feature
    cargo clean
    cargo test --features "$FEATURE"

    echo "Generating documentation with feature: $FEATURE"

    # Generate documentation with the current feature
    cargo doc --features "$FEATURE"
done

echo "All tests and documentation generation completed successfully."