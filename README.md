# RenderBaby

[![Documentation][doc-img]][doc-url]

[doc-img]: https://img.shields.io/badge/renderbaby-cargo--docs-blue?style=for-the-badge&logo=github
[doc-url]: https://github.com/FelixBronnhuber/RenderBaby/renderbaby/index.html

RenderBaby is a Rust project.  
This repository uses a workspace structure, with multiple crates located in the `crates/` directory.

## Local Development & CI Precheck

Before pushing changes or opening a pull request, you should verify your code passes all checks performed by our CI pipeline.  
A Makefile is provided to help you run these checks locally.

To run all checks:

```bash
make ci
# or just:
make
```

This command will:
- Check code formatting (`cargo fmt`)
- Run Clippy linter (`cargo clippy`)
- Run all tests (`cargo test`)
- Build the project (`cargo build`)
- Generate documentation (`cargo doc`)

Running `make ci` ensures your code meets the standards enforced by CI.

## Running Tests

To run all tests for every crate in the workspace:

```bash
cargo test --all
```
