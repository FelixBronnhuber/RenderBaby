use crate::{geometric_object::Sphere, scene::Scene};
/// Serves as an adpter between the scene plane and the render engine.
//use std::fmt::Error;
use anyhow::{Error, Result};
use engine_config::RenderCommand;
use engine_wgpu_wrapper::{EngineType, RenderOutput, WgpuWrapper};
type RenderSphere = engine_config::Sphere;
impl Sphere {
    fn to_render_engine_sphere(&self) -> RenderSphere {
        //! Creates and returns a engine_wgpu_wrapper::Sphere from self
        let center = self.get_center();
        RenderSphere::new(
            [center.x, center.y, center.x],
            self.get_radius(),
            self.get_color().map(|x| x as f32),
        )
        //center.as_slice?
        //todo: maybe do this when sphere is created/changed in scene to save preparation time when rendering
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

    pub fn render(&self) -> Result<RenderOutput, Error> {
        // todo: change return type to mask engine plane
        //! calls the render engine for the scene self. Returns ...( will be changed)
        // todo: get from camera
        let width = 1920 / 2;
        let height = 1080 / 2; // todo: add camera size / fov
        let fov = 5.0; //?
        let render_spheres = self.get_render_spheres();
        let rc = RenderCommand {
            fov: Some(fov),
            spheres: render_spheres,
        };
        let wgpu = WgpuWrapper::new(EngineType::Raytracer, width, height, fov);
        /*let res = wgpu.unwrap().render(rc);
        let res = res.unwrap(); // todo catch error...
        //res.pixels
        //(res.height, res.width, res.pixels)
        res
        */
        wgpu.unwrap().render(rc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
