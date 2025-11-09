use crate::geometric_object::GeometricObject;

mod scene;
mod geometric_object;
mod obj_parser;
mod scene_graph;
mod action_stack;

pub fn test_dyn(obj: Box<dyn GeometricObject>) {
    let mut objects: Vec<Box<dyn GeometricObject>> = Vec::new();
    objects.push(obj);
}
#[cfg(test)]
mod tests {
    use glam::Vec3;

    use crate::{geometric_object::{GeometricObject, Material, Sphere}, scene::Scene};

    use super::*;
    use crate::obj_parser::parseobj;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
        let sphere = geometric_object::Sphere::new(Vec3::new(0.0,0.0,0.0), 1.0, Material{});
        let radius = 1.0;
        assert_eq!(sphere.get_radius(), radius);
        let mut scene = Scene::new();
        //let mut objects: Vec<Box<dyn GeometricObject>>;
        //objects.push(Box::new(sphere));
        scene.add_object(Box::new(sphere));
        let objects = scene.get_objects();
        //assert!(objects.pop())
        let obj = objects.get(0);
        let obj_unpacked = obj.unwrap();
        let obj_obj = obj_unpacked.as_ref();
        //obj_obj.something();
        
        if let Some(sphere2) = obj_obj.as_any().downcast_ref::<Sphere>() {
            assert_eq!(sphere2.get_radius(), radius);
            //sphere2.scale(3.0);
        }
    }
    #[test]
    fn test_proto_init() {
        let mut scene = Scene::new();
        scene.proto_init();
        assert_eq!(scene.get_light_sources().len(), 1)
    }

    #[test]
    fn parse(){
        parseobj();
    }
}
