# engine-wgpu-wrapper

This crate provides geometric objects that can be used as elements in a ray tracer

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
scene-objects = { path = "../scene-objects" }
```

Import in your code:

```rust
use scene_objects::{
    camera::{Camera, Resolution},
    geometric_object::GeometricObject,
    light_source::{LightSource},
    material::Material,
    mesh::Mesh,
    sphere::Sphere,
};
```

## Features

- Provides geometric objects that are relevant for a ray tracer:
  - Sphere: Simple object defined by a centroid and a radius
  - Mesh: Complex object composed of by triangles
  - Camera: Camera that serves as the user point of view
  - Light Source: Light representation
    - Ambient Light
    - Directional Light
    - Point Light
- Includes fields for material and texture

## Example

```rust
let sphere0 = Sphere::new(Vec3::new(0.0, 0.6, 2.0), 0.5, Material::default(), magenta);
let sphere1 = Sphere::new(Vec3::new(-0.6, 0.0, 2.0), 0.5, Material::default(), green);
let mut v = vec![
            1.0, 1.0, 1.0, 2.0, 1.0, 1.0, 2.0, 2.0, 1.0, 1.0, 2.0, 1.0, 1.0, 1.0, 2.0, 2.0, 1.0,
            2.0, 2.0, 2.0, 2.0, 1.0, 2.0, 2.0,
        ];
        let mesh_res = Mesh::new(
            v.clone(),
            vec![
                1, 2, 3, 1, 3, 4, 5, 6, 7, 5, 7, 8, 1, 2, 6, 1, 6, 5, 2, 3, 7, 2, 7, 6, 3, 4, 8, 3,
                8, 7, 4, 1, 5, 4, 5, 8,
            ],
            None,
            None,
            None,
            None,
            None,
        );
```
