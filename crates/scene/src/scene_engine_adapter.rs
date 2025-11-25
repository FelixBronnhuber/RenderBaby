use crate::{
    geometric_object::{Camera, Sphere, TriGeometry},
    scene::Scene,
};
/// Serves as an adpter between the scene plane and the render engine.
use anyhow::{Error, Result};
use engine_config::RenderConfigBuilder;
use engine_main::{Engine, RenderEngine};
use engine_wgpu_wrapper::RenderOutput;
type RenderSphere = engine_config::Sphere;
type RenderCamera = engine_config::Camera;
impl Sphere {
    fn to_render_engine_sphere(&self) -> RenderSphere {
        //! Creates and returns a engine_wgpu_wrapper::Sphere from self
        let center = self.get_center();
        let color = self.get_color();
        let render_color = engine_config::Vec3::new(color[0], color[1], color[2]);
        let res = RenderSphere::new(
            engine_config::Vec3::new(center.x, center.y, center.x),
            self.get_radius(),
            render_color,
        );
        res.unwrap()
        //todo: maybe do this when sphere is created/changed in scene to save preparation time when rendering
        //todo: probably better as into
    }
}

impl Camera {
    fn to_render_engine_camera(&self) -> Result<RenderCamera, Error> {
        let [width, heigt] = self.get_resolution();
        let camera = RenderCamera::new(width, heigt, self.get_fov());
        Ok(camera.unwrap())
    }
}

impl TriGeometry {
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
}

impl Scene {
    fn get_render_spheres(&self) -> Vec<RenderSphere> {
        //! Returns a Vec that contains all Scene spheres as engine_config::Sphere
        let mut res = vec![];
        for obj in self.get_objects() {
            if let Some(sphere) = obj.as_any().downcast_ref::<Sphere>() {
                res.push(sphere.to_render_engine_sphere());
            }
        }
        res
    }
    fn get_render_camera(&self) -> RenderCamera {
        //! Returns the camera as a enginge_config::camera
        self.get_camera().to_render_engine_camera().unwrap()
    }

    fn get_render_tris(&self) -> Vec<(Vec<f32>, Vec<u32>)> {
        //! Returns all TriGeometries of the scene, each representet as a touple of a vector of vertices and a vector of triangles
        let mut res = vec![];
        for obj in self.get_objects() {
            if let Some(tri) = obj.as_any().downcast_ref::<TriGeometry>() {
                res.push(tri.to_render_engine_tri());
                //break;
            }
        }
        res
    }

    pub fn render(&mut self) -> Result<RenderOutput, Error> {
        //! calls the render engine for the scene self. Returns ...( will be changed)
        // todo: change return type to mask engine plane
        //todo: try to remove mut

        let render_spheres = self.get_render_spheres();
        let camera = self.get_render_camera();

        if let Some(engine) = self.get_render_engine() {
        } else {
            let rc = RenderConfigBuilder::new()
                .spheres(render_spheres)
                .camera(camera)
                .build()
                .unwrap();
            let engine = Engine::new(rc, RenderEngine::Raytracer);
            self.set_render_engine(engine);
        }

        //placeholder for just one tri

        let render_spheres = self.get_render_spheres();
        let camera = self.get_render_camera();
        /* let t = self.get_render_tris();
        let a_tri = t.first();
        let vertices;
        let triangles;
        match a_tri {
            Some(tri) => {
                vertices = tri.0.clone();
                triangles = tri.1.clone();
            }
            None => {
                vertices = vec![];
                triangles = vec![];
            }
        } */
        let mut rcb = RenderConfigBuilder::new()
            .spheres(render_spheres)
            .camera(camera);
        for tri in self.get_render_tris() {
            rcb = rcb.verticies(tri.0.clone()).triangles(tri.1.clone());
        }
        let rc = rcb.build().unwrap();
        let engine = self.get_render_engine_mut().as_mut().unwrap();
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
