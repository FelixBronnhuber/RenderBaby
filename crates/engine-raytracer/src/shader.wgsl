struct Dimensions {
    width: u32,
    height: u32,
};

struct Sphere {
    center: vec3<f32>,
    radius: f32,
    color: vec3<f32>,
    _pad: u32, // 4 bytes padding for alignment
};

@group(0) @binding(0) var<uniform> dimensions: Dimensions;
@group(0) @binding(1) var<storage, read_write> output: array<u32>;
@group(0) @binding(2) var<storage, read> spheres: array<Sphere>;

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

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let x: u32 = global_id.x;
    let y: u32 = global_id.y;

    let aspect = f32(dimensions.width) / f32(dimensions.height);
    let u = ((f32(x) / f32(dimensions.width - 1u)) * 2.0 - 1.0) * aspect;
    let v = (1.0 - f32(y) / f32(dimensions.height - 1u)) * 2.0 - 1.0;

    let camera_pos = vec3<f32>(0.0, 0.0, -2.0); // Camera behind the scene
    let screen_z: f32 = 0.0;

    let ray_dir = normalize(vec3<f32>(u, v, screen_z - camera_pos.z));

    let a = .5 * (ray_dir.y + 1.);
    var hit_color = vec3<f32>(1., 1., 1.); // Background color
    hit_color = (1. - a) * hit_color + a * vec3<f32>(.5, .7, 1.);

    var min_t = 1e20;

    for (var i = 0u; i < arrayLength(&spheres); i = i + 1u) {
        let sphere = spheres[i];
        let t = intersect_sphere(camera_pos, ray_dir, sphere);
        if t > 0.0 && t < min_t {
            min_t = t;
            hit_color = sphere.color;
        }
    }

    let index: u32 = y * dimensions.width + x;
    output[index] = color_map(hit_color);
}
