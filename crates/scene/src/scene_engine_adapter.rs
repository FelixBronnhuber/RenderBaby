use crate::{
    geometric_object::{Camera, Sphere},
    scene::Scene,
};
/// Serves as an adpter between the scene plane and the render engine.
use anyhow::{Error, Result};
use engine_config::RenderConfigBuilder;
use engine_wgpu_wrapper::RenderOutput;

type RenderSphere = engine_config::Sphere;
type RenderCamera = engine_config::Camera;

impl Sphere {
    fn to_render_engine_sphere(&self) -> RenderSphere {
        //! Creates and returns an engine_wgpu_wrapper::Sphere from self
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
        let [width, height] = self.get_resolution();
        let camera = RenderCamera::new(width, height, self.get_fov());
        Ok(camera?)
    }
}

impl Scene {
    pub(crate) fn get_render_spheres(&self) -> Vec<RenderSphere> {
        //! Returns a Vec that contains all Scene spheres as engine_config::Sphere
        let mut res = vec![];
        for obj in self.get_objects() {
            if let Some(sphere) = obj.as_any().downcast_ref::<Sphere>() {
                res.push(sphere.to_render_engine_sphere());
            }
        }
        res
    }
    pub(crate) fn get_render_camera(&self) -> RenderCamera {
        //! Returns the camera as a enginge_config::camera
        self.get_camera().to_render_engine_camera().unwrap()
    }

    pub fn render(&mut self) -> Result<RenderOutput, Error> {
        //! calls the render engine for the scene self. Returns ...( will be changed)
        // todo: change return type to mask engine plane

        let rc = RenderConfigBuilder::new()
            .spheres(self.get_render_spheres())
            .camera(self.get_render_camera())
            .build()?;

        let engine = self.get_render_engine_mut();

        engine.render(rc)
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn it_works() {}
}
