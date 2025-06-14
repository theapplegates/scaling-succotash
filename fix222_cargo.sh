#!/bin/bash
set -e

OPENPGP_DEP='sequoia-openpgp = { path = "../openpgp", version = "2.0.0", default-features = false, features = ["crypto-openssl"] }'

fix_package_section() {
  local f="$1"
  # If the name line is borked, fix it
  sed -i.bak -E 's|name = "sequoia-openpgp\s*=\s*\{.*\}|name = "sequoia-openpgp"|' "$f"
}

fix_openpgp_dependency() {
  local f="$1"
  # Remove any sequoia-openpgp dependency lines first, then append the correct one after [dependencies]
  sed -i.bak '/sequoia-openpgp\s*=\s*{/d' "$f"
  awk -v dep="$OPENPGP_DEP" '
    BEGIN {added=0}
    /^\[dependencies\]/ {
      print
      if (!added) {print dep; added=1}
      next
    }
    {print}
  ' "$f" > "$f.tmp" && mv "$f.tmp" "$f"
}

remove_duplicate_dependencies_headers() {
  local f="$1"
  awk '/^\[dependencies\]$/ {if (found++) next} {print}' "$f" > "$f.tmp" && mv "$f.tmp" "$f"
}

find . -name 'Cargo.toml' | while read f; do
  fix_package_section "$f"
  fix_openpgp_dependency "$f"
  remove_duplicate_dependencies_headers "$f"
  rm -f "${f}.bak"
done

echo "All Cargo.toml files fixed!"
