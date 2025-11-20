mod geometric_object;
mod obj_parser;
mod scene;
mod scene_graph;

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::*;
    use crate::geometric_object::{GeometricObject, LightSource};
    use crate::obj_parser::parseobj;
    use crate::{
        geometric_object::{Material, Sphere},
        scene::Scene,
    };

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
        let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.0, Material::default());
        assert_eq!(sphere.get_radius(), 1.0);
        let scene = Scene::new();
    }

    #[test]
    fn parse() {
        let triGeometry = parseobj("C:/Users/fucjo/RustroverProjects/Teapot.obj".into());
        let tris = triGeometry.get_triangles();
        let mut iterator = tris.into_iter();
    }
}
