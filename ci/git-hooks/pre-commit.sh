#!/usr/bin/env bash
set -e

echo "[pre-commit] Running pre-commit hook."

staged_files=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$' || true)
if [ -z "$staged_files" ]; then
    echo "[pre-commit] DONE: No Rust files staged for commit."
    exit 0
fi

staged_existing_files=""
while IFS= read -r file; do
    [ -f "$file" ] && staged_existing_files="$staged_existing_files$file"$'\n'
done <<< "$staged_files"

staged_existing_files=$(printf '%s' "$staged_existing_files")

if [ -z "$staged_existing_files" ]; then
    echo "[pre-commit] DONE: No existing Rust files staged for commit."
    exit 0
fi

echo "[pre-commit] Running rustfmt on staged Rust files..."

echo "$staged_existing_files" | while IFS= read -r file; do
    rustfmt "$file"
    git add "$file"
done

echo "[pre-commit] Running clippy (advisory, no failure)..."

clippy_output=$(cargo clippy --all-targets --all-features --message-format short --color=always 2>&1 || true)

# \ to / conversion for Windows paths
normalized_clippy=$(echo "$clippy_output" | sed 's|\\|/|g' | sed 's/^/    [clippy] /')

pattern=$(echo "$staged_existing_files" | paste -sd'|' -)

echo "$normalized_clippy" | grep -E "$pattern" || true

echo "[pre-commit] DONE: Finished running pre-commit hook."
