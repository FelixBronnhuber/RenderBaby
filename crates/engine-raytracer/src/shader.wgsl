const GROUND_Y: f32 = -1.0;
const GROUND_ENABLED: bool = true;
const MAX_DEPTH: i32 = 5;
const SHADOW_SAMPLES: u32 = 8u;
const SHADOW_EDGE: f32 = 0.2;

struct Camera {
    pane_distance: f32,
    pane_width: f32,
    _pad0: vec2<f32>,
    pos: vec3<f32>,
    _pad1: f32,
    dir: vec3<f32>,
    _pad2: f32,
};

struct ProgressiveRenderHelper {
    total_passes: u32,
    current_pass: u32,
    total_samples: u32,
    samples_per_pass: u32,
}

struct Uniforms {
    width: u32,
    height: u32,
    total_passes: u32,
    color_hash_enabled: u32,
    camera: Camera,
    spheres_count: u32,
    triangles_count: u32,
    _pad1: vec2<u32>,

};

struct Sphere {
    center: vec3<f32>,
    radius: f32,
    color: vec3<f32>,
    _pad: u32,
};

struct HitRecord {
    hit: bool,
    t: f32,
    pos: vec3<f32>,
    normal: vec3<f32>,
    color: vec3<f32>,
}

struct PointLight {
    position: vec3<f32>,
    intensity: f32,
    color: vec3<f32>,
    _pad: f32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read_write> output: array<u32>;
@group(0) @binding(2) var<storage, read> spheres: array<Sphere>;
@group(0) @binding(3) var<storage, read> vertices: array<f32>;
@group(0) @binding(4) var<storage, read> triangles: array<u32>;
@group(0) @binding(5) var<storage, read_write> accumulation: array<vec4<f32>>;
@group(0) @binding(6) var<uniform> prh: ProgressiveRenderHelper;
@group(0) @binding(7) var<storage, read>  point_lights: array<PointLight>;

fn linear_to_gamma(lin_color: f32) -> f32 {
    if (lin_color > 0.0) {
        return sqrt(lin_color);
    }
    return 0.0;
}

fn color_map(color: vec3<f32>) -> u32 {
    let r: u32 = u32(linear_to_gamma(color.x) * 255.999);
    let g: u32 = u32(linear_to_gamma(color.y) * 255.999);
    let b: u32 = u32(linear_to_gamma(color.z) * 255.999);
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

fn hash_to_color(n: u32) -> vec3<f32> {
    let h = n * 2654435761u;
    let r = f32(h % 41u) / 40.0;
    let g = f32(h % 29u) / 28.0;
    let b = f32(h % 19u) / 18.0;
    return vec3<f32>(r, g, b);
}

fn intersect_ground(ray_origin: vec3<f32>, ray_dir: vec3<f32>) -> f32 {
    if (abs(ray_dir.y) < 1e-6) {
        return -1.0;
    }

    let t = (GROUND_Y - ray_origin.y) / ray_dir.y;

    if (t > 0.0) {
        return t;
    }

    return -1.0;
}

fn hash(seed: u32) -> u32 {
    var state = seed * 747796405u + 2891336453u;
    var word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

fn random_float(seed: u32) -> f32 {
    return f32(hash(seed)) / 4294967296.0;
}

fn random_range(seed: u32, min: f32, max: f32) -> f32 {
    return min + random_float(seed) * (max - min);
}

fn random_unit_vector(seed: u32) -> vec3<f32> {
    var current_seed = seed;

    for (var i = 0u; i < 16u; i = i + 1u) {
        let p = vec3<f32>(
            random_range(current_seed, -1.0, 1.0),
            random_range(hash(current_seed), -1.0, 1.0),
            random_range(hash(hash(current_seed)), -1.0, 1.0)
        );

        let lensq = dot(p, p);

        if (lensq > 1e-16 && lensq <= 1.0) {
            return p / sqrt(lensq);
        }

        current_seed = hash(current_seed + 1u);
    }

    let fallback = vec3<f32>(
        random_range(current_seed, -1.0, 1.0),
        random_range(hash(current_seed), -1.0, 1.0),
        random_range(hash(hash(current_seed)), -1.0, 1.0)
    );

    return normalize(fallback);
}

fn collision(origin: vec3<f32>, light_dir: vec3<f32>, max_dist: f32) -> bool {
    if (GROUND_ENABLED) {
        let t = intersect_ground(origin, light_dir);
        if (t > 0.001 && t < max_dist) {
            return true;
        }
    }

    for (var k = 0u; k < uniforms.triangles_count; k = k + 1u) {
        let v0_idx = triangles[k * 3u + 0u];
        let v1_idx = triangles[k * 3u + 1u];
        let v2_idx = triangles[k * 3u + 2u];

        let v0 = vec3<f32>(
            vertices[v0_idx * 3u + 0u],
            vertices[v0_idx * 3u + 1u],
            vertices[v0_idx * 3u + 2u]
        );
        let v1 = vec3<f32>(
            vertices[v1_idx * 3u + 0u],
            vertices[v1_idx * 3u + 1u],
            vertices[v1_idx * 3u + 2u]
        );
        let v2 = vec3<f32>(
            vertices[v2_idx * 3u + 0u],
            vertices[v2_idx * 3u + 1u],
            vertices[v2_idx * 3u + 2u]
        );

        let tri = TriangleData(v0, v1, v2, 0u);
        let t = intersect_triangle(origin, light_dir, tri);
        if (t > 0.001 && t < max_dist) {
            return true;
        }
    }

    for (var k = 0u; k < uniforms.spheres_count; k = k + 1u) {
        let t = intersect_sphere(origin, light_dir, spheres[k]);
        if (t > 0.001 && t < max_dist) {
            return true;
        }
    }

    return false;
}

fn shadow(origin: vec3<f32>, light_pos: vec3<f32>, seed: u32) -> f32 {
    var visible_light: f32 = 0.0;
    var next_seed = seed;

    for (var i = 0u; i < SHADOW_SAMPLES; i = i + 1u) {
        let rand_jitter = random_unit_vector(next_seed) * SHADOW_EDGE;
        let jittered_pos = light_pos + rand_jitter;

        let light_dir = jittered_pos - origin;
        let distance = length(light_dir);
        let dir_normalized = light_dir / distance;

        if (!collision(origin, dir_normalized, distance)) {
            visible_light += 1.0;
        }

        next_seed = hash(next_seed + i);
    }

    return visible_light / f32(SHADOW_SAMPLES);
}

fn trace_ray(
    origin0: vec3<f32>,
    direction0: vec3<f32>,
    seed0: u32
) -> vec3<f32> {

    var origin     = origin0;
    var direction  = direction0;
    var seed       = seed0;

    var attenuation = vec3<f32>(1.0, 1.0, 1.0);
    var color       = vec3<f32>(0.0);

    for (var depth = 0; depth < MAX_DEPTH; depth = depth + 1) {
        var closest_hit = HitRecord(false, 1e20, vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0));

        // Ground
        if (GROUND_ENABLED) {
            let t = intersect_ground(origin, direction);
            if (t > 0.001 && t < closest_hit.t) {
                closest_hit.hit   = true;
                closest_hit.t     = t;
                closest_hit.pos = origin + t * direction;
                closest_hit.normal = vec3<f32>(0.0, 1.0, 0.0);
                closest_hit.color  = vec3<f32>(0.7);
            }
        }

        // Triangles
        for (var k = 0u; k < uniforms.triangles_count; k = k + 1u) {
            let v0_idx = triangles[k * 3u + 0u];
            let v1_idx = triangles[k * 3u + 1u];
            let v2_idx = triangles[k * 3u + 2u];

            let v0 = vec3<f32>(vertices[v0_idx * 3u + 0u],
                               vertices[v0_idx * 3u + 1u],
                               vertices[v0_idx * 3u + 2u]);

            let v1 = vec3<f32>(vertices[v1_idx * 3u + 0u],
                               vertices[v1_idx * 3u + 1u],
                               vertices[v1_idx * 3u + 2u]);

            let v2 = vec3<f32>(vertices[v2_idx * 3u + 0u],
                               vertices[v2_idx * 3u + 1u],
                               vertices[v2_idx * 3u + 2u]);

            let tri = TriangleData(v0, v1, v2, 0u);
            let t   = intersect_triangle(origin, direction, tri);

            if (t > 0.001 && t < closest_hit.t) {
                closest_hit.hit    = true;
                closest_hit.t      = t;
                closest_hit.pos  = origin + t * direction;
                closest_hit.normal = normalize(cross(v1 - v0, v2 - v0));
                if (uniforms.color_hash_enabled != 0u) {
                    closest_hit.color  = hash_to_color(k + 1u);
                } else {
                    closest_hit.color  = vec3<f32>(0.3, 0.3, 0.3);
                }
            }
        }

        // Spheres
        for (var k = 0u; k < uniforms.spheres_count; k = k + 1u) {
            let sphere = spheres[k];
            let t = intersect_sphere(origin, direction, sphere);

            if (t > 0.001 && t < closest_hit.t) {
                closest_hit.hit    = true;
                closest_hit.t      = t;
                closest_hit.pos  = origin + t * direction;
                closest_hit.normal = normalize(closest_hit.pos - sphere.center);
                closest_hit.color  = sphere.color;
            }
        }

        // Sky + exit recursion
        if (!closest_hit.hit) {
            let unit_dir = normalize(direction);
            let a = 0.5 * (unit_dir.y + 1.0);
            let sky = (1.0 - a) * vec3<f32>(1.0) + a * vec3<f32>(0.5, 0.7, 1.0);

            color += attenuation * sky;
            break;
        }

        let shadow_origin = closest_hit.pos + 0.001 * closest_hit.normal;
        var light_total = vec3<f32>(0.0);

        for (var i = 0u; i < arrayLength(&point_lights); i = i + 1u) {
            let light = point_lights[i];

            let light_dir = light.position - shadow_origin;
            let dist_pow2 = max(dot(light_dir, light_dir), 0.05);
            let dir_normalized = normalize(light_dir);

            //Lambert cosine term
            let dot = max(dot(closest_hit.normal, dir_normalized), 0.0);
            if (dot <= 0.0) {
                continue;
            }

            let visibility = shadow(
                shadow_origin,
                light.position,
                seed + i
            );

            light_total += visibility * dot * light.color * (light.intensity / dist_pow2);
        }

        color += attenuation * light_total * closest_hit.color;

        // Hit + scatter ray
        seed = hash(seed + u32(depth));
        let scatter_dir = closest_hit.normal + random_unit_vector(seed);

        // Update ray
        origin = closest_hit.pos + 0.001 * closest_hit.normal;
        direction = scatter_dir;

        // Apply diffuse attenuation
        attenuation *= closest_hit.color * 0.5;
    }

    return color;
}

@compute @workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    let x: u32 = global_id.x;
    let y: u32 = global_id.y;

    if (x >= uniforms.width || y >= uniforms.height) {
        return;
    }

    let pixel_index = y * uniforms.width + x;

    // Load previous accumulation
    var accumulated_color = accumulation[pixel_index].xyz;
    var total_samples = u32(accumulation[pixel_index].w);

    // Render new samples for this pass
    for (var sample: u32 = 0u; sample < prh.samples_per_pass; sample = sample + 1u) {
        let aspect = f32(uniforms.width) / f32(uniforms.height);

        // Use pass index to ensure different samples each pass
        let sample_offset = prh.current_pass * prh.samples_per_pass + sample;
        let seed = hash(pixel_index + hash(sample_offset));

        let offset_x = random_float(seed) - 0.5;
        let offset_y = random_float(hash(seed)) - 0.5;

        let u = (((f32(x) + offset_x) / f32(uniforms.width - 1u)) * 2.0 - 1.0) * aspect;
        let v = 1.0 - ((f32(y) + offset_y) / f32(uniforms.height - 1u)) * 2.0;

        let camera_pos = uniforms.camera.pos;

        let camera_forward = normalize(uniforms.camera.dir);
        let world_up = vec3<f32>(0.0, 1.0, 0.0);
        let camera_right = normalize(cross(world_up, camera_forward));
        let camera_up = cross(camera_forward, camera_right);

        let fov = uniforms.camera.pane_width / (2.0 * uniforms.camera.pane_distance * aspect);
        let ray_dir = normalize(fov * u * camera_right + fov * v * camera_up + camera_forward);

        let sample_color = trace_ray(camera_pos, ray_dir, seed);
        accumulated_color = accumulated_color + sample_color;
        total_samples = total_samples + 1u;
    }

    // Store accumulated result
    accumulation[pixel_index] = vec4<f32>(accumulated_color, f32(total_samples));

    // Write final color (averaged + reinhard tone mapping)
    let final_color = accumulated_color / f32(total_samples);
    let mapped = final_color / (final_color + vec3<f32>(1.0));
    output[pixel_index] = color_map(mapped);
}