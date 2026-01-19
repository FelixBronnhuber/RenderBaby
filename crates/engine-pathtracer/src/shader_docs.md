# Path Tracer WGSL Shader Documentation

## Overview

This WGSL compute shader implements a progressive Monte Carlo path tracer with support for:

- Triangle meshes with BVH acceleration
- Sphere primitives
- Point lights
- Physically-based materials (lambertian, metal, emissive)
- Texture mapping with sRGB conversion
- Progressive rendering with accumulation
- Tone mapping (Reinhard)

## Architecture

### Main Entry Point

```wgsl
@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>)
```

The shader is dispatched as a compute shader with a 16×16 workgroup size. Each invocation handles one pixel.

### Data Structures

#### Core Structures

- **`Camera`**: Defines the virtual camera with position, direction, view pane distance and width
- **`Uniforms`**: Global rendering parameters including resolution, sample counts, scene configuration
- **`ProgressiveRenderHelper`**: Manages multi-pass progressive rendering state

#### Geometry

- **`Sphere`**: Sphere primitive with center, radius, and material
- **`GPUTriangle`**: Triangle with three vertices, indices, and mesh reference
- **`Mesh`**: Collection of triangles sharing a material
- **`BVHNode`**: Bounding volume hierarchy node for ray-triangle acceleration

#### Materials & Lighting

- **`Material`**: Material with ambient, diffuse, specular, emissive, IOR, opacity, and texture index
- **`PointLight`**: Spherical light source with emissive material
- **`HitRecord`**: Ray intersection result containing position, normal, UV coordinates, and material

#### Textures

- **`TextureInfo`**: Metadata for texture (offset, width, height)
- Texture data stored as packed RGBA8 in linear buffer

## GPU Bindings

| Binding | Type      | Access     | Description                                      |
| ------- | --------- | ---------- | ------------------------------------------------ |
| 0       | `uniform` | read       | `Uniforms` - Global rendering parameters         |
| 1       | `storage` | read_write | `output` - RGBA8 output buffer                   |
| 2       | `storage` | read       | `spheres` - Array of sphere primitives           |
| 3       | `storage` | read_write | `accumulation` - Progressive accumulation buffer |
| 4       | `uniform` | read       | `ProgressiveRenderHelper` - Pass information     |
| 5       | `storage` | read       | `point_lights` - Array of point lights           |
| 6       | `storage` | read       | `meshes` - Array of mesh definitions             |
| 7       | `storage` | read       | `bvh_nodes` - BVH tree nodes                     |
| 8       | `storage` | read       | `bvh_indices` - Triangle indices for BVH         |
| 9       | `storage` | read       | `bvh_triangles` - Triangle geometry data         |
| 10      | `storage` | read       | `uvs` - Texture coordinates                      |
| 11      | `storage` | read       | `texture_data` - Packed RGBA8 texture data       |
| 12      | `storage` | read       | `texture_info` - Texture metadata array          |

## Algorithms

### Ray Tracing Pipeline

The main `trace_ray` function implements path tracing with:

1. **Ray generation**: Camera rays with anti-aliasing jitter
2. **Intersection testing**: Against all scene geometry
3. **Material evaluation**: Determines surface properties
4. **Light sampling**: Calculates direct and indirect lighting
5. **Scattering**: Generates new ray directions based on material type
6. **Accumulation**: Progressive refinement over multiple passes

### BVH Traversal

The `intersect_bvh` function uses iterative stack-based traversal:

- Stack size: 1024 elements (may need adjustment for low-end GPUs)
- AABB intersection testing for early rejection
- Leaf nodes contain triangle primitives
- Returns closest intersection with full hit information

### Material System

Three material types are supported:

#### Lambertian (Diffuse)

- Cosine-weighted hemisphere sampling
- Perfect diffuse reflection
- Texture mapping support

#### Metal (Specular)

- Glossy reflections with controllable roughness
- Fuzz parameter derived from shininess (Ns: 0-1000)
- Pure reflection with optional perturbation

#### Emissive

- Direct light emission
- No scattering (terminates ray path)

### Sampling & Random Numbers

- **PCG hash function**: Fast, high-quality pseudo-random number generation
- **Rejection sampling**: For uniform sampling on unit sphere
- **Stratified sampling**: Anti-aliasing via pixel jitter
- **Seed generation**: Based on pixel index and sample number for reproducibility

### Progressive Rendering

Multi-pass accumulation system:

1. Each pass renders `samples_per_pass` samples per pixel
2. Results accumulate in dedicated buffer
3. Final image averaged over total samples
4. Tone mapping applied to final output

### Texture Sampling

- Bilinear filtering with UV wrapping (repeat mode)
- sRGB to linear conversion (gamma 2.2)
- Checkerboard pattern for missing textures
- UV coordinate interpolation using barycentric coordinates

## Color Management

### Gamma Correction

- Input textures: sRGB → Linear (pow 2.2)
- Output: Linear → sRGB (sqrt approximation)

### Tone Mapping

- Reinhard operator: `color / (color + 1)`
- Applied after averaging all samples
- Prevents over-bright pixels

## Usage Example

```rust,ignore
// Include this documentation
#![doc = include_str!("shader_docs.md")]

pub const SHADER_SOURCE: &str = include_str!("shader.wgsl");

// Load and compile shader
let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    label: Some("Path Tracer Shader"),
    source: wgpu::ShaderSource::Wgsl(SHADER_SOURCE.into()),
});
```

## References

- [Ray Tracing in One Weekend](https://raytracing.github.io/)
- [Physically Based Rendering (PBR Book)](https://pbr-book.org/)
- [WGSL Specification](https://www.w3.org/TR/WGSL/)
