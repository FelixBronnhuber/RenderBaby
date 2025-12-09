# RenderBaby

[![Documentation][doc-img]][doc-url]

[doc-img]: https://img.shields.io/badge/renderbaby-cargo--docs-blue?style=for-the-badge&logo=github
[doc-url]: https://felixbronnhuber.github.io/RenderBaby/index.html

RenderBaby is a Rust project.  
This repository uses a workspace structure, with multiple crates located in the `crates/` directory.

## Git Hooks

This project uses git hooks to maintain code quality. The hooks run automatically when you commit or push code.

### Installing Git Hooks

To install the git hooks, run:

```bash
chmod +x ./ci/git-hooks/*.sh && \
./ci/add-git-hooks.sh
```

To uninstall them:

```bash
./ci/add-git-hooks.sh uninstall
```

### What the Hooks Do

**pre-commit** (runs on `git commit`):
- Automatically formats staged Rust files with `cargo fmt`
- Runs `cargo clippy` on staged files (advisory only, shows warnings but doesn't block commits)

**pre-push** (runs on `git push`):
- Checks code formatting with `cargo fmt --check`
- Runs `cargo clippy` with warnings as errors
- Runs all tests with `cargo test --all`

These hooks ensure your code meets CI standards before it reaches the remote repository.

## Running Tests

To run all tests for every crate in the workspace:

```bash
cargo test --all
```
