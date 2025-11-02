mod scene;
mod geometric_object;
mod scene_graph;

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::{geometric_object::{Material, Sphere}, scene::Scene};

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
        let sphere = Sphere::new(Vec3::new(0.0,0.0,0.0), 1.0, Material{});
        assert_eq!(sphere.get_radius(), 1.0);
        let scene = Scene::new();
    }
}
