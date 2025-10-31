pub mod scene_graph {
    use crate::scene_geometry::{self, SceneGeometry};

    pub struct SceneGraph<'a> {
        // elements pub as cheap solution for now...
        
        objects: &'a mut Vec<dyn SceneGeometry>,
        light_sources: &'a mut Vec<dyn SceneGeometry>,
        cameras: &'a mut Vec<dyn SceneGeometry>
    }

    impl<'a> SceneGraph<'a> {
        // probably add_obj, add_light...
        pub fn add(&self, obj: dyn scene_geometry::SceneGeometry) {
            self.objects.push(obj);
        }
        pub fn remove(&self) {}
        pub fn new(&self) -> Self {
            SceneGraph { objects: Vec::new(), light_sources: Vec::new(), cameras: Vec::new()}
        }
    }

}