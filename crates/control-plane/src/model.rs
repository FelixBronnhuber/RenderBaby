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
                triangulate: false,
                ..Default::default()
            },
        )
        .expect("Failed to load obj file");

        let mut builder = RenderConfigBuilder::new();

        for model in models {
            let mesh = &model.mesh;
            let vertex_offset = builder.vertices.clone().unwrap_or_default().len() as u32 / 3;

            for p in mesh.positions.chunks(3) {
                builder.add_vertex(p[0], p[1], p[2]);
            }

            let mut index_offset = 0;
            for &face_arity in &mesh.face_arities {
                match face_arity {
                    3 => {
                        builder.add_triangle(
                            mesh.indices[index_offset] + vertex_offset,
                            mesh.indices[index_offset + 1] + vertex_offset,
                            mesh.indices[index_offset + 2] + vertex_offset,
                        );
                    }
                    4 => {
                        builder.add_quad(
                            mesh.indices[index_offset] + vertex_offset,
                            mesh.indices[index_offset + 1] + vertex_offset,
                            mesh.indices[index_offset + 2] + vertex_offset,
                            mesh.indices[index_offset + 3] + vertex_offset,
                        );
                    }
                    _ => {
                        // ignore other polygons for now
                    }
                }
                index_offset += face_arity as usize;
            }
        }

        let num_vertices = builder.vertices.clone().unwrap_or_default().len();
        let num_triangles = builder.triangles.clone().unwrap_or_default().len() / 3;

        log::info!("Num Vertices: {}", num_vertices);
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
