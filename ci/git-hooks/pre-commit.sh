#!/usr/bin/env bash
set -e

echo "[pre-commit] Running pre-commit hook."

staged_files=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$' || true)
if [ -z "$staged_files" ]; then
    echo "[pre-commit] DONE: No Rust files staged for commit."
    exit 0
fi

echo "[pre-commit] Running rustfmt on staged Rust files..."

cargo fmt --all
echo "$staged_files" | xargs git add

echo "[pre-commit] Running clippy (advisory, no failure)..."

cargo clippy --all-targets --all-features --message-format short --color=always 2>&1 || true

echo "[pre-commit] DONE: Finished running pre-commit hook."
