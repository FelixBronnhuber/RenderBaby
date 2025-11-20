#!/usr/bin/env bash
set -e

get_changed_files() {
    local local_ref=$1
    local local_sha=$2
    local remote_sha=$3

    local branch
    branch=$(git rev-parse --abbrev-ref "$local_ref")

    local reference
    if [[ "$remote_sha" =~ ^0+$ ]]; then
        reference=$(git rev-list --remotes --max-count=1 2>/dev/null || git rev-parse "$local_sha~1")
    else
        reference=$remote_sha
    fi

    local changed_files
    changed_files=$(git diff --name-only "$reference..$local_sha" 2>/dev/null || echo "")

    echo "$changed_files"
}

get_changed_rust_files() {
    local local_ref=$1
    local local_sha=$2
    local remote_sha=$3

    local all_changed_files
    all_changed_files=$(get_changed_files "$local_ref" "$local_sha" "$remote_sha")

    if [ -z "$all_changed_files" ]; then
        echo ""
        return
    fi

    echo "$all_changed_files" | grep '\.rs$' || true
}

while read -r local_ref local_sha _ remote_sha
do
    files_to_check=$(get_changed_rust_files "$local_ref" "$local_sha" "$remote_sha")

    if [ -z "$files_to_check" ]; then
        echo "[pre-push] No Rust files to check."
        continue
    fi

    echo "[pre-push] Rust files changed:"
    echo "$files_to_check"

    echo "[pre-push] Running rustfmt on changed files..."
    while IFS= read -r file; do
        rustfmt --check "$file"
    done <<< "$files_to_check"

    echo "[pre-push] Running cargo clippy..."
    clippy_output=$(cargo clippy --all-targets --all-features --message-format short 2>&1)
    filtered_output=$(echo "$clippy_output" | grep -Ff <(echo "$files_to_check") || true)

    if [ -n "$filtered_output" ]; then
        echo "[pre-push] Clippy issues in changed files:"
        echo "$filtered_output"
        exit 1
    fi

    echo "[pre-push] Running cargo test..."
    cargo test --all --verbose

done

echo "[pre-push] All checks passed."
