use engine_config::*;
use engine_main::{Engine, RenderEngine};
use engine_wgpu_wrapper::RenderOutput;

pub struct Model {
    engine: Engine,
    builder: RenderConfigBuilder,
}

impl Model {
    pub fn new() -> Self {
        // Load the OBJ file once.
        let (models, _materials) = tobj::load_obj(
            "/Users/felixbronnhuber/thu.de-alias/Softwareprojekt/OBJ-Files/ferris3d_v1.0.obj",
            &tobj::LoadOptions {
                triangulate: true,
                ..Default::default()
            },
        )
        .expect("Failed to load obj file");

        let mut all_positions = Vec::new();
        let mut all_indices = Vec::new();
        let mut vertex_offset = 0;

        for model in models {
            let mesh = &model.mesh;
            all_positions.extend_from_slice(&mesh.positions);
            for index in &mesh.indices {
                all_indices.push(*index + vertex_offset);
            }
            vertex_offset += (mesh.positions.len() / 3) as u32;
        }

        let mut builder = RenderConfigBuilder::new();
        let num_verticies = &all_positions.len();
        let num_triangles = &all_indices.len();
        builder = builder.verticies(all_positions);
        builder = builder.triangles(all_indices);

        log::info!("Num Verticies: {}", num_verticies);
        log::info!("Num Triangles: {}", num_triangles);

        let rc = builder
            .clone()
            .uniforms(Uniforms::default())
            .build()
            .unwrap();

        Self {
            engine: Engine::new(rc, RenderEngine::Raytracer),
            builder,
        }
    }

    pub fn generate_render_output(&mut self, fov: f32, width: u32, height: u32) -> RenderOutput {
        let new_uniforms = Uniforms {
            fov,
            width,
            height,
            ..Default::default()
        };

        let rc = self.builder.clone().uniforms(new_uniforms).build().unwrap();
        self.engine.render(rc).expect("Render failed")
    }
}
