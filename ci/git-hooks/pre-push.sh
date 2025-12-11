#!/usr/bin/env bash
set -euo pipefail

echo "[pre-push] Running rustfmt..."
cargo fmt --all -- --check

echo "[pre-push] Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "[pre-push] Running tests..."
cargo test --all --verbose

echo "[pre-push] All checks passed."
