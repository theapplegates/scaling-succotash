#!/bin/bash
# Run from the root of your workspace (e.g. /Users/thor3/Documents/loveit)

find . -name "Cargo.toml" | while read -r toml; do
    # Remove [workspace.dependencies] sections and their contents
    sed -i.bak '/^\[workspace\.dependencies\]/,/^\[/ { /^\[/!d; }; /^\[workspace\.dependencies\]/d' "$toml"

    # Remove self-dependency (e.g., sequoia-openpgp in openpgp/Cargo.toml)
    crate_name=$(grep -m1 '^name =' "$toml" | awk -F\" '{print $2}')
    if [ "$crate_name" != "" ]; then
        sed -i.bak "/^$crate_name = /d" "$toml"
    fi
done

echo "Cleanup complete. .bak backups are in each crate directory."
