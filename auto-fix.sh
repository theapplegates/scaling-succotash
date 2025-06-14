#!/bin/bash

# 1. Remove self-reference in openpgp/Cargo.toml
sed -i.bak '/^sequoia-openpgp\s*=/d' openpgp/Cargo.toml

# 2. Fix sequoia-openpgp dependency in all leaf crates
for f in autocrypt/Cargo.toml net/Cargo.toml ipc/Cargo.toml sequoia-sq/Cargo.toml; do
  sed -i.bak '/^sequoia-openpgp\s*=/d' "$f"
  sed -i.bak '/^\[dependencies\]/a\
sequoia-openpgp = { path = "../openpgp", version = "2.0.0", default-features = false, features = ["crypto-openssl"] }
' "$f"
done

# 3. Clean up features in openpgp/Cargo.toml
# (Remove extra crypto backends, leave only crypto-openssl)
sed -i.bak '/crypto-[a-z]\+/!b;n;d' openpgp/Cargo.toml  # Remove any crypto-BOTAN etc lines in features

# 4. Only one [features] section per file, and only one [dependencies]
# (You may need to clean these up manually if more complex.)

# 5. Remove [workspace.dependencies] from root if not needed.
sed -i.bak '/^\[workspace\.dependencies\]/,/^$/d' ROOT-Cargo\ copy.toml

echo "Cleanup complete."
