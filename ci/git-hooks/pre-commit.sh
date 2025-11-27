#!/usr/bin/env bash
set -e

echo "Running pre-commit hook."

staged_files=$(git diff --cached --name-only --diff-filter=ACM | grep '\.rs$' || true)
if [ -z "$staged_files" ]; then
    echo "No Rust files staged for commit."
    exit 0
fi

#

staged_existing_files=""
while IFS= read -r file; do
    [ -f "$file" ] && staged_existing_files="$staged_existing_files$file"$'\n'
done <<< "$staged_files"

if [ -z "$staged_existing_files" ]; then
    echo "[pre-commit] No existing Rust files staged for commit."
    exit 0
fi

echo "[pre-commit] Running rustfmt on staged Rust files..."

echo "$staged_existing_files" | while IFS= read -r file; do
    if [ -f "$file" ]; then
        rustfmt "$file"
    fi
done

echo "$staged_files" | xargs git add

echo "[pre-commit] Put formatted Rust files back to the staging area."

echo "[pre-commit] Running clippy (advisory, filtered for staged files)..."

clippy_output=$(cargo clippy --all-targets --all-features --message-format short --color=always 2>&1 || true)

# \ to / conversion for Windows paths
normalized_clippy=$(echo "$clippy_output" | sed 's|\\|/|g' | sed 's/^/    [clippy] /')

pattern=$(echo "$staged_existing_files" | paste -sd'|' -)

echo "$normalized_clippy" | grep -E "$pattern" || true

echo "Finished running pre-commit hook."
