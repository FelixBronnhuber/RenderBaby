#!/usr/bin/env bash
set -e

echo "Running pre-commit hook."

staged_files=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$' || true)
if [ -z "$staged_files" ]; then
    echo "No Rust files staged for commit."
    exit 0
fi

echo "$staged_files" | while IFS= read -r file; do
    if [ -f "$file" ]; then
        rustfmt "$file"
    fi
done

echo "$staged_files" | xargs git add

echo "Put formatted Rust files back to the staging area."

clippy_output=$(cargo clippy --all-targets --all-features --message-format short 2>&1 || true)

echo "$clippy_output" | grep -Ff <(echo "$staged_files") || true

echo "Finished running pre-commit hook."
