#!/usr/bin/env bash
# septicomics dev bootstrap.
#
# Installs the Rust toolchain components and CLI tools the CI gates depend on, so a
# fresh checkout can reproduce the Phase 1 gate locally. Idempotent — safe to re-run.
set -euo pipefail

echo "==> rustup components (rustfmt, clippy)"
rustup component add rustfmt clippy

echo "==> cargo-semver-checks (SemVer gate on cdm / fed-protocol)"
cargo install --locked cargo-semver-checks

echo "==> cargo-skill (per ARCHITECTURE.md tooling note)"
cargo install --locked cargo-skill

cat <<'EOF'

dev bootstrap complete. Reproduce the Phase 1 CI gate with:

  cargo fmt --all --check
  cargo clippy --all-targets --all-features -- -D warnings
  cargo test --all-features
  cargo test --doc --all-features
EOF
