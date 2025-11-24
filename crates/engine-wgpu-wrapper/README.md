# engine-wgpu-wrapper

Provides reusable wgpu setup and buffer management code for GPU-based rendering
engines. Designed to be extended by the
[`engine-raytracer`](../engine-raytracer) and `engine-pathtracer` crates.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
engine-wgpu-wrapper = { path = "../engine-wgpu-wrapper" }
```

Import in your code:

```rust
use engine_wgpu_wrapper::{WgpuWrapper, EngineType};
```

## Features

- Simplifies `wgpu` device and buffer initialization.
- Provides a context for rendering engines to build upon.
- Easily extendable for ray tracing and path tracing engines.

## Example

```rust
use engine_wgpu_wrapper::{WgpuWrapper, EngineType};

fn main() -> anyhow::Result<()> {
    let mut renderer = WgpuWrapper::new(EngineType::Raytracer, 800, 600)?;
    let output = renderer.render()?;
    output.validate()?;
    // Use output...
    Ok(())
}
```

Temporary change to trigger CI.
TODO: Remove this!
