#!/usr/bin/env bash
set -e

echo "Running pre-commit hook."

staged_files=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$' || true)
if [ -z "$staged_files" ]; then
    echo "No Rust files staged for commit."
    exit 0
fi

while IFS= read -r file; do
    rustfmt "$file"
done <<< "$staged_files"

git add "$staged_files"

echo "Finished running pre-commit hook."
