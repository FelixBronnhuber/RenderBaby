#![cfg(test)]
use glam::Vec3;
use scene_objects::{material::Material, sphere::Sphere};
use crate::data_plane::scene::render_scene::Scene;

//#[test]
#[allow(unused)]
fn basic_scene_test() {
    let mut scene = Scene::new();
    let s_count = 10;
    for i in 0..s_count {
        scene.add_sphere(Sphere::new(
            Vec3::default(),
            i as f32,
            Material::default(),
            [1.0, 0.0, 0.0],
        ));
    }
    assert_eq!(scene.get_spheres().len(), s_count);
}
