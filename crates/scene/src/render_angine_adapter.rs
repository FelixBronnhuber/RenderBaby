use crate::geometric_object::Sphere;

impl Sphere {
    fn to_render_engine_sphere(&self) -> engine_wgpu_wrapper::Sphere {
        let center = self.get_center();
        
        engine_wgpu_wrapper::Sphere::new(
            [center.x, center.y, center.x], self.get_radius(), [1.0, 1.0, 1.0])
            //center.as_slice?
    }
}