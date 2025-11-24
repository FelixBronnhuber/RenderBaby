// TODO: Get this from the Data-Plane!
use engine_config::*;
use engine_main::{Engine, RenderEngine};
use engine_wgpu_wrapper::RenderOutput;

pub struct Model {
    engine: Engine,
}

impl Model {
    pub fn new() -> Self {
        // TODO: Get this from the Data-Plane!
        let builder = RenderConfigBuilder::new();

        let rc = builder.camera(Camera::default()).build().unwrap();
        Self {
            engine: Engine::new(rc, RenderEngine::Raytracer),
        }
    }

    pub fn generate_render_output(&mut self, fov: f32, width: u32, height: u32) -> RenderOutput {
        // TODO: Get this from the Data-Plane!
        let new_camera = Camera {
            fov,
            width,
            height,
            // width: (fov as u32 * 400).clamp(128, 2046),
            // height: (fov as u32 * 400).clamp(128, 2046),
        };
        let mut builder = RenderConfigBuilder::new();
        builder
            .add_sphere(Sphere::new(Vec3::new(0.0, 0.6, 4.0), 0.5, Vec3::COLOR_MAGENTA).unwrap())
            .add_sphere(Sphere::new(Vec3::new(-0.6, 0.0, 4.0), 0.5, Vec3::COLOR_GREEN).unwrap())
            .add_sphere(Sphere::new(Vec3::new(0.0, 0.0, 4.0), 0.5, Vec3::COLOR_RED).unwrap())
            .add_sphere(Sphere::new(Vec3::new(0.6, 0.0, 4.0), 0.5, Vec3::COLOR_BLUE).unwrap())
            .add_sphere(Sphere::new(Vec3::new(0.0, -0.6, 4.0), 0.5, Vec3::COLOR_CYAN).unwrap())
            .add_vertex(0.0, 0.0, 1.0) // Bottom-left
            .add_vertex(1.0, 0.0, 1.0) // Bottom-right
            .add_vertex(1.0, 1.0, 1.0) // Top-right
            // Alternatively call: .verticies(vec![...])
            .add_vertex(0.0, 1.0, 1.0) // Top-left
            .add_triangle(0, 1, 2) // First triangle
            .add_triangle(0, 2, 3); // Second triangle

        let rc = builder.camera(new_camera).build().unwrap();
        self.engine.render(rc).expect("Render failed")
    }

    pub fn import_obj(&mut self, obj_file_path: &str) {
        println!("Received path (obj): {}", obj_file_path);
        let mut scene = scene::scene::Scene::new();
        let _ = scene.load_object_from_file(obj_file_path.to_string());
        scene.proto_init();
    }

    pub fn import_scene(&mut self, scene_file_path: &str) {
        println!("Received path (scene): {}", scene_file_path);
        let _ = scene::scene::Scene::new().load_scene_from_file(scene_file_path.to_string());
    }
}
