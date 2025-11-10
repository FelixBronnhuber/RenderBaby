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

    pub fn render(&self) -> Vec<u8> {
    let render_state = RenderState::new(
        Arc::new(GpuDevice::new()),
            Arc::new(Queue::new()),
            Buffer::new(),
            Buffer::new(),
            BindGroupLayout,
            bind_group,
            dimensions);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
