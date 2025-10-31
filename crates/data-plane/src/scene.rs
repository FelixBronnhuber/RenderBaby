pub mod scene {
    use crate::{action_stack::ActionStack, scene_geometry::{self, Vec3d}, scene_graph};

    pub struct Scene<'a> {
        // todo: new, Singleton, maybe with crate...
        scene_graph: scene_graph::scene_graph::SceneGraph<'a>,
        action_stack: ActionStack
    }
    impl <'a> Scene <'a> {
        pub fn render(&self) -> u8 {1} // todo type, call something...
        pub fn new(&self) -> Self {
            Scene{
                scene_graph: scene_graph::scene_graph::SceneGraph::new(),
                action_stack: action_stack::action_stack::ActionStack::new()}
        }
        pub fn init_beta_version(&self) {
            let center = Vec3d{x: 0.0,y: 0.0,z: 0.0};
            let geom = scene_geometry::SphereGeom::new(&center, 1.0);
            let sphere = scene_geometry::Sphere::new(geom, "00FF00".to_owned(), "Test Sphere".to_owned());
            self.scene_graph.add(sphere);
        }
    }
}