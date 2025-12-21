/// Serves as an adpter between the scene plane and the render engine.
use anyhow::{Error, Result};
use engine_config::{RenderConfigBuilder, RenderOutput};
use glam::Vec3;
use log::{debug, error, info};
use scene_objects::{
    camera::{Camera, Resolution},
    mesh::Mesh,
    sphere::Sphere,
};
use crate::data_plane::scene::{render_scene::Scene};

type RenderSphere = engine_config::Sphere;
type RenderUniforms = engine_config::Uniforms;
pub type RenderCamera = engine_config::Camera;

fn sphere_to_render_sphere(sphere: &Sphere) -> RenderSphere {
    //! Converts a given scene_objects::sphere::Sphere to a engine_config::sphere
    //! so it can be passed to the render engine
    //! ## Parameter
    //! scene_objects::sphere::Sphere to be converted
    //! ## Returns
    //! engine_config::Sphere based on the given sphere
    RenderSphere::new(
        {
            let center = sphere.get_center();
            engine_config::Vec3::new(center.x, center.y, center.z)
        },
        sphere.get_radius(),
        {
            let color = sphere.get_color();
            engine_config::Vec3::new(color[0], color[1], color[2])
        },
    )
    .unwrap()
    //todo error handling
}
fn vec3_to_array(vec: Vec3) -> [f32; 3] {
    [vec.x, vec.y, vec.z]
}
fn camera_to_render_uniforms(
    camera: &Camera,
    spheres_count: u32,
    triangles_count: u32,
    color_hash_enabled: bool,
) -> Result<RenderUniforms, Error> {
    //! converts the given scene_object::camera::Camera to a render_config::Uniforms
    //! so that it can be passed to the render engine
    //! ## Parameter
    //! 'camera': scene_object::camer::Camera to be converted <br>
    //! 'spheres_count': Number of spheres to be rendered <br>
    //! 'triangles_count': Number of triangles to be rendered <br>
    //! 'color_hash_enabled': Whether color hash is enabled
    //! ## Returns
    //! render_config::Unfiforms for the given parameters
    let position = camera.get_position();
    let dir = camera.get_look_at() - camera.get_position(); //Engine uses currently a direction vector
    let render_camera = RenderCamera::new(
        camera.get_pane_distance(),
        camera.get_pane_width(),
        vec3_to_array(position),
        vec3_to_array(dir),
    );

    let Resolution { width, height } = camera.get_resolution();
    let uniforms = RenderUniforms::new(
        *width,
        *height,
        render_camera,
        camera.get_ray_samples(), //samples: ray per pixel
        spheres_count,
        triangles_count,
    )
    .with_color_hash(color_hash_enabled);
    Ok(uniforms)
}
fn mesh_to_render_data(mesh: &Mesh) -> (Vec<f32>, Vec<u32>) {
    //! Extracts vertices and point references from the given mesh
    //! ## Parameter
    //! 'mesh': Mesh from scene_objects crate that is to be converted
    //! Returns: touple of: Vec<f32> where 3 entries define one point in 3d space, and Vec<u32> referencing which points make up a triangle
    (mesh.get_vertices().clone(), mesh.get_tri_indices().clone())
}

/// Extends scene to offer functionalities needed for rendering with raytracer or pathtracer engine
impl Scene {
    fn get_render_spheres(&self) -> Vec<RenderSphere> {
        //! ## Returns
        //! a Vec that contains all Scene spheres as engine_config::Sphere
        self.get_spheres()
            .iter()
            .map(sphere_to_render_sphere)
            .collect()
    }
    pub(crate) fn get_render_uniforms(
        &self,
        spheres_count: u32,
        triangles_count: u32,
    ) -> RenderUniforms {
        //! ## Returns
        //! RenderUnfiform for the camera of the scene
        camera_to_render_uniforms(
            self.get_camera(),
            spheres_count,
            triangles_count,
            self.get_color_hash_enabled(),
        )
        .unwrap()
    }

    fn get_render_tris(&self) -> Vec<(Vec<f32>, Vec<u32>)> {
        //! ## Returns
        //! Vector of touples, with each of the touples representing a TriGeometry defined by the points and the triangles build from the points.
        self.get_meshes().iter().map(mesh_to_render_data).collect()
    }

    pub fn render(&mut self) -> Result<RenderOutput, Error> {
        //! calls the render engine for the scene self.
        //! ## Returns
        //! Result of either the RenderOutput or a error
        info!("{self}: Render has been called. Collecting render parameters");

        let render_spheres = self.get_render_spheres();
        let render_tris = self.get_render_tris();
        debug!("Scene mesh data: {:?}", self.get_meshes());
        debug!("Collected mesh data: {:?}", render_tris);

        let spheres_count = render_spheres.len() as u32;
        let triangles_count = render_tris
            .iter()
            .map(|(_, tri)| tri.len() as u32 / 3)
            .sum();

        let uniforms = self.get_render_uniforms(spheres_count, triangles_count);

        // Collect all vertices and triangles into flat vectors
        let (all_vertices, all_triangles) = if render_tris.is_empty() {
            (vec![], vec![])
        } else {
            let mut all_verts = vec![];
            let mut all_tris = vec![];
            let mut vertex_offset = 0u32;

            for (verts, tris) in render_tris {
                let vertex_count = (verts.len() / 3) as u32;

                for tri_idx in tris {
                    all_tris.push(tri_idx + vertex_offset);
                }

                all_verts.extend(verts);

                vertex_offset += vertex_count;
            }
            (all_verts, all_tris)
        };
        info!("Collected vertices: {:?}", all_vertices);
        info!("Collected tris: {:?}", all_triangles);
        info!(
            "{self}: Collected render parameter: {} spheres, {} triangles consisting of {} vertices. Building render config",
            render_spheres.len(),
            triangles_count,
            all_vertices.len() / 3
        );

        let rc = if self.get_first_render() {
            self.set_first_render(false);
            // NOTE: *_create is for the first initial render which initializes all the buffers etc.
            RenderConfigBuilder::new()
                .uniforms_create(uniforms)
                .spheres_create(render_spheres)
                .vertices_create(all_vertices)
                .triangles_create(all_triangles)
                .build()
        } else {
            // NOTE: * otherwise the values are updated with the new value an the unchanged fields
            // are kept as is. See: ../../../crates/engine-config/src/render_config.rs - `Change<T>`
            RenderConfigBuilder::new()
                .uniforms(uniforms)
                .spheres(render_spheres)
                .vertices(all_vertices)
                .triangles(all_triangles)
                .build()
        };

        let engine = self.get_render_engine_mut();

        let output = engine.render(rc);
        match output {
            Ok(res) => match res.validate() {
                Ok(_) => {
                    info!("{self}: Successfully got valid render output");
                    self.set_last_render(res.clone());
                    Ok(res)
                }
                Err(error) => {
                    error!("{self}: Received invalid render output");
                    Err(error)
                }
            },
            Err(error) => {
                error!("{self}: The following error occurred when rendering: {error}");
                Err(error)
            }
        }
    }
}
