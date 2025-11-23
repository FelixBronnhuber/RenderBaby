struct Uniforms {
    width: u32,
    height: u32,
    fov: f32,
    spheres_count: u32,
    triangles_count: u32,
    _pad1: u32,
    _pad2: u32,
    _pad3: u32,
};

struct Sphere {
    center: vec3<f32>,
    radius: f32,
    color: vec3<f32>,
    _pad: u32, // 4 bytes padding for alignment
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read_write> output: array<u32>;
@group(0) @binding(2) var<storage, read> spheres: array<Sphere>;

@group(0) @binding(3) var<storage, read> vertices: array<f32>;
@group(0) @binding(4) var<storage, read> triangles: array<u32>; // Indexes into vertices

fn color_map(color: vec3<f32>) -> u32 {
    let r: u32 = u32(color.x * 255.);
    let g: u32 = u32(color.y * 255.);
    let b: u32 = u32(color.z * 255.);
    let a: u32 = 255u;

    return (a << 24u) | (b << 16u) | (g << 8u) | r;
}

fn intersect_sphere(ray_origin: vec3<f32>, ray_dir: vec3<f32>, sphere: Sphere) -> f32 {
    let oc = ray_origin - sphere.center;
    let a = dot(ray_dir, ray_dir);
    let b = 2.0 * dot(oc, ray_dir);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    return (-b - sqrt(discriminant)) / (2.0 * a);
}

struct TriangleData {
    v0: vec3<f32>,
    v1: vec3<f32>,
    v2: vec3<f32>,
    _pad: u32,
};

// Möller–Trumbore algorithm
fn intersect_triangle(ray_origin: vec3<f32>, ray_dir: vec3<f32>, tri: TriangleData) -> f32 {
    let edge1 = tri.v1 - tri.v0;
    let edge2 = tri.v2 - tri.v0;
    let h = cross(ray_dir, edge2);
    let a = dot(edge1, h);

    if abs(a) < 1e-6 {
        return -1.0;
    }

    let f = 1.0 / a;
    let s = ray_origin - tri.v0;
    let u = f * dot(s, h);

    if u < 0.0 || u > 1.0 {
        return -1.0;
    }

    let q = cross(s, edge1);
    let v = f * dot(ray_dir, q);

    if v < 0.0 || u + v > 1.0 {
        return -1.0;
    }

    let t = f * dot(edge2, q);

    if t > 0.0 {
        return t;
    }

    return -1.0;
}

// Knuth's multiplicative hash
fn hash_to_color(n: u32) -> vec3<f32> {
    let h = n * 2654435761u;
    let r = f32(h % 41u) / 40.0;
    let g = f32(h % 29u) / 28.0;
    let b = f32(h % 19u) / 18.0;
    return vec3<f32>(r, g, b);
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x: u32 = global_id.x;
    let y: u32 = global_id.y;

    let aspect = f32(uniforms.width) / f32(uniforms.height);
    let u = ((f32(x) / f32(uniforms.width - 1u)) * 2.0 - 1.0) * aspect;
    let v = (1.0 - f32(y) / f32(uniforms.height - 1u)) * 2.0 - 1.0;

    let camera_pos = vec3<f32>(0.0, 0.0, -uniforms.fov); // Camera behind the scene
    let screen_z: f32 = 0.0;

    let ray_dir = normalize(vec3<f32>(u, v, screen_z - camera_pos.z));

    let a = .5 * (ray_dir.y + 1.);
    var hit_color = vec3<f32>(1., 1., 1.); // Background color
    hit_color = (1. - a) * hit_color + a * vec3<f32>(.5, .7, 1.);

    var min_t = 1e20;

    // Triangle intersection loop
    for (var i = 0u; i < uniforms.triangles_count; i = i + 1u) {
        let v0_idx = triangles[i * 3u];
        let v1_idx = triangles[i * 3u + 1u];
        let v2_idx = triangles[i * 3u + 2u];

        let v0 = vec3<f32>(vertices[v0_idx * 3u], vertices[v0_idx * 3u + 1u], vertices[v0_idx * 3u + 2u]);
        let v1 = vec3<f32>(vertices[v1_idx * 3u], vertices[v1_idx * 3u + 1u], vertices[v1_idx * 3u + 2u]);
        let v2 = vec3<f32>(vertices[v2_idx * 3u], vertices[v2_idx * 3u + 1u], vertices[v2_idx * 3u + 2u]);

        let tri = TriangleData(v0, v1, v2, 0u);

        let t_tri = intersect_triangle(camera_pos, ray_dir, tri);
        if t_tri > 0.0 && t_tri < min_t {
            min_t = t_tri;
            hit_color = hash_to_color(i + 1u);
        }
    }

    // Sphere intersection loop
    for (var i = 0u; i < uniforms.spheres_count; i = i + 1u) {
        let sphere = spheres[i];
        let t = intersect_sphere(camera_pos, ray_dir, sphere);
        if t > 0.0 && t < min_t {
            min_t = t;
            hit_color = sphere.color;
        }
    }

    let index: u32 = y * uniforms.width + x;
    output[index] = color_map(hit_color);
}
