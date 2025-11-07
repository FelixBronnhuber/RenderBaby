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
        assert_eq!(sphere.get_radius(), 1.0);
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
        
    }

    #[test]
    fn parse(){
        parseobj();
    }
}
