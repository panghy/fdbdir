#!/usr/bin/env bash
set -euo pipefail

VERSION_INPUT=${1:-}
if [[ -z "$VERSION_INPUT" ]]; then
  echo "Usage: $0 X.Y.Z" >&2
  exit 1
fi

TAG="v${VERSION_INPUT}"

# Ensure clean working tree
if ! git diff --quiet || ! git diff --cached --quiet; then
  echo "Working tree not clean. Commit or stash changes first." >&2
  exit 1
fi

# Bump version in Cargo.toml [package] only (portable awk)
awk -v ver="$VERSION_INPUT" '
  BEGIN { inpkg=0 }
  /^\\[package\]/ { inpkg=1; print; next }
  /^\[/ { if (inpkg) inpkg=0; print; next }
  inpkg && /^version *= *"[^"]+"/ { sub(/version *= *"[^"]+"/, "version = \"" ver "\""); print; next }
  { print }
' Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml

# Commit and tag
git add Cargo.toml
git commit -m "release ${TAG}"
git tag "${TAG}"

echo "Created tag ${TAG}. Push with:"
echo "  git push && git push --tags"

