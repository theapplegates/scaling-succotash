#!/bin/bash
set -e

WORKSPACE_DIR="$(pwd)" # must be run in /Users/thor3/Documents/loveit

echo "Fixing absolute paths in all Cargo.toml files below $WORKSPACE_DIR..."

find "$WORKSPACE_DIR" -name Cargo.toml | while read -r toml; do
    echo "Checking $toml"
    # Replace any absolute path
    sed -i.bak 's|/Users/thor3/Documents/openpgp|../openpgp|g' "$toml"
    # Optional: Also fix path = "openpgp" at the workspace root, if needed
    # If you want to force ALL path deps to be relative, add this:
    # sed -i '' 's|path *= *"openpgp"|path = "../openpgp"|g' "$toml"
done

echo "Done! All absolute paths replaced with '../openpgp'."
