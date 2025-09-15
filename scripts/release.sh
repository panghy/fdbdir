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

# Bump version in Cargo.toml [package] only
awk -v ver="$VERSION_INPUT" '
  BEGIN { inpkg=0 }
  /^\[package\]/ { inpkg=1; print; next }
  /^\[/ { if (inpkg) inpkg=0; print; next }
  inpkg && /^version *= *"[^"]+"/ { sub(/version *= *"[^"]+"/, "version = \"" ver "\""); print; next }
  { print }
' Cargo.toml > Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml

# Regenerate lockfile to reflect the new local package version without changing deps
if ! command -v cargo >/dev/null 2>&1; then
  echo "Error: 'cargo' command not found. This script requires cargo to release Rust projects." >&2
  exit 1
fi
# Prefer offline to avoid network; fall back to online if needed
if ! cargo generate-lockfile --offline >/dev/null 2>&1; then
  if ! cargo generate-lockfile >/dev/null 2>&1; then
    echo "Error: Failed to generate Cargo.lock file (tried both offline and online modes)." >&2
    exit 1
  fi
fi
# Verify Cargo.lock exists and is non-empty
if [[ ! -s Cargo.lock ]]; then
  echo "Error: Cargo.lock was not generated or is empty." >&2
  exit 1
fi

# Commit and tag (include Cargo.lock so CI with --locked won't fail)
git add Cargo.toml Cargo.lock
git commit -m "release ${TAG}"
git tag "${TAG}"

echo "Created tag ${TAG}. Push with:"
echo "  git push && git push --tags"
