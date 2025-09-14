fdbdir — FoundationDB Directory Explorer CLI

A small, fast Rust CLI to browse FoundationDB directories and inspect data. It supports an interactive REPL with tab completion, colorized output, tuple decoding, and command history.

**Highlights**
- List directories and sample keys in any DirectoryLayer path
- Tuple-aware decoding of keys and values (falls back to escaped bytes)
- Colorized output for readability
- Tab completion for commands and directory paths (no trailing '/' appended when multiple matches exist)
- Command history persisted to `~/.fdbdir_history`
- Flexible scans: limit, raw byte prefixes, and raw key display
- Safe error handling in REPL (errors never exit the session)

**Requirements**
- Rust toolchain (Cargo) to build
- FoundationDB client library (`libfdb_c`) available at runtime
  - macOS: `brew install foundationdb`
  - Linux: install official FoundationDB packages
- This binary targets FoundationDB API 7.1

Note: Headers are embedded at build time (`embedded-fdb-include`), so only the client library is needed at runtime.

**Build**
- `cargo build`

**Run**
- Interactive REPL: `cargo run -- -i`
- One‑shot commands:
  - `cargo run -- ls /`
  - `cargo run -- scan /app/foo -n 100`
  - `cargo run -- scan /app/foo -n 200 -p '\x00\x01abcd'` (raw prefix)
  - `cargo run -- scan /app/foo --raw` (raw keys)

Prebuilt binaries
- Releases include macOS (arm64, x86_64) and Linux (x86_64) tarballs.
- Homebrew Tap (macOS):
  - `brew tap panghy/homebrew-tap`
  - `brew install fdbdir`

Cluster file:
- Env: `FDB_CLUSTER_FILE=/path/to/fdb.cluster cargo run -- -i`
- Flag: `cargo run -- --cluster-file /path/to/fdb.cluster -i`

**REPL Commands**
- `help` — Show commands
- `pwd` — Print current directory path
- `cd <path>` — Change directory. Supports `/`, `..`, and relative paths
- `ls [path]` —
  - Always shows subdirectories (with trailing '/')
  - In non-root directories, also shows “Keys (first 50)” with tuple-decoded values
  - If there are more keys, a hint suggests using `scan`
- `scan [limit] [prefix] [--raw|-r]` —
  - Streams key/value pairs in the current directory’s subspace
  - `limit` defaults to 50
  - `prefix` is a raw byte prefix; supports escapes like `\x00`, `\n`, `\r`, `\t`, `\\`, `\"`
  - `--raw` prints keys as escaped bytes (no tuple parsing)
- `exit` / `quit` — Leave the REPL

Tab completion:
- Completes commands: `help`, `exit`, `quit`, `pwd`, `cd`, `ls`, `scan`
- Path completion for `cd`, `ls`, `scan`
- Does not auto-append `/` when multiple matches share a prefix (e.g., `segments/` and `segmentsIndex/` → completing `seg` yields `segments`)

History:
- Up/Down arrows navigate history
- Stored at `~/.fdbdir_history`

**Output Formatting**
- Keys are decoded as tuples relative to the current directory; fallback is escaped bytes
- Values attempt tuple decoding; fallback is pretty UTF‑8 or escaped bytes
- Colors: line number (dim), key (cyan), arrow (dim), value (green); directories are bold blue
- To disable colors, set `NO_COLOR=1` in your environment

**Behavior Notes**
- `ls /` shows only directories; it does not show keys at the directory layer root
- `scan` with a prefix applies the raw prefix after the current directory’s byte prefix
- Tuple decoding uses the DirectoryLayer subspace to interpret keys

**Troubleshooting**
- “libfdb_c not found”: ensure the client library is installed and visible
  - macOS (Apple Silicon): `export DYLD_LIBRARY_PATH=/opt/homebrew/lib`
  - macOS (Intel): `export DYLD_LIBRARY_PATH=/usr/local/lib`
  - Linux: `export LD_LIBRARY_PATH=/usr/lib` (or your distro path)
- Empty key list in `ls`: you may be at a non-leaf directory (only subdirectories exist). `cd` deeper or use `scan` with a meaningful prefix.

**Development**
- Pinned dependencies for reproducibility (see Cargo.toml)
- FoundationDB crate: `foundationdb = "=0.9.2"` with features `fdb-7_1`, `embedded-fdb-include`
- Ideas welcome: flags (e.g., `ls -n` for sample size, `--no-color`), additional layers, or exports (JSON/CSV)

**Releasing (maintainers)**
- One‑liner: `make release VERSION=X.Y.Z` (updates Cargo.toml, commits, tags `vX.Y.Z`)
- Push tag: `git push && git push --tags`
- GitHub Actions (cargo-dist) builds and uploads macOS (arm64/x86_64) and Linux (x86_64) artifacts to the release.
- Homebrew Tap: configured in cargo-dist to target `panghy/homebrew-tap`. You can wire cargo-dist to open a PR to the tap automatically, or copy the generated formula into `Formula/fdbdir.rb`.
