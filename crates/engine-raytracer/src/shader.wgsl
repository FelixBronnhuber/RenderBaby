const GROUND_Y: f32 = -1.0;
const GROUND_ENABLED: bool = true;
const MAX_DEPTH: i32 = 50;

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
    _pad2: vec2<u32>,
}

struct HitRecord {
    hit: bool,
    t: f32,
    pos: vec3<f32>,
    normal: vec3<f32>,
    material: Material,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read_write> output: array<u32>;
@group(0) @binding(2) var<storage, read> spheres: array<Sphere>;
@group(0) @binding(3) var<storage, read> vertices: array<f32>;
@group(0) @binding(4) var<storage, read> triangles: array<u32>;
@group(0) @binding(5) var<storage, read> meshes: array<Mesh>;
@group(0) @binding(6) var<storage, read_write> accumulation: array<vec4<f32>>;
@group(0) @binding(7) var<uniform> prh: ProgressiveRenderHelper;

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

    // if t > 0.001 {
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
    
    //if t > 0.001 {
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
fn random_cosine_direction(seed: ptr<function, u32>) -> vec3<f32> {
    let r1 = random_float(seed);
    let r2 = random_float(seed);
    
    let phi = 2.0 * 3.14159265359 * r1;
    let x = cos(phi) * sqrt(r2);
    let y = sin(phi) * sqrt(r2);
    let z = sqrt(1.0 - r2);
    
    return vec3<f32>(x, y, z);
}

fn reflect_vector(v: vec3<f32>, n: vec3<f32>) -> vec3<f32> {
    return v - 2.0 * dot(v, n) * n;
}

fn near_zero(v: vec3<f32>) -> bool {
    let s = 1e-8;
    return (abs(v.x) < s) && (abs(v.y) < s) && (abs(v.z) < s);
}

// Get orthonormal basis from normal
fn get_onb(normal: vec3<f32>) -> mat3x3<f32> {
    let up = select(vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(1.0, 0.0, 0.0), abs(normal.y) > 0.999);
    let tangent = normalize(cross(up, normal));
    let bitangent = cross(normal, tangent);
    return mat3x3<f32>(tangent, bitangent, normal);
}

fn scatter_lambertian(
    ray_dir: vec3<f32>,
    hit: HitRecord,
    seed: ptr<function, u32>
) -> vec3<f32> {
    // Cosine-weighted random direction
    let local_dir = random_cosine_direction(seed);
    let onb = get_onb(hit.normal);
    let scatter_direction = onb * local_dir;
    
    if near_zero(scatter_direction) {
        return hit.normal;
    }
    
    return scatter_direction;
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
            Material(
                vec3<f32>(0.0), 0.0,
                vec3<f32>(0.0), 0.0,
                vec3<f32>(0.0), 0.0,
                vec3<f32>(0.0), 0.0,
                0.0, 0u, vec2<u32>(0u)
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
            }
        }
        
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
                let t = intersect_triangle(origin, direction, tri);
                
                if (t > 0.001 && t < closest_hit.t) {
                    closest_hit.hit = true;
                    closest_hit.t = t;
                    closest_hit.pos = origin + t * direction;
                    closest_hit.normal = normalize(cross(v1 - v0, v2 - v0));
                    
                    if (uniforms.color_hash_enabled != 0u) {
                        closest_hit.material.diffuse = hash_to_color(k + 1u);
                        closest_hit.material.ambient = vec3<f32>(0.0);
                        closest_hit.material.specular = vec3<f32>(0.0);
                    } else {
                        closest_hit.material = mesh.material;
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
            }
        }
        
        // Miss - hit sky
        if (!closest_hit.hit) {
            let unit_dir = normalize(direction);
            let a = 0.5 * (unit_dir.y + 1.0);
            let sky = (1.0 - a) * vec3<f32>(1.0) + a * vec3<f32>(0.5, 0.7, 1.0);
            color += attenuation * sky;
            break;
        }
        
        // Add ambient on first hit
        if (depth == 0) {
            color += closest_hit.material.ambient;
        }
        
        // Determine material type and scatter
        let specular_strength = (closest_hit.material.specular.x + 
                                closest_hit.material.specular.y + 
                                closest_hit.material.specular.z) / 3.0;
        
        var scattered: vec3<f32>;
        var albedo: vec3<f32>;
        
        if (specular_strength > 0.01) {
            // Metal material
            // Use shininess as inverse fuzz (higher shininess = less fuzz)
            let fuzz = clamp(1.0 - closest_hit.material.shininess / 100.0, 0.0, 1.0);
            scattered = scatter_metal(direction, closest_hit, fuzz, &seed);
            
            // Check if scattered below surface
            if (dot(scattered, closest_hit.normal) <= 0.0) {
                break;
            }
            
            albedo = closest_hit.material.specular;
        } else {
            // Lambertian material
            scattered = scatter_lambertian(direction, closest_hit, &seed);
            albedo = closest_hit.material.diffuse;
        }
        
        // Russian Roulette path termination
        if (depth > 3) {
            let max_albedo = max(max(albedo.x, albedo.y), albedo.z);
            let continue_prob = min(max_albedo, 0.95);
            
            if (random_float(&seed) > continue_prob) {
                break;
            }
            
            albedo = albedo / continue_prob;
        }
        
        // Update for next bounce
        origin = closest_hit.pos + scattered * 0.001;
        direction = normalize(scattered);
        attenuation *= albedo;
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
    
    var accumulated_color = accumulation[pixel_index].xyz;
    var total_samples = u32(accumulation[pixel_index].w);
    
    for (var sample: u32 = 0u; sample < prh.samples_per_pass; sample = sample + 1u) {
        let aspect = f32(uniforms.width) / f32(uniforms.height);
        
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
    
    accumulation[pixel_index] = vec4<f32>(accumulated_color, f32(total_samples));
    
    let final_color = accumulated_color / f32(total_samples);
    output[pixel_index] = color_map(final_color);
}