use engine_wgpu_wrapper::{EngineType, RenderOutput, WgpuWrapper};

use crate::{geometric_object::Sphere, scene::Scene};
type RenderSphere = engine_wgpu_wrapper::Sphere;
impl Sphere {
    fn to_render_engine_sphere(&self) -> RenderSphere {
        let center = self.get_center();
        
        engine_wgpu_wrapper::Sphere::new(
            [center.x, center.y, center.x], self.get_radius(), self.get_color().map(|x| x as f32))
            //center.as_slice?
    }
}

impl Scene{
    fn get_render_spheres(&self) -> Vec<RenderSphere> {
        let mut res = vec![];
        for obj in self.get_objects() {
            if let Some(sphere) = obj.as_any().downcast_ref::<Sphere>() {
                res.push(sphere.to_render_engine_sphere());
            }
        }
        res
    }

    pub fn render(&self) -> RenderOutput { // todo: change return type to mask engine plane
        // todo: get from camera
        let width = 1920 / 2;
        let height = 1080 / 2; // todo: add camera size / fov
        //let render_spheres = self.get_render_spheres();
        let wgpu = WgpuWrapper::new(EngineType::Raytracer, width, height);
        let res = wgpu.unwrap().render();
        let res = res.unwrap(); // todo catch error...
        //res.pixels
        //(res.height, res.width, res.pixels)
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
