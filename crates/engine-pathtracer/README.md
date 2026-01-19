# engine-raytracer

Provides a reusable wgpu-based ray tracing implementation, extending the [`engine-wgpu-wrapper`](../engine-wgpu-wrapper) crate.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
engine-raytracer = { path = "../engine-raytracer" }
```

Import in your code:

```rust
use engine_raytracer::*;
```

## Features

- Defines data structures and memory layout for ray tracing types
- Handles creation and management of GPU buffers for scene data
- Loads and executes compute shaders for ray tracing

## Example

See [`engine-wgpu-wrapper`](../engine-wgpu-wrapper) for setup and usage.

## Notes

- This crate is intended for use with the `engine-wgpu-wrapper` crate.
