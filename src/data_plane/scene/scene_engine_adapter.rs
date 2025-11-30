/// Serves as an adpter between the scene plane and the render engine.
use anyhow::{Error, Result};
use engine_config::RenderConfigBuilder;
use engine_wgpu_wrapper::RenderOutput;
use scene_objects::{camera::Camera, sphere::Sphere, tri_geometry::TriGeometry};
use crate::data_plane::scene::{render_scene::Scene};

type RenderSphere = engine_config::Sphere;
type RenderUniforms = engine_config::Uniforms;

/* impl Sphere {
    fn to_render_engine_sphere(&self) -> RenderSphere {
        //! Creates and returns an engine_wgpu_wrapper::Sphere from self
        let center = self.get_center();
        let color = self.get_color();
        let render_color = engine_config::Vec3::new(color[0], color[1], color[2]);

        let res = RenderSphere::new(
            engine_config::Vec3::new(center.x, center.y, center.z),
            self.get_radius(),
            render_color,
        );

        res.unwrap()
        //todo: maybe do this when sphere is created/changed in scene to save preparation time when rendering
        //todo: probably better as into
    }
} */
fn sphere_to_render_sphere(sphere: &Sphere) -> RenderSphere {
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

/* impl Camera {
    fn to_render_engine_uniforms(
        &self,
        spheres_count: u32,
        triangles_count: u32,
    ) -> Result<RenderUniforms, Error> {
        let [width, height] = self.get_resolution();
        let uniforms = RenderUniforms::new(
            width,
            height,
            self.get_fov(),
            spheres_count,
            triangles_count,
        );
        Ok(uniforms)
    }
} */
fn camera_to_render_uniforms(
    camera: &Camera,
    spheres_count: u32,
    triangles_count: u32,
) -> Result<RenderUniforms, Error> {
    let [width, height] = camera.get_resolution();
    let uniforms = RenderUniforms::new(
        width,
        height,
        camera.get_fov(),
        spheres_count,
        triangles_count,
    );
    Ok(uniforms)
}

/* impl TriGeometry {
    fn to_render_engine_tri(&self) -> (Vec<f32>, Vec<u32>) {
        let mut res_points = vec![];
        let mut res_tri = vec![];
        let mut count = 0u32;
        for tri in self.get_triangles() {
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
} */
fn tri_geometry_to_render_tri(tri_geom: &TriGeometry) -> (Vec<f32>, Vec<u32>) {
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

impl Scene {
    pub(crate) fn get_render_spheres(&self) -> Vec<RenderSphere> {
        //! Returns a Vec that contains all Scene spheres as engine_config::Sphere
        let mut res = vec![];
        for obj in self.get_objects() {
            if let Some(sphere) = obj.as_any().downcast_ref::<Sphere>() {
                res.push(sphere_to_render_sphere(sphere));
            }
        }
        res
    }
    pub(crate) fn get_render_uniforms(
        &self,
        spheres_count: u32,
        triangles_count: u32,
    ) -> RenderUniforms {
        //! Returns the uniforms including camera settings
        camera_to_render_uniforms(self.get_camera(), spheres_count, triangles_count).unwrap()
    }

    fn get_render_tris(&self) -> Vec<(Vec<f32>, Vec<u32>)> {
        //! Returns all TriGeometries of the scene, each representet as a touple of a vector of vertices and a vector of triangles
        let mut res = vec![];
        for obj in self.get_objects() {
            if let Some(tri) = obj.as_any().downcast_ref::<TriGeometry>() {
                res.push(tri_geometry_to_render_tri(tri));
                //break;
            }
        }
        res
    }

    pub fn render(&mut self) -> Result<RenderOutput, Error> {
        //! calls the render engine for the scene self. Returns ...( will be changed)
        // todo: change return type to mask engine plane
        let render_spheres = self.get_render_spheres();
        let render_tris = self.get_render_tris();

        let spheres_count = render_spheres.len() as u32;
        let triangles_count = render_tris
            .iter()
            .map(|(_, tri)| tri.len() as u32 / 3)
            .sum();

        let uniforms = self.get_render_uniforms(spheres_count, triangles_count);
        let mut rcb = RenderConfigBuilder::new()
            .uniforms(uniforms)
            .spheres(render_spheres);
        for tri in render_tris {
            rcb = rcb.vertices(tri.0.clone()).triangles(tri.1.clone());
        }
        let rc = rcb.build().unwrap();
        let engine = self.get_render_engine_mut();
        //let res = engine.render(rc).unwrap();
        //Ok((res.pixels, res.width, res.height))
        engine.render(rc)
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {}
}
