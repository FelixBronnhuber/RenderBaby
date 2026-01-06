const GROUND_Y: f32 = -1.0;
const GROUND_ENABLED: bool = true;
const MAX_DEPTH: i32 = 5;

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
    bvh_node_count: u32,
    bvh_triangle_count: u32,
    bvh_root: u32,
    _pad1: u32,
    _pad2: u32,
    _pad3: u32,
};

struct Sphere {
    center: vec3<f32>,
    radius: f32,
    material: Material,
};

struct Mesh {
    triangle_index_start: u32,
    triangle_count: u32,
    _pad: vec2<u32>,
    material: Material,
}

struct Material {
    ambient: vec3<f32>,
    _pad0: f32,
    diffuse: vec3<f32>,
    _pad1: f32,
    specular: vec3<f32>,
    shininess: f32,
    emissive: vec3<f32>,
    ior: f32,
    opacity: f32,
    illum: u32,
    texture_index: i32,
    _pad2: u32,
}

struct HitRecord {
    hit: bool,
    t: f32,
    pos: vec3<f32>,
    normal: vec3<f32>,
    uv: vec2<f32>,
    use_texture: bool,
    material: Material,
}

struct PointLight {
    center: vec3<f32>,
    radius: f32,
    material: Material,
};

struct GPUTriangle {
    v0: vec3<f32>,
    _pad0: u32,
    v1: vec3<f32>,
    _pad1: u32,
    v2: vec3<f32>,
    _pad2: u32,
};

struct BVHNode {
    aabb_min: vec3<f32>,
    _pad0: u32,
    aabb_max: vec3<f32>,
    _pad1: u32,
    left: u32,
    right: u32,
    first_primitive: u32,
    primitive_count: u32,
};

struct TextureInfo {
    offset: u32,
    width: u32,
    height: u32,
    _pad: u32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read_write> output: array<u32>;
@group(0) @binding(2) var<storage, read> spheres: array<Sphere>;
@group(0) @binding(3) var<storage, read_write> accumulation: array<vec4<f32>>;
@group(0) @binding(4) var<uniform> prh: ProgressiveRenderHelper;
@group(0) @binding(5) var<storage, read> point_lights: array<PointLight>;
@group(0) @binding(6) var<storage, read> meshes: array<Mesh>;
@group(0) @binding(7) var<storage, read> bvh_nodes: array<BVHNode>;
@group(0) @binding(8) var<storage, read> bvh_indices: array<u32>;
@group(0) @binding(9) var<storage, read> bvh_triangles: array<GPUTriangle>;
@group(0) @binding(10) var<storage, read> uvs: array<f32>;
@group(0) @binding(11) var<storage, read> texture_data: array<u32>;
@group(0) @binding(12) var<storage, read> texture_info: array<TextureInfo>;

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

fn sample_texture(index: i32, uv: vec2<f32>) -> vec3<f32> {
    if (index < 0) {
        // Missing texture pattern (Magenta/Black checkerboard)
        let n = 10.0;
        let u2 = i32(floor(uv.x * n));
        let v2 = i32(floor(uv.y * n));
        if ((u2 + v2) % 2 == 0) {
            return vec3<f32>(0.0); // Black
        } else {
            return vec3<f32>(1.0, 0.0, 1.0); // Magenta
        }
    }
    let info = texture_info[u32(index)];

    // Wrap UVs (repeat)
    let u = fract(uv.x);
    let v = fract(uv.y);

    // Map to pixel coordinates
    // Flip V because standard UVs have (0,0) at bottom-left, but image data is top-left
    let x = min(u32(u * f32(info.width)), info.width - 1u);
    let y = min(u32((1.0 - v) * f32(info.height)), info.height - 1u);

    let pixel_index = info.offset + y * info.width + x;
    let pixel = texture_data[pixel_index];

    // Unpack RGBA8 (Little Endian: A B G R)
    let r = f32(pixel & 255u) / 255.0;
    let g = f32((pixel >> 8u) & 255u) / 255.0;
    let b = f32((pixel >> 16u) & 255u) / 255.0;

    // Convert sRGB to Linear
    return vec3<f32>(pow(r, 2.2), pow(g, 2.2), pow(b, 2.2));
}

fn intersect_sphere(ray_origin: vec3<f32>, ray_dir: vec3<f32>, sphere: Sphere) -> f32 {
    let oc = ray_origin - sphere.center;
    let a = dot(ray_dir, ray_dir);
    let half_b = dot(oc, ray_dir);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    }

    let sqrtd = sqrt(discriminant);
    var root = (-half_b - sqrtd) / a;

    if root <= 0.001 {
        root = (-half_b + sqrtd) / a;
        if root <= 0.001 {
            return -1.0;
        }
    }

    return root;
}

fn intersect_pointlight(ray_origin: vec3<f32>, ray_dir: vec3<f32>, pointlight: PointLight) -> f32 {
    let oc = ray_origin - pointlight.center;
    let a = dot(ray_dir, ray_dir);
    let half_b = dot(oc, ray_dir);
    let c = dot(oc, oc) - pointlight.radius * pointlight.radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    }

    let sqrtd = sqrt(discriminant);
    var root = (-half_b - sqrtd) / a;

    if root <= 0.001 {
        root = (-half_b + sqrtd) / a;
        if root <= 0.001 {
            return -1.0;
        }
    }

    return root;
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
        return vec3<f32>(-1.0, 0.0, 0.0);
    }

    let f = 1.0 / a;
    let s = ray_origin - tri.v0;
    let u = f * dot(s, h);

    if u < 0.0 || u > 1.0 {
        return vec3<f32>(-1.0, 0.0, 0.0);
    }

    let q = cross(s, edge1);
    let v = f * dot(ray_dir, q);

    if v < 0.0 || u + v > 1.0 {
        return vec3<f32>(-1.0, 0.0, 0.0);
    }

    let t = f * dot(edge2, q);

    if t > 0.0 {
        return vec3<f32>(t, u, v);
    }

    return vec3<f32>(-1.0, 0.0, 0.0);
}

fn intersect_bvh(ray_origin: vec3<f32>, ray_dir: vec3<f32>) -> HitRecord {
    var hit = HitRecord(false, 1e20, vec3<f32>(0.0), vec3<f32>(0.0), vec3<f32>(0.0));

    var stack: array<u32, 256>;     //very large, might be too big for bad GPUs, might have to make this variable
    var sp: i32 = 0;

    if (uniforms.bvh_node_count == 0u) {
        return hit;
    }

    stack[sp] = 0u;
    sp = sp + 1;

    loop {
        if sp == 0 {
            break;
        }
        sp = sp - 1;
        let node_idx = stack[sp];

        if (node_idx >= uniforms.bvh_node_count) {
            continue;
        }

        let node = bvh_nodes[node_idx];

        if (!intersect_aabb(ray_origin, ray_dir, node.aabb_min, node.aabb_max)) {
            continue;
        }

        if node.primitive_count > 0u {
            for (var i: u32 = 0u; i < node.primitive_count; i = i + 1u) {
                let tri_idx = node.first_primitive + i;

                if (tri_idx >= uniforms.bvh_triangle_count || tri_idx >= arrayLength(&bvh_indices)) {
                    continue;
                }

                let bvh_tri_idx = bvh_indices[tri_idx];

                if (bvh_tri_idx >= uniforms.bvh_triangle_count) {
                    continue;
                }

                let tri = bvh_triangles[bvh_tri_idx];

                let t = intersect_triangle(ray_origin, ray_dir, TriangleData(tri.v0, tri.v1, tri.v2, 0u));
                if t > 0.001 && t < hit.t {
                    hit.hit = true;
                    hit.t = t;
                    hit.pos = ray_origin + t * ray_dir;
                    hit.normal = normalize(cross(tri.v1 - tri.v0, tri.v2 - tri.v0));

                    if uniforms.color_hash_enabled != 0u {
                        hit.color = hash_to_color(bvh_tri_idx + 1u);
                    } else {
                        hit.color = vec3<f32>(0.3, 0.3, 0.3);
                    }
                }
            }
        } else {
            if node.left < uniforms.bvh_node_count {
                if (sp < 256) {
                    stack[sp] = node.left;
                    sp = sp + 1;
                }
            }
            if node.right < uniforms.bvh_node_count {
                if (sp < 256) {
                    stack[sp] = node.right;
                    sp = sp + 1;
                }
            }
        }
    }

    return hit;
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

// PCG random number generator
fn hash(seed: u32) -> u32 {
    var state = seed * 747796405u + 2891336453u;
    var word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

fn random_float(seed: ptr<function, u32>) -> f32 {
    *seed = hash(*seed);
    return f32(*seed) / 4294967296.0;
}

// Random vector in unit sphere (rejection sampling)
fn random_in_unit_sphere(seed: ptr<function, u32>) -> vec3<f32> {
    loop {
        let p = vec3<f32>(
            random_float(seed) * 2.0 - 1.0,
            random_float(seed) * 2.0 - 1.0,
            random_float(seed) * 2.0 - 1.0
        );
        if dot(p, p) < 1.0 {
            return p;
        }
    }
    return vec3<f32>(0.0);
}

// Random unit vector on hemisphere
fn random_unit_vector(seed: ptr<function, u32>) -> vec3<f32> {
    return normalize(random_in_unit_sphere(seed));
}

// Cosine-weighted hemisphere sampling
fn random_in_hemisphere(normal: vec3<f32>, seed: ptr<function, u32>) -> vec3<f32> {
    let in_unit_sphere = random_unit_vector(seed);

    if (dot(in_unit_sphere, normal) > 0.0) {
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

fn reflect_vector(v: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
    return v - 2.0 * dot(v, n) * n;
}

fn near_zero(v: vec3<f32>) -> bool {
    let s = 1e-8;
    return (abs(v.x) < s) && (abs(v.y) < s) && (abs(v.z) < s);
}

fn scatter_lambertian(
    normal: vec3<f32>,
    seed: ptr<function, u32>
) -> vec3<f32> {
    let scatter_direction = normal + random_unit_vector(seed);

    if near_zero(scatter_direction) {
        return normal;
    }

    return normalize(scatter_direction);
}

fn scatter_metal(
    ray_dir: vec3<f32>,
    hit: HitRecord,
    fuzz: f32,
    seed: ptr<function, u32>
) -> vec3<f32> {
    let reflected = reflect_vector(normalize(ray_dir), hit.normal);
    let fuzzed = reflected + fuzz * random_unit_vector(seed);
    return fuzzed;
}

fn collision(origin: vec3<f32>, light_dir: vec3<f32>, max_dist: f32) -> bool {
    if (GROUND_ENABLED) {
        let t = intersect_ground(origin, light_dir);
        if (t > 0.001 && t < max_dist) {
            return true;
        }
    }

    let bvh_hit = intersect_bvh(origin, light_dir);
    if (bvh_hit.hit && bvh_hit.t < max_dist) {
        return true;
    }

    for (var k = 0u; k < uniforms.spheres_count; k = k + 1u) {
        let t = intersect_sphere(origin, light_dir, spheres[k]);
        if (t > 0.001 && t < max_dist) {
            return true;
        }
    }

    for (var k = 0u; k < arrayLength(&point_lights); k = k + 1u) {
            let t = intersect_pointlight(origin, light_dir, point_lights[k]);
            if (t > 0.001 && t < max_dist) {
                return true;
            }
        }

    return false;
}

fn trace_ray(
    origin0: vec3<f32>,
    direction0: vec3<f32>,
    seed0: u32
) -> vec3<f32> {
    var origin = origin0;
    var direction = direction0;
    var seed = seed0;

    var color = vec3<f32>(0.0);
    var attenuation = vec3<f32>(1.0);

    for (var depth = 0; depth < MAX_DEPTH; depth = depth + 1) {
        var closest_hit = HitRecord(
            false,
            1e20,
            vec3<f32>(0.0),
            vec3<f32>(0.0),
            vec2<f32>(0.0),
            false,
            Material(
                vec3<f32>(0.0), 0.0,
                vec3<f32>(0.0), 0.0,
                vec3<f32>(0.0), 0.0,
                vec3<f32>(0.0), 0.0,
                0.0, 0u, -1, 0u
            )
        );

        // Ground
        if (GROUND_ENABLED) {
            let t = intersect_ground(origin, direction);
            if (t > 0.001 && t < closest_hit.t) {
                closest_hit.hit = true;
                closest_hit.t = t;
                closest_hit.pos = origin + t * direction;
                closest_hit.normal = vec3<f32>(0.0, 1.0, 0.0);
                closest_hit.material.diffuse = vec3<f32>(0.5);
                closest_hit.material.ambient = vec3<f32>(0.0);
                closest_hit.material.specular = vec3<f32>(0.0);
                closest_hit.uv = closest_hit.pos.xz;
                closest_hit.use_texture = true;
            }
        }

        // BVH Triangles
        let bvh_hit = intersect_bvh(origin, direction);
            if bvh_hit.hit && bvh_hit.t < closest_hit.t {
                closest_hit = bvh_hit;
        
        // Triangles
        for (var mesh_idx = 0u; mesh_idx < arrayLength(&meshes); mesh_idx = mesh_idx + 1u) {
            let mesh = meshes[mesh_idx];
            let tri_start = mesh.triangle_index_start;
            let tri_end = tri_start + mesh.triangle_count;
            
            for (var k = tri_start; k < tri_end; k = k + 1u) {
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
                
                let tri = TriangleData(v0, v1, v2, mesh_idx);
                let hit_data = intersect_triangle(origin, direction, tri);
                let t = hit_data.x;
                
                if (t > 0.001 && t < closest_hit.t) {
                    closest_hit.hit = true;
                    closest_hit.t = t;
                    closest_hit.pos = origin + t * direction;
                    closest_hit.normal = normalize(cross(v1 - v0, v2 - v0));
                    
                    let u = hit_data.y;
                    let v = hit_data.z;
                    let w = 1.0 - u - v;

                    let uv0 = vec2<f32>(uvs[v0_idx * 2u], uvs[v0_idx * 2u + 1u]);
                    let uv1 = vec2<f32>(uvs[v1_idx * 2u], uvs[v1_idx * 2u + 1u]);
                    let uv2 = vec2<f32>(uvs[v2_idx * 2u], uvs[v2_idx * 2u + 1u]);

                    closest_hit.uv = w * uv0 + u * uv1 + v * uv2;

                    if (uniforms.color_hash_enabled != 0u) {
                        closest_hit.material.diffuse = hash_to_color(k + 1u);
                        closest_hit.material.ambient = vec3<f32>(0.0);
                        closest_hit.material.specular = vec3<f32>(0.0);
                        closest_hit.use_texture = false;
                    } else {
                        closest_hit.material = mesh.material;
                        // Use texture if material has a valid texture index
                        closest_hit.use_texture = closest_hit.material.texture_index >= 0;
                    }
                }
            }
        }

        // Spheres
        for (var k = 0u; k < uniforms.spheres_count; k = k + 1u) {
            let sphere = spheres[k];
            let t = intersect_sphere(origin, direction, sphere);

            if (t > 0.001 && t < closest_hit.t) {
                closest_hit.hit = true;
                closest_hit.t = t;
                closest_hit.pos = origin + t * direction;
                closest_hit.normal = normalize(closest_hit.pos - sphere.center);
                closest_hit.material = sphere.material;
            closest_hit.use_texture = closest_hit.material.texture_index >= 0;
            }
        }

        // Point Light

        for (var k = 0u; k < arrayLength(&point_lights); k = k + 1u) {
            let point_light = point_lights[k];
            let t = intersect_pointlight(origin, direction, point_light);

            if (t > 0.001 && t < closest_hit.t) {
                closest_hit.hit = true;
                closest_hit.t = t;
                closest_hit.pos = origin + t * direction;
                closest_hit.normal = normalize(closest_hit.pos - point_light.center);
                closest_hit.material = point_light.material;
            }
        }

        if (!closest_hit.hit) {
            let unit_dir = normalize(direction);
            let a = 0.5 * (unit_dir.y + 1.0);
            let sky = (1.0 - a) * vec3<f32>(1.0) + a * vec3<f32>(0.5, 0.7, 1.0);

            color += attenuation * sky;
            break;
        }

        //has to be reviewed
        // if (depth == 0) {
        //     color += closest_hit.material.ambient;
        // }

        let specular_strength = (closest_hit.material.specular.x +
                                closest_hit.material.specular.y +
                                closest_hit.material.specular.z) / 3.0;
        let diffuse_strength = (closest_hit.material.diffuse.x +
                                closest_hit.material.diffuse.y +
                                closest_hit.material.diffuse.z) / 3.0;

        // Only treat as metal if it has specular but negligible diffuse
        let is_metal = specular_strength > 0.01 && diffuse_strength < 0.01;

        // Add emitted light
        color += attenuation * closest_hit.material.emissive;

        // Scatter
        var scattered: vec3<f32>;
        var albedo: vec3<f32>;

        if (is_metal) {
            // Metal material with glossy reflections
            // Map Ns (0-1000) to fuzz (1.0-0.0)
            // Higher Ns = sharper reflections (less fuzz)
            // Lower Ns = blurrier reflections (more fuzz)
            let fuzz = clamp(1.0 - (closest_hit.material.shininess / 1000.0), 0.0, 1.0);
            scattered = scatter_metal(direction, closest_hit, fuzz, &seed);

            if (dot(scattered, closest_hit.normal) <= 0.0) {
                break;
            }

            albedo = closest_hit.material.specular;
        } else {
            scattered = scatter_lambertian(closest_hit.normal, &seed);
            albedo = closest_hit.material.diffuse;
            // Apply texture
            if (closest_hit.use_texture) {
                albedo = albedo * sample_texture(closest_hit.material.texture_index, closest_hit.uv);
            }
        }

        // Update attenuation
        attenuation *= albedo;

        // Next ray
        origin = closest_hit.pos + 0.001 * closest_hit.normal;
        direction = normalize(scattered);
    }
    return color;
}

fn intersect_aabb(ray_origin: vec3<f32>, ray_dir: vec3<f32>, aabb_min: vec3<f32>, aabb_max: vec3<f32>) -> bool {
    let inv_dir = 1.0 / ray_dir;
    let t0s = (aabb_min - ray_origin) * inv_dir;
    let t1s = (aabb_max - ray_origin) * inv_dir;
    let tmin = max(max(min(t0s.x, t1s.x), min(t0s.y, t1s.y)), min(t0s.z, t1s.z));
    let tmax = min(min(max(t0s.x, t1s.x), max(t0s.y, t1s.y)), max(t0s.z, t1s.z));
    return tmax >= max(tmin, 0.0);
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
        var seed = hash(pixel_index + hash(sample_offset));

        let offset_x = random_float(&seed) - 0.5;
        let offset_y = random_float(&seed) - 0.5;

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