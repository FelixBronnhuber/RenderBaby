# Git Hooks Tutorial

This project includes **pre-commit** and **pre-push** Git hooks to help maintain code quality automatically.

---

## What is a Git Hook?

A **Git hook** is a script that runs automatically at certain points in Git workflows.  
In this project:

- **pre-commit**: runs before a commit is saved (e.g., auto-formats Rust files).
- **pre-push**: runs before pushing to the remote (e.g., checks formatting, lints, and runs tests).

---

## Installing the Hooks

### Linux / macOS

Open a terminal and run:

```bash
./add-git-hooks.sh
```

This will copy all hooks from `git-hooks/` into your local `.git/hooks/` folder.

### Windows

Use a Bash-capable CLI such as **Git Bash**, then run:

```bash
./add-git-hooks.sh
```

---

## Ignoring Hooks

If you ever want to skip a hook, use the standard Git flag:

```bash
git commit --no-verify
git push --no-verify
```

---

## Uninstalling the Hooks

To remove all installed hooks (except `.sample` files), run:

```bash
./add-git-hooks.sh uninstall
```

This cleans up your `.git/hooks/` folder safely.