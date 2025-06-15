#!/usr/bin/env bash
set -euo pipefail

# 1) In openpgp/Cargo.toml: remove any self-dependency.
sed -i.bak '/^sequoia-openpgp\s*=/d' openpgp/Cargo.toml

# 2) In every other crate, ensure exactly one sequoia-openpgp dep:
for crate in autocrypt ipc net sequoia-sq; do
  toml="$crate/Cargo.toml"
  # remove old lines
  sed -i.bak '/^sequoia-openpgp\s*=/d' "$toml"
  # insert correct dependency after [dependencies]
  sed -i.bak '/^\[dependencies\]/a\
sequoia-openpgp = { path = "../openpgp", version = "2.0.0", default-features = false, features = ["crypto-openssl"] }
' "$toml"
done

# 3) Strip any workspace.dependencies override in your root Cargo.toml:
sed -i.bak '/^\[workspace\.dependencies\]/,/^\[/d' Cargo.toml

echo "✅ All Cargo.toml files normalized. Backups saved as *.bak"
