/// Serves as an adpter between the scene plane and the render engine.
use anyhow::{Error, Result};
use engine_config::{RenderConfigBuilder, RenderOutput};
use log::{info, error};
use scene_objects::{
    camera::{Camera, Resolution},
    sphere::Sphere,
    tri_geometry::TriGeometry,
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

fn camera_to_render_uniforms(
    camera: &Camera,
    spheres_count: u32,
    triangles_count: u32,
) -> Result<RenderUniforms, Error> {
    //! converts the given scene_object::camera::Camera to a render_config::Uniforms
    //! so that it can be passed to the render engine
    //! ## Parameter
    //! 'camera': scene_object::camer::Camera to be converted <br>
    //! 'spheres_count': Number of spheres to be rendered <br>
    //! 'triangles_count': Number of triangles to be rendered
    //! ## Returns
    //! render_config::Unfiforms for the given parameters

    //TODO: Replace defaults
    //let [width, height] = camera.get_resolution();
    let Resolution { width, height } = camera.get_resolution();
    let position = camera.get_position();
    let rotation = RenderCamera::default().dir; //Engine uses currently a direction vector
    let pane_width = RenderCamera::default().pane_width;
    let render_camera = RenderCamera::new(
        camera.get_fov(),
        pane_width,
        [position.x, position.y, position.z],
        rotation,
    );
    let uniforms = RenderUniforms::new(
        *width,
        *height,
        render_camera,
        spheres_count,
        triangles_count,
    );
    Ok(uniforms)
}

fn tri_geometry_to_render_tri(tri_geom: &TriGeometry) -> (Vec<f32>, Vec<u32>) {
    //! Converts the given TriGeometry to a touple of a Vector represention the triangle vertices and a vector referencing which points make up the triangles
    //! Purpose of the conversion is to pass the result to the render engine
    //! ## Parameter
    //! 'tri_geom': Reference to the TriGeometry that is to be converted
    let mut res_points = vec![];
    let mut res_tri = vec![];
    let mut count = 0u32;
    for tri in tri_geom.get_triangles() {
        for point in tri.get_points() {
            res_points.push(point.x);
            res_points.push(point.y);
            res_points.push(point.z);
        }
        res_tri.push(count);
        res_tri.push(count + 1);
        res_tri.push(count + 2);
        count += 3;
    }
    (res_points, res_tri)
}

/// Extends scene to offer functionalities needed for rendering with raytracer or pathtracer engine
impl Scene {
    fn get_render_spheres(&self) -> Vec<RenderSphere> {
        //! ## Returns
        //! a Vec that contains all Scene spheres as engine_config::Sphere
        let mut res = vec![];
        for sphere in self.get_spheres() {
            res.push(sphere_to_render_sphere(sphere));
        }
        res
    }
    pub(crate) fn get_render_uniforms(
        &self,
        spheres_count: u32,
        triangles_count: u32,
    ) -> RenderUniforms {
        //! ## Returns
        //! RenderUnfiform for the camera of the scene
        camera_to_render_uniforms(self.get_camera(), spheres_count, triangles_count).unwrap()
    }

    fn get_render_tris(&self) -> Vec<(Vec<f32>, Vec<u32>)> {
        //! ## Returns
        //! Vector of touples, with each of the touples representing a TriGeometry defined by the points and the triangles build from the points.
        let mut res = vec![];
        for tri in self.get_tri_geometries() {
            res.push(tri_geometry_to_render_tri(tri))
        }
        res
    }

    pub fn render(&mut self) -> Result<RenderOutput, Error> {
        //! calls the render engine for the scene self.
        //! ## Returns
        //! Result of either the RenderOutput or a error
        info!("{self}: Render has been called. Collecting render parameters");

        let s: String = serde_json::to_string(&self).unwrap();
        info!("{}", s);

        let render_spheres = self.get_render_spheres();
        let render_tris = self.get_render_tris();

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
            for (verts, tris) in render_tris {
                all_verts.extend(verts);
                all_tris.extend(tris);
            }
            (all_verts, all_tris)
        };
        info!(
            "{self}: Collected render parameter: {} spheres, {} triangles consisting of {} vertices. Building render config",
            render_spheres.len(),
            all_triangles.len() / 3,
            all_vertices.len() / 3
        );

        let mut rc = if self.first_render {
            self.first_render = false;
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
                // TODO: Handle sphere deletion via state: `.spheres(render_spheres)`
                .spheres_delete()
                .vertices(all_vertices)
                .triangles(all_triangles)
                .build()
        };

        #[allow(deprecated)]
        rc.translate(0.0, 0.0, 2.0);

        let engine = self.get_render_engine_mut();

        let output = engine.render(rc);
        match output {
            Ok(res) => match res.validate() {
                Ok(_) => {
                    info!("{self}: Successfully got valid render output");
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

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
