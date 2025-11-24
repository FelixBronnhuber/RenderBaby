#!/usr/bin/env bash
set -e

HOOK_DIR=".git/hooks"
SRC_DIR="ci/git-hooks"

if [[ "$1" == "uninstall" ]]; then
    echo "[git hooks] Uninstalling all hooks..."

    for file in "$HOOK_DIR"/*; do
        if [[ "$file" != *.sample ]]; then
            echo "Removing $file"
            rm -f "$file"
        fi
    done

    echo "[git hooks] Done."
    exit 0
fi

echo "[git hooks] Installing all hooks from $SRC_DIR..."
mkdir -p "$HOOK_DIR"

for hook_file in "$SRC_DIR"/*.sh; do
    hook_name=$(basename "$hook_file" .sh)
    target="$HOOK_DIR/$hook_name"

    cat > "$target" <<EOF
#!/usr/bin/env bash
set -e
REPO_ROOT="\$(git rev-parse --show-toplevel)"
"\$REPO_ROOT/$SRC_DIR/$hook_name.sh" "\$@"
EOF

    chmod +x "$target"
    echo "[git hooks] Installed $hook_name"
done

echo "[git hooks] All hooks installed."
